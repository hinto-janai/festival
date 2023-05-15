//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use lofty::{
	Accessor,
	TaggedFile,
	TaggedFileExt,
	AudioFile,
	Picture,
};
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use crate::collection::{
	Art,
	Artist,
	Album,
	Song,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use benri::{
	log::*,
	sync::*,
	panic::*,
};
use crossbeam::channel::Sender;
use super::CcdToKernel;
use readable::{
	Runtime,
	Unsigned,
	Date,
};
use std::borrow::Cow;
use std::sync::{Arc,Mutex};
use std::sync::atomic::AtomicUsize;

//---------------------------------------------------------------------------------------------------- Tag Metadata (temporary) struct.
#[derive(Debug)]
// This is just a temporary container for
// holding some borrowed data.
//
// Whether we need to clone the `&str`'s below
// are in conditional branches, so this `struct`
// is held in scope throughout "The Loop" so
// we have the _choice_ to `to_string()` or not.
struct TagMetadata<'a> {
	artist: Cow<'a, str>,
	album: Cow<'a, str>,
	title: Cow<'a, str>,
	track: Option<u32>,
	disc: Option<u32>,
	track_total: Option<u32>,
	disc_total: Option<u32>,
	picture: Option<Vec<u8>>,

	runtime: f64,
	release: Option<&'a str>,
	track_artists: Option<String>,

	compilation: bool,
}

