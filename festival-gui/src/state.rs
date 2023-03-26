//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use super::{
	tab::Tab,
};
use std::path::PathBuf;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use super::constants::{
	STATE_VERSION,
	ALBUM_ART_DEFAULT_SIZE,
};
use shukusai::{
	FESTIVAL,
	FESTIVAL_HEADER,
};
use shukusai::key::{
	AlbumKey,
};

//---------------------------------------------------------------------------------------------------- State
/// `GUI`'s State.
///
/// Holds user-mutable `GUI` state.
bincode_file!(State, Dir::Data, FESTIVAL, "gui", "state", FESTIVAL_HEADER, STATE_VERSION);
#[derive(Clone,Debug,Default,PartialEq,PartialOrd,Serialize,Deserialize)]
pub struct State {
	/// Which [`Tab`] are currently on?
	pub tab: Tab,

	/// Which [`Key`] are we currently on?
	///
	/// This means which [`Song`] of what [`Album`]
	/// of what [`Artist`] are we currently listening to.
	///
	/// This acts as a local cache so we don't have to lock
	/// `KernelState` everytime we want to read the value.
	///
	/// [`Option::None`] indicates we aren't listening to anything right now.
	pub key: Option<Key>,

	/// Which [`Album`] are we on in the `Album` tab?
	///
	/// This doesn't necessarily mean we're listening to _this_
	/// [`Album`], but rather, it means _this_ is the [`Album`]
	/// that the user is looking at right now.
	///
	/// [`Option::None`] indicates we aren't looking at
	/// any [`Album`] and are in the full [`Album`] art view.
	pub album: Option<AlbumKey>,
}

impl State {
	/// Creates a mostly empty [`State`].
	pub fn new() -> Self {
		Self {
			key: None,
			album: None,
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
