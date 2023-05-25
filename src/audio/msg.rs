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
	Collection,Keychain,QueueKey,
	ArtistKey,AlbumKey,SongKey,
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

// These mostly map to `FrontendToKernel` messages.
pub(crate) enum KernelToAudio {
	// Audio playback.
	Toggle,
	Play,
	Stop,
	Next,
	Last,

	// Audio settings.
	Shuffle,
	Repeat,
	Volume(Volume),
	Seek(u32),

	// Queue.
	AddQueueSongFront(SongKey),
	AddQueueSongBack(SongKey),
	AddQueueAlbumFront(AlbumKey),
	AddQueueAlbumBack(AlbumKey),
	AddQueueArtistFront(ArtistKey),
	AddQueueArtistBack(ArtistKey),

	// Queue Index.
	PlayQueueIndex(QueueKey),
	RemoveQueueIndex(QueueKey),

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
