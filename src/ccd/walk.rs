//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
use benri::log::{
	skip_warn,ok_trace,
};
use super::msg::{
	CcdToKernel,
	KernelToCcd,
};
use walkdir::WalkDir;
use crossbeam::channel::{Sender,Receiver};
use std::path::{Path,PathBuf};
use crate::ccd::mime::{
	SUPPORTED_AUDIO_MIME_TYPES,
	SUPPORTED_IMG_MIME_TYPES,
};

//---------------------------------------------------------------------------------------------------- __NAME__
impl super::Ccd {
	// TODO:
	// Handle no PATHs.
	//
	// `WalkDir` given PATHs and filter for audio files.
	// Ignore non-existing PATHs in the array.
	pub(super) fn walkdir_audio(
		to_kernel: &Sender<CcdToKernel>,
		mut paths: Vec<PathBuf>,
	) -> Vec<PathBuf> {

		// Test PATHs, collect valid ones.
		// Sort, remove duplicates.
		paths.retain(|p| p.exists());
		paths.sort();
		paths.dedup();

		// Create our `WalkDir` entries.
		// This showcases some iterator black magic.
		//
		// Feeds `PathBuf`'s into that closure, flattening
		// all the iterators, and only collecting valid paths.
		let mut entries: Vec<PathBuf> = paths
			.into_iter()
			.flat_map(|p| WalkDir::new(p).follow_links(true))
			.filter_map(Result::ok)
			.map(walkdir::DirEntry::into_path)
			.filter_map(Self::path_is_audio)
			.collect();

		entries.sort();
		entries.dedup();
		entries

		//--- The old `for` loop version is below.
//		let len       = entries.len();
//		let increment = 5.0 / len as f64;
//
//		// Create our result `Vec`.
//		let mut result: Vec<PathBuf> = Vec::with_capacity(len);
//
//		for entry in entries.iter_mut() {
//			// To `PathBuf`.
//			let path = entry.into_path();
//			trace!("CCD - Walking PATH: {}", path.display());
//
//			// Push to result if MIME type was audio.
//			if Self::path_is_audio(&path) {
//				result.push(path);
//			} else {
//				debug!("CCD - Skipping non-audio PATH: {}", path.display());
//			}
//		}
//
//		result.sort();
//		result.dedup();
//		result
	}

	#[inline(always)]
	// Attempts to find a `.jpg/.png` file in the
	// parent directory of an audio file's PATH
	// and copies it into a `Vec`.
	pub(super) fn maybe_find_img(path: &Path) -> Option<Vec<u8>> {
		let warn = || skip_warn!("CCD - Find Image: {}", path.display());
		let ok   = || ok_trace!("CCD - Find Image: {}", path.display());

		let parent = match path.parent() {
			Some(p) => p,
			None    => { warn(); return None; },
		};

		for path in WalkDir::new(parent)
			.max_depth(2)
			.follow_links(true)
		{
			let path = match path {
				Ok(p) => p,
				_     => continue,
			};

			if Self::path_infer_img(path.path()) {

				let file = match std::fs::File::open(path.path()) {
					Ok(f) => f,
					_     => { warn(); return None; },
				};

				// SAFETY:
				// Attempt to copy bytes with `mmap`.
				let mmap = match unsafe { memmap2::Mmap::map(&file) } {
					Ok(f) => f,
					_     => { warn(); return None; },
				};

				ok();
				return Some(mmap.to_vec());
			}
		}

		warn();
		None
	}

	#[inline(always)]
	fn path_infer_img(path: &Path) -> bool {
		if let Ok(Some(mime)) = infer::get_from_path(path) {
			return SUPPORTED_IMG_MIME_TYPES.contains(&mime.mime_type())
		}

		if let Some(mime) = mime_guess::MimeGuess::from_path(path).first_raw() {
			return SUPPORTED_IMG_MIME_TYPES.contains(&mime)
		}

		false
	}

	#[inline(always)]
	fn path_is_audio(path: PathBuf) -> Option<PathBuf> {
		trace!("CCD - Walking PATH: {}", path.display());

		// Attempt MIME via file magic bytes first.
		if Self::path_infer_audio(&path) {
			return Some(path)
		}

		debug!("CCD - Skipping non-audio PATH: {}", path.display());
		None
	}

	#[inline(always)]
	fn path_infer_audio(path: &Path) -> bool {
		if let Ok(Some(mime)) = infer::get_from_path(path) {
			SUPPORTED_AUDIO_MIME_TYPES.contains(&mime.mime_type())
		} else if let Some(mime) = mime_guess::MimeGuess::from_path(path).first_raw() {
			SUPPORTED_AUDIO_MIME_TYPES.contains(&mime)
		} else {
			false
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use std::path::PathBuf;
	use crate::ccd::Ccd;
	use super::*;

	#[test]
	// Makes sure the MIME img types
	// can be detected correctly.
	fn _path_is_img() {
		assert!(Ccd::path_infer_img(&PathBuf::from("assets/images/art/unknown.png")));
	}


	#[test]
	// Makes sure the MIME audio types
	// can be detected correctly.
	fn _path_is_audio() {
		let path = ["aac", "m4a", "flac", "mp3", "ogg", "wav", "aiff"];
		for i in path {
			let file = PathBuf::from(format!("assets/audio/rain.{}", i));
			eprintln!("{}", file.display());
			assert!(Ccd::path_infer_audio(&file));
		}
	}

	#[test]
	#[cfg(unix)]
	// Makes sure the `WalkDir` function is correctly:
	// 1. Finding PATHs
	// 2. Filtering for audio MIMEs
	// 3. Remove duplicates
	fn _walkdir_audio() {
		// Set-up PATHs.
		let (to_kernel, _) = crossbeam::channel::unbounded::<CcdToKernel>();
		let paths = vec![
			PathBuf::from("src"),
			PathBuf::from("assets/audio"),
			PathBuf::from("assets/images"),
			PathBuf::from("assets/audio"),
			PathBuf::from("assets/images"),
		];

		// WalkDir and filter for audio.
		let result = Ccd::walkdir_audio(&to_kernel, paths);
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
