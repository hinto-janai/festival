 //---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use log::{error,info,warn,debug,trace};
use disk::Bincode2;
use std::{
	collections::{
		BTreeMap,VecDeque,
	},
	sync::Arc,
};
use crate::{
	audio::Append,
	collection::{
		Collection,
		ArtistKey,
		AlbumKey,
		SongKey,
	},
	constants::{
		FESTIVAL,
		HEADER,
		FRONTEND_SUB_DIR,
		STATE_SUB_DIR,
		PLAYLIST_VERSION,
	},
};
use const_format::formatcp;
use rayon::prelude::*;
use std::sync::{
	RwLock,
	RwLockReadGuard,
	RwLockWriteGuard,
	TryLockError,
};
use benri::{
	lockw,lockr,
};
use std::borrow::Cow;

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
	pub fn try_read(&'static self) -> Result<RwLockReadGuard<'static, Playlists>, TryLockError<RwLockReadGuard<'static, Playlists>>> {
		self.0.try_read()
	}

	#[inline(always)]
	// Private write.
	pub(crate) fn write(&'static self) -> RwLockWriteGuard<'static, Playlists> {
		lockw!(self.0)
	}

	#[inline(always)]
	// Private write.
	pub(crate) fn try_write(&'static self) -> Result<RwLockWriteGuard<'static, Playlists>, TryLockError<RwLockWriteGuard<'static, Playlists>>> {
		self.0.try_write()
	}
}

