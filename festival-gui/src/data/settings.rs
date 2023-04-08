//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use crate::ui::{
	Tab,
};
use std::path::PathBuf;
use disk::prelude::*;
use disk::{Toml,toml_file};
use crate::constants::{
	SETTINGS_VERSION,
	ALBUM_ART_DEFAULT_SIZE,
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
	AlbumSort,
};

//---------------------------------------------------------------------------------------------------- Settings
//bincode_file!(Settings, Dir::Data, FESTIVAL, "gui", "settings", FESTIVAL_HEADER, SETTINGS_VERSION);
toml_file!(Settings, Dir::Data, FESTIVAL, "gui", "settings");
#[derive(Clone,Debug,Default,PartialEq,Serialize,Deserialize)]
/// `GUI`'s settings.
///
/// Holds user-mutable `GUI` settings, e.g:
/// - Accent color
/// - Album art size
/// - etc
pub struct Settings {
	/// Collection sorting of album view.
	pub sort_order: AlbumSort,

	/// Static pixel width/height for each album cover.
	pub album_art_size: f32,

	/// Restore playback on re-open.
	pub restore_state: bool,

	/// Our accent color.
	pub accent_color: egui::Color32,

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
