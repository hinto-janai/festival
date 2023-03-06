//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use crate::macros::*;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use human::{HumanTime,HumanNumber};
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
#[derive(Serialize,Deserialize)]
pub struct Album {
	// User-facing data.
	pub title: String,                 //
	pub artist: ArtistKey,             //
	pub release_human: String,         //
	pub length_human: HumanTime,       //
	pub song_count_human: HumanNumber, //
	pub songs: Vec<SongKey>,           //

	// "Raw" data.
	pub(crate) release: u64,    // UNIX?
	pub(crate) length: f64,     //
	pub(crate) song_count: u32, //
	pub(crate) disk_count: u32, //

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

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
