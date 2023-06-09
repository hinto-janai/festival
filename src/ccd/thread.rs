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
		debug!("CCD ... Album threads: 1");
		return 1
	}

	// Make sure each thread has at least 1 album.
	if *THREADS_50 > albums {
		debug!("CCD ... Album threads: {}", albums);
		return albums
	}

	debug!("CCD ... Album threads: {}", *THREADS_50);
	*THREADS_50
}

// Get a reasonable amount of threads for processing `n` amount of PATHs.
pub(crate) fn threads_for_paths(paths: usize) -> usize {
	if paths <= PATH_THREAD_THRESHOLD {
		debug!("CCD ... PATH threads: 1");
		return 1
	}

	// Make sure each thread has at least 1 PATH.
	if *THREADS_50 > paths {
		debug!("CCD ... PATH threads: {}", paths);
		return paths
	}

	debug!("CCD ... PATH threads: {}", *THREADS_50);
	*THREADS_50
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

//	#[test]
//	fn _half_threads() {
//	}
}
