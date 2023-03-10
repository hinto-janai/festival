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
use crate::collection::Art;
use crossbeam_channel::{Sender,Receiver};
use std::path::{Path,PathBuf};
use std::sync::Arc;
use disk::Bincode;
use std::time::Instant;

//---------------------------------------------------------------------------------------------------- CCD
pub(crate) struct Ccd;

impl Ccd {
	#[inline(always)]
	//-------------------------------------------------------------------------------- CCD `convert_art()`
	// Public facing "front-end" function for image conversion.
	// Dynamically selects internal functions for single/multi-thread.
	pub(crate) fn convert_art(to_kernel: Sender<CcdToKernel>, collection: Collection) {
		debug!("CCD - Purpose in life: convert_art()");

		// If no albums, return.
		if collection.albums.len() == 0 {
			send!(to_kernel, CcdToKernel::NewCollection(collection));
		// Else, convert art, send to `Kernel`.
		} else {
			send!(to_kernel, CcdToKernel::NewCollection(Self::priv_convert_art(&to_kernel, collection)));
		}
	}

	#[inline(always)]
	//-------------------------------------------------------------------------------- CCD `new_collection()`
	// Public facing "front-end" function for making a new `Collection`.
	//
	// These operations are split up into different private
	// functions mostly for testing flexability.
	pub(crate) fn new_collection(
		to_kernel: Sender<CcdToKernel>,
		from_kernel: Receiver<KernelToCcd>,
		old_collection: Arc<Collection>,
		paths: Vec<PathBuf>,
	) {
		let beginning = Instant::now();
		debug!("CCD - Purpose in life: new_collection()");
		// `new_collection()` high-level overview:
		// 1. WalkDir given path(s), filtering for audio files.
		// 2. For each file, append metadata to appropriate `Vec`.
		// 3. Make sure `Vec<Album>` metadata matches the songs.
		// 4. Created sorted `Key`'s.
		// 5. Create our `Collection`.
		// 6. Save to disk.
		// 7. Transform in-memory `Collection` with `priv_convert_art()`
		// 8. Send to `Kernel`
		// 9. Wait for `Die` signal.
		// 10. Die, destruct the old `Collection`.

		// TODO: Handle potential errors:
		// 1. No albums
		// 2. Path error
		// 3. Permission error
		// 4. Disk error

		// TODO: Send updates to `Kernel` throughout and `log!()`.

		// 1.
		let now = Instant::now();
		let paths = Self::walkdir_audio(&to_kernel, paths);
		debug!("CCD [1/10] | WalkDir: {}", now.elapsed().as_secs_f32());

		// 2.
		let now = Instant::now();
		let (vec_artist, mut vec_album, vec_song) = Self::audio_paths_to_incomplete_vecs(&to_kernel, paths);
		debug!("CCD [2/10] | Metadata: {}", now.elapsed().as_secs_f32());

		// 3.
		let now = Instant::now();
		Self::fix_album_metadata_from_songs(&mut vec_album, &vec_song);
		debug!("CCD [3/10] | Fix: {}", now.elapsed().as_secs_f32());

		// 4.
		let now = Instant::now();
		let sort_artist_lexi                    = Self::sort_artist_lexi(&vec_artist);
		let sort_artist_album_count             = Self::sort_artist_album_count(&vec_artist);
		let sort_artist_song_count              = Self::sort_artist_song_count(&vec_artist, &vec_album);
		//--
		let sort_album_release_artist_lexi      = Self::sort_album_release_artist_lexi(&sort_artist_lexi, &vec_artist, &vec_album);
		let sort_album_lexi_artist_lexi         = Self::sort_album_lexi_artist_lexi(&sort_artist_lexi, &vec_artist, &vec_album);
		let sort_album_lexi                     = Self::sort_album_lexi(&vec_album);
		let sort_album_release                  = Self::sort_album_release(&vec_album);
		let sort_album_runtime                  = Self::sort_album_runtime(&vec_album);
		//--
		let sort_song_artist_lexi_album_release = Self::sort_song_iterating_over_albums(&sort_album_release_artist_lexi, &vec_artist, &vec_album);
		let sort_song_artist_lexi_album_lexi    = Self::sort_song_iterating_over_albums(&sort_album_lexi_artist_lexi, &vec_artist, &vec_album);
		let sort_song_lexi                      = Self::sort_song_lexi(&vec_song);
		let sort_song_release                   = Self::sort_song_iterating_over_albums(&sort_album_release, &vec_artist, &vec_album);
		let sort_song_runtime                   = Self::sort_song_runtime(&vec_song);
		debug!("CCD [4/10] | Sort: {}", now.elapsed().as_secs_f32());

		// 5.
		let now = Instant::now();
		let collection = Collection {
			empty: false,
			timestamp: Collection::timestamp_now(),
			count_artist: vec_artist.len(),
			count_album: vec_album.len(),
			count_song: vec_song.len(),

			artists: vec_artist,
			albums: vec_album,
			songs: vec_song,

			sort_artist_lexi,
			sort_artist_album_count,
			sort_artist_song_count,

			sort_album_release_artist_lexi,
			sort_album_lexi_artist_lexi,
			sort_album_lexi,
			sort_album_release,
			sort_album_runtime,

			sort_song_artist_lexi_album_release,
			sort_song_artist_lexi_album_lexi,
			sort_song_lexi,
			sort_song_release,
			sort_song_runtime,
		};
		debug!("CCD [5/10] | Collection: {}", now.elapsed().as_secs_f32());

		// 6.
		// TODO:
		// Consider moving this to the end so the user
		// doesn't have to wait for this write.
//		let now = Instant::now();
//		if let Err(e) = collection.write_atomic() {
//			send!(to_kernel, CcdToKernel::Failed(e));
//			debug!("CCD ... Collection failed, bye!");
//			return
//		}
//		debug!("CCD [6/10] | Disk: {}", now.elapsed().as_secs_f32());

		// 7.
		let now = Instant::now();
		let collection = Self::priv_convert_art(&to_kernel, collection);
		debug!("CCD [7/10] | Image: {}", now.elapsed().as_secs_f32());

		// 8.
		let now = Instant::now();
		send!(to_kernel, CcdToKernel::NewCollection(collection));
		debug!("CCD [8/10] | ToKernel: {}", now.elapsed().as_secs_f32());

		// 9.
		let now = Instant::now();
		match recv!(from_kernel) {
			KernelToCcd::Die => debug!("CCD [9/10] | Die: {}", now.elapsed().as_secs_f32()),
		}

		// 10.
		if Arc::strong_count(&old_collection) == 1 {
			let now = Instant::now();
			drop(old_collection);
			debug!("CCD [10/10] | Deconstruct: {}", now.elapsed().as_secs_f32());
		} else {
			error!("CCD [10/10] | Someone else is pointing to the old Collection...! I can't deconstruct it noooooooooo~");
		}

		// Thank you CCD, you can rest now.
		debug!("CCD ... Took {} seconds, bye!", beginning.elapsed().as_secs_f32());
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use crate::ccd::*;
	use std::path::PathBuf;
	use disk::Bincode;

	#[test]
	#[ignore]
	fn serialize_and_convert_collection() {
		// Set-up logger.
		crate::logger::init_logger(log::LevelFilter::Trace);

		// Set-up inputs.
		let (to_kernel, from_ccd) = crossbeam_channel::unbounded::<CcdToKernel>();

		// Serialize.
		let now = Instant::now();
		let collection = Collection::from_file().unwrap();
		info!("Read: {}", now.elapsed().as_secs_f32());

		// Convert.
		let now = Instant::now();
		let collection = Ccd::convert_art(to_kernel, collection);
		info!("Convert: {}", now.elapsed().as_secs_f32());
	}

	#[test]
	#[ignore]
	fn new_collection() {
		// Set-up logger.
		crate::logger::init_logger(log::LevelFilter::Trace);

		// Set-up inputs.
		let (to_kernel, from_ccd) = crossbeam_channel::unbounded::<CcdToKernel>();
		let (to_ccd, from_kernel) = crossbeam_channel::unbounded::<KernelToCcd>();
		let old_collection = Arc::new(Collection::new());
		let paths = vec![
			PathBuf::from("src"),
			PathBuf::from("assets"),
			PathBuf::from("assets"),
			PathBuf::from("assets/audio"),
			PathBuf::from("assets/images"),
			PathBuf::from("assets/albums"),
		];

		// Spawn `CCD`.
		let old_clone = Arc::clone(&old_collection);
		std::thread::spawn(move || Ccd::new_collection(to_kernel, from_kernel, old_clone, paths));

		// Act as `Kernel`.
		// Receive.
		let collection = loop {
			match recv!(from_ccd) {
				CcdToKernel::NewCollection(collection) => break collection,
				CcdToKernel::Failed(error)             => panic!("{}", error),
				CcdToKernel::Update(string)            => eprintln!("{}", string),
			}
		};

		// Send `Die` signal.
		drop(old_collection);
		send!(to_ccd, KernelToCcd::Die);

		crate::macros::sleep!(5000);
		info!("{}", collection.empty);
		info!("{}", collection.timestamp);
		info!("{}", collection.count_artist);
		info!("{}", collection.count_album);
		info!("{}", collection.count_song);

		assert!(collection.empty == false);
		assert!(collection.timestamp > 1678382892);
		assert!(collection.count_artist == 1);
		assert!(collection.count_album  == 1);
		assert!(collection.count_song   == 506);
	}
}

