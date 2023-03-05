//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use crate::macros::{
	ok_debug,
	recv,
	send,
};
use crate::collection::{
	Album,
	Collection,
	CollectionKeychain,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use super::msg::{
	CcdToKernel,
	KernelToCcd,
};
use crate::collection::{
	Art,
	UNKNOWN_ALBUM_BYTES,
};
use super::SUPPORTED_AUDIO_MIME_TYPES;
use crossbeam_channel::{Sender,Receiver};
use std::sync::{Arc,Mutex};
use std::path::{Path,PathBuf};
use walkdir::WalkDir;

// TODO:
// - Document code
// - Send `Kernel` messages
// - Log


//---------------------------------------------------------------------------------------------------- CCD
pub struct Ccd;

//---------------------------------------------------------------------------------------------------- CCD `convert_art()`
impl Ccd {
	#[inline(always)]
	// Public facing "front-end" function for image conversion.
	// Dynamically selects internal functions for single/multi-thread.
	pub fn convert_art(to_kernel: Sender<CcdToKernel>, collection: Collection) {
		ok_debug!("CCD - Purpose in life: convert_art()");

		// If no albums, return.
		if collection.albums.len() == 0 {
			send!(to_kernel, CcdToKernel::NewCollection(collection));
		// Else, convert art, send to `Kernel`.
		} else {
			send!(to_kernel, CcdToKernel::NewCollection(Self::priv_convert_art(&to_kernel, collection)));
		}
	}

	#[inline(always)]
	// Internal re-usable image conversion function.
	// This can be used in `new_collection()` as well.
	//
	// Order of operations:
	//     1. If multiple threads are detected -> `convert_art_multithread()`
	//     2. If single thread is detected     -> `convert_art_singlethread()`
	//     3. Return `Collection` after mutation.
	//
	fn priv_convert_art(to_kernel: &Sender<CcdToKernel>, collection: Collection) -> Collection {
		// How many threads should we use?
		let threads = super::threads_for_albums(collection.albums.len());

		// Single-threaded.
		if threads == 1 {
			Self::convert_art_singlethread(&to_kernel, collection)
		// Multi-threaded.
		} else {
			Self::convert_art_multithread(&to_kernel, collection, threads)
		}
	}

	#[inline(always)]
	// Multi-threaded `convert_art()` variant.
	fn convert_art_multithread(to_kernel: &Sender<CcdToKernel>, mut collection: Collection, threads: usize) -> Collection {
		// Multi-thread & scoped process of `Collection` album art:
		//
		// 1. Split the `Vec` of `Album` across an appropriate amount of threads
		// 2. Move work into each scoped thread
		// 3. Process data.
		// 4. Join threads, return.
		std::thread::scope(|scope| {
			// Divide albums (mostly) evenly across threads.
			for albums in collection.albums.chunks_mut(threads) {

				// Clone `Kernel` channel.
				let to_k  = to_kernel.clone();

				// Spawn scoped thread with chunked workload.
				scope.spawn(move || {
					Self::convert_art_worker(to_kernel, albums);
				});
			}
		});

		collection
	}

	#[inline(always)]
	// Single-threaded `convert_art()` variant.
	fn convert_art_singlethread(to_kernel: &Sender<CcdToKernel>, mut collection: Collection) -> Collection {
		Self::convert_art_worker(to_kernel, &mut collection.albums);

		collection
	}

	#[inline(always)]
	// The actual art conversion "processing" work.
	fn convert_art_worker(to_kernel: &Sender<CcdToKernel>, albums: &mut [Album]) {
		for album in albums {
			// Take raw image bytes.
			let bytes = album.art_bytes.take();

			// If bytes exist, convert, else provide the `Unknown` art.
			let art = match bytes {
				Some(b) => Art::Known(super::art_from_known(&b)),
				None    => Art::Unknown,
			};

			// Insert the `Art`.
			album.art = art;

			// TODO: send progress report
			// send!(to_kernel, ...);
		}
	}


//---------------------------------------------------------------------------------------------------- CCD `new_collection()`
	#[inline(always)]
	// Public facing "front-end" function for making a new `Collection`.
	//
	// These operations are split up into different private
	// functions mostly for testing flexability.
	pub fn new_collection<P>(
		to_kernel: Sender<CcdToKernel>,
		from_kernel: Receiver<KernelToCcd>,
		paths: &[&P],
	) where
		P: AsRef<Path>
	{
		ok_debug!("CCD - Purpose in life: new_collection()");
		// TODO: new_collection() steps:
		// 1. WalkDir given path(s).
		// 2. Filter for audio files.
		// 3. For each file, get metadata, add to `Collection`.
		// 4.
		//     a) If image metadata exists, append
		//     b) If not, search parent dir for `jpeg/png`
		//     c) Given multiple images, pick the highest quality image
		//     d) Given no image, append `None`
		//
		// 5. Save to disk.
		// 6. Transform in-memory `Collection` with `priv_convert_art()`
		// 7. Send to `Kernel`
		// 8. Wait for `Die` signal.
		// 9. Die, destruct the old `Collection`.

		// TODO: Handle potential errors:
		// 1. No albums
		// 2. Path error
		// 3. Permission error
		// 4. Disk error

		// TODO: Send updates to `Kernel` throughout and `log!()`.
	}

	#[inline(always)]
	// 1. `WalkDir` given PATHs and filter for audio files.
	// Ignore non-existing PATHs in the array.
	fn walkdir_audio(
		to_kernel: &Sender<CcdToKernel>,
		paths: &Vec<PathBuf>,
	) -> Vec<PathBuf> {

		// Test PATHs, collect valid ones.
		let mut vec: Vec<PathBuf> = Vec::with_capacity(paths.len());
		for path in paths {
			if let Ok(true) = path.try_exists() {
				vec.push(path.to_path_buf());
			} else {
				// TODO: log ignored path
			}
		}

		// Sort, remove duplicates.
		vec.sort();
		vec.dedup();

		// Create our result `Vec`.
		let mut result: Vec<PathBuf> = Vec::with_capacity(paths.len());

		for path in paths {
			for entry in WalkDir::new(path).follow_links(true) {
				// Handle potential PATH error.
				let entry = match entry {
					Ok(e)    => e,
					Err(err) => continue, // TODO: log error.
				};

				// To `PathBuf`.
				let path_buf = entry.into_path();

				// Push to result if MIME type was audio.
				if Self::path_is_audio(&path_buf) {
					result.push(path_buf);
				} else {
					// TODO: log
				}
			}
		}

		result.sort();
		result.dedup();
		result
	}

	#[inline(always)]
	fn path_is_audio(path: &Path) -> bool {
		// Attempt MIME via file magic bytes first.
		if Self::path_infer_audio(path) {
			return true
		}

		// Attempt guess via file extension.
		if Self::path_guess_audio(path) {
			return true
		}

		false
	}

	#[inline(always)]
	fn path_infer_audio(path: &Path) -> bool {
		if let Ok(Some(mime)) = infer::get_from_path(&path) {
			return SUPPORTED_AUDIO_MIME_TYPES.contains(&mime.mime_type())
		}

		false
	}

	#[inline(always)]
	fn path_guess_audio(path: &Path) -> bool {
		if let Some(mime) = mime_guess::MimeGuess::from_path(&path).first_raw() {
			return SUPPORTED_AUDIO_MIME_TYPES.contains(&mime)
		}

		false
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use std::path::PathBuf;
	use super::*;

	#[test]
	fn _path_is_audio() {
		let path = ["aac", "m4a", "flac", "mp3", "ogg", "wav", "aiff"];
		for i in path {
			let file = PathBuf::from(format!("assets/audio/rain.{}", i));
			eprintln!("{}", file.display());
			assert!(Ccd::path_infer_audio(&file));
			assert!(Ccd::path_guess_audio(&file));
		}
	}

	#[test]
	fn _walkdir_audio() {
		// Set-up PATHs.
		let (to_kernel, _) = crossbeam_channel::unbounded::<CcdToKernel>();
		let paths = vec![
			PathBuf::from("src"),
			PathBuf::from("assets"),
			PathBuf::from("assets"),
			PathBuf::from("assets/audio"),
			PathBuf::from("assets/images"),
		];

		// WalkDir and filter for audio.
		let result = Ccd::walkdir_audio(&to_kernel, &paths);
		eprintln!("{:#?}", result);

		// Assert.
		assert!(result[0].display().to_string() == "assets/audio/rain.aac");
		assert!(result[1].display().to_string() == "assets/audio/rain.aiff");
		assert!(result[2].display().to_string() == "assets/audio/rain.flac");
		assert!(result[3].display().to_string() == "assets/audio/rain.m4a");
		assert!(result[4].display().to_string() == "assets/audio/rain.mp3");
		assert!(result[5].display().to_string() == "assets/audio/rain.ogg");
		assert!(result[6].display().to_string() == "assets/audio/rain.wav");
		assert!(result.len() == 7);
	}
}