//---------------------------------------------------------------------------------------------------- Metadata functions.
impl super::Ccd {
	#[inline(always)]
	// `The Loop`.
	//
	// Takes in input of a filtered `Vec<PathBuf>` of audio files.
	// Loops over all `PathBuf`'s and adds metadata onto the `Vec`'s.
	//
	// Outputs the three main `Vec`'s of the `Collection` with
	// mostly done but incomplete data (needs sorting, addition, etc).
	//
	// Unlike the `convert_art()` functions, this one is too long to
	// justify making 2 copies for single/multi-threaded purposes.
	//
	// Instead, single-thread usage (which realistically only happens on small `Collection`'s)
	// will just have to pay the price of using syncing primitives (`Arc<Mutex<T>>`).
	//
	// `path_to_tagged_file()` is by far the most expensive function in this loop,
	// accounting for 90% of the time spent when making a new `Collection`.
	// It gains a 2-4x~ speed boost when multi-threaded, gaining relative speed on
	// its single-threaded counter-part as the `Song`'s we process approach the 10_000s.
	//
	// Although, it hits diminishing returns quickly, which is why
	// only `25%~` of the user's available threads are used.
	pub(super) fn the_loop(
		to_kernel: &Sender<CcdToKernel>,
		vec_paths: Vec<PathBuf>
	) -> (Vec<Artist>, Vec<Album>, Vec<Song>, usize) {
		// TODO:
		// Send messages to `Kernel` & log.

		// TODO:
		// Some metadata is missing.

		// ResetUpdate.
		//
		// These are sent to `Kernel` for progress updates.
		let vec_len:   usize = vec_paths.len();
		let increment: f64   = 50.0 / vec_len as f64;
		// Vec capacity estimation.
		let song_len_maybe   = vec_len;             // Assuming _every_ PATH is a valid `Song`.
		let album_len_maybe  = vec_len / 16;        // Assuming `~16` `Song`'s per `Album`.
		let artist_len_maybe = album_len_maybe / 5; // Assuming `~5` `Album`'s per `Artist`.

		// For efficiency reasons, it's best to do
		// all these operations in a single loop.
		//
		// This means there's a lot of variables in this
		// function scope to keep in mind, so here's a guide:
		//```
		//         Working Memory (HashMap)
		//
		// Vec<Artist>    Vec<Album>    Vec<Song>
		//
		//```
		// - We have a "Working Memory" that keeps track of what `Artist/Album` we've seen already.
		// - We have 3 `Vec`'s (that will eventually become the `Collection`).
		//
		// The "Working Memory" is a `HashMap` that takes in `String` input of an artist name and returns the `index` to it,
		// along with another `HashMap` which represents that `Artist`'s `Album`'s and its appropriate `indices`.
		//
		//                           Artist  Artist's index     Album  Album's index
		//                            Name   in `Vec<Artist>`   Name   in `Vec<Album>`
		//                              |          |              |         |
		//                              v          v              v         v
		let memory:       Mutex<HashMap<String, (usize, HashMap<String, usize>)>> = Mutex::new(HashMap::with_capacity(artist_len_maybe));
		let vec_artist:   Mutex<Vec<Artist>> = Mutex::new(Vec::with_capacity(artist_len_maybe));
		let vec_album:    Mutex<Vec<Album>>  = Mutex::new(Vec::with_capacity(album_len_maybe));
		let vec_song:     Mutex<Vec<Song>>   = Mutex::new(Vec::with_capacity(song_len_maybe));
		let count_art:    Mutex<usize>       = Mutex::new(0);

		// In this loop, each `PathBuf` represents a new `Song` with metadata.
		// There are 3 logical possibilities with 3 actions associated with them:
		//     1. `Artist` exists, `Album` exists         => Add `Song`
		//     2. `Artist` exists, `Album` DOESN'T exist  => Add `Album + Song`
		//     3. `Artist` DOESN'T exist                  => Add `Artist + Album + Song`
		//
		// Memory must be updated as well.

		// Get an appropriate amount of threads.
		let threads = super::threads_for_paths(vec_len);

		//------------------------------------------------------------- Begin `The Loop`.
		// No indentation because this function is crazy long.
		std::thread::scope(|scope| {
		for paths in vec_paths.chunks(threads) {
		scope.spawn(|| {
		for path in paths.iter() {
		let path = path.clone(); // TODO: figure out how to take ownership of this instead of cloning.

		// Get the tags for this `PathBuf`, skip on error.
		//
		// FIXME:
		// `lofty` doesn't have a partial-`Tag` API. It always reads all
		// the data from a file. AKA, the `Picture` data gets allocated
		// into an owned `Vec<u8>` for every single file...!
		//
		// This is obviously not ideal, we only need
		// the `Picture` data once per `Album`.
		//
		// For some reason though, this doesn't affect performance that much.
		// Basic tests show maybe `~1.5x-2x` speed improvements upon commenting
		// out all picture ops. Not that much faster.
		let mut tagged_file = match Self::path_to_tagged_file(&path) {
			Ok(t)  => t,
			Err(e) => { warn!("CCD - TaggedFile fail: {}", path.display()); continue; },
		};
		let mut tag = match Self::tagged_file_to_tag(&mut tagged_file) {
			Ok(t)  => t,
			Err(e) => { warn!("CCD - Tag fail: {}", path.display()); continue; },
		};
		let metadata = match Self::extract_tag_metadata(tagged_file, &mut tag) {
			Ok(t)  => t,
			Err(e) => { warn!("CCD - Metadata fail: {}", path.display()); continue; },
		};
		// Destructure tag metadata
		// into individual variables.
		let TagMetadata {
			artist,
			album,
			title,
			track,
			disc,
			track_total,
			disc_total,
			picture,
			runtime,
			release,
			track_artists,
			compilation,
		} = metadata;

		// Send update to `Kernel`.
		send!(to_kernel, CcdToKernel::UpdateIncrement((increment, title.to_string())));

		// Lock memory (HashMap).
		let mut memory = lock!(memory);

		//------------------------------------------------------------- If `Artist` exists.
		if let Some((artist_idx, album_map)) = memory.get_mut(&*artist) {

			//------------------------------------------------------------- If `Album` exists.
			if let Some(album_idx) = album_map.get(&*album) {
				// Create `Song`.
				let song = Song {
					title: title.to_string(),
					album: AlbumKey::from(*album_idx),
					runtime: Runtime::from(runtime),
					track,
					track_artists,
					disc,
					path,
					..Default::default()
				};

				// Lock.
				let mut vec_album = lock!(vec_album);
				let mut vec_song  = lock!(vec_song);

				// Push to `Vec<Song>`
				vec_song.push(song);

				// Update `Album`.
				vec_album[*album_idx].songs.push(SongKey::from(vec_song.len() - 1));

				continue
			}

			//------------------------------------------------------------- If `Artist` exists, but not `Album`.
			// Prepare `Song`.
			let song_title    = title.to_string();
			let runtime       = Runtime::from(runtime);

			// Prepare `Album`.
			let release = match release {
				Some(r) => Date::from_str_silent(r),
				None    => Date::unknown(),
			};
			let album_title   = album.to_string();
			let song_count    = Unsigned::zero();
			let runtime_album = Runtime::zero();
			let art = match picture {
				Some(bytes) => {
					*lock!(count_art) += 1;
					Art::Bytes(bytes.into())
				},
				_ => Art::Unknown,
			};

			// Lock.
			let mut vec_artist = lock!(vec_artist);
			let mut vec_album  = lock!(vec_album);
			let mut vec_song   = lock!(vec_song);

			// Create `Song`.
			let song = Song {
				title: song_title,
				runtime,
				track,
				track_artists,
				disc,
				path,
				album: AlbumKey::from(vec_album.len()),
				..Default::default()
			};

			// Create `Album`.
			let album_struct = Album {
				title: album_title,
				release,
				compilation,

				artist: ArtistKey::from(*artist_idx),
				songs: vec![SongKey::from(vec_song.len())],

				// Needs to be updated later.
				runtime: runtime_album,
				song_count,
				art,
				..Default::default()
			};

			// Update `Artist`.
			let count_album = vec_album.len();
			vec_artist[*artist_idx].albums.push(AlbumKey::from(count_album));

			// Push `Album/Song`.
			vec_album.push(album_struct);
			vec_song.push(song);

			// Drop locks.
			drop(vec_artist);
			drop(vec_album);
			drop(vec_song);

			// Add to `HashMap` memory.
			album_map.insert(album.to_string(), count_album);

			continue
		}

		//------------------------------------------------------------- If `Artist` DOESN'T exist.
		// Prepare `Song`.
		let title   = title.to_string();
		let runtime = Runtime::from(runtime);

		// Prepare `Album`.
		let release = match release {
			Some(r) => Date::from_str_silent(r),
			None    => Date::unknown(),
		};
		let album_title   = album.to_string();
		let song_count    = Unsigned::zero();
		let runtime_album = Runtime::zero();
		let art = match picture {
			Some(bytes) => {
				*lock!(count_art) += 1;
				Art::Bytes(bytes.into())
			},
			_ => Art::Unknown,
		};

		// Prepare `Artist`.
		let name = artist.to_string();

		// Lock.
		let mut vec_artist = lock!(vec_artist);
		let mut vec_album  = lock!(vec_album);
		let mut vec_song   = lock!(vec_song);

		// Create `Song`.
		let song = Song {
			title,
			runtime,
			track,
			track_artists,
			disc,
			path,
			album: AlbumKey::from(vec_album.len()),
			..Default::default()
		};

		// Create `Album`.
		let album_struct = Album {
			title: album_title,
			release,
			compilation,

			artist: ArtistKey::from(vec_artist.len()),
			songs: vec![SongKey::from(vec_song.len())],

			// Needs to be updated later.
			runtime: runtime_album,
			song_count,
			art,
			..Default::default()
		};

		// Create `Artist`.
		let count_artist = vec_artist.len();
		let count_album  = vec_album.len();
		let artist_struct = Artist {
			name,
			albums: vec![AlbumKey::from(count_album)],
			..Default::default()
		};

		// Push `Artist/Album/Song`.
		vec_artist.push(artist_struct);
		vec_album.push(album_struct);
		vec_song.push(song);

		// Drop locks.
		drop(vec_artist);
		drop(vec_album);
		drop(vec_song);

		// Add to `HashMap` memory.
		let artist = artist.to_string();
		let map    = HashMap::from([(album.to_string(), count_album)]);
		let tuple  = (count_artist, map);

//		lock!(memory).insert(artist, tuple);
		memory.insert(artist, tuple);

		//------------------------------------------------------------- End of `The Loop`.
		}   // for path in paths
		}); // scope.spawn
		}   // for paths in vec_paths
		}); // std::thread::scope

		// Unwrap the `Mutex`.
		//
		// INVARIANT:
		// As long as none of the above `scoped` threads
		// `panic()!`'ed, these `.into_inner()`'s are safe.
		(
			vec_artist.into_inner().unwrap(),
			vec_album.into_inner().unwrap(),
			vec_song.into_inner().unwrap(),
			count_art.into_inner().unwrap(),
		)
	}

