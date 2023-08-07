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
	collection::{
		SongKey,Collection,
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
pub type PlaylistsInner = BTreeMap<String, VecDeque<PlaylistEntry>>;

#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[serde(rename_all = "snake_case")]
/// `Option`-like enum for playlist entries.
///
/// Either song exists in the current `Collection` (`PlaylistEntry::Key`)
/// or it is missing (`PlaylistEntry::Invalid`).
pub enum PlaylistEntry {
	/// This is a valid song in the current `Collection`
	Valid {
		/// Song key
		key: SongKey,
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
	/// Create an empty `Self` with no allocation.
	pub const fn new() -> Self {
		Self(BTreeMap::new())
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
						PlaylistEntry::Valid { key, artist, album, song } => {
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
								key: s.key,
								artist: Arc::clone(&artist.name),
								album: Arc::clone(&album.title),
								song: Arc::clone(&song.title),
							};
						},
						PlaylistEntry::Invalid { artist, album, song } => {
							if let Some((s, _)) = collection.song(&artist, &album, &song) {
								let (artist, album, song) = collection.walk(s.key);
								*entry = PlaylistEntry::Valid {
									key: s.key,
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
					if let PlaylistEntry::Valid { key, artist, album, song } = entry {
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
								key: s.key,
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
			.map(|(s, v)| (s.as_str(), v.len()))
			.collect()
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
