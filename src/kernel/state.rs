//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
//use crate::macros::*;
//use std::{};
use std::sync::{Arc,RwLock};
use crate::collection::{
	Collection,
	Key,
	Keychain,
	Queue,
	Playlist,
};
use crate::constants::{
	FESTIVAL,
	SHUKUSAI,
	HEADER,
	STATE_VERSION,
};
use rolock::RoLock;
use super::Volume;
use crate::kernel::Kernel;
use readable::Percent;
use once_cell::sync::Lazy;

//---------------------------------------------------------------------------------------------------- Lazy
// This is the global `KernelState`.
pub(crate) static KERNEL_STATE: Lazy<Arc<RwLock<KernelState>>> = Lazy::new(|| Arc::new(RwLock::new(KernelState::new())));

//---------------------------------------------------------------------------------------------------- AudioState
/// Audio State
///
/// This is a container for various audio state data.
/// It is included within [`KernelState`], and can easily
/// be `copied` to `Frontend`'s cheaply so [`KernelState`]
/// doesn't have to be directly locked all the time.
#[derive(Copy,Clone,Debug,PartialOrd,PartialEq,Serialize,Deserialize,Encode,Decode)]
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
	pub(crate) fn new() -> Self {
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

impl Default for AudioState {
	fn default() -> Self {
		Self::new()
	}
}

//---------------------------------------------------------------------------------------------------- KernelState
#[cfg(debug_assertions)]
disk::json!(KernelState, disk::Dir::Data, FESTIVAL, SHUKUSAI, "state");
#[cfg(not(debug_assertions))]
disk::bincode2!(KernelState, disk::Dir::Data, FESTIVAL, SHUKUSAI, "state", HEADER, STATE_VERSION);
#[derive(Clone,Debug,PartialOrd,PartialEq,Serialize,Deserialize,Encode,Decode)]
/// Kernel State
///
/// This hold various bits of state that `Kernel` controls
/// but `Frontend` only has a read-only lock to.
///
/// There is only a single, global copy of this struct that `Kernel` uses.
///
/// To obtain a read-only copy, use `ResetState::get()`.
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

	// Saving.
	/// This [`bool`] represents if a [`Collection`] that was
	/// recently created is still being written to the disk.
	///
	/// For performance reasons, when the `Frontend` asks [`Kernel`]
	/// for a new [`Collection`], [`Kernel`] will return immediately upon
	/// having an in-memory [`Collection`]. However, `shukusai` will
	/// (in the background) be saving it disk.
	///
	/// If your `Frontend` exits around this time, it should probably hang
	/// (for a reasonable amount of time) if this is set to `true`, waiting
	/// for the [`Collection`] to be saved to disk.
	pub saving: bool,
}

impl KernelState {
	// Private RwLock version.
	pub(super) fn get_priv() -> Arc<RwLock<Self>> {
		Arc::clone(&KERNEL_STATE)
	}

	#[inline(always)]
	/// Creates an empty struct.
	pub(crate) fn new() -> Self {
		Self {
			audio: AudioState::new(),

			search_result: Keychain::new(),

			queue: Queue::new(),
			playlists: vec![],

			saving: false,
		}
	}

	#[inline(always)]
	/// Obtain a read-only lock to the global [`KernelState`].
	pub fn get() -> RoLock<Self> {
		RoLock::new(&KERNEL_STATE)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
