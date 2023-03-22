//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use super::{
	album::Album,
	artist::Artist,
	song::Song,
	key::{Key,ArtistKey,AlbumKey,SongKey},
};
use std::collections::HashMap;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use crate::constants::{
	FESTIVAL,
	FESTIVAL_HEADER,
	COLLECTION_VERSION,
};

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
/// The "3 Vecs" are (basically) in random order due to how `Collection` is created.
/// Iterating directly on these makes no sense, so use the pre-calculated sorted keys.
///
/// The sorted key fields all start with `sort_`.
///
/// `lexi` is shorthand for `lexicographically`, as defined [here.](https://doc.rust-lang.org/stable/std/primitive.str.html#impl-Ord-for-str)
pub struct Collection {
	/// All the [`Artist`]'s in mostly random order.
	pub artists: Vec<Artist>,
	/// All the [`Album`]'s in mostly random order.
	pub albums: Vec<Album>,
	/// All the [`Song`]'s in mostly random order.
	pub songs: Vec<Song>,

	// Sorted `Artist` keys.
	/// `Artist` in `lexi`.
	pub sort_artist_lexi: Vec<ArtistKey>,
	/// `Artist` with most `Album`'s to least.
	pub sort_artist_album_count: Vec<ArtistKey>,
	/// `Artist` with most `Song`'s to least.
	pub sort_artist_song_count: Vec<ArtistKey>,

	// Sorted `Album` keys.
	/// [`Artist`] with most [`Song`]'s to least.
	pub sort_album_release_artist_lexi: Vec<AlbumKey>,
	/// [`Artist`] lexi, [`Album`] lexi.
	pub sort_album_lexi_artist_lexi: Vec<AlbumKey>,
	/// [`Album`] lexi.
	pub sort_album_lexi: Vec<AlbumKey>,
	/// [`Album`] oldest to latest.
	pub sort_album_release: Vec<AlbumKey>,
	/// [`Album`] shortest to longest.
	pub sort_album_runtime: Vec<AlbumKey>,

	// Sorted `Song` keys.
	/// [`Artist`] lexi, [`Album`] release, [`Song`] track_number
	pub sort_song_artist_lexi_album_release: Vec<SongKey>,
	/// [`Artist`] lexi, [`Album`] lexi, [`Song`] track_number.
	pub sort_song_artist_lexi_album_lexi: Vec<SongKey>,
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
	#[inline(always)]
	/// Creates an empty [`Collection`].
	///
	/// The `timestamp` is set to `0`.
	pub const fn new() -> Self {
		Self {
			artists: vec![],
			albums: vec![],
			songs: vec![],

			sort_artist_lexi: vec![],
			sort_artist_album_count: vec![],
			sort_artist_song_count: vec![],

			sort_album_release_artist_lexi: vec![],
			sort_album_lexi_artist_lexi: vec![],
			sort_album_lexi: vec![],
			sort_album_release: vec![],
			sort_album_runtime: vec![],

			sort_song_artist_lexi_album_release: vec![],
			sort_song_artist_lexi_album_lexi: vec![],
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

	// Get current timestamp as UNIX time.
	pub(crate) fn timestamp_now() -> u64 {
		let now = std::time::SystemTime::now();
		match now.duration_since(std::time::SystemTime::UNIX_EPOCH) {
			Ok(ts) => ts.as_secs(),
			Err(e) => {
				warn!("Failed to get timestamp, returning UNIX_EPOCH (0)");
				0
			}
		}
	}

	/// Directly index the [`Collection`] with a [`Key`].
	///
	/// # Panics:
	/// The [`ArtistKey`], [`AlbumKey`] and [`SongKey`] within
	/// the [`Key`] must be valid indicies into the [`Collection`].
	#[inline(always)]
	pub fn index(&self, key: &Key) -> (&Artist, &Album, &Song) {
		let (artist, album, song) = key.inner_usize();
		(&self.artists[artist], &self.albums[album], &self.songs[song])
	}

	#[inline(always)]
	/// Directly index the [`Collection`] for an [`Artist`].
	///
	/// # Panics:
	/// The [`ArtistKey`] must be a valid index.
	pub fn artist(&self, key: ArtistKey) -> &Artist {
		&self.artists[key.inner()]
	}

	#[inline(always)]
	/// Directly index the [`Collection`] for an [`Album`].
	///
	/// # Panics:
	/// The [`AlbumKey`] must be a valid index.
	pub fn album(&self, key: AlbumKey) -> &Album {
		&self.albums[key.inner()]
	}

	#[inline(always)]
	/// Directly index the [`Collection`] for an [`Song`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn song(&self, key: SongKey) -> &Song {
		&self.songs[key.inner()]
	}

	// Key traversal.
	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`AlbumKey`].
	///
	/// # Panics:
	/// The [`AlbumKey`] must be a valid index.
	pub fn artist_from_album(&self, key: AlbumKey) -> &Artist {
		&self.artist(self.albums[key.inner()].artist)
	}
	#[inline(always)]
	/// Obtain an [`Album`], but from a [`SongKey`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn album_from_song(&self, key: SongKey) -> &Album {
		&self.album(self.songs[key.inner()].album)
	}
	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`SongKey`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn artist_from_song(&self, key: SongKey) -> &Artist {
		&self.artist_from_album(self.songs[key.inner()].album)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
