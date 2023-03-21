//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use crate::macros::*;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use super::{
	ArtistKey,
	SongKey,
};
use super::art::{
	Art,
	UNKNOWN_ALBUM,
	UNKNOWN_ALBUM_BYTES,
};

//---------------------------------------------------------------------------------------------------- Album
//#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[derive(Debug,Serialize,Deserialize)]
pub struct Album {
	// User-facing data.
	pub title: String,                    //
	pub artist: ArtistKey,                //
	pub release_human: String,            //
	pub runtime_human: readable::Runtime, //
	pub song_count_human: readable::Int,  //
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
	pub songs: Vec<SongKey>,           //

	// "Raw" data.
	pub(crate) release: (Option<i32>, Option<u32>, Option<u32>),    // (Year, Month, Day)
	pub(crate) runtime: f64,    //
	pub(crate) song_count: usize, //

	// Art data.
	#[serde(skip)]
	pub art: Art,                          // Always initialized after `CCD`.
	pub(crate) art_bytes: Option<Vec<u8>>, //

	// Misc data.
	pub compilation: bool, //
}

impl Album {
	#[inline]
	// Return the associated art or the default `[?]` image if `Unknown`
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
