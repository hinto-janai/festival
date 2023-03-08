//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
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
use human::{HumanTime,HumanNumber};
use std::borrow::Cow;

//---------------------------------------------------------------------------------------------------- Tag Metadata (temporary) struct.
#[derive(Debug)]
struct TagMetadata<'a> {
	artist: Cow<'a, str>,
	album: Cow<'a, str>,
	title: Cow<'a, str>,
	track: Option<u32>,
	disc: Option<u32>,
	track_total: Option<u32>,
	disc_total: Option<u32>,
	picture: Option<&'a [u8]>,

	length: f64,
	release: Option<&'a str>,
	track_artists: Option<String>,

	compilation: bool,
}

//---------------------------------------------------------------------------------------------------- Metadata functions.
impl super::Ccd {
	#[inline(always)]
	// Attempts to probe a `Path`.
	fn path_to_tagged_file(path: &Path) -> Result<lofty::TaggedFile, anyhow::Error> {
		Ok(lofty::Probe::open(path)?.guess_file_type()?.read()?)
	}

	#[inline(always)]
	// Attempts to extract tags from a `TaggedFile`.
	fn tagged_file_to_tag(tagged_file: &lofty::TaggedFile) -> Result<&lofty::Tag, anyhow::Error> {
		if let Some(t) = tagged_file.primary_tag() {
			Ok(t)
		} else if let Some(t) = tagged_file.first_tag() {
			Ok(t)
		} else {
			Err(anyhow!("No tag"))
		}
	}

	#[inline(always)]
	// Get the audio length of the `TaggedFile`.
	fn tagged_file_length(tagged_file: &lofty::TaggedFile) -> f64 {
		tagged_file.properties().duration().as_secs_f64()
	}

