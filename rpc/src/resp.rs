//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use std::path::PathBuf;
use std::borrow::Cow;
use shukusai::{
	collection::{
		json::{
			CollectionJson,
			SongJson,
			AlbumJson,
			ArtistJson,
		},
		SongKey,
		AlbumKey,
		ArtistKey,
	},
	audio::Repeat,
	search::SearchKind,
};
use crate::{
	impl_struct,
	impl_struct_lt,
};

//---------------------------------------------------------------------------------------------------- Response impl
// Generic response.
impl_struct!(Status, ok: bool);

// State retreival.
impl_struct_lt!(StateDaemon, uptime: u64, rest: bool, direct_download: bool, authorization: bool, version: Cow<'a, str>, commit: Cow<'a, str>, os: Cow<'a, str>);
impl_struct!(StateAudio, queue: Vec<SongKey>, queue_idx: Option<usize>, playing: bool, song: Option<SongKey>, elapsed: u32, runtime: u32, repeat: Repeat, volume: u8);
impl_struct!(StateReset, resetting: bool, saving: bool);
impl_struct!(StateCollection, collection: CollectionJson);

// Key (exact key)
impl_struct!(Artist, artist: ArtistJson);
impl_struct!(Album, album: AlbumJson);
impl_struct!(Song, song: SongJson);

// Map (exact hashmap)
impl_struct!(MapArtist, artist: ArtistJson);
impl_struct!(MapAlbum, album: AlbumJson);
impl_struct!(MapSong, song: SongJson);

// Search (fuzzy keys)
impl_struct!(Search, artists: Box<[ArtistJson]>, albums: Box<[AlbumJson]>, songs: Box<[SongJson]>);
impl_struct!(SearchArtist, artists: Box<[ArtistJson]>);
impl_struct!(SearchAlbum, albums: Box<[AlbumJson]>);
impl_struct!(SearchSong, songs: Box<[SongJson]>);

// Collection
impl_struct!(NewCollection, time: f64);

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	//------------------------------------- Serde sanity tests.
	// Testing function.
	fn t<T>(value: &T, expected: &'static str)
		where
			T: Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
	{
		let string = serde_json::to_string(value).unwrap();
		assert_eq!(string, expected);
		let t: T = serde_json::from_str(&string).unwrap();
		assert_eq!(t, *value);
		let e: T = serde_json::from_str(expected).unwrap();
		assert_eq!(e, *value);
	}

	#[test]
	fn status() {
		t(&Status { ok: true  }, r#"{"ok":true}"#);
		t(&Status { ok: false }, r#"{"ok":false}"#);
	}
}
