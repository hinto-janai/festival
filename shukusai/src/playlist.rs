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

//---------------------------------------------------------------------------------------------------- __NAME__
disk::bincode2!(Playlists, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/{STATE_SUB_DIR}"), "playlists", HEADER, PLAYLIST_VERSION);
#[derive(Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
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
/// `Option`-like enum for playlist entries.
///
/// Either song exists in the current `Collection` (`PlaylistEntry::Key`)
/// or it is missing (`PlaylistEntry::Missing`).
pub enum PlaylistEntry {
	/// This is a valid song in the current `Collection`
	Key(SongKey),

	/// This song is missing, this was the
	/// `artist.name`, `album.title`, `song.title`.
	Missing {
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
	/// Convert all inner `PlaylistEntry`'s
	/// into the string variants.
	pub fn all_missing(&mut self, collection: &Arc<Collection>) {
		self.0
			.par_iter_mut()
			.for_each(|(_, entry)| {
				entry
				.par_iter_mut()
				.for_each(|entry| {
					if let PlaylistEntry::Key(key) = entry {
						let (artist, album, song) = collection.walk(*key);
						*entry = PlaylistEntry::Missing {
							artist: Arc::clone(&artist.name),
							album: Arc::clone(&album.title),
							song: Arc::clone(&song.title),
						};
					}
				});
			});
	}

	/// For all `PlaylistEntry`'s, if it is missing
	/// but the key is found in the passed `Collection`,
	/// convert it to `PlaylistEntry::Key`.
	pub fn find_key(&mut self, collection: &Arc<Collection>) {
		self.0
			.par_iter_mut()
			.for_each(|(_, entry)| {
				entry
				.par_iter_mut()
				.for_each(|entry| {
					if let PlaylistEntry::Missing { artist, album, song } = entry {
						if let Some((song, _)) = collection.song(artist, album, song) {
							*entry = PlaylistEntry::Key(song.key);
						}
					}
				});
			});
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
