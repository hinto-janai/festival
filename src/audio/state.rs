//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
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
use readable::Runtime;
use once_cell::sync::Lazy;
use benri::sync::*;
use std::sync::{
	RwLock,
	RwLockReadGuard,
	RwLockWriteGuard,
};

//---------------------------------------------------------------------------------------------------- Lazy
/// This is the single, global copy of `AudioState` that `Kernel` uses.
///
/// To obtain a read-only lock, use `AUDIO_STATE.read()`.
pub static AUDIO_STATE: AudioStateLock = AudioStateLock(RwLock::new(AudioState::new()));

//---------------------------------------------------------------------------------------------------- AudioStateLock
/// There is only a single, global copy of `AudioState` that `Kernel` uses: [`AUDIO_STATE`].
///
/// To obtain a read-only lock, use `AUDIO_STATE.read()`.
pub struct AudioStateLock(RwLock<AudioState>);

impl AudioStateLock {
	#[inline(always)]
	/// Obtain a read-only lock to the global [`AudioState`].
	pub fn read(&'static self) -> RwLockReadGuard<'static, AudioState> {
		lockr!(self.0)
	}

	#[inline(always)]
	// Private write.
	pub(super) fn write(&'static self) -> RwLockWriteGuard<'static, AudioState> {
		lockw!(self.0)
	}
}

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
/// To obtain a read-only lock, use `AUDIO_STATE.read()`.
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
	pub elapsed: Runtime,
	/// What is the full runtime of the current song?
	pub runtime: Runtime,
	/// Is shuffle on?
	pub shuffle: bool,
	/// Is repeat on?
	pub repeat: bool,
}

impl AudioState {
	#[inline]
	/// Creates an empty struct.
	pub(crate) const fn new() -> Self {
		Self {
			queue: Queue::new(),

			playing: false,
			volume: Volume::new_50(),
			key: None,
			elapsed: Runtime::zero(),
			runtime: Runtime::zero(),
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

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
