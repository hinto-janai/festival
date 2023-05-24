//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
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
	AUDIO_VERSION,
};
use rolock::RoLock;
use crate::audio::{
	Volume,
};
use readable::Percent;
use once_cell::sync::Lazy;

//---------------------------------------------------------------------------------------------------- Lazy
// This is the global `AudioState`.
pub(super) static AUDIO_STATE: Lazy<Arc<RwLock<AudioState>>> = Lazy::new(|| Arc::new(RwLock::new(AudioState::new())));

//---------------------------------------------------------------------------------------------------- AudioState
#[cfg(debug_assertions)]
disk::json!(AudioState, disk::Dir::Data, FESTIVAL, SHUKUSAI, "audio");
#[cfg(not(debug_assertions))]
disk::bincode2!(AudioState, disk::Dir::Data, FESTIVAL, SHUKUSAI, "audio", HEADER, AUDIO_VERSION);
/// Audio State
///
/// This is a container for the audio state that `Kernel`
/// controls but `Frontend` only has a read-only lock to.
///
/// There is only a single, global copy of this struct that `Kernel` uses.
///
/// To obtain a read-only copy, use `AudioState::get()`.
#[derive(Clone,Debug,PartialOrd,PartialEq,Serialize,Deserialize,Encode,Decode)]
pub struct AudioState {
	// Queue/Playlist.
	/// The current song queue.
	pub queue: Queue,

	/// Are we playing audio right now?
	pub playing: bool,
	/// What is the current [`Volume`]?
	pub volume: Volume,
	/// Which song are we playing right now?
	pub key: Option<Key>,
	/// How much time has passed in this song?
	pub elapsed: f64,
	/// What is the full runtime of the current song?
	pub runtime: f64,
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
			queue: Queue::new(),

			playing: false,
			volume: Volume::new_50(),
			key: None,
			elapsed: 0.0,
			runtime: 0.0,
			shuffle: false,
			repeat: false,
		}
	}

	// Private RwLock version.
	pub(super) fn get_priv() -> Arc<RwLock<Self>> {
		Arc::clone(&AUDIO_STATE)
	}

	#[inline(always)]
	/// Obtain a read-only lock to the global [`KernelState`].
	pub fn get() -> RoLock<Self> {
		RoLock::new(&AUDIO_STATE)
	}
}

impl Default for AudioState {
	fn default() -> Self {
		Self::new()
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
