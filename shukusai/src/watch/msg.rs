use crate::audio::Volume;

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum WatchToKernel {
	// Signals.
	Toggle,   // Toggle playback.
	Play,     // Play playback (ignored if already).
	Pause,    // Pause playback (ignored if already).
	Next,     // Skip to next song in queue.
	Previous, // Skip to previous song in queue.
	Stop,     // Clear queue and stop playback.

	// Shuffle.
	Shuffle,

	// Repeat.
	RepeatSong,
	RepeatQueue,
	RepeatOff,

	// Content signals.
	Volume(Volume),
	Clear(bool),
	Seek(u64),
	SeekForward(u64),
	SeekBackward(u64),
	Index(usize),
	Skip(usize),
	Back(usize),
//	ArtistKey(usize),
//	AlbumKey(usize),
//	SongKey(usize),
//	Artist(String),
//	Album(String),
//	Song(String),
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