	#[inline]
	// Extracts `lofty`'s `ItemValue`.
	fn item_value_to_str<'a>(item: &'a lofty::ItemValue) -> Option<&'a str> {
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
	fn tag_release<'a>(tag: &'a lofty::Tag) -> Option<&'a str> {
		// Attempt #1.
		if let Some(t) = tag.get_item_ref(&lofty::ItemKey::OriginalReleaseDate) {
			if let Some(s) = Self::item_value_to_str(&t.value()) {
				return Some(s)
			}
		}

		// Attempt #2.
		if let Some(t) = tag.get_item_ref(&lofty::ItemKey::RecordingDate) {
			if let Some(s) = Self::item_value_to_str(&t.value()) {
				return Some(s)
			}
		}

		// Attempt #3.
		if let Some(t) = tag.get_item_ref(&lofty::ItemKey::Year) {
			if let Some(s) = Self::item_value_to_str(&t.value()) {
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
		if let Some(t) = tag.get_item_ref(&lofty::ItemKey::Performer) {
			if let Some(s) = Self::item_value_to_str(&t.value()) {
				return Some(s.to_string())
			}
		}

		// Attempt #2.
		if let Some(t) = tag.get_item_ref(&lofty::ItemKey::TrackArtist) {
			if let Some(s) = Self::item_value_to_str(&t.value()) {
				return Some(s.to_string())
			}
		}

		// Give up.
		None
	}

	#[inline(always)]
	// Find out if this `TaggedFile` belongs to a compilation.
	fn tag_compilation<'a>(artist: &str, tag: &'a lofty::Tag) -> bool {
		// `FlagCompilation`.
		if let Some(t) = tag.get_item_ref(&lofty::ItemKey::FlagCompilation) {
			if let Some(s) = Self::item_value_to_str(&t.value()) {
				if s == "1" {
					return true
				}
			}
		}

		// `Various Artists`
		// This metadata is unique to Itunes.
		if let Some(t) = tag.get_item_ref(&lofty::ItemKey::AlbumArtist) {
			if let Some(s) = Self::item_value_to_str(&t.value()) {
				if s == "Various Artists" && s != artist {
					return true
				}
			}
		}

		false
	}

	#[inline(always)]
	// Attempts to extract tags from a `TaggedFile`.
	fn extract_tag_metadata<'a>(tagged_file: &'a lofty::TaggedFile, tag: &'a lofty::Tag) -> Result<TagMetadata<'a>, anyhow::Error> {
		// Attempt to get _needed_ metadata.
		let artist      = match tag.artist()      { Some(t) => t, None => bail!("No artist") };
		let album       = match tag.album()       { Some(t) => t, None => bail!("No album") };
		let title       = match tag.title()       { Some(t) => t, None => bail!("No title") };
		// Attempt to get not necessarily needed metadata.
		let track       = tag.track();
		let disc        = tag.disk();
		let track_total = tag.track_total();
		let disc_total  = tag.disk_total();
		let picture = {
			let pictures = tag.pictures();

			if pictures.len() == 0 {
				None
			} else {
				Some(pictures[0].data())
			}
		};

		// Other data that can't be obtained from `tag._()`.
		let length        = Self::tagged_file_length(tagged_file);
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
			length,
			release,
			track_artists,
			compilation,
		})
	}

	#[inline(always)]
	// Takes in input of a filtered `Vec<PathBuf>` of audio files.
	// Loops over all `PathBuf`'s and adds metadata onto the `Vec`'s.
	//
	// Outputs the three main `Vec`'s of the `Collection` with
	// mostly done but incomplete data (needs sorting, addition, etc).
	fn audio_paths_to_metadata(paths: Vec<PathBuf>) -> Result<(Vec<Artist>, Vec<Album>, Vec<Song>), anyhow::Error> {
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
		//   usize          usize         usize
		//```
		// - We have a "Working Memory" that keeps track of what `Artist/Album` we've seen already.
		// - We have 3 `Vec`'s (that will eventually become the `Collection`).
		// - We have 3 `usize`'s that represent how many `Artist/Album/Song` we've seen.
		//
		// The "Working Memory" is a `HashMap` that takes in `String` input of an artist name and returns the `index` to it,
		// along with another `HashMap` which represents that `Artist`'s `Album`'s and its appropriate `indicies`.
		//
		//                       Artist  Artist's index     Album  Album's index
		//                        Name   in `Vec<Artist>`   Name   in `Vec<Album>`
		//                          |          |              |         |
		//                          v          v              v         v
		let mut memory:     HashMap<String, (usize, HashMap<String, usize>)> = HashMap::new();
		let mut vec_artist: Vec<Artist> = vec![];
		let mut vec_album:  Vec<Album>  = vec![];
		let mut vec_song:   Vec<Song>   = vec![];
		let mut count_artist: usize = 0;
		let mut count_album:  usize = 0;
		let mut count_song:   usize = 0;

		// In this loop, each `PathBuf` represents a new `Song` with metadata.
		// There are 3 logical possibilities with 3 actions associated with them:
		//     1. `Artist` exists, `Album` exists         => Add `Song`
		//     2. `Artist` exists, `Album` DOESN'T exist  => Add `Album + Song`
		//     3. `Artist` DOESN'T exist                  => Add `Artist + Album + Song`
		//
		// Counts and memory must be updated as well.

		//------------------------------------------------------------- Begin loop for each `PathBuf`.
		// No indentation because this function is crazy long.
		for path in paths {

		// Get the tags for this `PathBuf`.
		let tagged_file = Self::path_to_tagged_file(&path)?;
		let tag         = Self::tagged_file_to_tag(&tagged_file)?;
		let metadata    = Self::extract_tag_metadata(&tagged_file, &tag)?;
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
			length,
			release,
			track_artists,
			compilation,
		} = metadata;

		//------------------------------------------------------------- If `Artist` exists.
		if let Some((artist_idx, album_map)) = memory.get(&*artist) {

			//------------------------------------------------------------- If `Album` exists.
			if let Some(album_idx) = album_map.get(&*album) {
				// Create `Song`.
				let song = Song {
					title: title.to_string(),
					album: AlbumKey::from(*album_idx),
					length_human: HumanTime::new(),
					track,
					track_artists,
					disc,
					length,
					path: PathBuf::new(),
				};

				// Update `Album`.
				vec_album[*album_idx].songs.push(SongKey::from(count_song));

				// Push to `Vec<Song>`
				vec_song.push(song);

				// Increment `Song` count.
				count_song += 1;

				bail!(""); // TODO: replace with `return`.
			}

			//------------------------------------------------------------- If `Artist` exists, but not `Album`.
			// Create `Song`.
			let song = Song {
				title: title.to_string(),
				album: AlbumKey::from(count_album),
				length_human: HumanTime::new(),
				track,
				track_artists,
				disc,
				length,
				path: PathBuf::new(),
			};

			// Get `Album` art bytes.
			let art_bytes = match picture {
				Some(p) => Some(p.to_vec()),
				None    => None,
			};

			// Create `Album`.
			let album = Album {
				title: String::new(),
				artist: ArtistKey::from(count_artist),
				release_human: String::new(),
				length_human: HumanTime::new(),
				song_count_human: HumanNumber::unknown(),
				songs: vec![SongKey::from(count_song)],
				release: 0,
				length: 0.0,
				song_count: 0,
				disc_count: 0,
				art: Art::Unknown,
				art_bytes,
				compilation,
			};

			// Update `Artist`.
			vec_artist[*artist_idx].albums.push(AlbumKey::from(count_album));

			// Push `Album/Song`.
			vec_album.push(album);
			vec_song.push(song);

			// Increment `Album/Song` count.
			count_album += 1;
			count_song += 1;

			bail!(""); // TODO: replace with `return`.
		}

		//------------------------------------------------------------- If `Artist` DOESN'T exist.
		// Create `Song`.
		let song = Song {
			title: title.to_string(),
			album: AlbumKey::from(count_album),
			length_human: HumanTime::new(),
			track,
			track_artists,
			disc,
			length,
			path: PathBuf::new(),
		};

		// Get `Album` art bytes.
		let art_bytes = match picture {
			Some(p) => Some(p.to_vec()),
			None    => None,
		};

		// Create `Album`.
		let album = Album {
			title: String::new(),
			artist: ArtistKey::from(count_artist),
			release_human: String::new(),
			length_human: HumanTime::new(),
			song_count_human: HumanNumber::unknown(),
			songs: vec![SongKey::from(count_song)],
			release: 0,
			length: 0.0,
			song_count: 0,
			disc_count: 0,
			art: Art::Unknown,
			art_bytes,
			compilation,
		};

		// Create `Artist`.
		let artist = Artist {
			name: artist.to_string(),
			albums: vec![AlbumKey::from(count_album)],
		};

		// Push `Artist/Album/Song`.
		vec_artist.push(artist);
		vec_album.push(album);
		vec_song.push(song);

		// Increment `Artist/Album/Song` count.
		count_artist += 1;
		count_album += 1;
		count_song += 1;

		//------------------------------------------------------------- End of initial `for` loop.
		}

		// Return the resulting `Vec`'s.
		Ok((vec_artist, vec_album, vec_song))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use crate::ccd::Ccd;
	use std::path::PathBuf;
	use lofty::TaggedFile;

	fn mp3() -> TaggedFile {
		let mp3 = Ccd::path_to_tagged_file(PathBuf::from("assets/audio/rain.mp3").as_path()).unwrap();
		mp3
	}

	#[test]
	fn length() {
		let mp3 = mp3();
		let length = Ccd::tagged_file_length(&mp3);
		eprintln!("{}", length);
		assert!(length == 1.968);
	}

	#[test]
	fn release() {
		let mp3 = mp3();
		let tag = Ccd::tagged_file_to_tag(&mp3).unwrap();
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
		let mp3 = mp3();
		let tag = Ccd::tagged_file_to_tag(&mp3).unwrap();
		let track_artist = Ccd::tag_track_artists(tag).unwrap();
		eprintln!("{}", track_artist);
		assert!(track_artist == "hinto");
	}

	#[test]
	fn compilation() {
		let mp3 = mp3();
		let tag = Ccd::tagged_file_to_tag(&mp3).unwrap();
		let comp = Ccd::tag_compilation("hinto", tag);
		eprintln!("{}", comp);
		assert!(comp);
	}

	#[test]
	fn extract() {
		let mp3 = mp3();
		let tag = Ccd::tagged_file_to_tag(&mp3).unwrap();
		let meta = Ccd::extract_tag_metadata(&mp3, &tag).unwrap();
		eprintln!("{:#?}", meta);

		assert!(meta.artist        == "hinto");
		assert!(meta.album         == "Festival");
		assert!(meta.title         == "rain_mp3");
		assert!(meta.track         == Some(1));
		assert!(meta.disc          == None);
		assert!(meta.track_total   == None);
		assert!(meta.disc_total    == None);
		assert!(meta.picture       == None);
		assert!(meta.length        == 1.968);
		assert!(meta.release       == Some("2023-03-08"));
		assert!(meta.track_artists == Some("hinto".to_string()));
		assert!(meta.compilation   == true);
	}
}
