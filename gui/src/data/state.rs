//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use super::{
	Tab,
};
use crate::data::{
	ArtistSubTab,
	PlaylistSubTab,
};
use crate::constants::{
	GUI,
	STATE_VERSION,
};
use shukusai::{
	constants::{
		HEADER,
		FESTIVAL,
		STATE_SUB_DIR,
	},
	audio::{
		Volume,
		Repeat,
	},
	collection::{
		ArtistKey,
		AlbumKey,
		Keychain,
	},
};
use std::sync::Arc;
use const_format::formatcp;

//---------------------------------------------------------------------------------------------------- State
disk::bincode2!(State, disk::Dir::Data, FESTIVAL, formatcp!("{GUI}/{STATE_SUB_DIR}"), "state", HEADER, STATE_VERSION);
#[derive(Clone,Debug,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
/// `GUI`'s State.
///
/// Holds `copy`-able, user-mutable `GUI` state.
///
/// This struct holds an [`AudioState`] which is a local copy of [`KernelState`].
/// This is so that within the `GUI` loop, [`KernelState`] only needs to be locked _once_,
/// so its values can be locally cached, then used within the frame.
pub struct State {
	// Tab.
	/// Which [`Tab`] are currently on?
	pub tab: Tab,
	/// The last [`Tab`] the user was on.
	pub last_tab: Option<Tab>,

	/// Our current search input.
	pub search_string: String,
	/// The search result [`Keychain`] we got from `Kernel`.
	pub search_result: Keychain,

	/// What [`Volume`] are we at (0..100)?
	pub volume: u8,

	/// Repeat mode.
	pub repeat: Repeat,

	/// Which [`Album`] are we on in the `Album` tab?
	///
	/// This doesn't necessarily mean we're listening to _this_
	/// [`Album`], but rather, it means _this_ is the [`Album`]
	/// that the user will see when clicking the `Album` tab.
	///
	/// [`Option::None`] indicates we aren't looking at
	/// any [`Album`] and are in the full [`Album`] art view.
	pub album: Option<AlbumKey>,

	/// Which [`Artist`] are we on in the `Artist` tab?
	pub artist: Option<ArtistKey>,

	/// Which `ArtistSubTab` are we on?
	pub artist_sub_tab: ArtistSubTab,

	/// Which `PlaylistSubTab` are we on?
	pub playlist_sub_tab: PlaylistSubTab,

	/// Our current playlist
	pub playlist: Option<Arc<str>>,
	/// If we are editing a playlist's name.
	pub playlist_edit: Option<Arc<str>>,
	/// The playlist name edit text.
	pub playlist_edit_string: String,
	/// Our current playlist name input.
	pub playlist_string: String,
}

impl State {
	#[inline]
	/// Creates a mostly empty [`State`].
	pub fn new() -> Self {
		Self {
			volume: Volume::default().inner(),

			tab: Default::default(),
			last_tab: Default::default(),
			search_string: Default::default(),
			search_result: Default::default(),
			repeat: Default::default(),
			album: Default::default(),
			artist: Default::default(),
			artist_sub_tab: Default::default(),
			playlist_sub_tab: Default::default(),
			playlist: None,
			playlist_edit: None,
			playlist_edit_string: Default::default(),
			playlist_string: Default::default(),
		}
	}
}

impl Default for State {
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
	const S1: Lazy<State> = Lazy::new(|| State::from_path("../assets/festival/gui/state/state1_new.bin").unwrap());
	// Filled.
	const S2: Lazy<State> = Lazy::new(|| State::from_path("../assets/festival/gui/state/state1_real.bin").unwrap());

	#[test]
	// Compares `new()`.
	fn cmp() {
		assert_eq!(Lazy::force(&S1), &State::new());
		assert_ne!(Lazy::force(&S1), Lazy::force(&S2));

		let b1 = S1.to_bytes().unwrap();
		let b2 = S2.to_bytes().unwrap();
		assert_ne!(b1, b2);
	}

	#[test]
	// Attempts to deserialize the non-empty.
	fn real() {
		// `Playlists` tab was added, so enum number tag got incremented.
		assert_eq!(S2.tab,           Tab::Search);
		assert_eq!(S2.last_tab,      Some(Tab::Playlists));
		assert_eq!(S2.search_string, "asdf");
		assert_eq!(S2.volume,        25);
		assert_eq!(S2.repeat,        Repeat::Off);
		assert_eq!(S2.album,         Some(AlbumKey::from(1_u8)));
		assert_eq!(S2.artist,        Some(ArtistKey::zero()));
		assert_eq!(S2.search_result.artists.len(), 3);
		assert_eq!(S2.search_result.albums.len(), 4);
		assert_eq!(S2.search_result.songs.len(), 7);

		assert_eq!(S2.playlist, None);
		assert_eq!(S2.playlist_edit, None);
		assert_eq!(S2.playlist_edit_string, String::new());
		assert_eq!(S2.playlist_string, String::new());
	}
}