	#[inline(always)]
	// Takes in the incomplete `Vec`'s from above and fixes some stuff.
	//
	// The last `Album` field after this, `Art`, will be completed in the `convert` phase.
	pub(super) fn fix_metadata(vec_artist: &mut [Artist], vec_album: &mut [Album], vec_song: &[Song]) {
		// Fix `Album` order in the `Artist` (release order).
		for artist in vec_artist {
			artist.albums.sort_by(|a, b| {
				vec_album[a.inner()].release.cmp(
					&vec_album[b.inner()].release
				)
			});
		}

		for album in vec_album {
			// Song count.
			album.song_count = Unsigned::from(album.songs.len());

			// Total runtime.
			album.runtime = Runtime::from(album.songs
				.iter()
				.map(|key| vec_song[key.inner()].runtime.inner())
				.sum::<u32>());

			// Sort songs based off `track`.
			album.songs.sort_by(|a, b|
				vec_song[a.inner()].track.cmp(
					&vec_song[b.inner()].track
				)
			);
		}
	}

	//---------------------------------------------------------------------------------------------------- Private tag functions.
	#[inline(always)]
	// Attempts to probe a `Path`.
	//
	// This is the `heaviest` function within the entire `new_collection()` function.
	// It accounts for around 90% of the total time spent making the `Collection`.
	fn path_to_tagged_file(path: &Path) -> Result<lofty::TaggedFile, anyhow::Error> {
		use std::fs::File;
		use std::io::BufReader;

		// Open `Path`.
		let file = File::open(path)?;
		let reader = BufReader::new(file);

		// Create the `lofty::Probe` options.
		let options = lofty::ParseOptions::new().parsing_mode(lofty::ParsingMode::Relaxed);

		// Create `lofty::Probe`.
		let probe = lofty::Probe::new(reader).options(options);

		// This could include be a concrete type read since
		// we already have MIME information, but in testing,
		// it seems like it's actually the same speed whether
		// `lofty` guesses or knows beforehand.
		Ok(probe.guess_file_type()?.read()?)
	}

