//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use std::sync::Arc;
use crate::collection::{
	Collection,
	Keychain,
	QueueKey,
	Song,
};
use super::KernelState;
use rolock::RoLock;
use std::path::PathBuf;
use crate::kernel::Volume;

//---------------------------------------------------------------------------------------------------- Kernel Messages.
/// Messages `Frontend` can send to [`Kernel`]
///
/// This is the "API" that all frontends must implement
/// in order to communicate with `Festival`'s internals.
///
/// You can treat these as "commands" sent to [`Kernel`].
pub enum FrontendToKernel {
	// Audio playback.
	/// Play current song.
	Play,
	/// Stop playback.
	Stop,
	/// Play next song in queue (stop if none).
	Next,
	/// Play last song in queue.
	Last,
	/// Seek to point in current song.
	Seek(f64),

	// Audio settings.
	/// Toggle shuffling songs.
	Shuffle,
	/// Toggle repeating songs.
	Repeat,
	/// Change the audio volume.
	Volume(Volume),

	// Queue/playlist.
	/// Play the `n`'th index [`Song`] in the queue.
	PlayQueueKey(QueueKey),

	// Collection.
	/// I'd like a new [`Collection`], scanning these [`PathBuf]`'s for audio files.
	NewCollection(Vec<PathBuf>),
	/// I'd like to search the [`Collection`] with this [`String`].
	Search(String),
}

/// Messages [`Kernel`] can send to `Frontend`
///
/// This is the "API" that all frontends must implement
/// in order to communicate with `Festival`'s internals.
///
/// You can treat these as "commands" sent _from_ [`Kernel`] that you _**must**_ follow correctly.
///
/// [`Kernel`] assumes that all of these messages are implemented correctly.
///
/// # For example:
/// If your frontend does _not_ actually drop the `Arc<Collection>`
/// after receiving the message [`KernelToFrontend::DropCollection`],
/// then `Festival`'s internals will not be able to destruct the old
/// [`Collection`] correctly.
///
/// This will leave the deconstruction of the old [`Collection`] up to
/// your frontend thread, which is most likely not desired, as it will
/// probably skip a few frames or cause latency.
pub enum KernelToFrontend {
	// Collection.
	/// Drop your [`Arc`] pointer to the [`Collection`].
	DropCollection,
	/// Here's the new [`Collection`] pointer.
	NewCollection(Arc<Collection>),
	/// Here's an update on the new [`Collection`].
	Update(String),
	/// Creating the new [`Collection`] failed, here's the old pointer and error message.
	Failed((Arc<Collection>, String)),

	// Audio error.
	/// The audio file at this [`PathBuf`] has errored (probably doesn't exist).
	PathError(String),

	// Misc.
	/// Here's a new [`KernelState`] pointer.
	NewState(RoLock<KernelState>),
	/// Here's a search result
	SearchResult(Keychain),
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
