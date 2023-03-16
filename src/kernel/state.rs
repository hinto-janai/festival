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
	pub playing: bool,        // Are we playing audio right now?
	pub current_key: Key,     // Which song are we playing right now?
	pub current_elapsed: f64, // How much time has passed in this song?
	pub current_runtime: f64, // What is the full runtime of the current song?
	pub shuffle: bool,        // Is shuffle on?
	pub repeat: bool,         // Is repeat on?

	// Search.
	pub search_result: Keychain, // The result of the current search result.

	// Queue/Playlist.
	pub queue: Slice,     // The current song queue.
	pub playlists: Slice, // ALL the user's playlists.
}

impl State {
	#[inline(always)]
	// Create empty struct.
	pub fn new() -> Self {
		Self {
			playing: false,
			current_key: Key::new(),
			current_elapsed: 0.0,
			current_runtime: 0.0,
			shuffle: false,
			repeat: false,

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
