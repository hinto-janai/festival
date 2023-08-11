//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use crate::collection::{
	Collection,SongKey,MapKey,
};
use crate::constants::{
	HEADER,AUDIO_VERSION,
	FESTIVAL,
	FRONTEND_SUB_DIR,
	STATE_SUB_DIR,
};
use crate::audio::{
	Volume,Repeat,
};
use readable::Runtime;
use benri::sync::*;
use benri::ok;
use log::warn;
use std::sync::{
	Arc,
	RwLock,
	RwLockReadGuard,
	RwLockWriteGuard,
	TryLockError,
};
use std::collections::VecDeque;
use const_format::formatcp;
use disk::Bincode2;

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
}

impl AudioState {
	/// Creates an empty struct.
	pub const fn new() -> Self {
		Self {
			queue: VecDeque::new(),
			queue_idx: None,

			playing: false,
			song: None,
			elapsed: Runtime::zero(),
			runtime: Runtime::zero(),
			repeat: Repeat::new(),
			volume: Volume::const_default(),
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
}

impl Default for AudioState {
	fn default() -> Self {
		Self::new()
	}
}

//---------------------------------------------------------------------------------------------------- AudioStateRestore
#[derive(Debug,Clone)]
/// An "in-between Collection reset" representation of [`AudioState`].
///
/// Keys are replaced with string variants, which are attempted
/// to be converted after the `Collection` reset.
///
/// The _len_ of the queue is not changed at all, to
/// preserve the index we were on. Keys that don't
/// exist any more are `None` instead of not existing
/// so that the `len` is the same.
pub(crate) struct AudioStateRestore {
	pub(crate) queue: VecDeque<Option<MapKey>>,
	pub(crate) queue_idx: Option<usize>,
	pub(crate) playing: bool,
	pub(crate) song: Option<MapKey>,
	pub(crate) elapsed: Runtime,
	pub(crate) runtime: Runtime,
	pub(crate) repeat: Repeat,
	pub(crate) volume: Volume,

	// extra info
	pub(crate) queue_len: usize,
}

impl AudioStateRestore {
	// Attempts to convert all valid string keys into
	// normal keys, aka, `AudioState`.
	//
	// Everything related to a song that was "set",
	// (`queue_idx`, `song`, etc) will only be restored
	// IF the original song can be found in the exact
	// same index.
	//
	// So basically, it only makes sense to recover the
	// "current" song if it actually matches the song
	// that was set before.
	pub fn into_audio_state(self, collection: &Arc<Collection>) -> AudioState {
		let queue: VecDeque<SongKey> = self.queue
			.into_iter()
			.map(|o| {
				match o {
					Some(m) => m.to_key(collection),
					None => None,
				}
			})
			.filter_map(|o| o)
			.collect();

		if queue.len() == self.queue_len {
			if let Some(m) = self.song {
				if let Some((song, key)) = collection.song(m.artist, m.album, m.song) {
					if let Some(i) = self.queue_idx {
						ok!("AudioState was 100% recovered across Collection reset");
						return AudioState {
							queue,
							queue_idx: self.queue_idx,
							playing: self.playing,
							song: Some(song.key),
							elapsed: self.elapsed,
							runtime: self.runtime,
							repeat: self.repeat,
							volume: self.volume,
						};
					}
				}
			}
		}

		warn!("AudioStateRecover failed, AudioState will not be 100% the same");
		AudioState {
			queue,
			queue_idx: None,
			playing: false,
			song: None,
			elapsed: Runtime::zero(),
			runtime: Runtime::zero(),
			repeat: self.repeat,
			volume: self.volume,
		}
	}

	pub fn from_audio_state(a: &AudioState, collection: &Arc<Collection>) -> Self {
		let queue: VecDeque<Option<MapKey>> = a.queue
			.iter()
			.map(|k| Some(MapKey::from_song_key(*k, collection)))
			.collect();

		let song = a.song.map(|k| MapKey::from_song_key(k, collection));

		Self {
			queue,
			queue_idx: a.queue_idx,
			playing: a.playing,
			song,
			elapsed: a.elapsed,
			runtime: a.runtime,
			repeat: a.repeat,
			volume: Volume::new(atomic_load!(crate::state::VOLUME)),

			queue_len: a.queue.len(),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use disk::Bincode2;
	use readable::Runtime;
	use once_cell::sync::Lazy;

	// Empty new `AudioState`.
	const A1: Lazy<AudioState> = Lazy::new(|| AudioState::from_path("../assets/shukusai/state/audio0_new.bin").unwrap());
	// Filled, user `AudioState`.
	const A2: Lazy<AudioState> = Lazy::new(|| AudioState::from_path("../assets/shukusai/state/audio0_real.bin").unwrap());
	const SONG: SongKey = SongKey::new();

	#[test]
	// Tests `if_copy()` and asserts the following behavior:
	//
	// 1. If `self` and `dst` are the same, this does nothing
	// 2. If `self` and `dst`'s queue are the same, everything but that is `clone()`'ed
	// 3. If `self` and `dst`'s queue are different, all of `self` is `.clone()`'ed into `dst`
	fn if_copy() {
		let mut a = AudioState::new();
		let mut dst = AudioState::new();

		// 1.
		a.if_copy(&mut dst);
		assert_eq!(a, dst);

		// 2.
		a.playing   = true;
		a.queue_idx = Some(usize::MAX);
		a.song      = Some(SONG);
		a.elapsed   = Runtime::from(123_u32);
		assert_ne!(a, dst);
		a.if_copy(&mut dst);
		assert_eq!(dst.song.unwrap(), SONG);
		assert_eq!(dst.queue.len(), 0);
		assert_eq!(a, dst);

		// 3.
		let mut a   = AudioState::new();
		let mut dst = AudioState::new();
		a.queue_idx = Some(usize::MAX);
		a.song      = Some(SONG);
		a.queue.push_front(SongKey::from(usize::MAX));
		assert_ne!(a.queue, dst.queue);
		a.if_copy(&mut dst);
		assert_eq!(dst.song.unwrap(), SONG);
		assert_eq!(dst.queue.len(), 1);
		assert_eq!(a, dst);
	}

	#[test]
	// Tests `finish()` and asserts state is correct.
	fn finish() {
		let mut a = AudioState::new();

		// Set state as if we're playing a song.
		a.queue.push_front(SONG);
		a.queue_idx = Some(0);
		a.playing   = true;
		a.song      = Some(SONG);
		a.elapsed   = Runtime::from(123_u32);
		a.runtime   = Runtime::from(321_u32);

		a.finish();

		assert!(a.queue.is_empty());
		assert!(a.queue_idx.is_none());
		assert!(!a.playing);
		assert!(a.song.is_none());
		assert_eq!(a.elapsed, Runtime::zero());
		assert_eq!(a.runtime, Runtime::zero());
	}

	#[test]
	// Tests `next()` and asserts the following behavior:
	//
	// 1. `queue_idx` is `None` => Returns `None`;
	//
	// 2. Next key is `Some` => {
	//       Increments the `queue_idx`;
	//       Sets current song to the new index;
	//       Returns `Some(new_song)`
	//   }
	//
	// 3. Next key is `None` => {
	//       Returns `None`;
	//   }
	fn next() {
		let mut a = AudioState::new();

		// 1
		a.queue_idx = None;
		assert!(a.next().is_none());

		// 2
		a.queue_idx = Some(0);
		a.queue.push_front(SONG);
		a.queue.push_front(SONG);
		a.queue.push_front(SONG);
		assert_eq!(a.next(),    Some(SONG));
		assert_eq!(a.queue_idx, Some(1));
		assert_eq!(a.next(),    Some(SONG));
		assert_eq!(a.queue_idx, Some(2));

		// 3
		assert!(a.next().is_none());
		// FIXME: maybe this should be set to `None`.
		assert_eq!(a.queue_idx, Some(2));
	}

	#[test]
	// Tests `prev()` and asserts the following behavior:
	//
	// 1. `queue_idx` is `None` => Returns `None`;
	// 2. Returns the new `SongKey` if a previous is available
	// 3. Returns index `0` if at first index.
	// 4. Returns `None` if nothing in queue.
	fn prev() {
		let mut a = AudioState::new();

		// 1
		a.queue_idx = None;
		assert!(a.next().is_none());

		// 2
		a.queue_idx = Some(1);
		a.queue.push_front(SONG);
		a.queue.push_front(SONG);
		assert_eq!(a.prev(), Some(SONG));
		assert_eq!(a.queue_idx, Some(0));

		// 3
		assert_eq!(a.prev(),    Some(SONG));
		assert_eq!(a.queue_idx, Some(0));

		// 4
		a.queue.clear();
		assert!(a.prev().is_none());
	}

	#[test]
	// Compares `AudioState::new()` against A1 & A2.
	fn cmp() {
		assert_eq!(Lazy::force(&A1), &AudioState::new());
		assert_ne!(Lazy::force(&A1), Lazy::force(&A2));

		let b1 = A1.to_bytes().unwrap();
		let b2 = A2.to_bytes().unwrap();
		assert_ne!(b1, b2);
	}

	#[test]
	// Attempts to deserialize a non-empty `AudioState`.
	fn real() {
		// Assert data.
		assert_eq!(A2.queue[0],  SongKey::from(0_u8));
		assert_eq!(A2.queue[1],  SongKey::from(10_u8));
		assert_eq!(A2.queue[2],  SongKey::from(100_u8));
		assert_eq!(A2.queue_idx, Some(2));
		assert_eq!(A2.song,      Some(SongKey::from(100_u8)));
		assert_eq!(A2.elapsed,   Runtime::from(123_u16));
		assert_eq!(A2.runtime,   Runtime::from(321_u16));
		assert_eq!(A2.repeat,    Repeat::Queue);
		assert!(A2.playing);
	}
}