	#[inline(always)]
	// Attempts to extract tags from a `TaggedFile`.
	fn tagged_file_to_tag(tagged_file: &mut lofty::TaggedFile) -> Result<lofty::Tag, anyhow::Error> {
		if let Some(t) = tagged_file.remove(lofty::TagType::VorbisComments) {
			Ok(t)
		} else if let Some(t) = tagged_file.remove(lofty::TagType::Id3v2) {
			Ok(t)
		} else if let Some(t) = tagged_file.remove(lofty::TagType::Id3v1) {
			Ok(t)
		} else {
			Err(anyhow!("No tag"))
		}
	}

	#[inline(always)]
	// Get the audio runtime of the `TaggedFile`.
	fn tagged_file_runtime(tagged_file: lofty::TaggedFile) -> f64 {
		tagged_file.properties().duration().as_secs_f64()
	}

	#[inline]
	// Extracts `lofty`'s `ItemValue`.
	fn item_value_to_str(item: &lofty::ItemValue) -> Option<&str> {
		match item {
			lofty::ItemValue::Text(s)    => Some(s),
			lofty::ItemValue::Locator(s) => Some(s),
			lofty::ItemValue::Binary(b)  => {
				if let Ok(s) = std::str::from_utf8(b) {
					Some(s)
				} else {
					None
				}
			},
		}
	}

