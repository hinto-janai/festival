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


// TODO:
// CCD is a oneshot thing. Kernel sends 1 command and that's it.
// There's no need for doing these generic message channels since Kernel
// knows the exact context. Remove `from_kernel` and `old_collection` and
// just pass the needed data directly from `Kernel` in a function instead.





//---------------------------------------------------------------------------------------------------- CCD
pub struct Ccd;

//---------------------------------------------------------------------------------------------------- CCD `convert_art()`
impl Ccd {
	#[inline(always)]
	pub fn convert_art(to_kernel: Sender<CcdToKernel>, collection: Collection) {
		ok_debug!("CCD - Purpose in life: convert_art()");

		// TODO: convert art, send to kernel.
	}

	#[inline(always)]
	// Re-usable image conversion function.
	// This can be used in `new_collection()` as well.
	fn priv_convert_art(to_kernel: Sender<CcdToKernel>, collection: Collection) -> Result<Collection, anyhow::Error> {
		// Use half available threads.
		let threads = super::get_half_threads();

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

//---------------------------------------------------------------------------------------------------- CCD `new_collection()`
	pub fn new_collection(
		to_kernel: Sender<CcdToKernel>,
		from_kernel: Receiver<KernelToCcd>,
	) {
		/* TODO: create new collection */
		ok_debug!("CCD - Purpose in life: new_collection()");
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
