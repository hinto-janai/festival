//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use crate::ui::{
	Tab,
};
use std::path::PathBuf;
use disk::prelude::*;
//use disk::{Bincode,bincode_file};
use disk::{Toml,toml_file};
use crate::constants::{
	STATE_VERSION,
	ALBUM_ART_DEFAULT_SIZE,
};
use shukusai::{
	FESTIVAL,
	FESTIVAL_HEADER,
};
use shukusai::collection::{
	Album,
	Collection,
};
use shukusai::key::{
	Key,
	AlbumKey,
};
use shukusai::kernel::{
	AudioState,
	KernelState,
	Kernel,
};

//---------------------------------------------------------------------------------------------------- State
//bincode_file!(State, Dir::Data, FESTIVAL, "gui", "state", FESTIVAL_HEADER, STATE_VERSION);
toml_file!(State, Dir::Data, FESTIVAL, "gui", "state");
#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Serialize,Deserialize)]
/// `GUI`'s State.
///
/// Holds `copy`-able, user-mutable `GUI` state.
///
/// This struct holds an [`AudioState`] which is a local copy of [`KernelState`].
/// This is so that within the `GUI` loop, [`KernelState`] only needs to be locked _once_,
/// so its values can be locally cached, then used within the frame.
pub struct State {
	/// Which [`Tab`] are currently on?
	pub tab: Tab,

	/// Which [`Album`] are we on in the `Album` tab?
	///
	/// This doesn't necessarily mean we're listening to _this_
	/// [`Album`], but rather, it means _this_ is the [`Album`]
	/// that the user is looking at right now.
	///
	/// [`Option::None`] indicates we aren't looking at
	/// any [`Album`] and are in the full [`Album`] art view.
	pub album: Option<AlbumKey>,

	/// `GUI`'s local [`AudioState`].
	pub audio: AudioState,
}

impl State {
	#[inline]
	/// Creates a mostly empty [`State`].
	pub fn new() -> Self {
		Self {
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
