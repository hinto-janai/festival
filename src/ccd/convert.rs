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
use benri::{
	log::*,
	sync::*,
};
use crate::collection::{
	Art,
	Album,
	Collection,
};
use crate::key::{
	Keychain,
	ArtistKey,
	AlbumKey,
	SongKey,
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
	pub(super) fn priv_convert_art(
		to_kernel: &Sender<CcdToKernel>,
		collection: Collection,
		ctx: &egui::Context,
	) -> Collection {
		// How many albums total?
		let total = collection.albums.len();

		// How many threads should we use?
		let threads = super::threads_for_albums(total);

		// ResetUpdate.
		let increment = 39.0 / total as f64;

		// Single-threaded.
		if threads == 1 {
			Self::convert_art_singlethread(to_kernel, collection, ctx, total, increment)
		// Multi-threaded.
		} else {
			Self::convert_art_multithread(to_kernel, collection, ctx, threads, total, increment)
		}
	}

	#[inline(always)]
	// Multi-threaded `convert_art()` variant.
	fn convert_art_multithread(
		to_kernel: &Sender<CcdToKernel>,
		mut collection: Collection,
		ctx: &egui::Context,
		threads: usize,
		total: usize,
		increment: f64,
	) -> Collection {
		// Multi-thread & scoped process of `Collection` album art:
		//
		// 1. Split the `Vec` of `Album` across an appropriate amount of threads
		// 2. Move work into each scoped thread
		// 3. Process data.
		// 4. Join threads, return.
		std::thread::scope(|scope| {
			// Divide albums (mostly) evenly across threads.
			for albums in collection.albums.0.chunks_mut(threads) {

				// Spawn scoped thread with chunked workload.
				scope.spawn(|| {
					Self::convert_art_worker(to_kernel, albums, ctx, total, increment);
				});
			}
		});

		collection
	}

	#[inline(always)]
	// Single-threaded `convert_art()` variant.
	fn convert_art_singlethread(
		to_kernel: &Sender<CcdToKernel>,
		mut collection: Collection,
		ctx: &egui::Context,
		total: usize,
		increment: f64,
	) -> Collection {
		Self::convert_art_worker(to_kernel, &mut collection.albums.0, ctx, total, increment);

		collection
	}

	#[inline(always)]
	// The actual art conversion "processing" work.
	fn convert_art_worker(
		to_kernel: &Sender<CcdToKernel>,
		albums: &mut [Album],
		ctx: &egui::Context,
		total: usize,
		increment: f64,
	) {
		// Resizer.
		let mut resizer = crate::ccd::create_resizer();

		for album in albums {
			send!(to_kernel, CcdToKernel::UpdateIncrement((increment, format!("{}", album.title))));

			// Take raw image bytes.
			let bytes = album.art_bytes.take();

			// If bytes exist, convert, else provide the `Unknown` art.
			let art = match bytes {
				Some(b) => {
					ok_trace!("{}", album.title);
//					Art::Known(super::art_from_known(&b))
					match super::art_from_raw(&b, &mut resizer, ctx) {
						Ok(a) => Art::Known(a),
						_ => Art::Unknown,
					}
				},
				None => {
					skip_trace!("{}", album.title);
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
