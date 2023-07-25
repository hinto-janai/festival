//---------------------------------------------------------------------------------------------------- Use
use log::{warn,info,debug,trace};
use benri::{
	debug_panic,
	log::*,
	sync::*,
	thread::*,
	time::*,
};
use crate::collection::{
	Collection,
	Artists,
	Albums,
	Songs,
	Map,
	ArtistKey,
	AlbumKey,
	SongKey,
	Art,
	Image,
};
use crate::state::{
	Phase,
};
use crossbeam::channel::{Sender};
use std::path::{PathBuf};
use std::sync::{Arc};
use disk::{Bincode2,Json,Plain};
use readable::{
	Unsigned,
};
use crate::ccd::{
	msg::CcdToKernel,
};
use crate::constants::COLLECTION_VERSION;
use rayon::prelude::*;

#[cfg(feature = "gui")]
use crate::ccd::convert::ArtConvertType;

//---------------------------------------------------------------------------------------------------- CCD
pub(crate) struct Ccd;

impl Ccd {
	//-------------------------------------------------------------------------------- CCD `convert_art()`
	#[cfg(feature = "gui")]
	// Public facing "front-end" function for image conversion.
	// Dynamically selects internal functions for single/multi-thread.
	pub(crate) fn convert_art(
		to_kernel: Sender<CcdToKernel>,
		mut collection: Collection,
	) {
		let beginning = now!();
		debug!("CCD ... purpose in life: convert_art()");

		// If no albums, return.
		if collection.albums.is_empty() {
			send!(to_kernel, CcdToKernel::NewCollection(Arc::new(collection)));
		// Else, convert art, send to `Kernel`.
		} else {
			let total      = collection.albums.len();
			let increment  = 99.0 / total as f64;
			Self::priv_convert_art(&to_kernel, &mut collection, ArtConvertType::ToKnown, increment);
			send!(to_kernel, CcdToKernel::UpdatePhase((100.00, Phase::Finalize)));
			crate::ccd::img::alloc_textures(&collection.albums);
			send!(to_kernel, CcdToKernel::NewCollection(Arc::new(collection)));
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
		old_collection: Arc<Collection>,
		paths: Vec<PathBuf>,
	) {
		// `new_collection()` high-level overview.
		// This is for `GUI`, some steps may be
		// added or skipped in other `Frontend`'s.
		//
		// 1. Destruct the old `Collection` and dealloc textures.
		// 2. WalkDir given path(s), filtering for audio files.
		// 3. For each file, append metadata to appropriate `Vec`.
		// 4. Make sure `Vec<Album>` metadata matches the songs.
		// 5. Create sorted `Key`'s.
		// 6. Create the "Map"
		// 7. Create our `Collection`.
		// 8. Transform in-memory `Collection` album art to bytes
		// 9. Clone `Collection` for later saving
		// 10. Transform in-memory `Collection` album art bytes to `egui`'s `RetainedImage`
		// 11. Pre-allocate the `RetainedImage`'s into `egui`
		// 12. Send to `Kernel`
		// 13. Save `Collection` to disk.
		let beginning = now!();
		debug!("CCD ... purpose in life: new_collection()");

		//-------------------------------------------------------------------------------- 1
		//
		// This used to be at the very end, but turns out... WE USE WAY TOO MUCH MEMORY!
		//
		// Across the thread spawns, file probing, image conversions and terrible
		// workaround of cloning of the whole `Collection` to save it to disk in byte form,
		// there's not much wiggle room in terms of memory and the luxury of holding onto
		// the old `Collection` just in case we fail so we can at least send that back to
		// the user is a luxury we can't afford.
		//
		// So, deconstruct the old one, right here, before we start a new one.
		//
		// If `CCD` fails, `Kernel` just sends a `Collection::dummy()` back to everyone.
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((0.00, Phase::Deconstruct)));
		{
			let mut i = 1;
			loop {
				trace!("CCD [1/13] Deconstruct attempt {i}");

				if Arc::strong_count(&old_collection) == 1 {
					if let Some(c) = Arc::into_inner(old_collection) {
//						let ctx = crate::frontend::gui::gui_context();
//						crate::ccd::img::free_textures(&mut ctx.tex_manager().write());
						drop(c);
					} else {
						debug_panic!("old_collection strong count was 1 but .into_inner() failed");
					}
					break;
				}

				i += 1;
				sleep_millis!(1);
			}
		}
		let perf_deconstruct = secs_f32!(now);
		trace!("CCD [1/13] ... Deconstruct: {perf_deconstruct}");

		//-------------------------------------------------------------------------------- 2
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((2.50, Phase::WalkDir)));
		let paths = Self::walkdir_audio(paths);
		let perf_walkdir = secs_f32!(now);
		trace!("CCD [2/13] ... WalkDir: {perf_walkdir}");

