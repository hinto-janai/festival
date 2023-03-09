//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use super::msg::{
	CcdToKernel,
	KernelToCcd,
};
use crossbeam_channel::{Sender,Receiver};
use crate::macros::{
	skip_trace,
	ok_trace,
};
use crate::collection::{
	Album,
	Collection,
	CollectionKeychain,
	ArtistKey,
	AlbumKey,
	SongKey,
	Art,
};

//---------------------------------------------------------------------------------------------------- Conversion (bytes <-> egui image) functions
impl super::Ccd {
	#[inline(always)]
	// Internal re-usable image conversion function.
	// This can be used in `new_collection()` as well.
	//
	// Order of operations:
	//     1. If multiple threads are detected -> `convert_art_multithread()`
	//     2. If single thread is detected     -> `convert_art_singlethread()`
	//     3. Return `Collection` after mutation.
	//
	pub(super) fn priv_convert_art(to_kernel: &Sender<CcdToKernel>, collection: Collection) -> Collection {
		// How many threads should we use?
		let threads = super::threads_for_albums(collection.albums.len());

		// Single-threaded.
		if threads == 1 {
			Self::convert_art_singlethread(&to_kernel, collection)
		// Multi-threaded.
		} else {
			Self::convert_art_multithread(&to_kernel, collection, threads)
		}
	}

	#[inline(always)]
	// Multi-threaded `convert_art()` variant.
	fn convert_art_multithread(to_kernel: &Sender<CcdToKernel>, mut collection: Collection, threads: usize) -> Collection {
		// Multi-thread & scoped process of `Collection` album art:
		//
		// 1. Split the `Vec` of `Album` across an appropriate amount of threads
		// 2. Move work into each scoped thread
		// 3. Process data.
		// 4. Join threads, return.
		std::thread::scope(|scope| {
			// Divide albums (mostly) evenly across threads.
			for albums in collection.albums.chunks_mut(threads) {

				// Clone `Kernel` channel.
				let to_k  = to_kernel.clone();

				// Spawn scoped thread with chunked workload.
				scope.spawn(move || {
					Self::convert_art_worker(to_kernel, albums);
				});
			}
		});

		collection
	}

	#[inline(always)]
	// Single-threaded `convert_art()` variant.
	fn convert_art_singlethread(to_kernel: &Sender<CcdToKernel>, mut collection: Collection) -> Collection {
		Self::convert_art_worker(to_kernel, &mut collection.albums);

		collection
	}

	#[inline(always)]
	// The actual art conversion "processing" work.
	fn convert_art_worker(to_kernel: &Sender<CcdToKernel>, albums: &mut [Album]) {
		for album in albums {
			// Take raw image bytes.
			let bytes = album.art_bytes.take();

			// If bytes exist, convert, else provide the `Unknown` art.
			let art = match bytes {
				Some(b) => {
					ok_trace!(album.title);
					Art::Known(super::art_from_known(&b))
				},
				None => {
					skip_trace!(album.title);
					Art::Unknown
				},
			};

			// Insert the `Art`.
			album.art = art;

			// TODO: send progress report
			// send!(to_kernel, ...);
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
