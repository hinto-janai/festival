//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum WatchToKernel {
	// Signals.
	Toggle,   // Toggle playback.
	Play,     // Play playback (ignored if already).
	Pause,    // Pause playback (ignored if already).
	Next,     // Skip to next song in queue.
	Previous, // Skip to previous song in queue.

	// Shuffle.
	ShuffleOn,
	ShuffleOff,
	ShuffleToggle,

	// Repeat.
	RepeatSong,
	RepeatQueue,
	RepeatOff,
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
