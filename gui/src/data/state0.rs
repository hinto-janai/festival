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
use std::marker::PhantomData;
use crate::data::State;
use disk::Bincode2;

//---------------------------------------------------------------------------------------------------- State0
disk::bincode2!(State0, disk::Dir::Data, FESTIVAL, formatcp!("{GUI}/{STATE_SUB_DIR}"), "state", HEADER, 0);
#[derive(Clone,Debug,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
/// Version 0 of `State`.
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

	// Reserved fields.
	_reserved1: PhantomData<Vec<String>>,
	_reserved2: PhantomData<String>,
	_reserved3: PhantomData<Option<String>>,
	_reserved4: PhantomData<bool>,
	_reserved5: PhantomData<bool>,
	_reserved6: PhantomData<Option<bool>>,
	_reserved7: PhantomData<Option<bool>>,
	_reserved8: PhantomData<usize>,
	_reserved9: PhantomData<usize>,
	_reserved10: PhantomData<Option<usize>>,
	_reserved11: PhantomData<Option<usize>>,
}

impl State0 {
	#[inline]
	/// Creates a mostly empty [`State0`].
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
			_reserved1: PhantomData,
			_reserved2: PhantomData,
			_reserved3: PhantomData,
			_reserved4: PhantomData,
			_reserved5: PhantomData,
			_reserved6: PhantomData,
			_reserved7: PhantomData,
			_reserved8: PhantomData,
			_reserved9: PhantomData,
			_reserved10: PhantomData,
			_reserved11: PhantomData,
		}
	}

	/// Reads from disk, then calls `.into()` if `Ok`.
	pub fn disk_into() -> Result<State, anyhow::Error> {
		// SAFETY: memmap is used.
		unsafe { Self::from_file_memmap().map(Into::into) }
	}
}

impl Default for State0 {
	fn default() -> Self {
		Self::new()
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
			..
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

			// Reserved fields.
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
	const S1: Lazy<State0> = Lazy::new(|| State0::from_path("../assets/festival/gui/state/state0_new.bin").unwrap());
	// Filled.
	const S2: Lazy<State0> = Lazy::new(|| State0::from_path("../assets/festival/gui/state/state0_real.bin").unwrap());

	#[test]
	// Compares `new()`.
	fn cmp() {
		assert_eq!(Lazy::force(&S1), &State0::new());
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

	#[test]
	// Asserts previous versions can be converted.
	fn convert() {
		let s: State = Lazy::force(&S2).clone().into();
		assert_eq!(s.tab,           Tab::Settings);
		assert_eq!(s.last_tab,      Some(Tab::Search));
		assert_eq!(s.search_string, "asdf");
		assert_eq!(s.volume,        0);
		assert_eq!(s.repeat,        Repeat::Off);
		assert_eq!(s.album,         Some(AlbumKey::from(1_u8)));
		assert_eq!(s.artist,        Some(ArtistKey::zero()));
		assert_eq!(s.search_result.artists.len(), 3);
		assert_eq!(s.search_result.albums.len(), 4);
		assert_eq!(s.search_result.songs.len(), 7);
	}
}
