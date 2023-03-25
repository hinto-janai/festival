//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::key::{
	Key,
	Keychain,
};
use crate::slice::{
	Queue,
	Playlist,
};
use crate::constants::{
	FESTIVAL,
	FESTIVAL_HEADER,
	STATE_VERSION,
};

//---------------------------------------------------------------------------------------------------- State
bincode_file!(KernelState, Dir::Data, FESTIVAL, "", "state", FESTIVAL_HEADER, STATE_VERSION);
#[derive(Clone,Debug,Default,PartialEq,Serialize,Deserialize)]
/// Audio/Misc State
///
/// This hold various bits of state that `Kernel` controls
/// but `Frontend` only has a read-only lock to.
pub struct KernelState {
	// Audio.
	/// Are we playing audio right now?
	pub playing: bool,
	/// Which song are we playing right now?
	pub current_key: Key,
	/// How much time has passed in this song?
	pub current_elapsed: f64,
	/// What is the full runtime of the current song?
	pub current_runtime: f64,
	/// Is shuffle on?
	pub shuffle: bool,
	/// Is repeat on?
	pub repeat: bool,

	// Search.
	/// The result of the current search result.
	pub search_result: Keychain,

	// Queue/Playlist.
	/// The current song queue.
	pub queue: Queue,
	/// ALL of the user's playlists.
	pub playlists: Vec<Playlist>,
}

impl KernelState {
	#[inline(always)]
	/// Creates an empty struct.
	pub fn new() -> Self {
		Self {
			playing: false,
			current_key: Key::new(),
			current_elapsed: 0.0,
			current_runtime: 0.0,
			shuffle: false,
			repeat: false,

			search_result: Keychain::new(),

			queue: Queue::new(),
			playlists: vec![],
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
