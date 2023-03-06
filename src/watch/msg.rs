//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum WatchToKernel {
	// Signals.
	Play,    // Play playback (ignored if already).
	Stop,    // Pause playback (ignored if already).
	Next,    // Skip to next song in queue.
	Last,    // Skip to last song in queue.
	Shuffle, // Toggles shuffle.
	Repeat,  // Toggles repeat.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
