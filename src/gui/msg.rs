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
};
use crate::kernel::State;
use rolock::RoLock;
use std::path::PathBuf;
use crate::kernel::Volume;

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum GuiToKernel {
	// Audio playback.
	Play,       // Play current song.
	Stop,       // Stop.
	Next,       // Play next song in queue (stop if none).
	Last,       // Play last song in queue.
	Seek(f64),  // Seek to point in current song.

	// Audio settings.
	Shuffle,        // Toggle shuffling songs.
	Repeat,         // Toggle repeating songs.
	Volume(Volume), // Change the audio volume.

	// Queue/playlist.
	PlayQueueKey(QueueKey), // Play the first song (`[0]`) in the queue.

	// Collection.
	NewCollection(Vec<PathBuf>), // I'd like to reset the `Collection` from these `PATH`'s.
	Search(String),              // I'd like to search the `Collection`.
}

pub(crate) enum KernelToGui {
	// Collection.
	DropCollection,                    // Drop your pointer.
	NewCollection(Arc<Collection>),    // Here's the new `Collection` pointer.
	Update(String),                    // Here's an update on the new `Collection`.
	Failed((Arc<Collection>, String)), // Creating the new `Collection` failed, here's the old pointer and error message.

	// Audio error.
	PathError(String), // The audio file at this `PATH` has errored (probably doesn't exist).

	// Misc.
	NewState(RoLock<State>),           // Here's a new `State` pointer.
	SearchResult(Keychain),            // Here's a search result
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
