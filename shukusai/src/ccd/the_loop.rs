//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
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
use std::sync::{Arc,Mutex};
use std::sync::atomic::AtomicUsize;
use std::marker::PhantomData;
use symphonia::core::{
	io::MediaSourceStream,
	probe::{Probe,ProbeResult,Hint},
	formats::Track,
	meta::{Metadata,MetadataRevision,Tag,Visual,StandardTagKey},
};

//---------------------------------------------------------------------------------------------------- Tag Metadata (temporary) struct.
#[derive(Debug)]
// This is just a temporary container tag data.
struct TagMetadata {
	// Required or we skip the file.
	artist: String,
	album: String,
	title: String,
	runtime: u64,
	sample_rate: u32,

	// Optional.
	track: Option<u32>,
	disc: Option<u32>,
	art: Option<Box<[u8]>>,
	release: Option<String>,
}

//---------------------------------------------------------------------------------------------------- Metadata functions.
impl crate::ccd::Ccd {
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
		// Using a `Vec` and/or `BTreeMap` instead was considered, since 99% of the time,
		// user's `Artist`'s will have <10 `Album`'s and those `Album`'s will have <20 `Song`'s,
		// and thus a binary search would (most of the time) end up faster than a hash lookup.
		//
		// Although after testing, the speed increase was not much. Having `HashMap`'s all the
		// way down also means it can scale to ridiculous amounts if the user for whatever reason
		// has `Artist`'s with 1000s of `Album`s or `Album`'s with 1000s of `Song`'s.
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
		//     1. `Artist` exists && `Album` exists         => Add `Song`
		//     2. `Artist` exists && `Album` DOESN'T exist  => Add `Album + Song`
		//     3. `Artist` DOESN'T exist                    => Add `Artist + Album + Song`
		//
		// Memory must be updated as well.

		// Get an appropriate amount of threads.
		let threads = super::threads_for_paths(vec_len);
		let chunks  = {
			let c = vec_paths.len() / threads;
			match c {
				0 => 1,
				_ => c,
			}
		};

