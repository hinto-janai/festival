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
use rolock::RoLock;
use super::Volume;
use crate::kernel::Kernel;

//---------------------------------------------------------------------------------------------------- AudioState
/// Audio State
///
/// This is a container for various audio state data.
/// It is included within [`KernelState`], and can easily
/// be `copied` to `Frontend`'s cheaply so [`KernelState`]
/// doesn't have to be directly locked all the time.
#[derive(Copy,Clone,Debug,Default,PartialOrd,PartialEq,Serialize,Deserialize)]
pub struct AudioState {
	/// Are we playing audio right now?
	pub playing: bool,
	/// What is the current [`Volume`]?
	pub volume: Volume,
	/// Which song are we playing right now?
	pub current_key: Option<Key>,
	/// How much time has passed in this song?
	pub current_elapsed: f64,
	/// What is the full runtime of the current song?
	pub current_runtime: f64,
	/// Is shuffle on?
	pub shuffle: bool,
	/// Is repeat on?
	pub repeat: bool,
}

impl AudioState {
	#[inline]
	/// Creates an empty struct.
	pub fn new() -> Self {
		Self {
			playing: false,
			volume: Volume::new_50(),
			current_key: None,
			current_elapsed: 0.0,
			current_runtime: 0.0,
			shuffle: false,
			repeat: false,
		}
	}
}

//---------------------------------------------------------------------------------------------------- KernelState
bincode_file!(KernelState, Dir::Data, FESTIVAL, "", "state", FESTIVAL_HEADER, STATE_VERSION);
#[derive(Clone,Debug,Default,PartialOrd,PartialEq,Serialize,Deserialize)]
/// Kernel State
///
/// This hold various bits of state that `Kernel` controls
/// but `Frontend` only has a read-only lock to.
pub struct KernelState {
	// Audio.
	/// The current [`AudioState`].
	///
	/// All values within this can be cheaply `copied`.
	pub audio: AudioState,

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
	pub(crate) fn new() -> Self {
		Self {
			audio: AudioState::new(),

			search_result: Keychain::new(),

			queue: Queue::new(),
			playlists: vec![],
		}
	}

	#[inline(always)]
	/// Create an empty, dummy [`KernelState`] wrapped in an [`RoLock`].
	///
	/// This is useful when you need to initialize but don't want
	/// to wait on [`Kernel`] to hand you the _real_ `RoLock<KernelState>`.
	pub fn dummy() -> RoLock<Self> {
		let (_, ro) = RoLock::new_pair(Self::new());
		ro
	}
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
