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
#[derive(Serialize,Deserialize)]
pub struct Collection {
	// The actual (meta)data.
	pub artists: Vec<Artist>,
	pub albums: Vec<Album>,
	pub songs: Vec<Song>,

	// Pre-computed and sorted keys.
	pub sort_artist_release: Vec<CollectionKey>,
	pub sort_artist_title: Vec<CollectionKey>,
	pub sort_release: Vec<CollectionKey>,
	pub sort_title: Vec<CollectionKey>,

	// Metadata about the `Collection` itself.
	pub timestamp: u64,      // Creation date as UNIX time.
	pub count_artist: usize, // How many artists?
	pub count_album: usize,  // How many albums?
	pub count_song: usize,   // How many songs?
}

impl Collection {
	#[inline(always)]
	// Creates a "dummy" struct, aka, empty.
	pub fn dummy() -> Self {
		Self {
			artists: vec![],
			albums: vec![],
			songs: vec![],

			sort_artist_release: vec![],
			sort_artist_title: vec![],
			sort_release: vec![],
			sort_title: vec![],

			timestamp: 0,
			count_artist: 0,
			count_album: 0,
			count_song: 0,
		}
	}

	// Get current timestamp as UNIX time.
	fn timestamp_now() -> u64 {
		let now = std::time::SystemTime::now();
		match now.duration_since(std::time::SystemTime::UNIX_EPOCH) {
			Ok(ts) => ts.as_secs(),
			Err(e) => {
				warn!("Failed to get timestamp, returning UNIX_EPOCH (0)");
				0
			}
		}
	}

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
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
