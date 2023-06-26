//---------------------------------------------------------------------------------------------------- Use
use log::{warn};
use crate::ccd::msg::CcdToKernel;
use crossbeam::channel::{Sender};
use benri::{
	log::*,
	sync::*,
};
use crate::collection::{
	Art,
	Album,
	Collection,
};

//---------------------------------------------------------------------------------------------------- Types of conversions
pub (super) enum ArtConvertType {
	// The user requested a new `Collection`,
	// and this conversion is part of a bigger reset.
	// This resizes existing `Art::Bytes`.
	Resize,

	// We're converting `Art::Bytes` -> `Art::Known`.
	ToKnown,
}

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
		collection: &mut Collection,
		art_convert_type: ArtConvertType,
		increment: f64,
		total: usize,
		threads: usize,
	) {
		// Single-threaded.
		if threads == 1 {
			Self::convert_art_singlethread(to_kernel, collection, total, increment, art_convert_type);
		// Multi-threaded.
		} else {
			Self::convert_art_multithread(to_kernel, collection, threads, total, increment, art_convert_type);
		}
	}

	#[inline(always)]
	// Multi-threaded `convert_art()` variant.
	fn convert_art_multithread(
		to_kernel: &Sender<CcdToKernel>,
		collection: &mut Collection,
		threads: usize,
		total: usize,
		increment: f64,
		art_convert_type: ArtConvertType,
	) {
		// Multi-thread & scoped process of `Collection` album art:
		//
		// 1. Split the `Vec` of `Album` across an appropriate amount of threads
		// 2. Move work into each scoped thread
		// 3. Process data.
		// 4. Join threads, return.
		let chunks  = {
			let c = collection.albums.len() / threads;
			match c {
				0 => 1,
				_ => c,
			}
		};
		std::thread::scope(|scope| {
			match art_convert_type {
				ArtConvertType::Resize => {
					// Divide albums (mostly) evenly across threads.
					for albums in collection.albums.0.chunks_mut(chunks) {
						// Spawn scoped thread with chunked workload.
						scope.spawn(|| {
							Self::resize_worker(to_kernel, albums, total, increment);
						});
					}
				},
				ArtConvertType::ToKnown => {
					for albums in collection.albums.0.chunks_mut(chunks) {
						scope.spawn(|| {
							Self::toknown_worker(to_kernel, albums, total, increment);
						});
					}
				},
			}
		});
	}

	#[inline(always)]
	// Single-threaded `convert_art()` variant.
	fn convert_art_singlethread(
		to_kernel: &Sender<CcdToKernel>,
		collection: &mut Collection,
		total: usize,
		increment: f64,
		art_convert_type: ArtConvertType,
	) {
		match art_convert_type {
			ArtConvertType::Resize  => Self::resize_worker(to_kernel, &mut collection.albums.0, total, increment),
			ArtConvertType::ToKnown => Self::toknown_worker(to_kernel, &mut collection.albums.0, total, increment),
		};
	}

	#[inline(always)]
	// The actual art conversion "processing" work.
	// This is for `ArtConvertType::Resize`.
	fn resize_worker(
		to_kernel: &Sender<CcdToKernel>,
		albums: &mut [Album],
		_total: usize,
		increment: f64,
	) {
		// Resizer.
		let mut resizer = crate::ccd::create_resizer();

		for album in albums {
			send!(to_kernel, CcdToKernel::UpdateIncrement((increment, album.title.clone())));

			// Take raw image bytes.
//			let bytes = album.art_bytes.take();

			// If bytes exist, convert, else provide the `Unknown` art.
			let art = match &mut album.art {
				Art::Bytes(b) => {
					ok_trace!("{}", album.title);

					let b = std::mem::take(b);

					match super::art_from_raw(b, &mut resizer) {
						Ok(b)  => Art::Bytes(b),
						Err(e) => { warn!("Art error: {e} ... {}", album.title); Art::Unknown },
					}
				},
				_ => {
					skip_trace!("{}", album.title);
					Art::Unknown
				},
			};

			// Insert the `Art`.
			album.art = art;
		}
	}

	#[inline(always)]
	// This is for `ArtConvertType::ToKnown`.
	fn toknown_worker(
		to_kernel: &Sender<CcdToKernel>,
		albums: &mut [Album],
		_total: usize,
		increment: f64,
	) {
		for album in albums {
			send!(to_kernel, CcdToKernel::UpdateIncrement((increment, album.title.clone())));

			// If bytes exist, convert, else provide the `Unknown` art.
			let art = match &mut album.art {
				Art::Bytes(b) => {
					ok_trace!("{}", album.title);
					let b = std::mem::take(b);
					Art::Known(super::art_from_known(b))
				},
				_ => {
					skip_trace!("{}", album.title);
					Art::Unknown
				},
			};

			// Insert the `Art`.
			album.art = art;
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
