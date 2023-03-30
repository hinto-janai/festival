//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use crate::ui::{
	Tab,
};
use std::path::PathBuf;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use crate::constants::{
	SETTINGS_VERSION,
	ALBUM_ART_DEFAULT_SIZE,
};
use shukusai::{
	FESTIVAL,
	FESTIVAL_HEADER,
};
use shukusai::collection::{
	Collection,
};
use shukusai::sort::{
	AlbumSort,
};

//---------------------------------------------------------------------------------------------------- Settings
/// `GUI`'s settings.
///
/// Holds user-mutable `GUI` settings, e.g:
/// - Accent color
/// - Album art size
/// - etc
bincode_file!(Settings, Dir::Data, FESTIVAL, "gui", "settings", FESTIVAL_HEADER, SETTINGS_VERSION);
#[derive(Clone,Debug,Default,PartialEq,PartialOrd,Serialize,Deserialize)]
pub struct Settings {
	/// RGB (A is always added later as 255, no opacity).
	pub accent_color: [u8; 3],

	/// Restore playback on re-open.
	pub restore_state: bool,

	/// Collection sorting of album view.
	pub sort_order: AlbumSort,

	/// Static pixel width/height for each album cover.
	pub album_art_size: f32,

	/// List of [`PathBuf`]'s to source music
	/// data from when making a new [`Collection`].
	pub collection_paths: Vec<PathBuf>,
}

impl Settings {
	pub fn new() -> Self {
		Self {
			accent_color: [200, 100, 100], // Pinkish red
			restore_state: true,
			album_art_size: ALBUM_ART_DEFAULT_SIZE,
			collection_paths: vec![],
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
