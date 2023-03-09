//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
use std::num::NonZeroUsize;

//---------------------------------------------------------------------------------------------------- Constants.
const ONE_THREAD: usize = 1;
// How many albums should we _always_ process single-threaded
// until it's actually worth the cost of spawning threads?
const ALBUM_THREAD_THRESHOLD: usize = 10;

// How many PATHs should we _always_ process single-threaded
// until it's actually worth the cost of spawning threads?
const PATH_THREAD_THRESHOLD: usize = 40;

//---------------------------------------------------------------------------------------------------- Thread Functions.
// Get a reasonable amount of threads for processing `n` amount of albums.
pub(crate) fn threads_for_albums(albums: usize) -> usize {
	// Return 1 if it's not even worth spawning
	// threads due to small amount of albums.
	if albums <= ALBUM_THREAD_THRESHOLD {
		return ONE_THREAD
	}

	let threads = most_threads(available_threads());

	// Make sure each thread has at least 1 album.
	if threads > albums {
		return albums
	}

	threads
}

// Get a reasonable amount of threads for processing `n` amount of PATHs.
pub(crate) fn threads_for_paths(paths: usize) -> usize {
	if paths <= PATH_THREAD_THRESHOLD {
		return ONE_THREAD
	}

	let threads = quarter_threads(available_threads());

	// Make sure each thread has at least 1 PATH.
	if threads > paths {
		return paths
	}

	threads
}

fn available_threads() -> usize {
	match std::thread::available_parallelism() {
		Ok(t)  => t.get(),
		Err(_) => {
			warn!("Available thread function failed, defaulting to 1!");
			ONE_THREAD
		}
	}
}

fn quarter_threads(threads: usize) -> usize {
	match threads {
		// Special cases (low thread-count).
		1|2|3|4 => return 1,
		5|6|7|8 => return 2,
		9|10|11|12|13|14|15 => return 3,

		// Around 25%.
		_ => (threads as f64 * 0.25).floor() as usize,
	}
}

fn most_threads(threads: usize) -> usize {
	match threads {
		// Special cases (low thread-count).
		1 => return 1,
		2 => return 1,
		3 => return 2,
		4 => return 3,

		// Around 75%.
		_ => (threads as f64 * 0.75).floor() as usize,
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn _quarter_threads() {
		for i in 1..=4 {
			assert!(quarter_threads(i)  == 1);
		}
		for i in 5..=8 {
			assert!(quarter_threads(i)  == 2);
		}
		for i in 9..=15 {
			assert!(quarter_threads(i)  == 3);
		}
		for i in 16..=19 {
			assert!(quarter_threads(i)  == 4);
		}
		for i in 20..=23 {
			assert!(quarter_threads(i)  == 5);
		}
		for i in 24..=27 {
			assert!(quarter_threads(i)  == 6);
		}
		for i in 28..=31 {
			assert!(quarter_threads(i)  == 7);
		}
		assert!(quarter_threads(32)  == 8);
		// Who the hell is running festival on these CPUs
		assert!(quarter_threads(48)  == 12);
		assert!(quarter_threads(64)  == 16);
		assert!(quarter_threads(128) == 32);
		assert!(quarter_threads(256) == 64);
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
