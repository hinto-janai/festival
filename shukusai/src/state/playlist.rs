//---------------------------------------------------------------------------------------------------- Use
use crate::{
    audio::Append,
    collection::{AlbumKey, ArtistKey, Collection, SongKey},
    constants::{FESTIVAL, FRONTEND_SUB_DIR, HEADER, PLAYLIST_VERSION, STATE_SUB_DIR},
};
use benri::{lockr, lockw};
use bincode::{Decode, Encode};
use const_format::formatcp;
use disk::Bincode2;
use log::{debug, error, info, trace, warn};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};

//---------------------------------------------------------------------------------------------------- Lazy
/// This is the single, global copy of `Playlists` that `Kernel` uses.
///
/// To obtain a read-only lock, use `PLAYLISTS.read()`.
pub static PLAYLISTS: PlaylistsLock = PlaylistsLock(RwLock::new(Playlists::new()));

//---------------------------------------------------------------------------------------------------- PlaylistsLock
/// There is only a single, global copy of `Playlists` that `Kernel` uses: [`PLAYLISTS`].
///
/// To obtain a read-only lock, use `PLAYLISTS.read()`.
pub struct PlaylistsLock(RwLock<Playlists>);

impl PlaylistsLock {
    #[inline(always)]
    /// Obtain a read-only lock to the global [`Playlists`].
    pub fn read(&'static self) -> RwLockReadGuard<'static, Playlists> {
        lockr!(self.0)
    }

    #[inline(always)]
    /// Call the non-blocking `.try_read()` on the global [`Playlists`].
    pub fn try_read(
        &'static self,
    ) -> Result<
        RwLockReadGuard<'static, Playlists>,
        TryLockError<RwLockReadGuard<'static, Playlists>>,
    > {
        self.0.try_read()
    }

    #[inline(always)]
    /// Obtain a write lock to the global [`Playlists`].
    pub fn write(&'static self) -> RwLockWriteGuard<'static, Playlists> {
        lockw!(self.0)
    }

    #[inline(always)]
    /// Call the non-blocking `.try_write()` on the global [`Playlists`].
    pub fn try_write(
        &'static self,
    ) -> Result<
        RwLockWriteGuard<'static, Playlists>,
        TryLockError<RwLockWriteGuard<'static, Playlists>>,
    > {
        self.0.try_write()
    }
}

//---------------------------------------------------------------------------------------------------- __NAME__
disk::bincode2!(
    Playlists,
    disk::Dir::Data,
    FESTIVAL,
    formatcp!("{FRONTEND_SUB_DIR}/{STATE_SUB_DIR}"),
    "playlists",
    HEADER,
    PLAYLIST_VERSION
);
#[derive(
    Clone,
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
#[serde(rename_all = "snake_case")]
#[serde(transparent)]
#[repr(transparent)]
/// Playlist implementation.
///
/// Contains all user playlists, ordering via `BTreeMap`.
///
/// Each node in the `BTreeMap` is a `(String, VecDeque)` where
/// the `String` is the name of the playlist, and the `VecDeque`
/// contains [`Entry`]'s.
pub struct Playlists(pub PlaylistsInner);

/// The internal type of [`Playlists`].
///
/// [`Playlists`] is just a wrapper so methods/traits can be implemented on it.
pub type PlaylistsInner = BTreeMap<Arc<str>, VecDeque<Entry>>;

#[derive(
    Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode,
)]
#[serde(rename_all = "snake_case")]
/// `Option`-like enum for playlist entries.
///
/// Either song exists in the current `Collection` (`Entry::Key`)
/// or it is missing (`Entry::Invalid`).
pub enum Entry {
    /// This is a valid song in the current `Collection`
    Valid {
        /// Artist key
        key_artist: ArtistKey,
        /// Album key
        key_album: AlbumKey,
        /// Song key
        key_song: SongKey,
        /// Artist name
        artist: Arc<str>,
        /// Album title
        album: Arc<str>,
        /// Song title
        song: Arc<str>,
    },

    /// This song is missing, this was the
    /// `artist.name`, `album.title`, `song.title`.
    Invalid {
        /// Artist name
        artist: Arc<str>,
        /// Album title
        album: Arc<str>,
        /// Song title
        song: Arc<str>,
    },
}

