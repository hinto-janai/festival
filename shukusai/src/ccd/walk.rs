//---------------------------------------------------------------------------------------------------- Use
use crate::ccd::mime::{SUPPORTED_AUDIO_MIME_TYPES, SUPPORTED_IMG_MIME_TYPES};
use crate::ccd::msg::CcdToKernel;
use benri::log::{ok_trace, skip_warn};
use crossbeam::channel::Sender;
use log::trace;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

//---------------------------------------------------------------------------------------------------- __NAME__
impl super::Ccd {
    // `WalkDir` given PATHs and filter for audio files.
    // Ignore non-existing PATHs in the array.
    //
    // (PATH, MIME, file_extension)
    pub(crate) fn walkdir_audio(
        mut paths: Vec<PathBuf>,
    ) -> Vec<(PathBuf, &'static str, &'static str)> {
        // Test PATHs, collect valid ones.
        // Sort, remove duplicates.
        paths.retain(|p| p.exists() && p.is_absolute());
        paths.par_sort();
        paths.dedup();

        // Create our `WalkDir` entries.
        // This showcases some iterator black magic.
        //
        // Feeds `PathBuf`'s into that closure, flattening
        // all the iterators, and only collecting valid paths.
        let mut entries: Vec<(PathBuf, &'static str, &'static str)> = paths
            .into_par_iter()
            .flat_map_iter(|p| WalkDir::new(p).follow_links(true))
            .filter_map(Result::ok)
            .map(walkdir::DirEntry::into_path)
            .filter_map(Self::path_is_audio)
            .collect();

        entries.par_sort();
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
        let warn = || skip_warn!("Find Image: {}", path.display());
        let ok = || ok_trace!("Find Image: {}", path.display());

        let parent = match path.parent() {
            Some(p) => p,
            None => {
                warn();
                return None;
            }
        };

        for path in WalkDir::new(parent).max_depth(2).follow_links(true) {
            let path = match path {
                Ok(p) => p,
                _ => continue,
            };

            if Self::path_infer_img(path.path()) {
                let file = match std::fs::File::open(path.path()) {
                    Ok(f) => f,
                    _ => {
                        warn();
                        return None;
                    }
                };

                // SAFETY:
                // Attempt to copy bytes with `mmap`.
                let mmap = match unsafe { memmap2::Mmap::map(&file) } {
                    Ok(f) => f,
                    _ => {
                        warn();
                        return None;
                    }
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
            return SUPPORTED_IMG_MIME_TYPES.contains(&mime.mime_type());
        }

        if let Some(mime) = mime_guess::MimeGuess::from_path(path).first_raw() {
            return SUPPORTED_IMG_MIME_TYPES.contains(&mime);
        }

        false
    }

    #[inline(always)]
    fn path_is_audio(path: PathBuf) -> Option<(PathBuf, &'static str, &'static str)> {
        trace!("CCD - Walking PATH: {}", path.display());

        // Attempt MIME via file magic bytes first.
        if let Ok(Some(mime)) = infer::get_from_path(&path) {
            if SUPPORTED_AUDIO_MIME_TYPES.contains(&mime.mime_type()) {
                return Some((path, mime.mime_type(), mime.extension()));
            }
        }

        trace!("CCD - Skipping non-audio PATH: {}", path.display());
        None
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ccd::Ccd;
    use std::path::PathBuf;

    // Tests in `the_loop.rs` preclude the need for some tests here.

    #[test]
    // Assert the MIME img types
    // can be detected correctly.
    fn __path_infer_img() {
        for ext in ["jpg", "png", "bmp", "ico", "tiff", "webp"] {
            println!("{ext}");
            assert!(Ccd::path_infer_img(&PathBuf::from(format!(
                "../assets/images/test/512.{ext}"
            ))));
        }
    }

    #[test]
    // Asserts `maybe_find_img()` can find an image.
    fn __maybe_find_img() {
        assert!(
            !Ccd::maybe_find_img(&PathBuf::from("../assets/images/test/"))
                .unwrap()
                .is_empty()
        );
    }
}
