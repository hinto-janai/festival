//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use std::sync::Arc;
use crate::{
	collection::{
		Collection,
		ArtistKey,
		AlbumKey,
		SongKey,
	},
};
use std::borrow::Cow;
use std::path::{Path,PathBuf};

//---------------------------------------------------------------------------------------------------- Entry
#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[serde(rename_all = "snake_case")]
/// Like [`crate::state::Entry`] but assumed to always be valid.
pub struct Entry {
	/// Song PATH,
	pub path: PathBuf,
	/// Artist key
	pub key_artist: ArtistKey,
	/// Album key
	pub key_album: AlbumKey,
	/// Song key
	pub key_song: SongKey,
	/// Artist name
	pub artist: Arc<str>,
	/// Album title
	pub album: Arc<str>,
	/// Song title
	pub song: Arc<str>,
}

impl Entry {
	/// INVARIANT: assumes key is valid.
	pub fn from_song(key: SongKey, collection: &Arc<Collection>) -> Self {
		let (artist, album, song) = collection.walk(key);
		Self {
			path: song.path.clone(),
			key_artist: artist.key,
			key_album: album.key,
			key_song: song.key,
			artist: Arc::clone(&artist.name),
			album: Arc::clone(&album.title),
			song: Arc::clone(&song.title),
		}
	}
}

//---------------------------------------------------------------------------------------------------- JSON Representation
#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
/// Stable `JSON` representation of [`Entry`].
pub struct EntryJson<'a> {
	#[serde(borrow)]
	/// Song PATH
	pub path: Cow<'a, Path>,
	/// Artist key
	pub key_artist: ArtistKey,
	/// Album key
	pub key_album: AlbumKey,
	/// Song key
	pub key_song: SongKey,
	#[serde(borrow)]
	/// Artist name
	pub artist: Cow<'a, str>,
	#[serde(borrow)]
	/// Album title
	pub album: Cow<'a, str>,
	#[serde(borrow)]
	/// Song title
	pub song: Cow<'a, str>,
}

impl<'a> EntryJson<'a> {
	/// INVARIANT: assumes key is valid.
	pub fn from_song(key: SongKey, collection: &'a Arc<Collection>) -> Self {
		let (artist, album, song) = collection.walk(key);
		Self {
			path: Cow::Borrowed(&song.path),
			key_artist: artist.key,
			key_album: album.key,
			key_song: song.key,
			artist: Cow::Borrowed(&*artist.name),
			album: Cow::Borrowed(&*album.title),
			song: Cow::Borrowed(&*song.title),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}

