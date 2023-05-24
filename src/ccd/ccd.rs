//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
//use serde::{Serialize,Deserialize};
//use disk::prelude::*;
//use disk::{};
//use std::{};
use benri::{
	debug_panic,
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
use crate::collection::Art;
use crossbeam::channel::{Sender,Receiver};
use std::path::{Path,PathBuf};
use std::sync::{Arc,RwLock};
use disk::{Bincode2,Json};
use readable::{
	Unsigned,
	Percent,
};
use super::convert::{
	ArtConvertType,
};
use std::marker::PhantomData;

//---------------------------------------------------------------------------------------------------- CCD
pub(crate) struct Ccd;

impl Ccd {
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
			let total      = collection.albums.len();
			let increment  = 99.0 / total as f64;
			let threads    = super::threads_for_album_art(total);
			let collection = Arc::new(Self::priv_convert_art(&to_kernel, collection, ArtConvertType::ToKnown, increment, total, threads));
			// FIXME:
			// See below `new_collection()` FIXME for info.
			send!(to_kernel, CcdToKernel::UpdatePhase((100.00, Phase::Finalize)));
//			super::alloc_textures_no_sleep(&collection.albums, &ctx);
			super::alloc_textures(&collection.albums, &ctx);
			send!(to_kernel, CcdToKernel::NewCollection(collection));
		}

		debug!("CCD ... Took {} seconds, bye!", secs_f32!(beginning));
	}

	//-------------------------------------------------------------------------------- CCD `new_collection()`
	// Public facing "front-end" function for making a new `Collection`.
	//
	// These operations are split up into different private
	// functions mostly for testing flexibility.
	pub(crate) fn new_collection(
		to_kernel: Sender<CcdToKernel>,
		from_kernel: Receiver<KernelToCcd>,
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
		let perf_walkdir = secs_f32!(now);
		trace!("CCD [1/14] - WalkDir: {perf_walkdir}");

		// 2.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((5.00, Phase::Parse)));
		let (mut vec_artist, mut vec_album, vec_song, count_art) = Self::the_loop(&to_kernel, paths);
		// Update should be < 50% at this point.
		let perf_metadata = secs_f32!(now);
		trace!("CCD [2/14] - Metadata: {perf_metadata}");

		// 3.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((50.00, Phase::Fix)));
		Self::fix_metadata(&mut vec_artist, &mut vec_album, &vec_song);
		let perf_fix = secs_f32!(now);
		trace!("CCD [3/14] - Fix: {perf_fix}");

		// 4.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((52.50, Phase::Sort)));

		let sort_artist_lexi            = Self::sort_artist_lexi(&vec_artist);
		let sort_artist_lexi_rev        = sort_artist_lexi.iter().rev().copied().collect::<Box<[ArtistKey]>>();
		let sort_artist_album_count     = Self::sort_artist_album_count(&vec_artist);
		let sort_artist_album_count_rev = sort_artist_album_count.iter().rev().copied().collect::<Box<[ArtistKey]>>();
		let sort_artist_song_count      = Self::sort_artist_song_count(&vec_artist, &vec_album);
		let sort_artist_song_count_rev  = sort_artist_song_count.iter().rev().copied().collect::<Box<[ArtistKey]>>();
		let sort_artist_runtime         = Self::sort_artist_runtime(&vec_artist);
		let sort_artist_runtime_rev     = sort_artist_runtime.iter().rev().copied().collect::<Box<[ArtistKey]>>();
		let sort_artist_name            = Self::sort_artist_name(&vec_artist);
		let sort_artist_name_rev        = sort_artist_name.iter().rev().copied().collect::<Box<[ArtistKey]>>();

		let sort_album_release_artist_lexi         = Self::sort_album_release_artist_iter(&sort_artist_lexi, &vec_artist, &vec_album);
		let sort_album_release_artist_lexi_rev     = Self::sort_album_release_artist_iter(&sort_artist_lexi_rev, &vec_artist, &vec_album);
		let sort_album_release_rev_artist_lexi     = Self::sort_album_release_rev_artist_iter(&sort_artist_lexi, &vec_artist, &vec_album);
		let sort_album_release_rev_artist_lexi_rev = Self::sort_album_release_rev_artist_iter(&sort_artist_lexi_rev, &vec_artist, &vec_album);
		let sort_album_lexi_artist_lexi            = Self::sort_album_lexi_artist_iter(&sort_artist_lexi, &vec_artist, &vec_album);
		let sort_album_lexi_artist_lexi_rev        = Self::sort_album_lexi_artist_iter(&sort_artist_lexi_rev, &vec_artist, &vec_album);
		let sort_album_lexi_rev_artist_lexi        = Self::sort_album_lexi_rev_artist_iter(&sort_artist_lexi, &vec_artist, &vec_album);
		let sort_album_lexi_rev_artist_lexi_rev    = Self::sort_album_lexi_rev_artist_iter(&sort_artist_lexi_rev, &vec_artist, &vec_album);
		let sort_album_lexi                        = Self::sort_album_lexi(&vec_album);
		let sort_album_lexi_rev                    = sort_album_lexi.iter().rev().copied().collect::<Box<[AlbumKey]>>();
		let sort_album_release                     = Self::sort_album_release(&vec_album);
		let sort_album_release_rev                 = sort_album_release.iter().rev().copied().collect::<Box<[AlbumKey]>>();
		let sort_album_runtime                     = Self::sort_album_runtime(&vec_album);
		let sort_album_runtime_rev                 = sort_album_runtime.iter().rev().copied().collect::<Box<[AlbumKey]>>();
		let sort_album_title                       = Self::sort_album_title(&vec_album);
		let sort_album_title_rev                   = sort_album_title.iter().rev().copied().collect::<Box<[AlbumKey]>>();

		let sort_song_album_release_artist_lexi         = Self::sort_song(&sort_album_release_artist_lexi,         &vec_album);
		let sort_song_album_release_artist_lexi_rev     = Self::sort_song(&sort_album_release_artist_lexi_rev,     &vec_album);
		let sort_song_album_release_rev_artist_lexi     = Self::sort_song(&sort_album_release_rev_artist_lexi,     &vec_album);
		let sort_song_album_release_rev_artist_lexi_rev = Self::sort_song(&sort_album_release_rev_artist_lexi_rev, &vec_album);
		let sort_song_album_lexi_artist_lexi            = Self::sort_song(&sort_album_lexi_artist_lexi,            &vec_album);
		let sort_song_album_lexi_artist_lexi_rev        = Self::sort_song(&sort_album_lexi_artist_lexi_rev,        &vec_album);
		let sort_song_album_lexi_rev_artist_lexi        = Self::sort_song(&sort_album_lexi_rev_artist_lexi,        &vec_album);
		let sort_song_album_lexi_rev_artist_lexi_rev    = Self::sort_song(&sort_album_lexi_rev_artist_lexi_rev,    &vec_album);
		let sort_song_release                           = Self::sort_song(&sort_album_release, &vec_album);
		let sort_song_release_rev                       = sort_song_release.iter().rev().copied().collect::<Box<[SongKey]>>();
		let sort_song_lexi                              = Self::sort_song_lexi(&vec_song);
		let sort_song_lexi_rev                          = sort_song_lexi.iter().rev().copied().collect::<Box<[SongKey]>>();
		let sort_song_runtime                           = Self::sort_song_runtime(&vec_song);
		let sort_song_runtime_rev                       = sort_song_runtime.iter().rev().copied().collect::<Box<[SongKey]>>();
		let sort_song_title                             = Self::sort_song_title(&vec_song);
		let sort_song_title_rev                         = sort_song_title.iter().rev().copied().collect::<Box<[SongKey]>>();

		let perf_sort = secs_f32!(now);
		trace!("CCD [4/14] - Sort: {perf_sort}");

		// 5.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((55.00, Phase::Search)));
		let map = Map::from_3_vecs(&vec_artist, &vec_album, &vec_song);
		let perf_map = secs_f32!(now);
		trace!("CCD [5/14] - Map: {perf_map}");

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

			// We calculated this during "The Loop".
			count_art: Unsigned::from(count_art),

			map,

			artists: Artists::from_vec(vec_artist),
			albums: Albums::from_vec(vec_album),
			songs: Songs::from_vec(vec_song),

			sort_artist_lexi,
			sort_artist_lexi_rev,
			sort_artist_album_count,
			sort_artist_album_count_rev,
			sort_artist_song_count,
			sort_artist_song_count_rev,
			sort_artist_runtime,
			sort_artist_runtime_rev,
			sort_artist_name,
			sort_artist_name_rev,

			sort_album_release_artist_lexi,
			sort_album_release_artist_lexi_rev,
			sort_album_release_rev_artist_lexi,
			sort_album_release_rev_artist_lexi_rev,
			sort_album_lexi_artist_lexi,
			sort_album_lexi_artist_lexi_rev,
			sort_album_lexi_rev_artist_lexi,
			sort_album_lexi_rev_artist_lexi_rev,
			sort_album_lexi,
			sort_album_lexi_rev,
			sort_album_release,
			sort_album_release_rev,
			sort_album_runtime,
			sort_album_runtime_rev,
			sort_album_title,
			sort_album_title_rev,

			sort_song_album_release_artist_lexi,
			sort_song_album_release_artist_lexi_rev,
			sort_song_album_release_rev_artist_lexi,
			sort_song_album_release_rev_artist_lexi_rev,
			sort_song_album_lexi_artist_lexi,
			sort_song_album_lexi_artist_lexi_rev,
			sort_song_album_lexi_rev_artist_lexi,
			sort_song_album_lexi_rev_artist_lexi_rev,
			sort_song_lexi,
			sort_song_lexi_rev,
			sort_song_release,
			sort_song_release_rev,
			sort_song_runtime,
			sort_song_runtime_rev,
			sort_song_title,
			sort_song_title_rev,

			_reserved1: PhantomData,
			_reserved2: PhantomData,
			_reserved4: PhantomData,
			_reserved5: PhantomData,
			_reserved6: PhantomData,
			_reserved7: PhantomData,
			_reserved8: PhantomData,
			_reserved9: PhantomData,
			_reserved10: PhantomData,
			_reserved11: PhantomData,
			_reserved12: PhantomData,
			_reserved13: PhantomData,
			_reserved14: PhantomData,
			_reserved15: PhantomData,
			_reserved16: PhantomData,
			_reserved17: PhantomData,
			_reserved18: PhantomData,
			_reserved19: PhantomData,
			_reserved20: PhantomData,
			_reserved21: PhantomData,
			_reserved22: PhantomData,
			_reserved23: PhantomData,
			_reserved24: PhantomData,
		};
		// Fix metadata.
		let collection = collection.set_metadata();
		let perf_prepare = secs_f32!(now);
		trace!("CCD [6/14] - Prepare: {perf_prepare}");

		// 7.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((60.00, Phase::Art)));
		let increment    = 30.0 / collection.albums.len() as f64;
		let total_albums = collection.albums.len();
		let threads      = super::threads_for_album_art(total_albums);
		let collection   = Self::priv_convert_art(&to_kernel, collection, ArtConvertType::Resize, increment, total_albums, threads);
		// Update should be <= 90% at this point.
		let perf_resize = secs_f32!(now);
		trace!("CCD [7/14] - Resize: {perf_resize}");

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
		let perf_clone = secs_f32!(now);
		trace!("CCD [8/14] - Clone: {perf_clone}");
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
		// Convert `Collection` art.
		let collection = Self::priv_convert_art(&to_kernel, collection, ArtConvertType::ToKnown, increment, total_albums, threads);
		// Update should be <= 99% at this point.
		let perf_convert = secs_f32!(now);
		trace!("CCD [9/14] - Convert: {perf_convert}");

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
		let perf_textures = secs_f32!(now);
		trace!("CCD [10/14] - Textures: {perf_textures}");

		// 11.
		let now = now!();
		send!(to_kernel, CcdToKernel::NewCollection(Arc::new(collection)));
		let perf_to_kernel = secs_f32!(now);
		trace!("CCD [11/14] - ToKernel: {perf_to_kernel}");

		// 12.
		// INVARIANT:
		// In order to access `perf_die` out of scope
		// there is not match/if_else here.
		//
		// Instead, we just block on `recv!()` assuming
		// `KernelToCcd::Die` is the only message `CCD`
		// can receive.
		let now = now!();
		let _ = recv!(from_kernel);
		let perf_die = secs_f32!(now);
		trace!("CCD [12/14] - Die: {perf_die}");

		let user_time = secs_f32!(beginning);
		trace!("CCD - User time: {}", user_time);

		// 13.
		let now = now!();
		// Set `saving` state.
		atomic_store!(crate::kernel::SAVING, true);
		// Attempt atomic save.
		//
		// SAFETY:
		// `Collection` is saved to disk via `memmap`.
		//
		// We (`CCD`) are the only "entity" that should
		// be touching `collection.bin` at this point.
		let total_bytes = match unsafe { collection_for_disk.save_atomic_memmap() } {
			Ok(md) => { debug!("CCD - Collection: {md}"); md.size()},
			Err(e) => {
				debug_panic!("CCD - Collection: {e}");
				fail!("CCD - Collection: {e}");
				0
			},
		};
		// Set `saving` state.
		atomic_store!(crate::kernel::SAVING, false);
		let perf_disk = secs_f32!(now);
		trace!("CCD [13/14] - Disk: {perf_disk}");

		// Get perf stats.
		let objects_artists = collection_for_disk.count_artist.usize();
		let objects_albums  = collection_for_disk.count_album.usize();
		let objects_songs   = collection_for_disk.count_song.usize();
		let objects_art     = count_art;

		// Don't need this anymore.
		drop(collection_for_disk);

		// 14.
		// Try 3 times before giving up.
		let now = now!();
		for i in 1..=4 {
			if i == 4 {
				debug_panic!("CCD couldn't deconstruct the Collection");

				error!("CCD [14/14] - Someone else is pointing to the old Collection...! I can't deconstruct it noooooooooo~");
				break
			}

			if Arc::strong_count(&old_collection) == 1 {
				let now = now!();
				drop(old_collection);
				break
			}

			warn!("CCD [14/14] Attempt ({}/3) - Failed to deconstruct old Collection...", i);
			sleep!(1000); // Sleep 1 second.
		}
		let perf_deconstruct = secs_f32!(now);
		trace!("CCD [14/14] - Deconstruct: {perf_deconstruct}");

		// Gather and save perf data.
		let phases = crate::ccd::perf::Phases {
			walkdir:     perf_walkdir,
			metadata:    perf_metadata,
			fix:         perf_fix,
			sort:        perf_sort,
			map:         perf_map,
			prepare:     perf_prepare,
			resize:      perf_resize,
			clone:       perf_clone,
			convert:     perf_convert,
			textures:    perf_textures,
			to_kernel:   perf_to_kernel,
			die:         perf_die,
			disk:        perf_disk,
			deconstruct: perf_deconstruct,
		};
		let objects = crate::ccd::perf::Objects {
			artists: objects_artists,
			albums:  objects_albums,
			songs:   objects_songs,
			art:     objects_art,
		};
		let total = crate::ccd::perf::Total {
			bytes: total_bytes,
			user: user_time,
			ccd: secs_f32!(beginning),
		};
		let perf = crate::ccd::perf::Perf {
			objects,
			phases,
			total,
		};
 		format!("{perf:#?}").lines().for_each(|l| debug!("{l}"));
		match perf.save() {
			Ok(i)  => debug!("CCD - Perf: {i}"),
			Err(e) => warn!("CCD - Couldn't save perf data: {e}"),
		}

		// Thank you CCD, you can rest now.
		ok_debug!("CCD");
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use crate::ccd::*;
	use std::path::PathBuf;
	use disk::*;

	#[test]
	#[ignore]
	fn serialize_and_convert_collection() {
		// Set-up logger.
		crate::logger::init_logger(log::LevelFilter::Trace);

		// Set-up inputs.
		let (to_kernel, from_ccd) = crossbeam::channel::unbounded::<CcdToKernel>();
		let ctx = egui::Context::default();

		// Serialize.
		let now = now!();
		let collection = Collection::from_file().unwrap();
		info!("Read: {}", secs_f32!(now));

		// Convert.
		let now = now!();
		Ccd::convert_art(to_kernel, collection, ctx);
		let collection = match from_ccd.recv().unwrap() {
			CcdToKernel::NewCollection(c) => c,
			_ => panic!("wrong msg received"),
		};
		info!("Convert: {}", secs_f32!(now));

		// Print.
		info!("{:#?}", collection);

		// Assert.
		assert!(collection.count_artist >= 1);
		assert!(collection.count_album  >= 1);
		assert!(collection.count_song   >= 1);
		assert!(collection.timestamp >= 1);
		assert!(!collection.empty);
	}

	#[test]
	#[ignore]
	fn new_collection() {
		// Set-up logger.
		crate::logger::init_logger(log::LevelFilter::Trace);

		// Set-up inputs.
		let (to_kernel, from_ccd) = crossbeam::channel::unbounded::<CcdToKernel>();
		let (to_ccd, from_kernel) = crossbeam::channel::unbounded::<KernelToCcd>();
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

		assert!(!collection.empty);
		assert!(collection.timestamp > 1678382892);
		assert!(collection.count_artist == 1 || collection.count_artist == 501);
		assert!(collection.count_album  == 1 || collection.count_album  == 501);
		assert!(collection.count_song   == 4 || collection.count_song   == 505);
	}
}

