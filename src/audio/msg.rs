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
use rolock::RoLock;
use crate::audio::{
	AudioState,Volume,
};

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum AudioToKernel {
	TimestampUpdate(f64), // We've played the current song for `x` seconds.
	PathError(String),    // `Path` error occurred when trying to play a song.
}

pub(crate) enum KernelToAudio {
	// Audio playback.
	Toggle,      // Toggle playback.
	Play,        // Play currently stored audio.
	Stop,        // Stop.
	Next,        // Play next song in queue (stop if none).
	Last,        // Play last song in queue.

	// Audio settings.
	/// Toggle shuffling songs.
	Shuffle,
	/// Toggle repeating songs.
	Repeat,
	/// Change the audio volume.
	Volume(Volume),
	/// Seek to point in current song.
	Seek(f64),

	// Queue/playlist.
	PlayQueueKey(QueueKey), // Play the first song (`[0]`) in the queue.

	// Collection.
	DropCollection,                 // Drop your pointer.
	NewCollection(Arc<Collection>), // Here's a new `Collection` pointer.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