impl Entry {
    /// INVARIANT: assumes key is valid.
    ///
    /// Returns valid entries of all `Song`'s by an `Artist`.
    pub fn valid_from_artist(key: ArtistKey, collection: &Arc<Collection>) -> Vec<Self> {
        let artist = &collection.artists[key];
        artist
            .songs
            .iter()
            .map(|s| {
                let song = &collection.songs[s];
                let album = &collection.albums[song.album];
                Self::Valid {
                    key_artist: key,
                    key_album: album.key,
                    key_song: song.key,
                    artist: Arc::clone(&artist.name),
                    album: Arc::clone(&album.title),
                    song: Arc::clone(&song.title),
                }
            })
            .collect()
    }

    /// INVARIANT: assumes key is valid.
    ///
    /// Returns valid entries of all `Song`'s in an `Album`.
    pub fn valid_from_album(key: AlbumKey, collection: &Arc<Collection>) -> Vec<Self> {
        let album = &collection.albums[key];
        let artist = &collection.artists[album.artist];
        album
            .songs
            .iter()
            .map(|s| {
                let song = &collection.songs[s];
                Self::Valid {
                    key_artist: artist.key,
                    key_album: album.key,
                    key_song: song.key,
                    artist: Arc::clone(&artist.name),
                    album: Arc::clone(&album.title),
                    song: Arc::clone(&song.title),
                }
            })
            .collect()
    }

    /// INVARIANT: assumes key is valid.
    ///
    /// Returns a valid entry of a `Song`
    pub fn valid_from_song(key: SongKey, collection: &Arc<Collection>) -> Self {
        let (artist, album, song) = collection.walk(key);
        Self::Valid {
            key_artist: artist.key,
            key_album: album.key,
            key_song: song.key,
            artist: Arc::clone(&artist.name),
            album: Arc::clone(&album.title),
            song: Arc::clone(&song.title),
        }
    }
}

impl std::ops::Deref for Playlists {
    type Target = PlaylistsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Playlists {
    fn deref_mut(&mut self) -> &mut PlaylistsInner {
        &mut self.0
    }
}

impl Playlists {
    //-------------------------------------------------- Construction.
    /// Create an empty `Self` with no allocation.
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }

    //-------------------------------------------------- Playlist handling.
    /// Create a new playlist with this name, overwriting if it already exists.
    pub fn playlist_new(&mut self, s: &str) -> Option<VecDeque<Entry>> {
        self.insert(s.into(), VecDeque::with_capacity(8))
    }

    /// Remove the playlist with this name.
    pub fn playlist_remove(&mut self, s: Arc<str>) -> Option<VecDeque<Entry>> {
        self.remove(&s)
    }

    /// Clone the playlist from the 1st input, into a new one called the 2nd input.
    ///
    /// `Ok(Some(_))` => from existed, into was overwritten
    /// `Ok(None)`    => from existed, into was created
    /// `Err(())`     => from did not exist, nothing was created
    pub fn playlist_clone(
        &mut self,
        from: Arc<str>,
        into: &str,
    ) -> Result<Option<VecDeque<Entry>>, ()> {
        let vec = self.get(&from).map(|v| v.clone());
        if let Some(vec) = vec {
            Ok(self.insert(into.into(), vec))
        } else {
            Err(())
        }
    }

    /// Get the [`Entry`] with index `index` within the playlist `playlist`.
    ///
    /// `Ok(Some(_))` => playlist existed, song existed
    /// `Ok(None)`    => playlist existed, song did not exist
    /// `Err(())`     => playlist did not exist
    pub fn playlist_get_index(
        &self,
        index: usize,
        playlist: Arc<str>,
    ) -> Result<Option<&Entry>, ()> {
        if let Some(p) = self.get(&playlist) {
            Ok(p.get(index))
        } else {
            Err(())
        }
    }

