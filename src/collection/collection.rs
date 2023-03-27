//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use super::{
	album::Album,
	artist::Artist,
	song::Song,
	plural::{Artists,Albums,Songs},
};
use crate::key::{
	Key,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use crate::sort::{
	ArtistSort,
	AlbumSort,
	SongSort,
};
use std::collections::HashMap;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use crate::constants::{
	FESTIVAL,
	FESTIVAL_HEADER,
	COLLECTION_VERSION,
};
use rand::{
	Rng,
	SeedableRng,
	prelude::SliceRandom,
};
use std::sync::Mutex;
use crate::macros::{
	lock,
	mass_panic,
};

//---------------------------------------------------------------------------------------------------- Constant/lazy_static
lazy_static::lazy_static! {
	static ref RNG: Mutex<rand::rngs::SmallRng> = Mutex::new(rand::rngs::SmallRng::from_entropy());
}

//---------------------------------------------------------------------------------------------------- The Collection™
bincode_file!(Collection, Dir::Data, FESTIVAL, "", "collection", FESTIVAL_HEADER, COLLECTION_VERSION);
#[derive(Debug,Serialize,Deserialize)]
/// The main music `Collection`
///
/// This is the `struct` that holds all the (meta)data about the user's music.
///
/// This holds:
/// - The "3 Vecs", holding _all_ [`Artist`]'s, [`Album`]'s, and [`Song`]'s.
/// - Pre-computed, sorted keys
/// - Metadata about the [`Collection`] itself
///
/// ### Sort
/// The "3 Vecs" are (basically) in random order due to how `Collection` is created.
///
/// Iterating directly on them is not very useful, so use the pre-calculated sorted keys.
///
/// The sorted key fields all start with `sort_`.
///
/// `lexi` is shorthand for `lexicographically`, as defined [here.](https://doc.rust-lang.org/stable/std/primitive.str.html#impl-Ord-for-str)
///
/// ### Index
/// To properly index the [`Collection`], for example, an [`Album`], you CAN use the `[]` operators, however,
/// they must be type-safe. Meaning: it CANNOT be a random [`usize`], it must be the proper type of [`Key`].
///
/// Example:
/// ```rust,ignore
/// let my_usize = 0;
/// let key = AlbumKey::from(my_usize);
///
/// // NOT type-safe, compile error!.
/// collection.albums[my_usize];
///
/// // Type-safe, compiles.
/// collection.albums[key];
/// ```
pub struct Collection {
	// The "3 Vecs".
	/// All the [`Artist`]'s in mostly random order.
	pub artists: Artists,
	/// All the [`Album`]'s in mostly random order.
	pub albums: Albums,
	/// All the [`Song`]'s in mostly random order.
	pub songs: Songs,

	// Sorted `Artist` keys.
	/// [`Artist`] `lexi`.
	pub sort_artist_lexi: Vec<ArtistKey>,
	/// [`Artist`] with most [`Album`]'s to least.
	pub sort_artist_album_count: Vec<ArtistKey>,
	/// [`Artist`] with most [`Song`]'s to least.
	pub sort_artist_song_count: Vec<ArtistKey>,

	// Sorted `Album` keys.
	/// [`Artist`] `lexi`, [`Album`]'s oldest release to latest.
	pub sort_album_release_artist_lexi: Vec<AlbumKey>,
	/// [`Artist`] `lexi`, [`Album`]'s `lexi`.
	pub sort_album_lexi_artist_lexi: Vec<AlbumKey>,
	/// [`Album`] lexi.
	pub sort_album_lexi: Vec<AlbumKey>,
	/// [`Album`] oldest to latest.
	pub sort_album_release: Vec<AlbumKey>,
	/// [`Album`] shortest to longest.
	pub sort_album_runtime: Vec<AlbumKey>,

	// Sorted `Song` keys.
	/// [`Artist`] lexi, [`Album`] release, [`Song`] track_number
	pub sort_song_album_release_artist_lexi: Vec<SongKey>,
	/// [`Artist`] lexi, [`Album`] lexi, [`Song`] track_number.
	pub sort_song_album_lexi_artist_lexi: Vec<SongKey>,
	/// [`Song`] lexi.
	pub sort_song_lexi: Vec<SongKey>,
	/// [`Song`] oldest to latest.
	pub sort_song_release: Vec<SongKey>,
	/// [`Song`] shortest to longest.
	pub sort_song_runtime: Vec<SongKey>,

	// Metadata about the `Collection` itself.
	/// Is this [`Collection`] empty?
	///
	/// Meaning, are there absolutely no [`Artist`]'s, [`Album`]'s and [`Song`]'s?
	pub empty: bool,
	/// UNIX timestamp of the [`Collection`]'s creation date.
	pub timestamp: u64,
	/// How many [`Artist`]'s in this [`Collection`]?
	pub count_artist: usize,
	/// How many [`Album`]'s in this [`Collection`]?
	pub count_album: usize,
	/// How many [`Song`]'s in this [`Collection`]?
	pub count_song: usize,
}

impl Collection {
	//-------------------------------------------------- New.
	/// Creates an empty [`Collection`].
	///
	/// All [`Vec`]'s are empty.
	///
	/// The `timestamp` and `count_*` fields are set to `0`.
	///
	/// `empty` is set to `true`.
	pub const fn new() -> Self {
		Self {
			artists: Artists::new(),
			albums: Albums::new(),
			songs: Songs::new(),

			sort_artist_lexi: vec![],
			sort_artist_album_count: vec![],
			sort_artist_song_count: vec![],

			sort_album_release_artist_lexi: vec![],
			sort_album_lexi_artist_lexi: vec![],
			sort_album_lexi: vec![],
			sort_album_release: vec![],
			sort_album_runtime: vec![],

			sort_song_album_release_artist_lexi: vec![],
			sort_song_album_lexi_artist_lexi: vec![],
			sort_song_lexi: vec![],
			sort_song_release: vec![],
			sort_song_runtime: vec![],

			empty: true,
			timestamp: 0,
			count_artist: 0,
			count_album: 0,
			count_song: 0,
		}
	}

	//-------------------------------------------------- Metadata functions.
	#[inline]
	// Set the proper metadata for this `Collection`.
	pub(crate) fn set_metadata(mut self) -> Self {
		// Get `Vec` lengths.
		let artists = self.artists.len();
		let albums  = self.albums.len();
		let songs   = self.songs.len();

		// Set `empty`.
		if artists == 0 && albums == 0 && songs == 0 {
			self.empty = true;
		} else {
			self.empty = false;
		}

		// Set `count_*`.
		self.count_artist = artists;
		self.count_album  = albums;
		self.count_song   = songs;

		// Set `timestamp`.
		self.timestamp = Self::unix_now();

		self
	}

	#[inline]
	// Get the current UNIX time.
	pub(crate) fn unix_now() -> u64 {
		let now = std::time::SystemTime::now();
		match now.duration_since(std::time::SystemTime::UNIX_EPOCH) {
			Ok(ts) => ts.as_secs(),
			Err(e) => {
				warn!("Failed to get timestamp, returning UNIX_EPOCH (0)");
				0
			}
		}
	}

	//-------------------------------------------------- Indexing.
	/// Directly index the [`Collection`] with a [`Key`].
	///
	/// # Panics:
	/// The [`ArtistKey`], [`AlbumKey`] and [`SongKey`] within
	/// the [`Key`] must be valid indicies into the [`Collection`].
	#[inline(always)]
	pub fn index(&self, key: &Key) -> (&Artist, &Album, &Song) {
		let (artist, album, song) = key.inner_usize();
		(&self.artists.0[artist], &self.albums.0[album], &self.songs.0[song])
	}

	/// [`slice::get`] the [`Collection`] with a [`Key`].
	///
	/// # Errors:
	/// The [`ArtistKey`], [`AlbumKey`] and [`SongKey`] within
	/// the [`Key`] must be valid indicies into the [`Collection`].
	pub fn get(&self, key: &Key) -> Option<(&Artist, &Album, &Song)> {
		let (artist, album, song) = key.inner_usize();

		let artists = match self.artists.0.get(artist) {
			Some(a) => a,
			None    => return None,
		};

		let album = match self.albums.0.get(album) {
			Some(a) => a,
			None    => return None,
		};

		let song = match self.songs.0.get(song) {
			Some(a) => a,
			None    => return None,
		};

		Some((artists, album, song))
	}

	//-------------------------------------------------- Key traversal (index).
	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`AlbumKey`].
	///
	/// # Panics:
	/// The [`AlbumKey`] must be a valid index.
	pub fn artist_from_album(&self, key: AlbumKey) -> &Artist {
		&self.artists[self.albums[key].artist]
	}

	#[inline(always)]
	/// Obtain an [`Album`], but from a [`SongKey`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn album_from_song(&self, key: SongKey) -> &Album {
		&self.albums[self.songs[key].album]
	}

	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`SongKey`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn artist_from_song(&self, key: SongKey) -> &Artist {
		&self.artist_from_album(self.songs[key].album)
	}

	//-------------------------------------------------- Key traversal (`.get()`).
	#[inline]
	/// Obtain an [`Artist`], but from a [`AlbumKey`].
	///
	/// # Errors:
	/// The [`AlbumKey`] must be a valid index.
	pub fn get_artist_from_album(&self, key: AlbumKey) -> Option<&Artist> {
		let artist = match self.albums.get(key) {
			Some(a) => a.artist,
			None    => return None,
		};

		self.artists.get(artist)
	}

	#[inline]
	/// Obtain an [`Album`], but from a [`SongKey`].
	///
	/// # Errors:
	/// The [`SongKey`] must be a valid index.
	pub fn get_album_from_song(&self, key: SongKey) -> Option<&Album> {
		let album = match self.songs.get(key) {
			Some(a) => a.album,
			None    => return None,
		};

		self.albums.get(album)
	}

	#[inline]
	/// Obtain an [`Artist`], but from a [`SongKey`].
	///
	/// # Errors:
	/// The [`SongKey`] must be a valid index.
	pub fn get_artist_from_song(&self, key: SongKey) -> Option<&Artist> {
		let album = match self.songs.get(key) {
			Some(a) => a.album,
			None    => return None,
		};

		self.get_artist_from_album(album)
	}

	//-------------------------------------------------- Sorting
	/// Access a particular `sort_artist_` field in the [`Collection`] via a [`ArtistSort`].
	pub fn artist_sort(&self, sort: &ArtistSort) -> &Vec<ArtistKey> {
		use ArtistSort::*;
		match sort {
			Lexi       => &self.sort_artist_lexi,
			AlbumCount => &self.sort_artist_album_count,
			SongCount  => &self.sort_artist_song_count,
		}
	}

	/// Access a particular `sort_album_` field in the [`Collection`] via a [`AlbumSort`].
	pub fn album_sort(&self, sort: &AlbumSort) -> &Vec<AlbumKey> {
		use AlbumSort::*;
		match sort {
			ReleaseArtistLexi => &self.sort_album_release_artist_lexi,
			LexiArtistLexi    => &self.sort_album_lexi_artist_lexi,
			Lexi              => &self.sort_album_lexi,
			Release           => &self.sort_album_release,
			Runtime           => &self.sort_album_runtime,
		}
	}

	/// Access a particular `sort_song_` field in the [`Collection`] via a [`SongSort`].
	pub fn song_sort(&self, sort: &SongSort) -> &Vec<SongKey> {
		use SongSort::*;
		match sort {
			AlbumReleaseArtistLexi => &self.sort_song_album_release_artist_lexi,
			AlbumLexiArtistLexi    => &self.sort_song_album_lexi_artist_lexi,
			Lexi                   => &self.sort_song_lexi,
			Release                => &self.sort_song_release,
			Runtime                => &self.sort_song_runtime,
		}
	}

	//-------------------------------------------------- Random
	/// Get a random _valid_ [`ArtistKey`].
	///
	/// If you provide a `Some<ArtistKey>`, this function
	/// will _not_ return that same [`ArtistKey`].
	///
	/// # Notes
	/// - If there is only 1 [`ArtistKey`], `ArtistKey(0)` will always be returned.
	/// - If there are _no_ [`Artist`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_artist(&self, key: Option<ArtistKey>) -> Option<ArtistKey> {
		match self.count_artist {
			0 => return None,
			1 => return Some(ArtistKey::zero()),
			_ => (),
		}

		if let Some(key) = key {
			loop {
				let rand_usize: usize = lock!(RNG).gen_range(0..self.count_artist);
				if rand_usize != key {
					return Some(ArtistKey::from(rand_usize))
				}
			}
		}

		let rand_usize: usize = lock!(RNG).gen_range(0..self.count_artist);
		Some(ArtistKey::from(rand_usize))
	}

	/// Get a random _valid_ [`AlbumKey`].
	///
	/// If you provide a `Some<AlbumKey>`, this function
	/// will _not_ return that same [`AlbumKey`].
	///
	/// # Notes
	/// - If there is only 1 [`AlbumKey`], `AlbumKey(0)` will always be returned.
	/// - If there are _no_ [`Album`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_album(&self, key: Option<AlbumKey>) -> Option<AlbumKey> {
		match self.count_album {
			0 => return None,
			1 => return Some(AlbumKey::zero()),
			_ => (),
		}

		if let Some(key) = key {
			loop {
				let rand_usize: usize = lock!(RNG).gen_range(0..self.count_album);
				if rand_usize != key {
					return Some(AlbumKey::from(rand_usize))
				}
			}
		}

		let rand_usize: usize = lock!(RNG).gen_range(0..self.count_album);
		Some(AlbumKey::from(rand_usize))
	}

	/// Get a random _valid_ [`SongKey`].
	///
	/// If you provide a `Some<SongKey>`, this function
	/// will _not_ return that same [`SongKey`].
	///
	/// # Notes
	/// - If there is only 1 [`SongKey`], `SongKey(0)` will always be returned.
	/// - If there are _no_ [`Song`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_song(&self, key: Option<SongKey>) -> Option<SongKey> {
		match self.count_song {
			0 => return None,
			1 => return Some(SongKey::zero()),
			_ => (),
		}

		if let Some(key) = key {
			loop {
				let rand_usize: usize = lock!(RNG).gen_range(0..self.count_song);
				if rand_usize != key {
					return Some(SongKey::from(rand_usize))
				}
			}
		}

		let rand_usize: usize = lock!(RNG).gen_range(0..self.count_song);
		Some(SongKey::from(rand_usize))
	}

	/// Get a [`Vec`] of random _valid_ [`ArtistKey`]'s.
	///
	/// The length of the returned [`Vec`] will match [`Collection::count_artist`]'s length.
	///
	/// # Notes
	/// - If there are _no_ [`Artist`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_artists(&self) -> Option<Vec<ArtistKey>> {
		match self.count_artist {
			0 => None,
			1 => Some(vec![ArtistKey::zero()]),
			_ => {
				let mut vec: Vec<ArtistKey> = (0..self.count_artist)
					.map(|i: usize| ArtistKey::from(i))
					.collect();
				vec.shuffle(&mut *lock!(RNG));
				Some(vec)
			}
		}
	}

	/// Get a [`Vec`] of random _valid_ [`AlbumKey`]'s.
	///
	/// The length of the returned [`Vec`] will match [`Collection::count_album`]'s length.
	///
	/// # Notes
	/// - If there are _no_ [`Album`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_albums(&self) -> Option<Vec<AlbumKey>> {
		match self.count_album {
			0 => None,
			1 => Some(vec![AlbumKey::zero()]),
			_ => {
				let mut vec: Vec<AlbumKey> = (0..self.count_album)
					.map(|i: usize| AlbumKey::from(i))
					.collect();
				vec.shuffle(&mut *lock!(RNG));
				Some(vec)
			}
		}
	}

	/// Get a [`Vec`] of random _valid_ [`SongKey`]'s.
	///
	/// The length of the returned [`Vec`] will match [`Collection::count_song`]'s length.
	///
	/// # Notes
	/// - If there are _no_ [`Song`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_songs(&self) -> Option<Vec<SongKey>> {
		match self.count_song {
			0 => None,
			1 => Some(vec![SongKey::zero()]),
			_ => {
				let mut vec: Vec<SongKey> = (0..self.count_song)
					.map(|i: usize| SongKey::from(i))
					.collect();
				vec.shuffle(&mut *lock!(RNG));
				Some(vec)
			}
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