//---------------------------------------------------------------------------------------------------- __NAME__
disk::bincode2!(Playlists, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/{STATE_SUB_DIR}"), "playlists", HEADER, PLAYLIST_VERSION);
#[derive(Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[serde(rename_all = "snake_case")]
#[serde(transparent)]
#[repr(transparent)]
/// Playlist implementation.
///
/// Contains all user playlists, ordering via `BTreeMap`.
///
/// Each node in the `BTreeMap` is a `(String, VecDeque)` where
/// the `String` is the name of the playlist, and the `VecDeque`
/// contains [`PlaylistEntry`]'s.
pub struct Playlists(pub PlaylistsInner);

/// The internal type of [`Playlists`].
///
/// [`Playlists`] is just a wrapper so methods/traits can be implemented on it.
pub type PlaylistsInner = BTreeMap<Arc<str>, VecDeque<PlaylistEntry>>;

#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[serde(rename_all = "snake_case")]
/// `Option`-like enum for playlist entries.
///
/// Either song exists in the current `Collection` (`PlaylistEntry::Key`)
/// or it is missing (`PlaylistEntry::Invalid`).
pub enum PlaylistEntry {
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
	/// Create a new playlist with this name.
	pub fn playlist_new(&mut self, s: &str) {
		self.insert(s.into(), VecDeque::with_capacity(8));
	}

	/// Remove the playlist with this name.
	pub fn playlist_remove(&mut self, s: Arc<str>) {
		self.remove(&s);
	}

	/// Clone the playlist from the 1st input, into a new one called the 2nd input.
	pub fn playlist_clone(&mut self, from: Arc<str>, into: &str) {
		let vec = self.get(&from).map(|v| v.clone());
		if let Some(vec) = vec {
			self.insert(into.into(), vec);
		}
	}

	/// Remove the [`Song`] with index `index` within the playlist `playlist`.
	pub fn playlist_remove_song(&mut self, index: usize, playlist: Arc<str>) {
		if let Some(p) = self.get_mut(&playlist) {
			p.remove(index);
		}
	}

	/// Add this artist to this playlist.
	pub fn playlist_add_artist(&mut self, playlist: Arc<str>, key: ArtistKey, append: Append, collection: &Arc<Collection>) {
		let keys: Box<[SongKey]> = collection.all_songs(key);
		let iter = keys.iter();

		let v = self
			.entry(playlist)
			.or_insert_with(|| VecDeque::with_capacity(keys.len()));

		match append {
			Append::Back => iter.for_each(|k| {
				let (artist, album, song) = collection.walk(k);
				let entry = PlaylistEntry::Valid {
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
				let entry = PlaylistEntry::Valid {
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
				let entry = PlaylistEntry::Valid {
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
	}

	/// Add this album to this playlist.
	pub fn playlist_add_album(&mut self, playlist: Arc<str>, key: AlbumKey, append: Append, collection: &Arc<Collection>) {
		let keys = &collection.albums[key].songs;
		let iter = keys.iter();

		let v = self
			.entry(playlist)
			.or_insert_with(|| VecDeque::with_capacity(keys.len()));

		let album  = &collection.albums[key];
		let artist = &collection.artists[album.artist];

		match append {
			Append::Back => iter.for_each(|k| {
				let entry = PlaylistEntry::Valid {
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
				let entry = PlaylistEntry::Valid {
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
				let entry = PlaylistEntry::Valid {
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
	}

	/// Add this song to this playlist.
	pub fn playlist_add_song(&mut self, playlist: Arc<str>, key: SongKey, append: Append, collection: &Arc<Collection>) {
		let (artist, album, song) = collection.walk(key);

		let entry = PlaylistEntry::Valid {
			key_artist: artist.key,
			key_album: album.key,
			key_song: key,
			artist: Arc::clone(&artist.name),
			album: Arc::clone(&album.title),
			song: Arc::clone(&song.title),
		};

		let v = self
			.entry(playlist)
			.or_insert_with(|| VecDeque::with_capacity(8));

		match append {
			Append::Back     => v.push_back(entry),
			Append::Front    => v.push_front(entry),
			Append::Index(i) => v.insert(i, entry),
		}
	}

	//-------------------------------------------------- Misc.
	/// INVARIANT: this assumes the playlist's validity is already correct.
	///
	/// Given a playlist name, extract out all the valid keys.
	///
	/// `None` if playlist doesn't exist.
	///
	/// Empty `Box<[]>` if it had no valid keys.
	pub fn valid_keys(&self, playlist_name: &str, collection: &Arc<Collection>) -> Option<Box<[SongKey]>> {
		let Some(playlist) = self.get(playlist_name) else {
			return None;
		};

		Some(playlist
			.iter()
			.filter_map(|e| {
				if let PlaylistEntry::Valid { key_song, .. } = e {
					Some(*key_song)
				} else {
					None
				}
			})
			.collect()
		)
	}

	/// Validate all keys (and strings), replace invalid ones with `Invalid`.
	///
	/// Also, clone the `Arc`'s from the `Collection` as to not use more space.
	pub fn validate(&mut self, collection: &Arc<Collection>) {
		self.0
			.par_iter_mut()
			.for_each(|(_, entry)| {
				entry
				.par_iter_mut()
				.for_each(|entry| {
					match entry {
						PlaylistEntry::Valid { artist, album, song, .. } => {
							let Some((s, _)) = collection.song(&artist, &album, &song) else {
								*entry = PlaylistEntry::Invalid {
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
//								*entry = PlaylistEntry::Invalid {
//									artist: Arc::clone(artist),
//									album: Arc::clone(album),
//									song: Arc::clone(song),
//								};
//								return;
//							}

							let (artist, album, song) = collection.walk(s.key);
							*entry = PlaylistEntry::Valid {
								key_artist: artist.key,
								key_album: album.key,
								key_song: s.key,
								artist: Arc::clone(&artist.name),
								album: Arc::clone(&album.title),
								song: Arc::clone(&song.title),
							};
						},
						PlaylistEntry::Invalid { artist, album, song } => {
							if let Some((s, _)) = collection.song(&artist, &album, &song) {
								let (artist, album, song) = collection.walk(s.key);
								*entry = PlaylistEntry::Valid {
									key_artist: artist.key,
									key_album: album.key,
									key_song: s.key,
									artist: Arc::clone(&artist.name),
									album: Arc::clone(&album.title),
									song: Arc::clone(&song.title),
								};
							}
						},
					}
				});
			});
	}

	/// Convert all inner `PlaylistEntry`'s
	/// into the string variants.
	pub fn all_missing(&mut self, collection: &Arc<Collection>) {
		self.0
			.par_iter_mut()
			.for_each(|(_, entry)| {
				entry
				.par_iter_mut()
				.for_each(|entry| {
					if let PlaylistEntry::Valid { artist, album, song, .. } = entry {
						*entry = PlaylistEntry::Invalid {
							artist: Arc::clone(artist),
							album: Arc::clone(album),
							song: Arc::clone(song),
						};
					}
				});
			});
	}

	/// Convert all `PlaylistEntry`'s to `Valid` is possible.
	pub fn convert(&mut self, collection: &Arc<Collection>) {
		self.0
			.par_iter_mut()
			.for_each(|(_, entry)| {
				entry
				.par_iter_mut()
				.for_each(|entry| {
					if let PlaylistEntry::Invalid { artist, album, song } = entry {
						if let Some((s, _)) = collection.song(&artist, &album, &song) {
							let (artist, album, song) = collection.walk(s.key);
							*entry = PlaylistEntry::Valid {
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

	/// Returns a `Vec` of (`playlist_name_str`, `entry_count`).
	pub fn name_count_iter(&self) -> Vec<(&str, usize)> {
		self.0
			.iter()
			.map(|(s, v)| (&**s, v.len()))
			.collect()
	}

	/// Returns a `Vec` of all playlist names, cheaply cloned.
	pub fn name_arcs(&self) -> Vec<Arc<str>> {
		self.0
			.keys()
			.map(Arc::clone)
			.collect()
	}
}

//---------------------------------------------------------------------------------------------------- JSON Representation
#[derive(Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(transparent)]
#[repr(transparent)]
/// Stable `JSON` representation of [`Playlists`].
pub struct PlaylistsJson<'a>(#[serde(borrow)] BTreeMap<Cow<'a, str>, VecDeque<PlaylistEntryJson<'a>>>);

#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
/// Stable `JSON` representation of [`PlaylistEntry`].
pub enum PlaylistEntryJson<'a> {
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
