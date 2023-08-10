//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use crate::data::{
	Tab,State,
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
use disk::Bincode2;
use const_format::formatcp;

//---------------------------------------------------------------------------------------------------- State
disk::bincode2!(State0, disk::Dir::Data, FESTIVAL, formatcp!("{GUI}/{STATE_SUB_DIR}"), "state", HEADER, 0);
#[derive(Clone,Debug,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
/// Version 0 of `GUI`'s State.
pub struct State0 {
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
}

impl State0 {
	/// Reads from disk, then calls `.into()` if `Ok`.
	pub fn disk_into() -> Result<State, anyhow::Error> {
		// SAFETY: memmap is used.
		unsafe { Self::from_file_memmap().map(Into::into) }
	}
}

impl Into<State> for State0 {
	fn into(self) -> State {
		let State0 {
			volume,
			tab,
			last_tab,
			search_string,
			search_result,
			repeat,
			album,
			artist,
		} = self;

		State {
			volume,
			tab,
			last_tab,
			search_string,
			search_result,
			repeat,
			album,
			artist,

			// New fields
			playlist: None,
			playlist_edit: None,
			playlist_edit_string: Default::default(),
			playlist_string: Default::default(),
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
	const S1: Lazy<State> = Lazy::new(|| State::from_path("../assets/festival/gui/state/state0_new.bin").unwrap());
	// Filled.
	const S2: Lazy<State> = Lazy::new(|| State::from_path("../assets/festival/gui/state/state0_real.bin").unwrap());

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
		assert_eq!(S2.tab,           Tab::Settings);
		assert_eq!(S2.last_tab,      Some(Tab::Search));
		assert_eq!(S2.search_string, "asdf");
		assert_eq!(S2.volume,        0);
		assert_eq!(S2.repeat,        Repeat::Off);
		assert_eq!(S2.album,         Some(AlbumKey::from(1_u8)));
		assert_eq!(S2.artist,        Some(ArtistKey::zero()));
		assert_eq!(S2.search_result.artists.len(), 3);
		assert_eq!(S2.search_result.albums.len(), 4);
		assert_eq!(S2.search_result.songs.len(), 7);
	}
}
