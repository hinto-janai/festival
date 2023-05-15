//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use super::{
	Tab,
};
use std::path::PathBuf;
use crate::constants::{
	SETTINGS_VERSION,
	ALBUM_ART_SIZE_MIN,
	ALBUMS_PER_ROW_MIN,
	ALBUM_ART_SIZE_MAX,
	ALBUMS_PER_ROW_MAX,
	ALBUM_ART_SIZE_DEFAULT,
	ALBUMS_PER_ROW_DEFAULT,
	ACCENT_COLOR,
	VISUALS,
};
use shukusai::{
	FESTIVAL,
	FESTIVAL_HEADER,
};
use shukusai::collection::{
	Collection,
};
use shukusai::sort::{
	ArtistSort,
	AlbumSort,
};
use super::AlbumSizing;

//---------------------------------------------------------------------------------------------------- Settings
disk::bincode2!(Settings, disk::Dir::Data, FESTIVAL, "gui", "settings", FESTIVAL_HEADER, SETTINGS_VERSION);
//toml_file!(Settings, Dir::Data, FESTIVAL, "gui", "settings");
#[derive(Clone,Debug,Default,PartialEq,Serialize,Deserialize,Encode,Decode,garde::Validate)]
/// `GUI`'s settings.
///
/// Holds user-mutable `GUI` settings, e.g:
/// - Accent color
/// - Album art size
/// - etc
pub struct Settings {
	#[garde(skip)]
	/// Collection sorting of album view.
	pub album_sort_order: AlbumSort,

	#[garde(skip)]
	/// Collection sorting of artist view.
	pub artist_sort_order: ArtistSort,

	#[garde(skip)]
	/// Does the user want a certain amount of
	//// `Album`'s per row or a static pixel size?
	pub album_sizing: AlbumSizing,
//	#[garde(ALBUM_ART_SIZE_MIN, max=ALBUM_ART_SIZE_MAX))]
	#[garde(skip)]
	pub album_pixel_size: f32,
	#[garde(range(min=ALBUMS_PER_ROW_MIN, max=ALBUMS_PER_ROW_MAX))]
	pub albums_per_row: u8,

	#[garde(skip)]
	/// Restore playback on re-open.
	pub restore_state: bool,

	#[garde(skip)]
	#[bincode(with_serde)]
	/// Our accent color.
	pub accent_color: egui::Color32,

	#[garde(length(min=0, max=10))]
	/// List of [`PathBuf`]'s to source music
	/// data from when making a new [`Collection`].
	pub collection_paths: Vec<PathBuf>,
}

impl Settings {
//	/// Returns the accent color in [`Settings`] in tuple form.
//	pub const fn accent_color(&self) -> (u8, u8, u8) {
//		let (r, g, b, _) = self.visuals.selection.bg_fill.to_tuple();
//		(r, g, b)
//	}

	pub fn new() -> Self {
		Self {
			accent_color: ACCENT_COLOR,
			restore_state: true,
			collection_paths: vec![],
			album_pixel_size: ALBUM_ART_SIZE_DEFAULT,
			albums_per_row: ALBUMS_PER_ROW_DEFAULT,
			..Default::default()
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod test {
//  #[test]
//  fn _() {
//  }
//}
