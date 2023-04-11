//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
//use std::{};
use std::sync::{Arc,RwLock};
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
use crate::collection::Collection;
use readable::Percent;

//---------------------------------------------------------------------------------------------------- Lazy
lazy_static::lazy_static! {
	// This is an empty, dummy `KernelState`.
	pub(crate) static ref DUMMY_KERNEL_STATE: Arc<RwLock<KernelState>> = Arc::new(RwLock::new(KernelState::new()));
}

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

//---------------------------------------------------------------------------------------------------- ResetState
/// Reset State.
///
/// Some in-progress status updates for when the [`Collection`] is reset.
///
/// This holds a:
/// - [`Percent`] representing the total work done out of `100%`
/// - [`String`] representing what work we're currently doing
///
/// This values in this struct will be updated during the process.
///
/// [`None`] represents we're not currently reseting the [`Collection`].
#[derive(Clone,Debug,PartialOrd,PartialEq,Serialize,Deserialize)]
pub struct ResetState {
	/// [`bool`] representing: Are we currently resetting the [`Collection`]?
	pub resetting: bool,

	/// [`Percent`] representing the total work done out of `100%`
	pub percent: Percent,

	/// [`String`] representing what phase we're on
	///
	/// Example: `Walking Directories`, `Parsing Metadata`, etc.

	pub phase: String,

	/// [`String`] representing the specific work we're currently doing
	///
	/// Example: Current `Artist/Album/Song`.
	pub specific: String,
}

impl ResetState {
	/// Returns an initial starting version.
	pub(super) fn start() -> Self {
		Self {
			resetting: true,
			percent: Percent::zero(),
			phase: "Starting...".to_string(),
			specific: String::new(),
		}
	}

	/// Resets, use this after we're done.
	pub(super) fn done() -> Self {
		Self {
			resetting: false,
			percent: Percent::const_100(),
			phase: "Done".to_string(),
			specific: String::new(),
		}
	}
}

impl Default for ResetState {
	fn default() -> Self {
		Self {
			resetting: false,
			percent: Percent::zero(),
			phase: String::new(),
			specific: String::new(),
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

	/// Reset State.
	///
	/// Some in-progress status updates for when the [`Collection`] is reset.
	///
	/// This holds a:
	/// - [`bool`] representing if we're _currently_ in the process of resetting the [`Collection`]
	/// - [`Percent`] representing the total work done out of `100%`
	/// - [`String`] representing what phase we're on
	/// - [`String`] representing what work we're currently doing
	///
	/// This values in this struct will be updated during the process.
	pub reset: ResetState,
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

			saving: false,

			reset: ResetState::default(),
		}
	}

	#[inline(always)]
	/// Obtain an empty, dummy [`Collection`] wrapped in an [`Arc`].
	///
	/// This is useful when you need to initialize but don't want
	/// to wait on [`Kernel`] to hand you the _real_ `RoLock<KernelState>`.
	///
	/// This is implemented in the exact same way as [`Collection::dummy`].
	///
	/// For more information, read that documentation.
	pub fn dummy() -> RoLock<Self> {
		RoLock::new(DUMMY_KERNEL_STATE)
	}
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
