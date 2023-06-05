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
use crate::audio::append::Append;
use crate::audio::shuffle::Shuffle;
use crate::audio::repeat::Repeat;

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum AudioToKernel {
	DeviceError(anyhow::Error),          // The device error'ed during initialization
	PlayError(anyhow::Error),            // There was an error while attempting to play a sound.
	SeekError(anyhow::Error),            // There was an error while attempting to seek audio.
	PathError((SongKey, anyhow::Error)), // `Path` error occurred when trying to play a song (probably doesn't exist).
}

// These mostly map to `FrontendToKernel` messages.
pub(crate) enum KernelToAudio {
	// Audio playback.
	Toggle,
	Play,
	Pause,
	Next,
	Previous,

	// Audio settings.
	Shuffle(Shuffle),
	Repeat(Repeat),
	Volume(Volume),
	Seek(usize),

	// Queue.
	AddQueueSong((SongKey, Append, bool)),
	AddQueueAlbum((AlbumKey, Append, bool, usize)),
	AddQueueArtist((ArtistKey, Append, bool, usize)),
	Skip(usize),

	// Queue Index.
	PlayQueueIndex(usize),
	RemoveQueueIndex(usize),

	// Audio State.
	RestoreAudioState,

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
