//---------------------------------------------------------------------------------------------------- Use
use crate::collection::{Album, AlbumKey, Artist, ArtistKey, Collection, Song, SongKey};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

//---------------------------------------------------------------------------------------------------- MapEntry
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
/// An absolute "Key" for the [`Map`].
pub struct MapKey {
    /// Artist name
    pub artist: Arc<str>,
    /// Album title
    pub album: Arc<str>,
    /// Song title
    pub song: Arc<str>,
}

impl MapKey {
    /// Create `self` by walking a `Song`.
    pub fn from_song(song: &Song, collection: &Arc<Collection>) -> Self {
        let album = &collection.albums[song.album];
        let artist = &collection.artists[album.artist];
        Self {
            artist: Arc::clone(&artist.name),
            album: Arc::clone(&album.title),
            song: Arc::clone(&song.title),
        }
    }

    /// INVARIANT: assumes key is valid
    ///
    /// Create `self` by walking a `SongKey`.
    pub fn from_song_key(key: SongKey, collection: &Arc<Collection>) -> Self {
        let (artist, album, song) = collection.walk(key);
        Self {
            artist: Arc::clone(&artist.name),
            album: Arc::clone(&album.title),
            song: Arc::clone(&song.title),
        }
    }

    /// Attempts to look in the `Collection` for 100% matching `Song`.
    pub fn to_key(&self, collection: &Arc<Collection>) -> Option<SongKey> {
        collection
            .song(&*self.artist, &*self.album, &*self.song)
            .map(|(s, _)| s.key)
    }
}

//---------------------------------------------------------------------------------------------------- Map
#[derive(Clone, Debug, Default, PartialEq, Encode, Decode)]
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
    pub(crate) fn from_3_vecs(artists: &[Artist], albums: &[Album], songs: &[Song]) -> Self {
        let mut map = Self::default();

        // For each `Artist`...
        for (i, artist) in artists.iter().enumerate() {
            let mut album_map = AlbumMap::default();

            // For each `Album` within `Artist`...
            for album in artist.albums.iter() {
                let mut song_map = SongMap::default();

                // For each `Song` within the `Album`...
                for song in albums[album.inner()].songs.iter() {
                    song_map.0.insert(songs[song.inner()].title.clone(), *song);
                }

                // Insert the `SongMap` into the `AlbumMap`.
                album_map
                    .0
                    .insert(albums[album.inner()].title.clone(), (*album, song_map));
            }

            // Insert the `AlbumMap` into the `(Artist)Map`.
            map.0
                .insert(artist.name.clone(), (ArtistKey::from(i), album_map));
        }

        map
    }
}

//---------------------------------------------------------------------------------------------------- AlbumMap
#[derive(Clone, Debug, Default, PartialEq, Encode, Decode)]
pub(crate) struct AlbumMap(pub(crate) HashMap<Arc<str>, (AlbumKey, SongMap)>);

//---------------------------------------------------------------------------------------------------- SongMap
#[derive(Clone, Debug, Default, PartialEq, Encode, Decode)]
pub(crate) struct SongMap(pub(crate) HashMap<Arc<str>, SongKey>);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
