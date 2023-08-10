//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use disk::Bincode2;
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
	Settings,
	AlbumSizing,
	SearchSort,
	ArtistSubTab,
	WindowTitle,
};
use const_format::formatcp;
use std::marker::PhantomData;

//---------------------------------------------------------------------------------------------------- Settings
disk::bincode2!(Settings1, disk::Dir::Data, FESTIVAL, formatcp!("{GUI}/{STATE_SUB_DIR}"), "settings", HEADER, 1);
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,Encode,Decode)]
/// Version 1 of `GUI`'s settings.
pub struct Settings1 {
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
}

impl Settings1 {
	/// Reads from disk, then calls `.into()` if `Ok`.
	pub fn disk_into() -> Result<Settings, anyhow::Error> {
		// SAFETY: memmap is used.
		unsafe { Self::from_file_memmap().map(Into::into) }
	}
}

impl Into<Settings> for Settings1 {
	fn into(self) -> Settings {
		let Settings1 {
			artist_sort,
			album_sort,
			song_sort,
			search_kind,
			artist_sub_tab,
			search_sort,
			window_title,
			album_sizing,
			album_pixel_size,
			albums_per_row,
			previous_threshold,
			restore_state,
			empty_autoplay,
			accent_color,
			collection_paths,
			pixels_per_point,
		} = self;

		Settings {
			artist_sort,
			album_sort,
			song_sort,
			search_kind,
			artist_sub_tab,
			search_sort,
			window_title,
			album_sizing,
			album_pixel_size,
			albums_per_row,
			previous_threshold,
			restore_state,
			empty_autoplay,
			accent_color,
			collection_paths,
			pixels_per_point,

			// New fields.
			playlist_sub_tab: Default::default(),
		}
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