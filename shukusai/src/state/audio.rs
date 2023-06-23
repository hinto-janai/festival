//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use crate::collection::{
	Collection,
	SongKey,
	Keychain,
};
use crate::constants::{
	FESTIVAL,
	FRONTEND_SUB_DIR,
	STATE_SUB_DIR,
	HEADER,
	AUDIO_VERSION,
};
use crate::audio::{
	Volume,Repeat,
};
use readable::Runtime;
use once_cell::sync::Lazy;
use benri::sync::*;
use std::sync::{
	RwLock,
	RwLockReadGuard,
	RwLockWriteGuard,
	TryLockError,
};
use std::collections::VecDeque;
use const_format::formatcp;
use std::marker::PhantomData;

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
	/// Call the non-blocking `.try_read()` on the global [`AudioState`].
	pub fn try_read(&'static self) -> Result<RwLockReadGuard<'static, AudioState>, TryLockError<RwLockReadGuard<'static, AudioState>>> {
		self.0.try_read()
	}

	#[inline(always)]
	// Private write.
	pub(crate) fn write(&'static self) -> RwLockWriteGuard<'static, AudioState> {
		lockw!(self.0)
	}

	#[inline(always)]
	// Private write.
	pub(crate) fn try_write(&'static self) -> Result<RwLockWriteGuard<'static, AudioState>, TryLockError<RwLockWriteGuard<'static, AudioState>>> {
		self.0.try_write()
	}
}

//---------------------------------------------------------------------------------------------------- AudioState
#[cfg(debug_assertions)]
disk::json!(AudioState, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/{STATE_SUB_DIR}"), "audio");
#[cfg(not(debug_assertions))]
disk::bincode2!(AudioState, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/{STATE_SUB_DIR}"), "audio", HEADER, AUDIO_VERSION);
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
	// Queue.
	/// The current song queue.
	pub queue: VecDeque<SongKey>,
	/// The currently playing index in the queue.
	pub queue_idx: Option<usize>,

	/// Are we playing audio right now?
	pub playing: bool,
	/// Which song are we playing right now?
	pub song: Option<SongKey>,
	/// How much time has passed in this song?
	pub elapsed: Runtime,
	/// What is the full runtime of the current song?
	pub runtime: Runtime,
	/// Repeat mode.
	pub repeat: Repeat,

	/// # WARNING
	/// This is simply for saving to disk.
	/// It does not represent the current volume.
	/// See [`crate::state::VOLUME`] for more info.
	pub volume: Volume,

	// Reserved fields.
	_reserved1: PhantomData<bool>,
	_reserved2: PhantomData<bool>,
	_reserved3: PhantomData<bool>,
	_reserved4: PhantomData<usize>,
	_reserved5: PhantomData<usize>,
	_reserved6: PhantomData<Option<usize>>,
	_reserved7: PhantomData<Option<usize>>,
	_reserved8: PhantomData<VecDeque<usize>>,
}

impl AudioState {
	/// Creates an empty struct.
	pub const fn new() -> Self {
		Self {
			queue: VecDeque::new(),
			queue_idx: None,

			playing: false,
			volume: Volume::const_default(),
			song: None,
			elapsed: Runtime::zero(),
			runtime: Runtime::zero(),
			repeat: Repeat::new(),

			_reserved1: PhantomData,
			_reserved2: PhantomData,
			_reserved3: PhantomData,
			_reserved4: PhantomData,
			_reserved5: PhantomData,
			_reserved6: PhantomData,
			_reserved7: PhantomData,
			_reserved8: PhantomData,
		}
	}

	/// Clone `Self`, conditionally.
	///
	/// - If `self` and `dst` are the same, this does nothing
	/// - If `self` and `dst`'s queue are the same, everything but that is `clone()`'ed
	/// - If `self` and `dst`'s queue are different, all of `self` is `.clone()`'ed into `dst`
	pub fn if_copy(&self, dst: &mut Self) {
		if self == dst {
			return;
		} else if self.queue != dst.queue {
			*dst = self.clone();
		} else {
			*dst = Self {
				queue: std::mem::take(&mut dst.queue),
				..self.clone()
			};
		}
	}

	// Clear `Self` and assume we are done playing.
	pub(crate) fn finish(&mut self) {
		self.queue.clear();
		self.queue_idx = None;
		self.playing   = false;
		self.song      = None;
		self.elapsed   = Runtime::zero();
		self.runtime   = Runtime::zero();
	}

	// - Increments the `queue_idx`
	// - Sets current song to the new index
	//
	// Returns the new `SongKey`.
	// Returns `None` if none left.
	pub(crate) fn next(&mut self) -> Option<SongKey> {
		if let Some(i) = self.queue_idx {
			let i = i + 1;

			if let Some(key) = self.queue.get(i) {
				self.song      = Some(*key);
				self.queue_idx = Some(i);
				return Some(*key);
			}
		}

		None
	}

	// - Decrements the `queue_idx`
	// - Sets current song to the new index
	//
	// Returns the new `SongKey`.
	// Returns index `0` if at first index.
	// Returns `None` if nothing in queue.
	pub(crate) fn prev(&mut self) -> Option<SongKey> {
		if let Some(i) = self.queue_idx {
			let i = i.saturating_sub(1);
			if let Some(key) = self.queue.get(i) {
				self.song      = Some(*key);
				self.queue_idx = Some(i);
				return Some(*key);
			}
		}

		None
	}

	// Checks if we are at the last index in the queue.
	pub(crate) fn at_last_queue_idx(&self) -> bool {
		match self.queue_idx {
			Some(i) => i + 1 == self.queue.len(),
			None => false,
		}
	}

	// Checks if we are at the first index in the queue.
	pub(crate) fn at_first_queue_idx(&self) -> bool {
		match self.queue_idx {
			Some(i) => i == 0,
			None => false,
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
