//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};

use std::path::PathBuf;
use crate::constants::{
	GUI,
	ALBUM_ART_SIZE_DEFAULT,
	ALBUMS_PER_ROW_DEFAULT,
	ACCENT_COLOR,
	SETTINGS_VERSION,
	PIXELS_PER_POINT_DEFAULT,
};
use shukusai::{
	constants::{
		HEADER,
		FESTIVAL,
		STATE_SUB_DIR,
	},
	sort::{
		ArtistSort,
		AlbumSort,
		SongSort,
	},
	audio::PREVIOUS_THRESHOLD_DEFAULT,
	search::SearchKind,
};
use crate::data::{
	AlbumSizing,
	SearchSort,
	ArtistSubTab,
	WindowTitle,
};
use const_format::formatcp;
use std::marker::PhantomData;

//---------------------------------------------------------------------------------------------------- Settings
disk::bincode2!(Settings, disk::Dir::Data, FESTIVAL, formatcp!("{GUI}/{STATE_SUB_DIR}"), "settings", HEADER, SETTINGS_VERSION);
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,Encode,Decode)]
/// `GUI`'s settings.
///
/// Holds user-mutable `GUI` settings, e.g:
/// - Accent color
/// - Album art size
/// - etc
pub struct Settings {
	/// Collection sorting of artist view.
	pub artist_sort: ArtistSort,

	/// Collection sorting of album view.
	pub album_sort: AlbumSort,

	/// Collection sorting of album view.
	pub song_sort: SongSort,

	/// Which search kind to use for `Kernel`
	pub search_kind: SearchKind,

	/// Which `ArtistSubTab` are we on?
	pub artist_sub_tab: ArtistSubTab,

	/// To sort by `Song` title or
	/// `Artist` name in the search tab?
	pub search_sort: SearchSort,

	/// Which way to set the window title when changing songs.
	pub window_title: WindowTitle,

	/// Does the user want a certain amount of
	/// `Album`'s per row or a static pixel size?
	pub album_sizing: AlbumSizing,
	pub album_pixel_size: f32,
	pub albums_per_row: u8,

	/// How many seconds does a song need to play
	/// before the `Previous` button resets the current
	/// instead of going to the previous?
	pub previous_threshold: u32,

	/// Restore playback on re-open.
	pub restore_state: bool,

	/// Start playback if we added stuff to an empty queue.
	pub empty_autoplay: bool,

	#[bincode(with_serde)]
	/// Our accent color.
	pub accent_color: egui::Color32,

	/// List of [`PathBuf`]'s to source music
	/// data from when making a new [`Collection`].
	pub collection_paths: Vec<PathBuf>,

	/// What `egui::Context::pixels_per_point` are we set to?
	/// Default is `1.0`, this allows the user to scale manually.
	pub pixels_per_point: f32,

	// Reserved fields.
	pub _reserved1: String,
	pub _reserved2: String,
	pub _reserved3: Option<String>,
	pub _reserved4: Vec<String>,
	pub _reserved5: bool,
	pub _reserved6: bool,
	pub _reserved7: bool,
	pub _reserved8: Option<bool>,
	pub _reserved9: usize,
	pub _reserved10: usize,
	pub _reserved11: usize,
	pub _reserved12: Option<usize>,
	pub _reserved13: f32,
	pub _reserved14: f32,
	pub _reserved15: Option<f32>,
	pub _reserved16: f64,
	pub _reserved17: f64,
	pub _reserved18: Option<f64>,
}

impl Settings {
	pub fn new() -> Self {
		Self {
			artist_sort:        Default::default(),
			album_sort:         Default::default(),
			song_sort:          Default::default(),
			search_kind:        Default::default(),
			artist_sub_tab:     Default::default(),
			search_sort:        Default::default(),
			window_title:       Default::default(),
			album_sizing:       Default::default(),
			album_pixel_size:   ALBUM_ART_SIZE_DEFAULT,
			albums_per_row:     ALBUMS_PER_ROW_DEFAULT,
			previous_threshold: PREVIOUS_THRESHOLD_DEFAULT,
			restore_state:      true,
			empty_autoplay:     true,
			accent_color:       ACCENT_COLOR,
			collection_paths:   vec![],
			pixels_per_point:   PIXELS_PER_POINT_DEFAULT,

			// Reserved fields.
			_reserved1: String::new(),
			_reserved2: String::new(),
			_reserved3: None,
			_reserved4: Vec::with_capacity(0),
			_reserved5: false,
			_reserved6: false,
			_reserved7: false,
			_reserved8: None,
			_reserved9: 0,
			_reserved10: 0,
			_reserved11: 0,
			_reserved12: None,
			_reserved13: 0.0,
			_reserved14: 0.0,
			_reserved15: None,
			_reserved16: 0.0,
			_reserved17: 0.0,
			_reserved18: None,
		}
	}
}

impl Default for Settings {
	fn default() -> Self {
		Self::new()
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod test {
	use super::*;
	use once_cell::sync::Lazy;
	use std::path::PathBuf;
	use disk::Bincode2;

	// Empty.
	const S1: Lazy<Settings> = Lazy::new(|| Settings::from_path("../assets/festival/gui/state/settings1_new.bin").unwrap());
	// Filled.
	const S2: Lazy<Settings> = Lazy::new(|| Settings::from_path("../assets/festival/gui/state/settings1_real.bin").unwrap());

	#[test]
	// Compares `new()`.
	fn cmp() {
		#[cfg(not(target_os = "macos"))]
		assert_eq!(Lazy::force(&S1), &Settings::new());
		#[cfg(target_os = "macos")]
		{
			let mut settings = Settings::new();
			settings.pixels_per_point = 1.5;
			assert_eq!(Lazy::force(&S1), &settings);
		}

		assert_ne!(Lazy::force(&S1), Lazy::force(&S2));

		let b1 = S1.to_bytes().unwrap();
		let b2 = S2.to_bytes().unwrap();
		assert_ne!(b1, b2);
	}

	#[test]
	// Attempts to deserialize the non-empty.
	fn real() {
		assert_eq!(S2.artist_sort,        ArtistSort::RuntimeRev);
		assert_eq!(S2.album_sort,         AlbumSort::LexiRevArtistLexi);
		assert_eq!(S2.song_sort,          SongSort::Runtime);
		assert_eq!(S2.search_kind,        SearchKind::All);
		assert_eq!(S2.artist_sub_tab,     ArtistSubTab::View);
		assert_eq!(S2.search_sort,        SearchSort::Album);
		assert_eq!(S2.window_title,       WindowTitle::Queue);
		assert_eq!(S2.album_sizing,       AlbumSizing::Row);
		assert_eq!(S2.album_pixel_size,   227.0);
		assert_eq!(S2.albums_per_row,     10);
		assert_eq!(S2.previous_threshold, 10);
		assert_eq!(S2.restore_state,      false);
		assert_eq!(S2.empty_autoplay,     false);
		assert_eq!(S2.accent_color,       egui::Color32::from_rgb(97,101,119));
		assert_eq!(S2.collection_paths,   [PathBuf::from("/home/main/Music")]);
		assert_eq!(S2.pixels_per_point.round(), 2.0);
	}
}
