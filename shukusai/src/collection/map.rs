//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use std::collections::HashMap;
use crate::collection::{
	Artist,
	Album,
	Song,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use std::sync::Arc;

//---------------------------------------------------------------------------------------------------- Map
#[derive(Clone,Debug,Default,PartialEq,Encode,Decode)]
/// A [`HashMap`] that knows all [`Artist`]'s, [`Album`]'s and [`Song`]'s.
///
/// No public functions are implemented on this type directly,
/// use [`Collection`]'s functions instead.
pub struct Map(pub(crate) HashMap<Arc<str>, (ArtistKey, AlbumMap)>);

impl Map {
	#[inline(always)]
	pub(crate) fn new() -> Self {
		Self::default()
	}

	#[inline(always)] // This only gets called once.
	// Iterates over the the "3 Slices"
	// and creates a matching `Map`.
	pub(crate) fn from_3_vecs(
		artists: &[Artist],
		albums: &[Album],
		songs: &[Song],
	) -> Self {
		let mut map = Self::default();

		// For each `Artist`...
		for (i, artist) in artists.iter().enumerate() {
			let mut album_map = AlbumMap::default();

			// For each `Album` within `Artist`...
			for album in artist.albums.iter() {
				let mut song_map  = SongMap::default();

				// For each `Song` within the `Album`...
				for song in albums[album.inner()].songs.iter() {
					song_map.0.insert(songs[song.inner()].title.clone(), *song);
				}

				// Insert the `SongMap` into the `AlbumMap`.
				album_map.0.insert(albums[album.inner()].title.clone(), (*album, song_map));
			}

			// Insert the `AlbumMap` into the `(Artist)Map`.
			map.0.insert(artist.name.clone(), (ArtistKey::from(i), album_map));
		}

		map
	}
}

//---------------------------------------------------------------------------------------------------- AlbumMap
#[derive(Clone,Debug,Default,PartialEq,Encode,Decode)]
pub(crate) struct AlbumMap(pub(crate) HashMap<Arc<str>, (AlbumKey, SongMap)>);

//---------------------------------------------------------------------------------------------------- SongMap
#[derive(Clone,Debug,Default,PartialEq,Encode,Decode)]
pub(crate) struct SongMap(pub(crate) HashMap<Arc<str>, SongKey>);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
