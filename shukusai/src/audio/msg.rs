//---------------------------------------------------------------------------------------------------- Use
use std::sync::Arc;
use crate::{
	collection::{
		Collection,
		ArtistKey,AlbumKey,SongKey,
	},
	audio::{
		Append,
		Repeat,
		Seek,
		Volume,
	},
};

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
	Previous(Option<u32>),

	// Audio settings.
	Repeat(Repeat),
	Volume(Volume),

	// Queue.
	AddQueueSong((SongKey, Append, bool)),
	AddQueueAlbum((AlbumKey, Append, bool, usize)),
	AddQueueArtist((ArtistKey, Append, bool, usize)),
	Shuffle,
	Clear(bool),
	Seek((Seek, u64)),
	Skip(usize),
	Back(usize),

	// Queue Index.
	SetQueueIndex(usize),
	RemoveQueueRange((std::ops::Range<usize>, bool)),

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
