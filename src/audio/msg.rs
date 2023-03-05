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
	CollectionKeychain,
	QueueKey,
};
use crate::kernel::State;
use rolock::RoLock;

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum AudioToKernel {
	TimestampUpdate(f32), // We've played the current song for `x` seconds.
	PathError(String),    // `Path` error occured when trying to play a song.
}

pub(crate) enum KernelToAudio {
	// Audio playback.
	Play,       // Play currently stored audio.
	Stop,       // Stop.
	Next,       // Play next song in queue (stop if none).
	Last,       // Play last song in queue.
	Seek(f32),  // Seek to point in current song.

	// Queue/playlist.
	PlayQueueKey(QueueKey), // Play the first song (`[0]`) in the queue.

	// Collection.
	DropCollection,                 // Drop your pointer.
	NewCollection(Arc<Collection>), // Here's a new `Collection` pointer.
	NewState(RoLock<State>),        // Here's a new `State` pointer.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
