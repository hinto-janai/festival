//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
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
	artists: Vec<Artist>,
	albums: Vec<Album>,
	songs: Vec<Song>,
}

impl Collection {
	#[inline(always)]
	pub fn new() -> Self {
		Self {
			artists: vec![],
			albums: vec![],
			songs: vec![],
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
//mod test {
//  #[test]
//  fn _() {
//  }
//}
