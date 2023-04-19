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
use super::phase::{
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
use disk::Bincode;
use readable::{
	Unsigned,
	Percent,
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
		// 7. Transform in-memory `Collection` with `priv_convert_art()`
		// 8. Pre-allocate the `RetainedImage`'s into `egui`
		// 9. Send to `Kernel`
		// 10. Wait for `Die` signal.
		// 11. Save `Collection` to disk.
		// 12. Destruct the old `Collection`.

		// TODO: Handle potential errors:
		// 1. No albums
		// 2. Path error
		// 3. Permission error
		// 4. Disk error

		// TODO: Send updates to `Kernel` throughout and `log!()`.

		// 1.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((0.00, Phase::WalkDir)));
		let paths = Self::walkdir_audio(&to_kernel, paths);
		debug!("CCD [1/12] - WalkDir: {}", secs_f64!(now));

		// 2.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((5.00, Phase::Parse)));
		let (vec_artist, mut vec_album, vec_song) = Self::audio_paths_to_incomplete_vecs(&to_kernel, paths);
		// Update should be < 50% at this point.
		debug!("CCD [2/12] - Metadata: {}", secs_f64!(now));

		// 3.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((50.00, Phase::Fix)));
		Self::fix_album_metadata_from_songs(&mut vec_album, &vec_song);
		debug!("CCD [3/12] - Fix: {}", secs_f64!(now));

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
		debug!("CCD [4/12] - Sort: {}", secs_f64!(now));

		// 5.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((55.00, Phase::Search)));
		let map = Map::from_3_vecs(&vec_artist, &vec_album, &vec_song);
		debug!("CCD [5/12] - Map: {}", secs_f64!(now));

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
		debug!("CCD [6/12] - Collection: {}", secs_f64!(now));

		// 7.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((60.00, Phase::Resize)));
		let collection = Self::priv_convert_art(&to_kernel, collection);
		// Update should be <= 99% at this point.
		debug!("CCD [7/12] - Image: {}", secs_f64!(now));

		// 8.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((100.00, Phase::Finalize)));
		// FIXME:
		// This is a huge workaround around `egui`'s lack of bulk texture
		// allocation. Although this is good enough for now, figure out
		// how to bulk allocate all these images without causing the GUI
		// to freeze. Currently it's done with `try_write()` which doesn't
		// starve GUI, but can take way longer (0.00008 secs -> 1.xx secs...!!!).
		super::alloc_textures(&collection.albums, &ctx);
//		ctx.request_repaint();
		debug!("CCD [8/12] - Textures: {}", secs_f64!(now));

		// 9.
		let now = now!();
		let collection = Arc::new(collection);
		send!(to_kernel, CcdToKernel::NewCollection(Arc::clone(&collection)));
		debug!("CCD [9/12] - ToKernel: {}", secs_f64!(now));

		debug!("CCD - Created Collection and sent to Kernel: {}", secs_f64!(beginning));

		// 10.
		let now = now!();
		match recv!(from_kernel) {
			KernelToCcd::Die => debug!("CCD [10/12] - Die: {}", secs_f64!(now)),
		}

		// 11.
		let now = now!();
		// Set `saving` state.
		lock_write!(kernel_state).saving = true;
		// Attempt atomic save.
		if let Err(e) = collection.save_atomic() {
			fail!("CCD - Collection write to disk: {}", e);
		}
		// Set `saving` state.
		lock_write!(kernel_state).saving = false;
		debug!("CCD [11/12] - Disk: {}", secs_f64!(now));
		// Don't need these anymore.
		drop!(kernel_state, collection);

		// 12.
		// Try 3 times before giving up.
		for i in 1..=4 {
			if i == 4 {
				error!("CCD [12/12] - Someone else is pointing to the old Collection...! I can't deconstruct it noooooooooo~");
				break
			}

			if Arc::strong_count(&old_collection) == 1 {
				let now = now!();
				drop(old_collection);
				debug!("CCD [12/12] - Deconstruct: {}", secs_f64!(now));
				break
			}

			warn!("CCD [12/12] Attempt ({}/3) - Failed to deconstruct old Collection...", i);
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

