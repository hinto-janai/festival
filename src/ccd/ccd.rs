//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
//use serde::{Serialize,Deserialize};
//use disk::prelude::*;
//use disk::{};
//use std::{};
use benri::{
	log::*,
	sync::*,
	thread::*,
	time::*,
	drop,
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
use crate::kernel::{
	Phase,
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
use disk::Bincode2;
use readable::{
	Unsigned,
	Percent,
};
use super::convert::{
	ArtConvertType,
};

//---------------------------------------------------------------------------------------------------- CCD
pub(crate) struct Ccd;

impl Ccd {
	#[inline(always)]
	//-------------------------------------------------------------------------------- CCD `convert_art()`
	// Public facing "front-end" function for image conversion.
	// Dynamically selects internal functions for single/multi-thread.
	pub(crate) fn convert_art(
		to_kernel: Sender<CcdToKernel>,
		collection: Collection,
		ctx: egui::Context,
	) {
		let beginning = now!();
		debug!("CCD - Purpose in life: convert_art()");

		// If no albums, return.
		if collection.albums.is_empty() {
			send!(to_kernel, CcdToKernel::NewCollection(Arc::new(collection)));
		// Else, convert art, send to `Kernel`.
		} else {
			let increment = 99.0 / collection.albums.len() as f64;
			let collection = Arc::new(Self::priv_convert_art(&to_kernel, collection, ArtConvertType::ToKnown, increment));
			// FIXME:
			// See below `new_collection()` FIXME for info.
			send!(to_kernel, CcdToKernel::UpdatePhase((100.00, Phase::Finalize)));
//			super::alloc_textures_no_sleep(&collection.albums, &ctx);
			super::alloc_textures(&collection.albums, &ctx);
			send!(to_kernel, CcdToKernel::NewCollection(collection));
		}

		debug!("CCD ... Took {} seconds, bye!", secs_f64!(beginning));
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
		ctx: egui::Context,
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
		// 7. Transform in-memory `Collection` album art to bytes
		// 8. Clone `Collection` for later saving
		// 9. Transform in-memory `Collection` album art bytes to `egui`'s `RetainedImage`
		// 10. Pre-allocate the `RetainedImage`'s into `egui`
		// 11. Send to `Kernel`
		// 12. Wait for `Die` signal.
		// 13. Save `Collection` to disk.
		// 14. Destruct the old `Collection`.

		// 1.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((0.00, Phase::WalkDir)));
		let paths = Self::walkdir_audio(&to_kernel, paths);
		debug!("CCD [1/14] - WalkDir: {}", secs_f64!(now));

		// 2.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((5.00, Phase::Parse)));
		let (vec_artist, mut vec_album, vec_song) = Self::audio_paths_to_incomplete_vecs(&to_kernel, paths);
		// Update should be < 50% at this point.
		debug!("CCD [2/14] - Metadata: {}", secs_f64!(now));

		// 3.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((50.00, Phase::Fix)));
		Self::fix_album_metadata_from_songs(&mut vec_album, &vec_song);
		debug!("CCD [3/14] - Fix: {}", secs_f64!(now));

		// 4.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((52.50, Phase::Sort)));
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
		debug!("CCD [4/14] - Sort: {}", secs_f64!(now));

		// 5.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((55.00, Phase::Search)));
		let map = Map::from_3_vecs(&vec_artist, &vec_album, &vec_song);
		debug!("CCD [5/14] - Map: {}", secs_f64!(now));

		// 6.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((60.00, Phase::Prepare)));
		let collection = Collection {
			// These will be fixed after construction.
			empty: false,
			timestamp: 0,
			count_artist: Unsigned::zero(),
			count_album: Unsigned::zero(),
			count_song: Unsigned::zero(),

			map,

			artists: Artists::from_vec(vec_artist),
			albums: Albums::from_vec(vec_album),
			songs: Songs::from_vec(vec_song),

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
		debug!("CCD [6/14] - Collection: {}", secs_f64!(now));

		// 7.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((60.00, Phase::Art)));
		let increment = 30.0 / collection.albums.len() as f64;
		let collection = Self::priv_convert_art(&to_kernel, collection, ArtConvertType::Resize, increment);
		// Update should be <= 90% at this point.
		debug!("CCD [7/14] - Resize: {}", secs_f64!(now));

		// 8.
		// FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME
		// We need to serialize `Collection` and save to disk while we
		// have the art bytes as an actual `Vec<u8>`. `egui` does not
		// make it easy to retrieve the original bytes after you turn
		// it into a `RetainedImage`, specifically, you have to access:
		//
		// `Art` -> `RetainedImage` -> `TextureHandle` -> `TextureManager` ->
		// `TextureDelta` -> `ImageDelta` -> `ImageData` -> `ColorImage` which
		// can finally be serialized by serde.
		//
		// In this conversion, there are multiple locks, unwraps and some
		// fields aren't public so I'd have to fork epaint and make things `pub`.
		//
		// Instead of doing this, we `.clone()` the `Collection` before
		// converting the `Art`. `CCD` will save this copy to disk later on.
		//
		// This is terrible. We're using 2x the memory we should be using.
		//
		// Q. Why not save right now?
		// A. We want to return to the user
		//    as soon as possible, even if
		//    it means being sneaky and saving
		//    the `Collection` to disk in the
		//    background while they are
		//    accessing it in the `GUI`.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((90.00, Phase::Clone)));
		let collection_for_disk = collection.clone();
		debug!("CCD [8/14] - Clone: {}", secs_f64!(now));
		// FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME

		// 9.
		// FIXME:
		// Consider using a threadpool.
		//
		// We're spinning up threads 3 times:
		// - "The Loop"
		// - Art Resize
		// - Art ToKnown
		//
		// Spawning threads is surprisingly fast but still.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((95.00, Phase::Convert)));
		let increment = 4.0 / collection.albums.len() as f64;
		let collection = Self::priv_convert_art(&to_kernel, collection, ArtConvertType::ToKnown, increment);
		// Update should be <= 99% at this point.
		debug!("CCD [9/14] - Convert: {}", secs_f64!(now));

		// 10.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((100.00, Phase::Finalize)));
		// FIXME:
		// This is a huge workaround around `egui`'s lack of bulk texture
		// allocation. Although this is good enough for now, figure out
		// how to bulk allocate all these images without causing the GUI
		// to freeze. Currently it's done with `try_write()` which doesn't
		// starve GUI, but can take way longer (0.00008 secs -> 1.xx secs...!!!).
		super::alloc_textures(&collection.albums, &ctx);
		debug!("CCD [10/14] - Textures: {}", secs_f64!(now));

		// 11.
		let now = now!();
		send!(to_kernel, CcdToKernel::NewCollection(Arc::new(collection)));
		debug!("CCD [11/14] - ToKernel: {}", secs_f64!(now));

		debug!("CCD - Created Collection and sent to Kernel: {}", secs_f64!(beginning));

		// 12.
		let now = now!();
		match recv!(from_kernel) {
			KernelToCcd::Die => debug!("CCD [12/14] - Die: {}", secs_f64!(now)),
		}

		// 13.
		let now = now!();
		// Set `saving` state.
		lock_write!(kernel_state).saving = true;
		// Attempt atomic save.
		//
		// SAFETY:
		// `Collection` is saved to disk via `memmap`.
		//
		// We (`CCD`) are the only "entity" that should
		// be touching `collection.bin` at this point.
		if let Err(e) = unsafe { collection_for_disk.save_atomic_memmap() } {
			fail!("CCD - Collection write to disk: {}", e);
		}
		// Set `saving` state.
		lock_write!(kernel_state).saving = false;
		debug!("CCD [13/14] - Disk: {}", secs_f64!(now));
		// Don't need these anymore.
		drop!(kernel_state, collection_for_disk);

		// 14.
		// Try 3 times before giving up.
		for i in 1..=4 {
			if i == 4 {
				error!("CCD [14/14] - Someone else is pointing to the old Collection...! I can't deconstruct it noooooooooo~");
				break
			}

			if Arc::strong_count(&old_collection) == 1 {
				let now = now!();
				drop(old_collection);
				debug!("CCD [14/14] - Deconstruct: {}", secs_f64!(now));
				break
			}

			warn!("CCD [14/14] Attempt ({}/3) - Failed to deconstruct old Collection...", i);
			sleep!(1000); // Sleep 1 second.
		}

		// Thank you CCD, you can rest now.
		debug!("CCD ... Took {} seconds, bye!", secs_f64!(beginning));
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
		let ctx = egui::Context::default();

		// Serialize.
		let now = now!();
		let collection = Collection::from_file().unwrap();
		info!("Read: {}", secs_f64!(now));

		// Convert.
		let now = now!();
		Ccd::convert_art(to_kernel, collection, ctx);
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
		let ctx = egui::Context::default();
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
		std::thread::spawn(move || Ccd::new_collection(to_kernel, from_kernel, kernel_state, old_clone, paths, ctx));

		// Act as `Kernel`.
		// Receive.
		let collection = loop {
			match recv!(from_ccd) {
				CcdToKernel::NewCollection(collection) => break collection,
				CcdToKernel::Failed(error)                    => panic!("{}", error),
				CcdToKernel::UpdatePhase((float, string))     => eprintln!("percent: {float}, string: {string}"),
				CcdToKernel::UpdateIncrement((float, string)) => eprintln!("percent: {float}, string: {string}"),
			}
		};

		// Send `Die` signal.
		drop(old_collection);
		send!(to_ccd, KernelToCcd::Die);

		sleep!(5000);
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

