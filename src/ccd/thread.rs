//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
use std::num::NonZeroUsize;
use crate::{
	THREADS,
	THREADS_25,
	THREADS_50,
	THREADS_75,
};

//---------------------------------------------------------------------------------------------------- Constants.
// How many albums should we _always_ process single-threaded
// until it's actually worth the cost of spawning threads?
const ALBUM_THREAD_THRESHOLD: usize = 9;

// How many PATHs should we _always_ process single-threaded
// until it's actually worth the cost of spawning threads?
const PATH_THREAD_THRESHOLD: usize = 40;

//---------------------------------------------------------------------------------------------------- Thread Functions.
// Get a reasonable amount of threads for processing `n` amount of album art.
pub(crate) fn threads_for_album_art(albums: usize) -> usize {
	// Return 1 if it's not even worth spawning
	// threads due to small amount of albums.
	if albums <= ALBUM_THREAD_THRESHOLD {
		debug!("Album threads: 1");
		return 1
	}

	// Make sure each thread has at least 1 album.
	if *THREADS_50 > albums {
		debug!("Album threads: {}", albums);
		return albums
	}

	debug!("Album threads: {}", *THREADS_50);
	*THREADS_50
}

// Get a reasonable amount of threads for processing `n` amount of PATHs.
pub(crate) fn threads_for_paths(paths: usize) -> usize {
	if paths <= PATH_THREAD_THRESHOLD {
		debug!("PATH threads: 1");
		return 1
	}

	// Make sure each thread has at least 1 PATH.
	if *THREADS_50 > paths {
		debug!("PATH threads: {}", paths);
		return paths
	}

	debug!("PATH threads: {}", *THREADS_50);
	*THREADS_50
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn _half_threads() {
		assert!(half_threads(1)  == 1);
		assert!(half_threads(2)  == 1);
		assert!(half_threads(3)  == 1);
		assert!(half_threads(4)  == 2);
		assert!(half_threads(5)  == 2);
		assert!(half_threads(6)  == 3);
		assert!(half_threads(7)  == 3);
		assert!(half_threads(8)  == 4);
		assert!(half_threads(9)  == 4);
		assert!(half_threads(10) == 5);
		assert!(half_threads(11) == 5);
		assert!(half_threads(12) == 6);
		assert!(half_threads(13) == 6);
		assert!(half_threads(14) == 7);
		assert!(half_threads(15) == 7);
		assert!(half_threads(16) == 8);
		assert!(half_threads(17) == 8);
		assert!(half_threads(18) == 9);
		assert!(half_threads(19) == 9);
		assert!(half_threads(20) == 10);
		assert!(half_threads(21) == 10);
		assert!(half_threads(22) == 11);
		assert!(half_threads(23) == 11);
		assert!(half_threads(24) == 12);
		assert!(half_threads(25) == 12);
		assert!(half_threads(26) == 13);
		assert!(half_threads(27) == 13);
		assert!(half_threads(28) == 14);
		assert!(half_threads(29) == 14);
		assert!(half_threads(30) == 15);
		assert!(half_threads(31) == 15);
		assert!(half_threads(32) == 16);
		// Who the hell is running festival on these CPUs
		assert!(half_threads(48)  == 24);
		assert!(half_threads(64)  == 32);
		assert!(half_threads(128) == 64);
		assert!(half_threads(256) == 128);
	}

	#[test]
	fn _most_threads() {
		assert!(most_threads(1)  == 1);
		assert!(most_threads(2)  == 1);
		assert!(most_threads(3)  == 2);
		assert!(most_threads(4)  == 3);
		assert!(most_threads(5)  == 3);
		assert!(most_threads(6)  == 4);
		assert!(most_threads(7)  == 5);
		assert!(most_threads(8)  == 6);
		assert!(most_threads(9)  == 6);
		assert!(most_threads(10) == 7);
		assert!(most_threads(11) == 8);
		assert!(most_threads(12) == 9);
		assert!(most_threads(13) == 9);
		assert!(most_threads(14) == 10);
		assert!(most_threads(15) == 11);
		assert!(most_threads(16) == 12);
		assert!(most_threads(17) == 12);
		assert!(most_threads(18) == 13);
		assert!(most_threads(19) == 14);
		assert!(most_threads(20) == 15);
		assert!(most_threads(21) == 15);
		assert!(most_threads(22) == 16);
		assert!(most_threads(23) == 17);
		assert!(most_threads(24) == 18);
		assert!(most_threads(25) == 18);
		assert!(most_threads(26) == 19);
		assert!(most_threads(27) == 20);
		assert!(most_threads(28) == 21);
		assert!(most_threads(29) == 21);
		assert!(most_threads(30) == 22);
		assert!(most_threads(31) == 23);
		assert!(most_threads(32) == 24);
		assert!(most_threads(48)  == 36);
		assert!(most_threads(64)  == 48);
		assert!(most_threads(128) == 96);
		assert!(most_threads(256) == 192);
	}
}