		//-------------------------------------------------------------------------------- 3
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((5.00, Phase::Parse)));
		let (mut vec_artist, mut vec_album, vec_song, count_art) = Self::the_loop(&to_kernel, paths);
		// Update should be < 50% at this point.
		let perf_metadata = secs_f32!(now);
		trace!("CCD [3/13] ... Metadata: {perf_metadata}");

		//-------------------------------------------------------------------------------- 4
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((50.00, Phase::Fix)));
		Self::fix_metadata(&mut vec_artist, &mut vec_album, &vec_song);
		let perf_fix = secs_f32!(now);
		trace!("CCD [4/13] ... Fix: {perf_fix}");

		//-------------------------------------------------------------------------------- 5
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
		trace!("CCD [5/13] ... Sort: {perf_sort}");

		//-------------------------------------------------------------------------------- 6
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((55.00, Phase::Search)));
		let map = Map::from_3_vecs(&vec_artist, &vec_album, &vec_song);
		let perf_map = secs_f32!(now);
		trace!("CCD [6/13] ... Map: {perf_map}");

		//-------------------------------------------------------------------------------- 7
		let now = now!();
		send!(to_kernel, CcdToKernel::UpdatePhase((60.00, Phase::Prepare)));
		let mut collection = Collection {
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
		};
		// Fix metadata.
		{
			// Get `Vec` lengths.
			let artists = collection.artists.len();
			let albums  = collection.albums.len();
			let songs   = collection.songs.len();

			// Set `empty`.
			if artists == 0 && albums == 0 && songs == 0 {
				collection.empty = true;
			} else {
				collection.empty = false;
			}

			// Set `count_*`.
			collection.count_artist = Unsigned::from(artists);
			collection.count_album  = Unsigned::from(albums);
			collection.count_song   = Unsigned::from(songs);

			// Set `timestamp`.
			collection.timestamp = benri::unix!();
		}
		let perf_prepare = secs_f32!(now);
		trace!("CCD [7/13] ... Prepare: {perf_prepare}");

		#[cfg(feature = "gui")]
		let (
			perf_resize,
			perf_clone,
			perf_convert,
			perf_textures,
			collection_for_disk,
		) = {
			//-------------------------------------------------------------------------------- 8
			let now = now!();
			send!(to_kernel, CcdToKernel::UpdatePhase((60.00, Phase::Art)));
			let increment    = 30.0 / collection.albums.len() as f64;
			Self::priv_convert_art(&to_kernel, &mut collection, ArtConvertType::Resize, increment);
			// Update should be <= 90% at this point.
			let perf_resize = secs_f32!(now);
			trace!("CCD [8/13] ... Resize: {perf_resize}");

			//-------------------------------------------------------------------------------- 9
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
			trace!("CCD [9/13] ... Clone: {perf_clone}");
			// FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME

			//-------------------------------------------------------------------------------- 10
			let now = now!();
			send!(to_kernel, CcdToKernel::UpdatePhase((95.00, Phase::Convert)));
			let increment = 4.0 / collection.albums.len() as f64;
			// Convert `Collection` art.
			Self::priv_convert_art(&to_kernel, &mut collection, ArtConvertType::ToKnown, increment);
			// Update should be <= 99% at this point.
			let perf_convert = secs_f32!(now);
			trace!("CCD [10/13] ... Convert: {perf_convert}");

			//-------------------------------------------------------------------------------- 11
			let now = now!();
			send!(to_kernel, CcdToKernel::UpdatePhase((100.00, Phase::Finalize)));
			// FIXME: See `img.rs`.
			crate::ccd::img::alloc_textures(&collection.albums);
			let perf_textures = secs_f32!(now);
			trace!("CCD [11/13] ... Textures: {perf_textures}");

			(perf_resize, perf_clone, perf_convert, perf_textures, collection_for_disk)
		};

		#[cfg(feature = "daemon")]
		let (
			perf_resize,
			perf_clone,
			perf_convert,
			perf_textures,
		) = {
			trace!("CCD [8/13] ... Resize: SKIP");
			trace!("CCD [9/13] ... Clone: SKIP");

			let now = now!();
			send!(to_kernel, CcdToKernel::UpdatePhase((95.00, Phase::Convert)));
			let increment = 4.0 / collection.albums.len() as f64;

			atomic_store!(crate::state::SAVING, true);

			// Delete old images.
			let _ = Image::rm_base();
			let (a, b) = (Image::base_path(), Image(collection.timestamp).save());
			match (a, b) {
				(Ok(path), Ok(_)) => {
					// Convert `Collection` art.
					collection.albums.0
						.par_iter_mut()
						.enumerate()
						.for_each(|(i, a)| crate::ccd::img::save_image_and_convert(i, a, &path));
				},
				_ => {
					fail!("CCD ... Error, Skipping Image");
					collection.albums.0
						.par_iter_mut()
						.for_each(|a| a.art = Art::Unknown);
				},
			}

			// Update should be <= 99% at this point.
			let perf_convert = secs_f32!(now);
			trace!("CCD [10/13] ... Convert: {perf_convert}");

			trace!("CCD [11/13] ... Textures: SKIP");

			(0.0, 0.0, perf_convert, 0.0)
		};

		// Get perf stats.
		let objects_artists = collection.count_artist.usize();
		let objects_albums  = collection.count_album.usize();
		let objects_songs   = collection.count_song.usize();
		let objects_art     = count_art;
		let timestamp       = collection.timestamp;

		#[cfg(feature = "gui")]
		let collection = Arc::new(collection);

		#[cfg(feature = "daemon")]
		let (collection, collection_for_disk) = {
			let collection = Arc::new(collection);
			let disk       = Arc::clone(&collection);
			(collection, disk)
		};

		//-------------------------------------------------------------------------------- 12
		send!(to_kernel, CcdToKernel::NewCollection(collection));
		let user_time = secs_f32!(beginning);
		info!("CCD [12/13] ... User time: {}", user_time);

		//-------------------------------------------------------------------------------- 13
		let now = now!();

		// Set `saving` state.
		atomic_store!(crate::state::SAVING, true);

		// Attempt atomic save.
		//
		// SAFETY:
		// `Collection` is saved to disk via `memmap`.
		//
		// We (`CCD`) are the only "entity" that should
		// be touching `collection.bin` at this point.
		let total_bytes = match unsafe { collection_for_disk.save_atomic_memmap() } {
			Ok(md) => { debug!("CCD ... Collection{COLLECTION_VERSION}: {md}"); md.size()},
			Err(e) => {
				debug_panic!("CCD ... Collection{COLLECTION_VERSION}: {e}");
				fail!("CCD ... Collection{COLLECTION_VERSION}: {e}");
				0
			},
		};

		#[cfg(feature = "gui")]
		{
			// Delete old images.
			let _ = Image::rm_base();
			let (a, b) = (Image::base_path(), Image(timestamp).save());
			// This deconstructs `Collection`.
			let albums = collection_for_disk.albums.0.into_vec();
			match (a, b) {
				(Ok(path), Ok(_)) => {
					albums
						.into_par_iter()
						.enumerate()
						.for_each(|(a, i)| crate::ccd::img::save_image(a, i, &path));
				},
				_ => fail!("CCD ... Error, Skipping Image"),
			}
		}

		// Set `saving` state.
		atomic_store!(crate::state::SAVING, false);
		let perf_disk = secs_f32!(now);
		trace!("CCD [13/13] ... Disk: {perf_disk}");

		//-------------------------------------------------------------------------------- Print & save `Perf` stats.
		let ccd_time = secs_f32!(beginning);
		info!("CCD ... CCD time: {}", ccd_time);

		// Gather and save perf data.
		let phases = crate::ccd::perf::Phases {
			deconstruct: perf_deconstruct,
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
			disk:        perf_disk,
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
			ccd: ccd_time,
		};
		let perf = crate::ccd::perf::Perf {
			objects,
			phases,
			total,
		};
 		format!("{perf:#?}").lines().for_each(|l| debug!("{l}"));
		match perf.save() {
			Ok(i)  => debug!("CCD ... Perf: {i}"),
			Err(e) => warn!("CCD ... Couldn't save perf data: {e}"),
		}

		//-------------------------------------------------------------------------------- End.
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
	use crate::constants::COLLECTION_VERSION;

	#[test]
	#[cfg(feature = "gui")]
	// Converts the pre-saved `Collection`'s art.
	fn convert_art() {
		// Set-up inputs.
		let (to_kernel, from_ccd) = crossbeam::channel::unbounded::<CcdToKernel>();
		crate::frontend::gui::GUI_CONTEXT.set(egui::Context::default());

		// Convert.
		std::thread::spawn(move || {
			Ccd::convert_art(to_kernel, Collection::from_path(format!("../assets/shukusai/state/collection{COLLECTION_VERSION}_real.bin")).unwrap())
		});

		let c = loop {
			match recv!(from_ccd) {
				CcdToKernel::NewCollection(c) => break c,
				_ => (),
			}
		};

		// Print.
		println!("{c:#?}");

		// Assert.
		assert!(!c.empty);
		assert_eq!(c.count_artist, 3);
		assert_eq!(c.count_album,  4);
		assert_eq!(c.count_song,   7);
		assert_eq!(c.count_art,    4);
		assert_eq!(c.timestamp,    1688690421);
	}

	#[test]
	#[cfg(feature = "gui")]
	// Creates a new `Collection` from `assets/audio`.
	// Metadata should be the same as the pre-saved `Collection`.
	fn new_collection() {
		// Set-up inputs.
		let (to_kernel, from_ccd) = crossbeam::channel::unbounded::<CcdToKernel>();
		crate::frontend::gui::GUI_CONTEXT.set(egui::Context::default());
		let old = Collection::new();
		let old = Arc::new(old);
		let paths = vec![PathBuf::from("../assets")];

		// Spawn `CCD`.
		std::thread::spawn(move || {
			Ccd::new_collection(to_kernel, old, paths)
		});

		// Act as `Kernel`.
		// Receive.
		let c = loop {
			match recv!(from_ccd) {
				CcdToKernel::NewCollection(c) => break c,
				_ => (),
			}
		};

		println!("{c:#?}");

		assert!(!c.empty);
		assert_eq!(c.count_artist, 3);
		assert_eq!(c.count_album,  4);
		assert_eq!(c.count_song,   7);
		assert_eq!(c.count_art,    4);
		assert!(c.timestamp > 1688690421);
	}
}
