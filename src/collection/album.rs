//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use crate::macros::*;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use human::{HumanTime,HumanNumber};
use egui_extras::image::RetainedImage;
use super::{
	ArtistKey,
	SongKey,
};

//----------------------------------------------------------------------------------------------------
lazy_static::lazy_static! {
	pub static ref UNKNOWN_ALBUM: RetainedImage =
		RetainedImage::from_image_bytes(
			"Unknown",
			include_bytes!("../../assets/images/art/unknown.png")
		).expect("Default album image failed to load");

	pub static ref UNKNOWN_ALBUM_BYTES: &'static [u8] = include_bytes!("../../assets/images/art/unknown.png");
}

//----------------------------------------------------------------------------------------------------
//#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
//bincode_file!(Album, Dir::Data, "Festival", "", "album");
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
	pub release: u64,    // UNIX?
	pub lenght: f64,     //
	pub song_count: u32, //

	// Art data.
	#[serde(skip)]
	pub art: Option<RetainedImage>, //
	pub art_bytes: Option<Vec<u8>>, //

	// Misc data.
	pub compilation: bool, //
}

//impl Album {
//	#[inline]
//	// Return the associated art or the default `[?]` image if `None`.
//	pub fn art_or_default(&self) -> &RetainedImage {
//		match &self.art {
//			Some(art) => art,
//			None      => &*UNKNOWN_ALBUM,
//		}
//	}
//}

//pub struct AlbumArt {
//	pub exists: bool,
//	pub img: Option<RetainedImage>,
//}
//
//impl AlbumArt {
//	pub fn new() -> Self {
//		Self {
//			exists: false,
//			img: RetainedImage::from_image_bytes("Unknown", UNKNOWN_ALBUM).expect("Default album image failed"),
//		}
//	}
//}
//
//impl std::default::Default for AlbumArt {
//	fn default() -> Self {
//		Self::new()
//	}
//}
//
//impl std::fmt::Display for Img {
//	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//		write!(f, __DISPLAY__)
//	}
//}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