	#[inline(always)]
	// Attempt to get the release date of the `TaggedFile`.
	fn tag_release(tag: &lofty::Tag) -> Option<&str> {
		// Attempt #1.
		if let Some(t) = tag.get(&lofty::ItemKey::OriginalReleaseDate) {
			if let Some(s) = Self::item_value_to_str(t.value()) {
				return Some(s)
			}
		}

		// Attempt #2.
		if let Some(t) = tag.get(&lofty::ItemKey::RecordingDate) {
			if let Some(s) = Self::item_value_to_str(t.value()) {
				return Some(s)
			}
		}

		// Attempt #3.
		if let Some(t) = tag.get(&lofty::ItemKey::Year) {
			if let Some(s) = Self::item_value_to_str(t.value()) {
				return Some(s)
			}
		}

		// Give up.
		None
	}

	#[inline(always)]
	// Attempt to get the _maybe_ multiple track artists of the `TaggedFile`.
	fn tag_track_artists(tag: &lofty::Tag) -> Option<String> {
		// Attempt #1.
		if let Some(t) = tag.get(&lofty::ItemKey::Performer) {
			if let Some(s) = Self::item_value_to_str(t.value()) {
				return Some(s.to_string())
			}
		}

		// Attempt #2.
		if let Some(t) = tag.get(&lofty::ItemKey::TrackArtist) {
			if let Some(s) = Self::item_value_to_str(t.value()) {
				return Some(s.to_string())
			}
		}

		// Give up.
		None
	}

	#[inline(always)]
	// Find out if this `TaggedFile` belongs to a compilation.
	fn tag_compilation(artist: &str, tag: &lofty::Tag) -> bool {
		// `FlagCompilation`.
		if let Some(t) = tag.get(&lofty::ItemKey::FlagCompilation) {
			if let Some(s) = Self::item_value_to_str(t.value()) {
				if s == "1" {
					return true
				}
			}
		}

		// `Various Artists`
		// This metadata is unique to Itunes.
		if let Some(t) = tag.get(&lofty::ItemKey::AlbumArtist) {
			if let Some(s) = Self::item_value_to_str(t.value()) {
				if s == "Various Artists" && s != artist {
					return true
				}
			}
		}

		false
	}

