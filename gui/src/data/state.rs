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
	STATE_VERSION,
	GUI,
	ALBUM_ART_SIZE_DEFAULT,
};
use shukusai::{
	constants::{
		FESTIVAL,
		STATE_SUB_DIR,
		HEADER,
	},
	state::AudioState,
	audio::{
		Volume,
		Repeat,
	},
	collection::{
		Album,
		Collection,
		Key,
		ArtistKey,
		AlbumKey,
		Keychain,
	},
};
use shukusai::kernel::Kernel;
use const_format::formatcp;
use std::marker::PhantomData;

//---------------------------------------------------------------------------------------------------- State
#[cfg(debug_assertions)]
disk::json!(State, disk::Dir::Data, FESTIVAL, formatcp!("{GUI}/{STATE_SUB_DIR}"), "state");
#[cfg(not(debug_assertions))]
disk::bincode2!(State, disk::Dir::Data, FESTIVAL, formatcp!("{GUI}/{STATE_SUB_DIR}"), "state", HEADER, STATE_VERSION);
#[derive(Clone,Debug,Default,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
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

impl State {
	#[inline]
	/// Creates a mostly empty [`State`].
	pub fn new() -> Self {
		Self {
			volume: Volume::default().inner(),
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