    /// Remove the [`Entry`] with index `index` within the playlist `playlist`.
    ///
    /// `Ok(Some(_))` => playlist existed, song was removed
    /// `Ok(None)`    => playlist existed, song did not exist
    /// `Err(())`     => playlist did not exist, nothing was removed
    pub fn playlist_remove_index(
        &mut self,
        index: usize,
        playlist: Arc<str>,
    ) -> Result<Option<Entry>, ()> {
        if let Some(p) = self.get_mut(&playlist) {
            Ok(p.remove(index))
        } else {
            Err(())
        }
    }

    /// Add this artist to this playlist.
    ///
    /// Creates playlist if it did not exist.
    ///
    /// # Return
    /// - `bool`  => did the playlist already existed?
    /// - `usize` => playlist old length
    /// - `usize` => playlist new length
    ///
    /// # INVARIANT
    /// - Assumes `Append` index is not out-of-bounds
    /// - Assumes key is not out-of-bounds
    pub fn playlist_add_artist(
        &mut self,
        playlist: Arc<str>,
        key: ArtistKey,
        append: Append,
        collection: &Arc<Collection>,
    ) -> (bool, usize, usize) {
        let keys: Box<[SongKey]> = collection.all_songs(key);
        let iter = keys.iter();

        let mut existed = true;

        let v = self.entry(playlist).or_insert_with(|| {
            existed = false;
            VecDeque::with_capacity(keys.len())
        });

        let old_len = v.len();

        match append {
            Append::Back => iter.for_each(|k| {
                let (artist, album, song) = collection.walk(k);
                let entry = Entry::Valid {
                    key_artist: artist.key,
                    key_album: album.key,
                    key_song: *k,
                    artist: Arc::clone(&artist.name),
                    album: Arc::clone(&album.title),
                    song: Arc::clone(&song.title),
                };
                v.push_back(entry);
            }),
            Append::Front => iter.rev().for_each(|k| {
                let (artist, album, song) = collection.walk(k);
                let entry = Entry::Valid {
                    key_artist: artist.key,
                    key_album: album.key,
                    key_song: *k,
                    artist: Arc::clone(&artist.name),
                    album: Arc::clone(&album.title),
                    song: Arc::clone(&song.title),
                };
                v.push_front(entry);
            }),
            Append::Index(mut i) => iter.for_each(|k| {
                let (artist, album, song) = collection.walk(k);
                let entry = Entry::Valid {
                    key_artist: artist.key,
                    key_album: album.key,
                    key_song: *k,
                    artist: Arc::clone(&artist.name),
                    album: Arc::clone(&album.title),
                    song: Arc::clone(&song.title),
                };
                v.insert(i, entry);
                i += 1;
            }),
        }

        (existed, old_len, v.len())
    }

    /// Add this album to this playlist.
    ///
    /// Creates playlist if it did not exist.
    ///
    /// # Return
    /// - `bool`  => did the playlist already existed?
    /// - `usize` => playlist old length
    /// - `usize` => playlist new length
    ///
    /// # INVARIANT
    /// - Assumes `Append` index is not out-of-bounds
    /// - Assumes key is not out-of-bounds
    pub fn playlist_add_album(
        &mut self,
        playlist: Arc<str>,
        key: AlbumKey,
        append: Append,
        collection: &Arc<Collection>,
    ) -> (bool, usize, usize) {
        let keys = &collection.albums[key].songs;
        let iter = keys.iter();

        let mut existed = true;
        let v = self.entry(playlist).or_insert_with(|| {
            existed = false;
            VecDeque::with_capacity(keys.len())
        });

        let old_len = v.len();

        let album = &collection.albums[key];
        let artist = &collection.artists[album.artist];

        match append {
            Append::Back => iter.for_each(|k| {
                let entry = Entry::Valid {
                    key_artist: artist.key,
                    key_album: album.key,
                    key_song: *k,
                    artist: Arc::clone(&artist.name),
                    album: Arc::clone(&album.title),
                    song: Arc::clone(&collection.songs[*k].title),
                };
                v.push_back(entry)
            }),
            Append::Front => iter.rev().for_each(|k| {
                let entry = Entry::Valid {
                    key_artist: artist.key,
                    key_album: album.key,
                    key_song: *k,
                    artist: Arc::clone(&artist.name),
                    album: Arc::clone(&album.title),
                    song: Arc::clone(&collection.songs[*k].title),
                };
                v.push_front(entry)
            }),
            Append::Index(mut i) => iter.for_each(|k| {
                let entry = Entry::Valid {
                    key_artist: artist.key,
                    key_album: album.key,
                    key_song: *k,
                    artist: Arc::clone(&artist.name),
                    album: Arc::clone(&album.title),
                    song: Arc::clone(&collection.songs[*k].title),
                };
                v.insert(i, entry);
                i += 1;
            }),
        }

        (existed, old_len, v.len())
    }

