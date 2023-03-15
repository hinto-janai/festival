//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::collection::{
	Key,
	Keychain,
	Slice,
};
use crate::constants::{
	FESTIVAL,
	FESTIVAL_HEADER,
	STATE_VERSION,
};

//---------------------------------------------------------------------------------------------------- State
bincode_file!(State, Dir::Data, FESTIVAL, "", "state", FESTIVAL_HEADER, STATE_VERSION);
#[derive(Clone,Debug,Default,PartialEq,Serialize,Deserialize)]
pub struct State {
	// Audio.
	pub current_key: Key,
	pub current_elapsed: f64,

	// Search.
	pub search_result: Keychain,

	// Queue/Playlist.
	pub queue: Slice,
	pub playlists: Slice,
}

impl State {
	#[inline(always)]
	// Create empty struct.
	pub fn new() -> Self {
		Self {
			current_key: Key::new(),
			current_elapsed: 0.0,

			search_result: Keychain::new(),

			queue: Slice::new(),
			playlists: Slice::new(),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
