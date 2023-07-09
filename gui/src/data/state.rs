//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use super::{
	Tab,
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

	// Reserved fields.
	pub _reserved1: Option<Vec<String>>,
	pub _reserved2: Option<String>,
	pub _reserved3: Option<String>,
	pub _reserved4: Option<Option<String>>,
	pub _reserved5: Option<bool>,
	pub _reserved6: Option<bool>,
	pub _reserved7: Option<Option<bool>>,
	pub _reserved8: Option<Option<bool>>,
	pub _reserved9: Option<usize>,
	pub _reserved10: Option<usize>,
	pub _reserved11: Option<f32>,
	pub _reserved12: Option<f32>,
	pub _reserved13: Option<f64>,
	pub _reserved14: Option<f64>,
	pub _reserved15: Option<Option<usize>>,
	pub _reserved16: Option<Option<usize>>,
	pub _reserved17: Option<u8>,
	pub _reserved18: Option<u8>,
	pub _reserved19: Option<u16>,
	pub _reserved20: Option<u16>,
	pub _reserved21: Option<u32>,
	pub _reserved22: Option<u32>,
	pub _reserved23: Option<usize>,
	pub _reserved24: Option<usize>,
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
			_reserved1: None,
			_reserved2: None,
			_reserved3: None,
			_reserved4: None,
			_reserved5: None,
			_reserved6: None,
			_reserved7: None,
			_reserved8: None,
			_reserved9: None,
			_reserved10: None,
			_reserved11: None,
			_reserved12: None,
			_reserved13: None,
			_reserved14: None,
			_reserved15: None,
			_reserved16: None,
			_reserved17: None,
			_reserved18: None,
			_reserved19: None,
			_reserved20: None,
			_reserved21: None,
			_reserved22: None,
			_reserved23: None,
			_reserved24: None,
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
		assert_eq!(S2.tab,           Tab::Settings);
		assert_eq!(S2.last_tab,      Some(Tab::Search));
		assert_eq!(S2.search_string, "asdf");
		assert_eq!(S2.volume,        25);
		assert_eq!(S2.repeat,        Repeat::Off);
		assert_eq!(S2.album,         Some(AlbumKey::from(1_u8)));
		assert_eq!(S2.artist,        Some(ArtistKey::zero()));
		assert_eq!(S2.search_result.artists.len(), 3);
		assert_eq!(S2.search_result.albums.len(), 4);
		assert_eq!(S2.search_result.songs.len(), 7);
	}
}
