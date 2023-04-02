//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use disk::prelude::*;
//use disk::{};
//use std::{};
use benri::{
	log::*,
	sync::*,
	thread::*,
	time::*,
};
use crate::collection::{
	Album,
	Collection,
	Artists,
	Albums,
	Songs,
	Map,
};
use crate::key::{
	Keychain,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use super::msg::{
	CcdToKernel,
	KernelToCcd,
};
use crate::kernel::{
	KernelState,
};
use crate::collection::Art;
use crossbeam_channel::{Sender,Receiver};
use std::path::{Path,PathBuf};
use std::sync::{Arc,RwLock};
use disk::Bincode;
use std::time::Instant;

//---------------------------------------------------------------------------------------------------- CCD
pub(crate) struct Ccd;

impl Ccd {
	#[inline(always)]
	//-------------------------------------------------------------------------------- CCD `convert_art()`
	// Public facing "front-end" function for image conversion.
	// Dynamically selects internal functions for single/multi-thread.
	pub(crate) fn convert_art(to_kernel: Sender<CcdToKernel>, collection: Collection) {
		debug!("CCD - Purpose in life: convert_art()");

		// If no albums, return.
		if collection.albums.is_empty() {
			send!(to_kernel, CcdToKernel::NewCollection(Arc::new(collection)));
		// Else, convert art, send to `Kernel`.
		} else {
			let collection = Arc::new(Self::priv_convert_art(&to_kernel, collection));
			send!(to_kernel, CcdToKernel::NewCollection(collection));
		}
	}

	#[inline(always)]
	//-------------------------------------------------------------------------------- CCD `new_collection()`
	// Public facing "front-end" function for making a new `Collection`.
	//
	// These operations are split up into different private
	// functions mostly for testing flexability.
	pub(crate) fn new_collection(
		to_kernel: Sender<CcdToKernel>,
		from_kernel: Receiver<KernelToCcd>,
		kernel_state: Arc<RwLock<KernelState>>,
		old_collection: Arc<Collection>,
		paths: Vec<PathBuf>,
	) {
		let beginning = now!();
		debug!("CCD - Purpose in life: new_collection()");
		// `new_collection()` high-level overview:
		// 1. WalkDir given path(s), filtering for audio files.
		// 2. For each file, append metadata to appropriate `Vec`.
		// 3. Make sure `Vec<Album>` metadata matches the songs.
		// 4. Create sorted `Key`'s.
		// 5. Create the "Map"
		// 6. Create our `Collection`.
		// 7. Transform in-memory `Collection` with `priv_convert_art()`
		// 8. Send to `Kernel`
		// 9. Wait for `Die` signal.
		// 10. Save `Collection` to disk.
		// 11. Destruct the old `Collection`.

		// TODO: Handle potential errors:
		// 1. No albums
		// 2. Path error
		// 3. Permission error
		// 4. Disk error

		// TODO: Send updates to `Kernel` throughout and `log!()`.

		// 1.
		let now = now!();
		let paths = Self::walkdir_audio(&to_kernel, paths);
		debug!("CCD [1/11] - WalkDir: {}", secs_f64!(now));

		// 2.
		let now = now!();
		let (vec_artist, mut vec_album, vec_song) = Self::audio_paths_to_incomplete_vecs(&to_kernel, paths);
		debug!("CCD [2/11] - Metadata: {}", secs_f64!(now));

		// 3.
		let now = now!();
		Self::fix_album_metadata_from_songs(&mut vec_album, &vec_song);
		debug!("CCD [3/11] - Fix: {}", secs_f64!(now));

		// 4.
		let now = now!();
		let sort_artist_lexi                    = Self::sort_artist_lexi(&vec_artist);
		let sort_artist_album_count             = Self::sort_artist_album_count(&vec_artist);
		let sort_artist_song_count              = Self::sort_artist_song_count(&vec_artist, &vec_album);
		//--
		let sort_album_release_artist_lexi      = Self::sort_album_release_artist_lexi(&sort_artist_lexi, &vec_artist, &vec_album);
		let sort_album_lexi_artist_lexi         = Self::sort_album_lexi_artist_lexi(&sort_artist_lexi, &vec_artist, &vec_album);
		let sort_album_lexi                     = Self::sort_album_lexi(&vec_album);
		let sort_album_release                  = Self::sort_album_release(&vec_album);
		let sort_album_runtime                  = Self::sort_album_runtime(&vec_album);
		//--
		let sort_song_album_release_artist_lexi = Self::sort_song_iterating_over_albums(&sort_album_release_artist_lexi, &vec_artist, &vec_album);
		let sort_song_album_lexi_artist_lexi    = Self::sort_song_iterating_over_albums(&sort_album_lexi_artist_lexi, &vec_artist, &vec_album);
		let sort_song_lexi                      = Self::sort_song_lexi(&vec_song);
		let sort_song_release                   = Self::sort_song_iterating_over_albums(&sort_album_release, &vec_artist, &vec_album);
		let sort_song_runtime                   = Self::sort_song_runtime(&vec_song);
		debug!("CCD [4/11] - Sort: {}", secs_f64!(now));

		// 5.
		let now = now!();
		let map = Map::from_3_vecs(&vec_artist, &vec_album, &vec_song);
		debug!("CCD [5/11] - Map: {}", secs_f64!(now));

		// 6.
		let now = now!();
		let collection = Collection {
			// These will be fixed after construction.
			empty: false,
			timestamp: 0,
			count_artist: 0,
			count_album: 0,
			count_song: 0,

			map,

			artists: Artists::from(vec_artist),
			albums: Albums::from(vec_album),
			songs: Songs::from(vec_song),

			sort_artist_lexi,
			sort_artist_album_count,
			sort_artist_song_count,

			sort_album_release_artist_lexi,
			sort_album_lexi_artist_lexi,
			sort_album_lexi,
			sort_album_release,
			sort_album_runtime,

			sort_song_album_release_artist_lexi,
			sort_song_album_lexi_artist_lexi,
			sort_song_lexi,
			sort_song_release,
			sort_song_runtime,
		};
		// Fix metadata.
		let collection = collection.set_metadata();
		debug!("CCD [6/11] - Collection: {}", secs_f64!(now));

		// 7.
		let now = now!();
		let collection = Self::priv_convert_art(&to_kernel, collection);
		debug!("CCD [7/11] - Image: {}", secs_f64!(now));

		// 8.
		let now = now!();
		let collection = Arc::new(collection);
		send!(to_kernel, CcdToKernel::NewCollection(Arc::clone(&collection)));
		debug!("CCD [8/11] - ToKernel: {}", secs_f64!(now));

		// 9.
		let now = now!();
		match recv!(from_kernel) {
			KernelToCcd::Die => debug!("CCD [9/11] - Die: {}", secs_f64!(now)),
		}

		// 10.
		let now = now!();
		// Set `saving` state.
		lock_write!(kernel_state).saving = true;
		// Attempt atomic save.
		if let Err(e) = collection.save_atomic() {
			fail!("CCD - Collection write to disk: {}", e);
		}
		// Set `saving` state.
		lock_write!(kernel_state).saving = false;
		debug!("CCD [10/11] - Disk: {}", secs_f64!(now));
		// Don't need these anymore.
		drop(kernel_state);
		drop(collection);

		// 11.
		// Try 3 times before giving up.
		for i in 1..=4 {
			if i == 4 {
				error!("CCD [11/11] - Someone else is pointing to the old Collection...! I can't deconstruct it noooooooooo~");
				break
			}

			if Arc::strong_count(&old_collection) == 1 {
				let now = now!();
				drop(old_collection);
				debug!("CCD [11/11] - Deconstruct: {}", secs_f64!(now));
				break
			}

			warn!("CCD [11/11] Attempt ({}/3) - Failed to deconstruct old Collection...", i);
			sleep!(1000); // Sleep 1 second.
		}

		// Thank you CCD, you can rest now.
		debug!("CCD ... Took {} seconds, bye!", beginning.elapsed().as_secs_f32());
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use crate::ccd::*;
	use std::path::PathBuf;
	use disk::Bincode;

	#[test]
	#[ignore]
	fn serialize_and_convert_collection() {
		// Set-up logger.
		crate::logger::init_logger(log::LevelFilter::Trace);

		// Set-up inputs.
		let (to_kernel, from_ccd) = crossbeam_channel::unbounded::<CcdToKernel>();

		// Serialize.
		let now = now!();
		let collection = Collection::from_file().unwrap();
		info!("Read: {}", secs_f64!(now));

		// Convert.
		let now = now!();
		Ccd::convert_art(to_kernel, collection);
		let collection = match from_ccd.recv().unwrap() {
			CcdToKernel::NewCollection(c) => c,
			_ => panic!("wrong msg received"),
		};
		info!("Convert: {}", secs_f64!(now));

		// Print.
		info!("{:#?}", collection);

		// Assert.
		assert!(collection.count_artist >= 1);
		assert!(collection.count_album  >= 1);
		assert!(collection.count_song   >= 1);
		assert!(collection.timestamp >= 1);
		assert!(collection.empty == false);
	}

	#[test]
	#[ignore]
	fn new_collection() {
		// Set-up logger.
		crate::logger::init_logger(log::LevelFilter::Trace);

		// Set-up inputs.
		let (to_kernel, from_ccd) = crossbeam_channel::unbounded::<CcdToKernel>();
		let (to_ccd, from_kernel) = crossbeam_channel::unbounded::<KernelToCcd>();
		let kernel_state   = Arc::new(RwLock::new(KernelState::new()));
		let old_collection = Arc::new(Collection::new());
		let paths = vec![
			PathBuf::from("src"),
			PathBuf::from("assets"),
			PathBuf::from("assets"),
			PathBuf::from("assets/audio"),
			PathBuf::from("assets/images"),
			PathBuf::from("assets/albums"),
		];

		// Spawn `CCD`.
		let old_clone = Arc::clone(&old_collection);
		std::thread::spawn(move || Ccd::new_collection(to_kernel, from_kernel, kernel_state, old_clone, paths));

		// Act as `Kernel`.
		// Receive.
		let collection = loop {
			match recv!(from_ccd) {
				CcdToKernel::NewCollection(collection) => break collection,
				CcdToKernel::Failed(error)             => panic!("{}", error),
				CcdToKernel::Update(string)            => eprintln!("{}", string),
			}
		};

		// Send `Die` signal.
		drop(old_collection);
		send!(to_ccd, KernelToCcd::Die);

		crate::macros::sleep!(5000);
		info!("empty        | {}", collection.empty);
		info!("timestamp    | {}", collection.timestamp);
		info!("count_artist | {}", collection.count_artist);
		info!("count_album  | {}", collection.count_album);
		info!("count_song   | {}", collection.count_song);

		assert!(collection.empty == false);
		assert!(collection.timestamp > 1678382892);
		assert!(collection.count_artist == 1 || collection.count_artist == 501);
		assert!(collection.count_album  == 1 || collection.count_album  == 501);
		assert!(collection.count_song   == 4 || collection.count_song   == 505);
	}
}