    /// Add this song to this playlist.
    ///
    /// Creates playlist if it did not exist.
    ///
    /// # Return
    /// - `bool`  => did the playlist already existed?
    /// - `usize` => playlist old length
    /// - `usize` => playlist new length
    ///
    /// # INVARIANT
    /// - Assumes `Append` index is not out-of-bounds
    /// - Assumes key is not out-of-bounds
    pub fn playlist_add_song(
        &mut self,
        playlist: Arc<str>,
        key: SongKey,
        append: Append,
        collection: &Arc<Collection>,
    ) -> (bool, usize, usize) {
        let (artist, album, song) = collection.walk(key);

        let entry = Entry::Valid {
            key_artist: artist.key,
            key_album: album.key,
            key_song: song.key,
            artist: Arc::clone(&artist.name),
            album: Arc::clone(&album.title),
            song: Arc::clone(&song.title),
        };

        let mut existed = true;
        let v = self.entry(playlist).or_insert_with(|| {
            existed = false;
            VecDeque::with_capacity(8)
        });

        let old_len = v.len();

        match append {
            Append::Back => v.push_back(entry),
            Append::Front => v.push_front(entry),
            Append::Index(i) => v.insert(i, entry),
        }

        (existed, old_len, v.len())
    }

    //-------------------------------------------------- Misc.
    /// INVARIANT: this assumes the playlist's validity is already correct.
    ///
    /// Given a playlist name, extract out all the valid keys.
    ///
    /// `None` if playlist doesn't exist.
    ///
    /// Empty `Box<[]>` if it had no valid keys.
    pub fn valid_keys(
        &self,
        playlist_name: &str,
        collection: &Arc<Collection>,
    ) -> Option<Box<[SongKey]>> {
        let Some(playlist) = self.get(playlist_name) else {
            return None;
        };

        Some(
            playlist
                .iter()
                .filter_map(|e| {
                    if let Entry::Valid { key_song, .. } = e {
                        Some(*key_song)
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }

    /// Validate all keys (and strings), replace invalid ones with `Invalid`.
    ///
    /// Also, clone the `Arc`'s from the `Collection` as to not use more space.
    pub fn validate(&mut self, collection: &Arc<Collection>) {
        self.0.par_iter_mut().for_each(|(_, entry)| {
            entry.par_iter_mut().for_each(|entry| {
                match entry {
                    Entry::Valid {
                        artist,
                        album,
                        song,
                        ..
                    } => {
                        let Some((s, _)) = collection.song(&artist, &album, &song) else {
                            *entry = Entry::Invalid {
                                artist: Arc::clone(artist),
                                album: Arc::clone(album),
                                song: Arc::clone(song),
                            };
                            return;
                        };

                        // FIXME:
                        // This will cause songs that have the same name
                        // to be invalidated. Songs with the same name in the
                        // same album is not compatible `shukusai` in general.
                        //
                        // These are quite common with `interlude` type of songs
                        // so multiple songs with the same name should be supported...
                        // somehow... eventually... SOMEDAY.
                        //							if *key != s.key {
                        //								*entry = Entry::Invalid {
                        //									artist: Arc::clone(artist),
                        //									album: Arc::clone(album),
                        //									song: Arc::clone(song),
                        //								};
                        //								return;
                        //							}

                        let (artist, album, song) = collection.walk(s.key);
                        *entry = Entry::Valid {
                            key_artist: artist.key,
                            key_album: album.key,
                            key_song: s.key,
                            artist: Arc::clone(&artist.name),
                            album: Arc::clone(&album.title),
                            song: Arc::clone(&song.title),
                        };
                    }
                    Entry::Invalid {
                        artist,
                        album,
                        song,
                    } => {
                        if let Some((s, _)) = collection.song(&artist, &album, &song) {
                            let (artist, album, song) = collection.walk(s.key);
                            *entry = Entry::Valid {
                                key_artist: artist.key,
                                key_album: album.key,
                                key_song: s.key,
                                artist: Arc::clone(&artist.name),
                                album: Arc::clone(&album.title),
                                song: Arc::clone(&song.title),
                            };
                        }
                    }
                }
            });
        });
    }

    /// Convert all inner `Entry`'s into the `invalid` variants.
    pub fn all_invalid(&mut self) {
        self.0.par_iter_mut().for_each(|(_, entry)| {
            entry.par_iter_mut().for_each(|entry| {
                if let Entry::Valid {
                    artist,
                    album,
                    song,
                    ..
                } = entry
                {
                    *entry = Entry::Invalid {
                        artist: Arc::clone(artist),
                        album: Arc::clone(album),
                        song: Arc::clone(song),
                    };
                }
            });
        });
    }

    /// Convert all `Entry`'s to `Valid` is possible.
    pub fn convert(&mut self, collection: &Arc<Collection>) {
        self.0.par_iter_mut().for_each(|(_, entry)| {
            entry.par_iter_mut().for_each(|entry| {
                if let Entry::Invalid {
                    artist,
                    album,
                    song,
                } = entry
                {
                    if let Some((s, _)) = collection.song(&artist, &album, &song) {
                        let (artist, album, song) = collection.walk(s.key);
                        *entry = Entry::Valid {
                            key_artist: artist.key,
                            key_album: album.key,
                            key_song: s.key,
                            artist: Arc::clone(&artist.name),
                            album: Arc::clone(&album.title),
                            song: Arc::clone(&song.title),
                        };
                    }
                }
            });
        });
    }

    /// Return the length of valid entries.
    pub fn valid_len(playlist: &VecDeque<Entry>) -> usize {
        playlist
            .iter()
            .map(|e| match e {
                Entry::Valid { .. } => 1,
                _ => 0,
            })
            .sum()
    }

    /// Return the length of invalid entries.
    pub fn invalid_len(playlist: &VecDeque<Entry>) -> usize {
        playlist
            .iter()
            .map(|e| match e {
                Entry::Invalid { .. } => 1,
                _ => 0,
            })
            .sum()
    }

    /// Returns a `Vec` of (`playlist_name_str`, `entry_count`).
    pub fn name_count_iter(&self) -> Vec<(&str, usize)> {
        self.0.iter().map(|(s, v)| (&**s, v.len())).collect()
    }

    /// Returns a `Vec` of all playlist names, cheaply cloned.
    pub fn name_arcs(&self) -> Vec<Arc<str>> {
        self.0.keys().map(Arc::clone).collect()
    }
}

//---------------------------------------------------------------------------------------------------- JSON Representation
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(transparent)]
#[repr(transparent)]
/// Stable `JSON` representation of [`Playlists`].
pub struct PlaylistsJson<'a>(#[serde(borrow)] BTreeMap<Cow<'a, str>, VecDeque<EntryJson<'a>>>);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Stable `JSON` representation of [`Entry`].
pub enum EntryJson<'a> {
    /// This is a valid song in the current `Collection`
    Valid {
        /// Artist key
        key_artist: ArtistKey,
        /// Album key
        key_album: AlbumKey,
        /// Song key
        key_song: SongKey,
        /// Artist name
        artist: Cow<'a, str>,
        #[serde(borrow)]
        /// Album title
        album: Cow<'a, str>,
        #[serde(borrow)]
        /// Song title
        song: Cow<'a, str>,
    },

    /// This song is missing, this was the
    /// `artist.name`, `album.title`, `song.title`.
    Invalid {
        #[serde(borrow)]
        /// Artist name
        artist: Cow<'a, str>,
        #[serde(borrow)]
        /// Album title
        album: Cow<'a, str>,
        #[serde(borrow)]
        /// Song title
        song: Cow<'a, str>,
    },
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
