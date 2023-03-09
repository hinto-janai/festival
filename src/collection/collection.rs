//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use super::{
	album::Album,
	artist::Artist,
	song::Song,
	key::{CollectionKey,ArtistKey,AlbumKey,SongKey},
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
//#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
bincode_file!(Collection, Dir::Data, FESTIVAL, "", "collection", FESTIVAL_HEADER, COLLECTION_VERSION);
#[derive(Debug,Serialize,Deserialize)]
pub struct Collection {
	// The actual (meta)data.
	// These are (basically) in random order due to the `Collection` creation process.
	// Iterating directly on these makes no sense, use the sorted keys below.
	pub artists: Vec<Artist>,
	pub albums: Vec<Album>,
	pub songs: Vec<Song>,

	// `lexi` == `lexicographically`
	// Sorted `Artist` keys.
	pub sort_artist_lexi: Vec<ArtistKey>,        // `Artist` in lexi
	pub sort_artist_album_count: Vec<ArtistKey>, // `Artist` with most `Album`'s to least.
	pub sort_artist_song_count: Vec<ArtistKey>,  // `Artist` with most `Song`'s to least.

	// Sorted `Album` keys.
	pub sort_album_release_artist_lexi: Vec<AlbumKey>, // `Artist` lexi, `Album` in release order.
	pub sort_album_lexi_artist_lexi: Vec<AlbumKey>,    // `Artist` lexi, `Album` lexi.
	pub sort_album_lexi: Vec<AlbumKey>,                // `Album` lexi.
	pub sort_album_release: Vec<AlbumKey>,             // `Album` oldest to latest.
	pub sort_album_runtime: Vec<AlbumKey>,             // `Album` shortest to longest.

	// Sorted `Song` keys.
	pub sort_song_artist_lexi_album_release: Vec<SongKey>, // `Artist` lexi, `Album` release, `Song` track_number
	pub sort_song_artist_lexi_album_lexi: Vec<SongKey>,    // `Artist` lexi, `Album` lexi, `Song` track_number.
	pub sort_song_lexi: Vec<SongKey>,                      // `Song` lexi.
	pub sort_song_release: Vec<SongKey>,                   // `Song` oldest to latest.
	pub sort_song_runtime: Vec<SongKey>,                   // `Song` shortest to longest.

	// Metadata about the `Collection` itself.
	pub empty: bool,         // Is this `Collection` empty?
	pub timestamp: u64,      // Creation date as UNIX time.
	pub count_artist: usize, // How many artists?
	pub count_album: usize,  // How many albums?
	pub count_song: usize,   // How many songs?
}

impl Collection {
	#[inline(always)]
	// Creates an empty struct.
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

	// INVARIANT:
	// During the lifetime of a `Collection` (even during initial creation),
	// `Key`'s are always created with a valid index into these global `Vec`'s.
	//
	// Thus directly indexing into them like this (in theory) should never fail.
	#[inline(always)]
	pub fn index(&self, key: &CollectionKey) -> (&Artist, &Album, &Song) {
		let (artist, album, song) = key.to_tuple();
		(&self.artists[artist], &self.albums[album], &self.songs[song])
	}

	#[inline(always)]
	pub fn index_artist(&self, key: &ArtistKey) -> &Artist {
		&self.artists[key.inner()]
	}

	#[inline(always)]
	pub fn index_album(&self, key: &AlbumKey) -> &Album {
		&self.albums[key.inner()]
	}

	#[inline(always)]
	pub fn index_song(&self, key: &SongKey) -> &Song {
		&self.songs[key.inner()]
	}

	// Key conversions.
	#[inline(always)]
	pub fn artist_from_album(&self, key: &AlbumKey) -> ArtistKey {
		self.albums[key.inner()].artist
	}
	#[inline(always)]
	pub fn album_from_song(&self, key: &SongKey) -> AlbumKey {
		self.songs[key.inner()].album
	}
	#[inline(always)]
	pub fn artist_from_song(&self, key: &SongKey) -> ArtistKey {
		self.artist_from_album(&self.songs[key.inner()].album)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