		//------------------------------------------------------------- Begin `The Loop`.
		// No indentation because this function is crazy long.
		std::thread::scope(|scope| {             // Enter thread scope.
		for paths in vec_paths.chunks(chunks) {  // Chunk the total paths for each thread.
		scope.spawn(|| {                         // Spawn a thread.
		for path in paths.iter() {               // Make thread work over the chunked paths.

		// FIXME:
		// Figure out how to take ownership of this instead of cloning.
		let path = path.clone();

		// Get the tags for this `PathBuf`, skip on error.
		//
		// FIXME:
		// `symphonia` doesn't have a partial-`Tag` API. It always reads all
		// the data from a file. AKA, the `Picture` data gets allocated
		// into an owned `Vec<u8>` for every single file...!
		//
		// This is obviously not ideal, we only need
		// the `Picture` data once per `Album`.
		//
		// For some reason though, this doesn't affect performance that much.
		// Basic tests show maybe `~1.5x-2x` speed improvements upon commenting
		// out all picture ops. Not that much faster.
		let metadata = match Self::extract(&path) {
			Ok(t)  => t,
			Err(e) => {
				warn!("{} ... {e}", path.display());
				continue;
			},
		};

		// Destructure tag metadata
		// into individual variables.
		let TagMetadata {
			artist,
			album,
			title,
			runtime,
			sample_rate,

			track,
			disc,
			art,
			release,
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
					title,
					album: AlbumKey::from(*album_idx),
					runtime: Runtime::from(runtime),
					sample_rate,
					track,
					disc,
					path,

    				_track_artists: PhantomData,
    				_lyrics: PhantomData,
    				_reserved1: PhantomData,
				    _reserved2: PhantomData,
				    _reserved3: PhantomData,
				    _reserved4: PhantomData,
				    _reserved5: PhantomData,
				    _reserved6: PhantomData,
				};

				// Lock.
				let mut vec_album = lock!(vec_album);
				let mut vec_song  = lock!(vec_song);

				// Push to `Vec<Song>`
				vec_song.push(song);

				// Update `Album`.
				vec_album[*album_idx].songs.push(SongKey::from(vec_song.len() - 1));

				continue;
			}

			//------------------------------------------------------------- If `Artist` exists, but not `Album`.
			// Prepare `Song`.
			let song_title    = title;
			let runtime       = Runtime::from(runtime);

			// Prepare `Album`.
			let release = match release {
				Some(r) => Date::from_str_silent(&r),
				None    => Date::unknown(),
			};
			let album_title   = album.clone();
			let song_count    = Unsigned::zero();
			let runtime_album = Runtime::zero();
			let art = match art {
				Some(bytes) => {
					*lock!(count_art) += 1;
					Art::Bytes(bytes.into())
				},
				_ => {
					if let Some(bytes) = Self::maybe_find_img(&path) {
						*lock!(count_art) += 1;
						Art::Bytes(bytes.into())
					} else {
						Art::Unknown
					}
				},
			};
			let path_parent = match path.parent() {
				Some(p) => p.to_path_buf(),
				None    => path.clone(),
			};

			// Lock.
			let mut vec_artist = lock!(vec_artist);
			let mut vec_album  = lock!(vec_album);
			let mut vec_song   = lock!(vec_song);

			// Create `Song`.
			let song = Song {
				title: song_title,
				runtime,
				sample_rate,
				track,
				disc,
				path,
				album: AlbumKey::from(vec_album.len()),

				_track_artists: PhantomData,
				_lyrics: PhantomData,
				_reserved1: PhantomData,
			    _reserved2: PhantomData,
			    _reserved3: PhantomData,
			    _reserved4: PhantomData,
			    _reserved5: PhantomData,
			    _reserved6: PhantomData,
			};

			// Create `Album`.
			let album_struct = Album {
				title: album_title,
				release,

				artist: ArtistKey::from(*artist_idx),
				songs: vec![SongKey::from(vec_song.len())],
				path: path_parent,

				// Needs to be updated later.
				runtime: runtime_album,
				discs: 0,
				song_count,
				art,

			    _genre: PhantomData,
			    _compilation: PhantomData,
			    _reserved1: PhantomData,
			    _reserved2: PhantomData,
			    _reserved3: PhantomData,
			    _reserved4: PhantomData,
			    _reserved5: PhantomData,
			    _reserved6: PhantomData,
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
			album_map.insert(album, count_album);

			continue
		}

		//------------------------------------------------------------- If `Artist` DOESN'T exist.
		// Prepare `Song`.
		let runtime = Runtime::from(runtime);

		// Prepare `Album`.
		let release = match release {
			Some(r) => Date::from_str_silent(&r),
			None    => Date::unknown(),
		};
		let album_title   = album.clone();
		let song_count    = Unsigned::zero();
		let runtime_album = Runtime::zero();
		let art = match art {
			Some(bytes) => {
				*lock!(count_art) += 1;
				Art::Bytes(bytes.into())
			},
			_ => {
				if let Some(bytes) = Self::maybe_find_img(&path) {
					*lock!(count_art) += 1;
					Art::Bytes(bytes.into())
				} else {
					Art::Unknown
				}
			},
		};
		let path_parent = match path.parent() {
			Some(p) => p.to_path_buf(),
			None    => path.clone(),
		};

		// Prepare `Artist`.
		let name = artist.clone();

		// Lock.
		let mut vec_artist = lock!(vec_artist);
		let mut vec_album  = lock!(vec_album);
		let mut vec_song   = lock!(vec_song);

		// Create `Song`.
		let song = Song {
			title,
			runtime,
			sample_rate,
			track,
			disc,
			path,
			album: AlbumKey::from(vec_album.len()),

			_track_artists: PhantomData,
			_lyrics: PhantomData,
			_reserved1: PhantomData,
		    _reserved2: PhantomData,
		    _reserved3: PhantomData,
		    _reserved4: PhantomData,
		    _reserved5: PhantomData,
		    _reserved6: PhantomData,
		};

		// Create `Album`.
		let album_struct = Album {
			title: album_title,
			release,

			artist: ArtistKey::from(vec_artist.len()),
			songs: vec![SongKey::from(vec_song.len())],
			path: path_parent,

			// Needs to be updated later.
			runtime: runtime_album,
			discs: 0,
			song_count,
			art,

		    _genre: PhantomData,
		    _compilation: PhantomData,
		    _reserved1: PhantomData,
		    _reserved2: PhantomData,
		    _reserved3: PhantomData,
		    _reserved4: PhantomData,
		    _reserved5: PhantomData,
		    _reserved6: PhantomData,
		};

		// Create `Artist`.
		let count_artist = vec_artist.len();
		let count_album  = vec_album.len();
		let artist_struct = Artist {
			name,

			// Will be updated later.
			runtime: Runtime::zero(),
			albums: vec![AlbumKey::from(count_album)],
    		songs: Box::new([]),

		    _reserved1: PhantomData,
		    _reserved2: PhantomData,
		    _reserved3: PhantomData,
		    _reserved4: PhantomData,
		    _reserved5: PhantomData,
		    _reserved6: PhantomData,
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
		let map    = HashMap::from([(album.to_string(), count_album)]);
		let tuple  = (count_artist, map);

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
		let (
			mut vec_artist,
			mut vec_album,
			mut vec_song,
			count_art,
		) = (
			vec_artist.into_inner().unwrap(),
			vec_album.into_inner().unwrap(),
			vec_song.into_inner().unwrap(),
			count_art.into_inner().unwrap(),
		);

		vec_artist.shrink_to_fit();
		vec_album.shrink_to_fit();
		vec_song.shrink_to_fit();

		(vec_artist, vec_album, vec_song, count_art)
	}

	#[inline(always)]
	// Takes in the incomplete `Vec`'s from above and fixes some stuff.
	//
	// The last `Album` field after this, `Art`, will be completed in the `convert` phase.
	pub(super) fn fix_metadata(vec_artist: &mut [Artist], vec_album: &mut [Album], vec_song: &[Song]) {
		// `Album`'s.
		for album in vec_album.iter_mut() {
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

			// Fix `Album` disc count.
			let mut last_disc = vec_song[album.songs[0].inner()].disc;
			for key in album.songs.iter() {
				let song = &vec_song[key.inner()];
				if last_disc != song.disc {
					album.discs += 1;
				}
				last_disc = song.disc;
			}

			// Sort songs based off `disc` (if there's more than 1).
			if album.discs > 1 {
				album.songs.sort_by(|a, b|
					vec_song[a.inner()].disc.cmp(
						&vec_song[b.inner()].disc
					)
				);
			}
		}

		// Fix `Album` order in the `Artist` (release order).
		for artist in vec_artist {
			artist.albums.sort_by(|a, b| {
				vec_album[a.inner()].release.cmp(
					&vec_album[b.inner()].release
				)
			});

			// Total runtime.
			let runtime: u32 = artist.albums.iter().map(|a| vec_album[a.inner()].runtime.inner()).sum();
			artist.runtime = Runtime::from(runtime);

			// Collect `SongKey` for the `Artist`'s.
			artist.songs = artist.albums
				.iter()
				.flat_map(|k| vec_album[k.inner()].songs.iter().map(|k| *k))
				.collect();
		}

	}

	//---------------------------------------------------------------------------------------------------- Private tag functions.
	#[inline(always)]
	// Attempts to probe a `Path`.
	//
	// This is the 2nd `heaviest` function within the entire `new_collection()` function.
	// It accounts for around 20% of the total time spent making the `Collection`.
	fn probe(path: &Path) -> Result<ProbeResult, anyhow::Error> {
		let file = std::fs::File::open(path)?;
		let mss = MediaSourceStream::new(Box::new(file), Default::default());

		let probe = symphonia::default::get_probe();

		Ok(probe.format(
			&Hint::new(),
			mss,
			&Default::default(),
			&Default::default(),
		)?)
	}

	#[inline(always)]
	// Gets the metadata tags and the visuals.
	fn metadata(mut p: ProbeResult) -> Result<MetadataRevision, anyhow::Error> {
		// This is more likely to contain metadata.
		if let Some(md) = p.format.metadata().into_current() {
			return Ok(md);
		}

		// But, sometimes it is found here.
		if let Some(ml) = p.metadata.into_inner() {
			let mut md = ml.into_inner();
			if let Some(md) = md.pop_front() {
				return Ok(md);
			}
		}

		Err(anyhow!("No metadata found"))
	}

	#[inline(always)]
	// Get a tracks sample rate.
	fn sample_rate(track: &Track) -> Option<u32> {
		track.codec_params.sample_rate
	}

	#[inline(always)]
	// Get a tracks runtime.
	fn runtime(track: &Track) -> Option<u64> {
		let timestamp = match track.codec_params.n_frames {
			Some(ts) => ts,
			_        => return None,
		};

		let time = match track.codec_params.time_base {
			Some(tb) => tb.calc_time(timestamp),
			_        => return None,
		};

		Some(time.seconds)
	}

	#[inline(always)]
	// Attempt to get artist.
	fn tag_artist(tag: &mut [Tag]) -> Option<String> {
		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::AlbumArtist)) {
			let o = Self::value(t);
			if o.is_some() { return o; }
		}

		// This isn't first because many `Artist` metadata
		// fields contain the featured artists, e.g `Artist A x Artist B`.
		// `AlbumArtist` usually contains just the main `Artist` name, which we want.
		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::Artist)) {
			let o = Self::value(t);
			if o.is_some() { return o; }
		}

		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::Composer)) {
			let o = Self::value(t);
			if o.is_some() { return o; }
		}

		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::Performer)) {
			let o = Self::value(t);
			if o.is_some() { return o; }
		}

		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::OriginalArtist)) {
			let o = Self::value(t);
			if o.is_some() { return o; }
		}

		None
	}

	#[inline(always)]
	// Attempt to get album title.
	fn tag_album(tag: &mut [Tag]) -> Option<String> {
		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::Album)) {
			let o = Self::value(t);
			if o.is_some() { return o; }
		}

		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::OriginalAlbum)) {
			let o = Self::value(t);
			if o.is_some() { return o; }
		}

		None
	}

	#[inline(always)]
	// Attempt to get song title.
	fn tag_title(tag: &mut [Tag], path: &Path) -> Option<String> {
		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::TrackTitle)) {
			let o = Self::value(t);
			if o.is_some() { return o; }
		}

		// Fallback to file name.
		if let Some(os_str) = path.file_stem() {
			Some(os_str.to_string_lossy().into_owned())
		} else {
			None
		}
	}

	#[inline(always)]
	// Attempt to get track number.
	fn tag_track(tag: &mut [Tag]) -> Option<u32> {
		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::TrackNumber)) {
			Self::value_unsigned(t)
		} else {
			None
		}
	}

	#[inline(always)]
	// Attempt to get track disc number.
	fn tag_disc(tag: &mut [Tag]) -> Option<u32> {
		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::DiscNumber)) {
			Self::value_unsigned(t)
		} else {
			None
		}
	}

	#[inline(always)]
	// Attempt to get the release date.
	fn tag_release(tag: &mut [Tag]) -> Option<String> {
		if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::Date)) {
			Self::value(t)
		} else if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::ReleaseDate)) {
			Self::value(t)
		} else if let Some(t) = tag.iter_mut().find(|i| i.std_key == Some(StandardTagKey::OriginalDate)) {
			Self::value(t)
		} else {
			None
		}
	}

	#[inline(always)]
	fn art(mut visuals: Vec<Visual>) -> Option<Box<[u8]>> {
		if !visuals.is_empty() {
			Some(visuals.swap_remove(0).data)
		} else {
			None
		}
	}

	#[inline(always)]
	// Get the compilation bool.
	// Assume `false` if it doesn't exist.
	fn tag_compilation(tag: &[Tag]) -> bool {
		if let Some(t) = tag.iter().find(|i| i.std_key == Some(StandardTagKey::Compilation)) {
			Self::value_bool(t)
		} else {
			false
		}
	}

	#[inline(always)]
	// Extract a `Tag`'s `Value` to a string.
	//
	// This expects values that are supposed to be strings.
	//
	// If the value is empty, this returns none.
	fn value(tag: &mut Tag) -> Option<String> {
		use symphonia::core::meta::Value;
		match &mut tag.value {
			Value::String(s) => {
				match s.split_whitespace().next() {
					Some(_) => Some(std::mem::take(s)),
					None    => None,
				}
			},
			Value::Binary(b) => {
				let mut dst: Box<[u8]> = Box::new([]);
				std::mem::swap(b, &mut dst);
				let vec = Vec::from(dst);
				match std::string::String::from_utf8(vec) {
					Ok(s) => {
						match s.split_whitespace().next() {
							Some(_) => Some(s),
							None    => None,
						}
					},
					_     => None,
				}
			},
			Value::UnsignedInt(u) => Some(u.to_string()),
			Value::SignedInt(s)   => Some(s.to_string()),
			Value::Float(f)       => Some(f.to_string()),
			Value::Boolean(b)     => Some(b.to_string()),
			Value::Flag           => None,
		}
	}

	#[inline(always)]
	// Extract a `Tag`'s `Value` to a number.
	//
	// This expects values that are supposed to be unsigned integers.
	fn value_unsigned(tag: &mut Tag) -> Option<u32> {
		use symphonia::core::meta::Value;
		match &tag.value {
			Value::UnsignedInt(u) => Some(*u as u32),
			Value::SignedInt(s)   => Some(*s as u32),
			Value::Float(f)       => Some(*f as u32),
			Value::Boolean(b) => {
				match b {
					true  => Some(1),
					false => Some(0),
				}
			},
			Value::String(s) => {
				if let Ok(u) = s.parse::<u32>() {
					Some(u)
				// Some `TrackNumber` fields are strings like `1/12`.
				} else if let Some(u) = s.split('/').next() {
					u.parse::<u32>().ok()
				} else {
					None
				}
			},
			Value::Binary(b) => {
				match std::str::from_utf8(b) {
					Ok(s) => {
						if let Ok(u) = s.parse::<u32>() {
							Some(u)
						} else if let Some(u) = s.split('/').next() {
							u.parse::<u32>().ok()
						} else {
							None
						}
					},
					_ => None,
				}
			},
			Value::Flag => None,
		}
	}

	#[inline(always)]
	// Extract a `Tag`'s `Value` to a bool
	//
	// This expects values that are supposed to be bool.
	fn value_bool(tag: &Tag) -> bool {
		use symphonia::core::meta::Value;
		match &tag.value {
			Value::Boolean(b) => *b,
			Value::String(s) => {
				match s.parse::<bool>() {
					Ok(b) => b,
					_     => false,
				}
			},
			Value::Binary(b) => {
				match std::str::from_utf8(b) {
					Ok(s) => {
						match s.parse::<bool>() {
							Ok(b) => b,
							_     => false,
						}
					},
					_ => false,
				}
			},

			_ => false,
		}
	}

	#[inline(always)]
	// Attempts to extract tags from a `TaggedFile`.
	// `TaggedFile` gets dropped here, since it is no longer needed.
	fn extract(path: &Path) -> Result<TagMetadata, anyhow::Error> {
		let probe_result = match Self::probe(path) {
			Ok(p)  => p,
			Err(e) => bail!(e),
		};

		let track       = match probe_result.format.tracks().get(0) { Some(t) => t, _ => bail!("track metadata missing") };
		let sample_rate = match Self::sample_rate(track)            { Some(t) => t, _ => bail!("sample rate metadata missing") };
		let runtime     = match Self::runtime(track)                { Some(t) => t, _ => bail!("runtime metadata missing") };
		let metadata    = match Self::metadata(probe_result)        { Ok(md) => md, Err(e) => bail!(e) };

		let (mut tags, visuals, _) = metadata.into_inner();

		// SOMEDAY:
		// We should handle compilations correctly.
		// But... for now, skip them entirely.
		if Self::tag_compilation(&tags) {
			bail!("compilation not supported");
		}

		// Attempt to get required metadata.
		let artist      = match Self::tag_artist(&mut tags)      { Some(t) => t, _ => bail!("artist metadata missing") };
		let album       = match Self::tag_album(&mut tags)       { Some(t) => t, _ => bail!("album metadata missing") };
		let title       = match Self::tag_title(&mut tags, path) { Some(t) => t, _ => bail!("title metadata missing") };

		// Optional metadata.
		let art     = Self::art(visuals);
		let track   = Self::tag_track(&mut tags);
		let disc    = Self::tag_disc(&mut tags);
		let release = Self::tag_release(&mut tags);

		Ok(TagMetadata {
			artist,
			album,
			title,
			runtime,
			sample_rate,

			track,
			disc,
			art,
			release,
		})
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use crate::ccd::Ccd;
	use std::path::PathBuf;

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
		assert!(meta.disc.is_none());
		assert!(meta.track_total.is_none());
		assert!(meta.disc_total.is_none());
		assert!(meta.art.is_none());
		assert!(meta.runtime       == 1.968);
		assert!(meta.release       == Some("2023-03-08"));
		assert!(meta.track_artists == Some("hinto".to_string()));
		assert!(meta.compilation);
	}
}
