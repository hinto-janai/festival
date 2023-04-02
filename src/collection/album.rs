//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use super::{
	Collection,
	Artist,
	Song,
};
use crate::key::{
	ArtistKey,
	SongKey,
};
use super::art::{
	Art,
	UNKNOWN_ALBUM,
	UNKNOWN_ALBUM_BYTES,
};

//---------------------------------------------------------------------------------------------------- Album
#[derive(Debug,Serialize,Deserialize)]
/// Struct holding [`Album`] metadata, with pointers to an [`Artist`] and [`Song`]\(s\)
///
/// This struct holds all the metadata about a particular [`Album`].
///
/// It contains an [`ArtistKey`] that is the index of the owning [`Artist`], in the [`Collection`].
///
/// It also contains [`SongKey`]\(s\) that are the indicies of [`Song`]\(s\) belonging to this [`Album`], in the [`Collection`].
pub struct Album {
	// User-facing data.
	/// Title of the [`Album`].
	pub title: String,
	/// Key to the [`Artist`].
	pub artist: ArtistKey,
	/// Human-readable release date of this [`Album`].
	pub release_human: String,
	/// Human-readable total runtime of this [`Album`].
	pub runtime_human: readable::Runtime,
	/// Human-readable [`Song`] count of this [`Album`].
	pub song_count_human: readable::Int,
	// This `Vec<SongKey>` is _always_ sorted based
	// off incrementing disc and track numbers, e.g:
	//
	// DISC 1:
	//   - 1. ...
	//   - 2. ...
	// DISC 2:
	//   - 1. ...
	//   - 2. ...
	//
	// So, doing `my_album.songs.iter()` will always
	// result in the correct `Song` order for `my_album`.
	/// Key\(s\) to the [`Song`]\(s\).
	pub songs: Vec<SongKey>,           //

	// "Raw" data.
	pub(crate) release: (Option<i32>, Option<u32>, Option<u32>),    // (Year, Month, Day)
	pub(crate) runtime: f64,    //
	pub(crate) song_count: usize, //

	// Art data.
	#[serde(skip)]
	// The [`Album`]'s art.
	pub(crate) art: Art,                          // Always initialized after `CCD`.
	pub(crate) art_bytes: Option<Vec<u8>>, //

	// Misc data.
	/// Boolean representing if this is a compilation or not.
	pub compilation: bool, //
}

impl Album {
	#[inline(always)]
	/// Return the [`Album`] art.
	///
	/// Some [`Album`]'s may not have art. In this case, we'd like to show a "unknown" image anyway.
	///
	/// This function will always return a valid [`egui_extras::RetainedImage`], either:
	/// 1. The real [`Album`] art (if it exists)
	/// 2. An "unknown" image
	///
	/// The returned "unknown" image is actually just a pointer to the single image created with [`lazy_static`].
	///
	/// The "unknown" image is from `assets/images/art/unknown.png`.
	pub fn art_or(&self) -> &egui_extras::RetainedImage {
		self.art.art_or()
	}
}

//impl Default for Album {
//	fn default() -> Self {
//		Self {
//			title: String::new(),
//			artist: ArtistKey::default(),
//			release_human: String::new(),
//			length_human: HumanTime::unknown(),
//			song_count_human: HumanNumber::new(),
//			songs: vec![],
//			release: (None, None, None),
//			length: 0.0,
//			song_count: 0,
//			disc_count: 0,
//			art: Art::Unknown,
//			art_bytes: None,
//			compilation: false,
//		}
//	}
//}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
