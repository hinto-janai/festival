//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::constants::{
    ACCENT_COLOR, ALBUMS_PER_ROW_DEFAULT, ALBUM_ART_SIZE_DEFAULT, AUTO_SAVE_INTERVAL_SECONDS, GUI,
    PIXELS_PER_POINT_DEFAULT, SETTINGS_VERSION,
};
use crate::data::{AlbumSizing, SearchSort, WindowTitle};
use const_format::formatcp;
use shukusai::{
    audio::PREVIOUS_THRESHOLD_DEFAULT,
    constants::{FESTIVAL, HEADER, STATE_SUB_DIR},
    search::SearchKind,
    sort::{AlbumSort, ArtistSort, SongSort},
};
use std::marker::PhantomData;
use std::path::PathBuf;

//---------------------------------------------------------------------------------------------------- Settings
disk::bincode2!(
    Settings,
    disk::Dir::Data,
    FESTIVAL,
    formatcp!("{GUI}/{STATE_SUB_DIR}"),
    "settings",
    HEADER,
    SETTINGS_VERSION
);
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
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

    /// Auto-save the audio state to disk every `auto_save` seconds.
    pub auto_save: u8,

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

impl Settings {
    pub fn new() -> Self {
        Self {
            artist_sort: Default::default(),
            album_sort: Default::default(),
            song_sort: Default::default(),
            search_kind: Default::default(),
            search_sort: Default::default(),
            window_title: Default::default(),
            album_sizing: Default::default(),
            album_pixel_size: ALBUM_ART_SIZE_DEFAULT,
            albums_per_row: ALBUMS_PER_ROW_DEFAULT,
            previous_threshold: PREVIOUS_THRESHOLD_DEFAULT,
            auto_save: AUTO_SAVE_INTERVAL_SECONDS,
            restore_state: true,
            empty_autoplay: true,
            accent_color: ACCENT_COLOR,
            collection_paths: vec![],
            pixels_per_point: PIXELS_PER_POINT_DEFAULT,
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
    use disk::Bincode2;
    use once_cell::sync::Lazy;
    use std::path::PathBuf;

    // Empty.
    const S1: Lazy<Settings> = Lazy::new(|| {
        Settings::from_path("../assets/festival/gui/state/settings3_new.bin").unwrap()
    });
    // Filled.
    const S2: Lazy<Settings> = Lazy::new(|| {
        Settings::from_path("../assets/festival/gui/state/settings3_real.bin").unwrap()
    });

    #[test]
    // Attempts to deserialize the non-empty.
    fn real() {
        assert_eq!(S2.artist_sort, ArtistSort::RuntimeRev);
        assert_eq!(S2.album_sort, AlbumSort::LexiRevArtistLexi);
        assert_eq!(S2.song_sort, SongSort::Runtime);
        assert_eq!(S2.search_kind, SearchKind::All);
        assert_eq!(S2.search_sort, SearchSort::Album);
        assert_eq!(S2.window_title, WindowTitle::Queue);
        assert_eq!(S2.album_sizing, AlbumSizing::Row);
        assert_eq!(S2.album_pixel_size, 227.0);
        assert_eq!(S2.albums_per_row, 10);
        assert_eq!(S2.previous_threshold, 10);
        assert_eq!(S2.auto_save, 30);
        assert_eq!(S2.restore_state, false);
        assert_eq!(S2.empty_autoplay, false);
        assert_eq!(S2.accent_color, egui::Color32::from_rgb(97, 101, 119));
        assert_eq!(S2.collection_paths, [PathBuf::from("/home/main/Music")]);
        assert_eq!(S2.pixels_per_point.round(), 2.0);
    }
}
