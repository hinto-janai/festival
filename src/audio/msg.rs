//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use std::sync::Arc;
use crate::collection::Collection;
use crate::key::{
	Keychain,
	QueueKey,
};
use crate::kernel::KernelState;
use rolock::RoLock;

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum AudioToKernel {
	TimestampUpdate(f64), // We've played the current song for `x` seconds.
	PathError(String),    // `Path` error occured when trying to play a song.
}

pub(crate) enum KernelToAudio {
	// Audio playback.
	Toggle,      // Toggle playback.
	Play,        // Play currently stored audio.
	Stop,        // Stop.
	Next,        // Play next song in queue (stop if none).
	Last,        // Play last song in queue.
	Seek(f64),   // Seek to point in current song.
	Volume(u8),  // Change the volume.

	// Queue/playlist.
	PlayQueueKey(QueueKey), // Play the first song (`[0]`) in the queue.

	// Collection.
	DropCollection,                 // Drop your pointer.
	NewCollection(Arc<Collection>), // Here's a new `Collection` pointer.
	NewState(RoLock<KernelState>),  // Here's a new `KernelState` pointer.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
