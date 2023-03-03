//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use crate::macros::{
	ok_debug,
	recv,
	send,
};
use crate::collection::{
	Collection,
	CollectionKeychain,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use super::msg::{
	CcdToKernel,
	KernelToCcd,
};
use crate::collection::UNKNOWN_ALBUM_BYTES;
use crossbeam_channel::{Sender,Receiver};
use std::sync::{Arc,Mutex};
use std::path::Path;

//---------------------------------------------------------------------------------------------------- CCD
pub struct Ccd;

//---------------------------------------------------------------------------------------------------- CCD `convert_art()`
impl Ccd {
	#[inline(always)]
	// Public facing "front-end" to image conversion.
	// Dynamically selects internal functions for single/multi-thread.
	pub fn convert_art(to_kernel: Sender<CcdToKernel>, collection: Collection) {
		ok_debug!("CCD - Purpose in life: convert_art()");

		// If no albums, return.
		if collection.albums.len() == 0 {
			send!(to_kernel, CcdToKernel::NewCollection(collection));
			return
		}

		// Convert art, send to `Kernel`.
		match Self::priv_convert_art(&to_kernel, collection) {
			Ok(collection) => send!(to_kernel, CcdToKernel::NewCollection(collection)),
			Err(error)     => send!(to_kernel, CcdToKernel::Failed(error)),
		}
	}

	#[inline(always)]
	// Internal re-usable image conversion function.
	// This can be used in `new_collection()` as well.
	//
	// Order of operations:
	//     1. If multiple threads are detected -> `priv_convert_art_multithread()`
	//     2. If single thread is detected     -> `priv_convert_art_singlethread()`
	//     3. Get `Collection` back (wrapped in `Result`), return to `Kernel`
	//
	fn priv_convert_art(to_kernel: &Sender<CcdToKernel>, collection: Collection) -> Result<Collection, anyhow::Error> {
		// Use half available threads.
		let threads = super::get_half_threads();

		// Single-thread never fails.
		if threads == 1 {
			return Ok(Self::priv_convert_art_singlethread(&to_kernel, collection));
		}

		// Workload needs to be wrapped in `Arc<Mutex<T>>` IF multi-threaded (it might fail).
		match Self::priv_convert_art_multithread(&to_kernel, collection, threads) {
			Ok(collection) => Ok(collection),
			Err(error)     => Err(anyhow!("")),
		}
	}

	#[inline(always)]
	fn priv_convert_art_multithread(to_kernel: &Sender<CcdToKernel>, collection: Collection, threads: usize) -> Result<Collection, anyhow::Error> {
		// How many albums are we processing?
		let album_count = collection.albums.len();

		// TODO: document
		let collection      = Arc::new(Mutex::new(collection));
		let divided_albums  = album_count / threads;
		let remainder       = album_count % threads;
		let mut start_index = 0;
		let mut end_index   = start_index + divided_albums;
		let mut workers     = vec![];

		// TODO: document
		for thread in 1..=threads {
			let c     = Arc::clone(&collection);
			let to_k  = to_kernel.clone();
			let start = start_index;
			let end   = end_index;

			// Spawn worker threads.
			let worker = std::thread::spawn(move || {
				for index in start..end {
					// Take raw image bytes.
					let bytes = c.lock().unwrap().albums[index].art_bytes.take();

					// If `None`, provide the `?` art.
					let art = match bytes {
						Some(b) => super::art_from_known(&b),
						None    => super::art_from_known(*UNKNOWN_ALBUM_BYTES),
					};

					// Insert the `RetainedImage`.
					c.lock().unwrap().albums[index].art = Some(art);

					/* TODO: send progress report     send!(to_k)   */
				}
			});

			// TODO: document
			workers.push(worker);

			// Update start/end indexes for next thread OR
			// if the last thread, make sure we include everything (remainder).
			if thread == threads {
				start_index += divided_albums + remainder;
				end_index   += divided_albums + remainder;
			} else {
				start_index += divided_albums;
				end_index   += divided_albums;
			}
		}

		// TODO: handle error message.
		// Join workers.
		for worker in workers {
			if let Err(e) = worker.join() {
				bail!("");
			}
		}

		// TODO: handle error message.
		// Unwrap the `Arc`.
		let collection = match Arc::try_unwrap(collection) {
			Ok(c)  => c,
			Err(e) => bail!(""),
		};

		// TODO: handle error message.
		// Unwrap the `Mutex`, return `Collection`.
		match collection.into_inner() {
			Ok(c)  => Ok(c),
			Err(e) => bail!(""),
		}
	}

	#[inline(always)]
	fn priv_convert_art_singlethread(to_kernel: &Sender<CcdToKernel>, mut collection: Collection) -> Collection {
		for mut album in collection.albums.iter_mut() {
			// Take raw image bytes.
			let bytes = album.art_bytes.take();

			// If `None`, provide the `?` art.
			let art = match bytes {
				Some(b) => super::art_from_known(&b),
				None    => super::art_from_known(*UNKNOWN_ALBUM_BYTES),
			};

			// Insert the `RetainedImage`.
			album.art = Some(art);

			/* TODO: send progress report     send!(to_k)   */
		}

		collection
	}

//---------------------------------------------------------------------------------------------------- CCD `new_collection()`
	pub fn new_collection<P>(
		to_kernel: Sender<CcdToKernel>,
		from_kernel: Receiver<KernelToCcd>,
		paths: &[&P],
	) where
		P: AsRef<Path>
	{
		ok_debug!("CCD - Purpose in life: new_collection()");

		// TODO: new_collection() steps:
		// 1. WalkDir given path(s).
		// 2. Filter for audio files.
		// 3. For each file, get metadata, add to `Collection`.
		// 4.
		//     a) If image metadata exists, append
		//     b) If not, search parent dir for `jpeg/png`
		//     c) Given multiple images, pick the highest quality image
		//     d) Given no image, append `None`
		//
		// 5. Save to disk.
		// 6. Transform in-memory `Collection` with `priv_convert_art()`
		// 7. Send to `Kernel`
		// 8. Wait for `Die` signal.
		// 9. Die, destruct the old `Collection`.

		// TODO: Handle potential errors:
		// 1. No albums
		// 2. Path error
		// 3. Permission error
		// 4. Disk error
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