	#[inline(always)]
	// Attempts to extract tags from a `TaggedFile`.
	// `TaggedFile` gets dropped here, since it is no longer needed.
	fn extract_tag_metadata(tagged_file: lofty::TaggedFile, tag: &mut lofty::Tag) -> Result<TagMetadata, anyhow::Error> {
		// Image metadata.
		// This needs to be first because it needs `mut`.
		// and the next operations create `&`, meaning I
		// can't mutate `tag` after that.
		let picture = {
			if tag.pictures().is_empty() {
				None
			} else {
				// This removes the `Picture`, and cheaply
				// takes ownership of the inner `Vec`.
				Some(tag.remove_picture(0).into_data())
			}
		};

		// Attempt to get _needed_ metadata.
		let artist      = match tag.artist()      { Some(t) => t, None => bail!("No artist") };
		let album       = match tag.album()       { Some(t) => t, None => bail!("No album") };
		let title       = match tag.title()       { Some(t) => t, None => bail!("No title") };

		// Attempt to get not necessarily needed metadata.
		let track       = tag.track();
		let disc        = tag.disk();
		let track_total = tag.track_total();
		let disc_total  = tag.disk_total();

		// Other data that can't be obtained from `tag._()`.
		let runtime       = Self::tagged_file_runtime(tagged_file);
		let release       = Self::tag_release(tag);
		let track_artists = Self::tag_track_artists(tag);
		let compilation   = Self::tag_compilation(&artist, tag);

		Ok(TagMetadata {
			artist,
			album,
			title,
			track,
			disc,
			track_total,
			disc_total,
			picture,
			runtime,
			release,
			track_artists,
			compilation,
		})
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use crate::ccd::Ccd;
	use std::path::PathBuf;
	use lofty::TaggedFile;

	#[test]
	#[ignore]
	fn vecs() {
		// Set-up logger.
		crate::logger::init_logger(log::LevelFilter::Trace);

		// Convert `PathBuf` into `Vec`.
		let paths = vec![
			PathBuf::from("assets/audio/rain.mp3"),
			PathBuf::from("assets/audio/rain.flac"),
			PathBuf::from("assets/audio/rain.ogg"),
		];
		// Prepare inputs.
		let (to_kernel, _) = crossbeam::channel::unbounded::<super::CcdToKernel>();
		let (mut vec_artist, mut vec_album, vec_song, count_art) = Ccd::the_loop(&to_kernel, paths);

		println!("{:#?}", vec_artist);
		println!("{:#?}", vec_album);
		println!("{:#?}", vec_song);
		println!("{:#?}", count_art);

		// Assert `Vec`s are correct.
		assert!(vec_artist.len() == 1);
		assert!(vec_album.len()  == 1);
		assert!(vec_song.len()   == 3);
		assert!(count_art        == 1);

		// Assert `Artist` is correct.
		assert!(vec_artist[0].name         == "hinto");
		assert!(vec_artist[0].albums.len() == 1);

		// Assert `Album` is correct.
		assert!(vec_album[0].title          == "Festival");
		assert!(vec_album[0].artist.inner() == 0);
		assert!(vec_album[0].songs.len()    == 3);
		assert!(vec_album[0].release        == "2023-03-08");
		assert!(vec_album[0].release        == (2023, 3, 8));
		// TODO: this should be true
//		assert!(vec_album[0].compilation    == true);

		// Fix the metadata.
		Ccd::fix_metadata(&mut vec_artist, &mut vec_album, &vec_song);

		println!("{:#?}", vec_artist);
		println!("{:#?}", vec_album);
		println!("{:#?}", vec_song);

		// Assert metadata is fixed.
		assert!(vec_album[0].runtime    >= readable::Runtime::from(3.8));
		assert!(vec_album[0].runtime    >= 3);
		assert!(vec_album[0].song_count == "3");
		assert!(vec_album[0].song_count == 3);
	}

	fn mp3() -> TaggedFile {
		let mp3 = Ccd::path_to_tagged_file(PathBuf::from("assets/audio/rain.mp3").as_path()).unwrap();
		mp3
	}

	#[test]
	fn runtime() {
		let mp3 = mp3();
		let runtime = Ccd::tagged_file_runtime(mp3);
		eprintln!("{}", runtime);
		assert!(runtime >= 1.9);
	}

	#[test]
	fn release() {
		let mut mp3 = mp3();
		let tag = Ccd::tagged_file_to_tag(&mut mp3).unwrap();
		let release = Ccd::tag_release(&tag).unwrap();
		eprintln!("{}", release);
		assert!(release == "2023-03-08");
	}

	#[test]
	// TODO:
	// This isn't picking up the right tag.
	// Probably a bug with the `mp3` file metadata
	// instead of the function.
	fn track_artists() {
		let mut mp3 = mp3();
		let tag = Ccd::tagged_file_to_tag(&mut mp3).unwrap();
		let track_artist = Ccd::tag_track_artists(&tag).unwrap();
		eprintln!("{}", track_artist);
		assert!(track_artist == "hinto");
	}

	#[test]
	fn compilation() {
		let mut mp3 = mp3();
		let tag = Ccd::tagged_file_to_tag(&mut mp3).unwrap();
		let comp = Ccd::tag_compilation("hinto", &tag);
		eprintln!("{}", comp);
		assert!(comp);
	}

	#[test]
	fn extract() {
		let mut mp3 = mp3();
		let mut tag = Ccd::tagged_file_to_tag(&mut mp3).unwrap();
		let meta = Ccd::extract_tag_metadata(mp3, &mut tag).unwrap();
		eprintln!("{:#?}", meta);

		assert!(meta.artist        == "hinto");
		assert!(meta.album         == "Festival");
		assert!(meta.title         == "rain_mp3");
		assert!(meta.track         == Some(1));
		assert!(meta.disc          == None);
		assert!(meta.track_total   == None);
		assert!(meta.disc_total    == None);
		assert!(meta.picture       == None);
		assert!(meta.runtime       == 1.968);
		assert!(meta.release       == Some("2023-03-08"));
		assert!(meta.track_artists == Some("hinto".to_string()));
		assert!(meta.compilation   == true);
	}
}
