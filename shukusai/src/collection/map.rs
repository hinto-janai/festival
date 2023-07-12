//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use std::collections::HashMap;
use crate::collection::{
	Collection,
	Artist,
	Album,
	Song,
	ArtistKey,
	AlbumKey,
	SongKey,
	ArtistPtr,
	AlbumPtr,
	SongPtr,
};
use std::sync::Arc;

//---------------------------------------------------------------------------------------------------- Map
#[derive(Clone,Debug,Default,PartialEq,Encode,Decode)]
#[repr(transparent)]
/// A [`HashMap`] that knows all [`Artist`]'s, [`Album`]'s and [`Song`]'s.
///
/// No public functions are implemented on this type directly,
/// use [`Collection`]'s functions instead.
pub struct Map(pub(crate) HashMap<Arc<str>, ((ArtistKey, ArtistPtr), AlbumMap)>);

impl Map {
	#[inline(always)]
	pub(crate) fn new() -> Self {
		Self::default()
	}

	#[inline(always)] // This only gets called once.
	// Iterates over the the "3 Slices"
	// and creates a matching `Map`.
	pub(crate) fn from_collection(c: &Collection) -> Self {
		let mut map = Map::default();

		// For each `Artist`...
		for (i, artist) in c.artists.iter().enumerate() {
			let mut album_map = AlbumMap::default();

			// For each `Album` within `Artist`...
			for album in artist.albums.iter() {
				let mut song_map  = SongMap::default();

				// For each `Song` within the `Album`...
				for song in c.albums[album.0].songs.iter() {
					song_map.0.insert(c.songs[song.0].title.clone(), *song);
				}

				// Insert the `SongMap` into the `AlbumMap`.
				album_map.0.insert(c.albums[album.0].title.clone(), (*album, song_map));
			}

			// Insert the `AlbumMap` into the `(Artist)Map`.
			map.0.insert(artist.name.clone(), ((ArtistKey::from(i), ArtistPtr::from(artist)), album_map));
		}

		map
	}
}

//---------------------------------------------------------------------------------------------------- AlbumMap
#[derive(Clone,Debug,Default,PartialEq,Encode,Decode)]
#[repr(transparent)]
pub(crate) struct AlbumMap(pub(crate) HashMap<Arc<str>, ((AlbumKey, AlbumPtr), SongMap)>);

//---------------------------------------------------------------------------------------------------- SongMap
#[derive(Clone,Debug,Default,PartialEq,Encode,Decode)]
#[repr(transparent)]
pub(crate) struct SongMap(pub(crate) HashMap<Arc<str>, (SongKey, SongPtr)>);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
