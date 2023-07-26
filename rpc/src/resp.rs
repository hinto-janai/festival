//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use std::path::PathBuf;
use std::borrow::Cow;
use strum::{
	AsRefStr,
	Display,
	EnumCount,
	EnumIter,
	EnumString,
	EnumVariantNames,
	IntoStaticStr,
};

//---------------------------------------------------------------------------------------------------- Impl macros
// Implement a named map of JSON.
macro_rules! impl_resp {
	($struct:ident, $($field:ident: $type:ty),*) => {
		#[derive(Clone,Debug,PartialEq,PartialOrd,Serialize,Deserialize)]
		pub struct $struct {
			$(
				pub $field: $type,
			)*
		}
	}
}

// Implement an anonymous map of JSON.
macro_rules! impl_resp_anon {
	($struct:ident, $type:ty) => {
		#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
		#[serde(transparent)]
		#[repr(transparent)]
		pub struct $struct(pub $type);
	}
}

// Implement a named map of JSON with a lifetime: `'a`.
macro_rules! impl_resp_lt {
	($struct:ident, $($field:ident: $type:ty),*) => {
		#[derive(Clone,Debug,PartialEq,PartialOrd,Serialize,Deserialize)]
		pub struct $struct<'a> {
			$(
				pub $field: $type,
			)*
		}
	}
}

//---------------------------------------------------------------------------------------------------- Response impl
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

impl_resp!(Status, ok: bool);

// State retreival.
impl_resp_lt!(Info, uptime: u64, version: Cow<'a, str>, commit: Cow<'a, str>, os: Cow<'a, str>);
impl_resp!(StateAudio, queue: Vec<SongKey>, queue_idx: Option<usize>, playing: bool, song: Option<SongKey>, elapsed: u32, runtime: u32, repeat: Repeat, volume: u8);
impl_resp!(StateReset, resetting: bool, saving: bool);
impl_resp_anon!(StateCollection, CollectionJson);

// Playback control.
impl_resp!(Toggle, playing: bool);
impl_resp!(Search, artists: Box<[ArtistKey]>, albums: Box<[AlbumKey]>, songs: Box<[SongKey]>);

// Search (fuzzy keys)
impl_resp!(SearchArtist, artists: Box<[ArtistKey]>);
impl_resp!(SearchAlbum, albums: Box<[AlbumKey]>);
impl_resp!(SearchSong, songs: Box<[SongKey]>);

// Map (exact hashmap)
impl_resp_anon!(MapArtist, ArtistJson);
impl_resp_anon!(MapAlbum, AlbumJson);
impl_resp_anon!(MapSong, SongJson);

// Key (exact key)
impl_resp_anon!(KeyArtist, ArtistJson);
impl_resp_anon!(KeyAlbum, AlbumJson);
impl_resp_anon!(KeySong, SongJson);

// Collection
impl_resp!(NewCollection, time: f64);

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
