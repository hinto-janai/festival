//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use std::path::PathBuf;
use std::borrow::Cow;

//---------------------------------------------------------------------------------------------------- Impl macros
// Implement a named map of JSON.
macro_rules! impl_param {
	($struct:ident, $($field:ident: $type:ty),*) => {
		#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
		pub struct $struct {
			$(
				pub $field: $type,
			)*
		}
	}
}

// Implement a named map of JSON with a lifetime: `'a`.
macro_rules! impl_param_lt {
	($struct:ident, $($field:ident: $type:ty),*) => {
		#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
		pub struct $struct<'a> {
			$(
				pub $field: $type,
			)*
		}
	}
}

// Implement a fixed size, anonymous JSON array.
macro_rules! impl_param_array {
	($struct:ident, $type:ty, $len:literal) => {
		#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
		#[serde(transparent)]
		#[repr(transparent)]
		pub struct $struct(pub [$type; $len]);
	}
}

// Implement a dynamically size, anonymous JSON array.
macro_rules! impl_param_vec {
	($struct:ident, $type:ty) => {
		#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
		#[serde(transparent)]
		#[repr(transparent)]
		pub struct $struct(pub Vec<$type>);
	}
}

//---------------------------------------------------------------------------------------------------- Param impl
use shukusai::{
	collection::{SongKey,AlbumKey,ArtistKey},
	audio::Append,
	search::SearchKind,
};

impl_param!(Previous, threshold: Option<u32>);
impl_param!(Volume, volume: u8);
impl_param!(Clear, playback: bool);
impl_param!(Skip, skip: usize);
impl_param!(Back, back: usize);
impl_param!(SetQueueIndex, index: usize);
impl_param!(RemoveQueueRange, start: usize, end: usize, skip: bool);
impl_param!(AddQueueSong, key: SongKey, append: Append, clear: bool);
impl_param!(AddQueueAlbum, key: AlbumKey, append: Append, clear: bool, offset: usize);
impl_param!(AddQueueArtist, key: ArtistKey, append: Append, clear: bool, offset: usize);
impl_param!(Seek, seek: shukusai::audio::Seek, second: u64);
impl_param!(Artist, key: ArtistKey);
impl_param!(Album, key: AlbumKey);
impl_param!(Song, key: SongKey);
impl_param_lt!(Search, input: Cow<'a, str>, kind: SearchKind);
impl_param_lt!(SearchArtist, input: Cow<'a, str>, kind: SearchKind);
impl_param_lt!(SearchAlbum, input: Cow<'a, str>, kind: SearchKind);
impl_param_lt!(SearchSong, input: Cow<'a, str>, kind: SearchKind);
impl_param_lt!(MapArtist, artist: Cow<'a, str>);
impl_param_lt!(MapAlbum, artist: Cow<'a, str>, album: Cow<'a, str>);
impl_param_lt!(MapSong, artist: Cow<'a, str>, album: Cow<'a, str>, song: Cow<'a, str>);
impl_param_vec!(NewCollection, PathBuf);

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
	fn previous() {
		t(&Previous { threshold: Some(u32::MAX) }, r#"{"threshold":4294967295}"#);
		t(&Previous { threshold: Some(0) },        r#"{"threshold":0}"#);
		t(&Previous { threshold: None },           r#"{"threshold":null}"#);
	}

	#[test]
	fn volume() {
		t(&Volume { volume: u8::MAX }, r#"{"volume":255}"#);
		t(&Volume { volume: 0 },       r#"{"volume":0}"#);
	}

	#[test]
	fn clear() {
		t(&Clear { playback: true }, r#"{"playback":true}"#);
		t(&Clear { playback: false }, r#"{"playback":false}"#);
	}

	#[test]
	fn skip() {
		t(&Skip { skip: usize::MAX }, r#"{"skip":18446744073709551615}"#);
		t(&Skip { skip: 0 },          r#"{"skip":0}"#);
	}

	#[test]
	fn back() {
		t(&Back { back: usize::MAX }, r#"{"back":18446744073709551615}"#);
		t(&Back { back: 0 },          r#"{"back":0}"#);
	}

	#[test]
	fn set_queue_index() {
		t(&SetQueueIndex { index: usize::MAX }, r#"{"index":18446744073709551615}"#);
		t(&SetQueueIndex { index: 0 },          r#"{"index":0}"#);
	}

	#[test]
	fn remove_queue_range() {
		t(&RemoveQueueRange { start: usize::MAX, end: usize::MAX, skip: true }, r#"{"start":18446744073709551615,"end":18446744073709551615,"skip":true}"#);
		t(&RemoveQueueRange { start: 0, end: 0, skip: false },                  r#"{"start":0,"end":0,"skip":false}"#);
	}

	#[test]
	fn add_queue_song() {
		t(&AddQueueSong { key: SongKey::from(0_u8), append: shukusai::audio::Append::Front, clear: true }, r#"{"key":0,"append":"front","clear":true}"#);
		t(&AddQueueSong { key: SongKey::from(1_u8), append: shukusai::audio::Append::Back, clear: false }, r#"{"key":1,"append":"back","clear":false}"#);
		t(&AddQueueSong { key: SongKey::from(2_u8), append: shukusai::audio::Append::Index(0), clear: true }, r#"{"key":2,"append":{"index":0},"clear":true}"#);
	}

	#[test]
	fn add_queue_album() {
		t(&AddQueueAlbum { key: AlbumKey::from(0_u8), append: shukusai::audio::Append::Front, clear: true, offset: 0 }, r#"{"key":0,"append":"front","clear":true,"offset":0}"#);
		t(&AddQueueAlbum { key: AlbumKey::from(1_u8), append: shukusai::audio::Append::Back, clear: false, offset: 1 }, r#"{"key":1,"append":"back","clear":false,"offset":1}"#);
		t(&AddQueueAlbum { key: AlbumKey::from(2_u8), append: shukusai::audio::Append::Index(0), clear: true, offset: 2 }, r#"{"key":2,"append":{"index":0},"clear":true,"offset":2}"#);
	}

	#[test]
	fn add_queue_artist() {
		t(&AddQueueArtist { key: ArtistKey::from(0_u8), append: shukusai::audio::Append::Front, clear: true, offset: 0 }, r#"{"key":0,"append":"front","clear":true,"offset":0}"#);
		t(&AddQueueArtist { key: ArtistKey::from(1_u8), append: shukusai::audio::Append::Back, clear: false, offset: 1 }, r#"{"key":1,"append":"back","clear":false,"offset":1}"#);
		t(&AddQueueArtist { key: ArtistKey::from(2_u8), append: shukusai::audio::Append::Index(0), clear: true, offset: 2 }, r#"{"key":2,"append":{"index":0},"clear":true,"offset":2}"#);
	}

	#[test]
	fn seek() {
		t(&Seek { seek: shukusai::audio::Seek::Forward, second: 0 }, r#"{"seek":"forward","second":0}"#);
		t(&Seek { seek: shukusai::audio::Seek::Backward, second: 1 }, r#"{"seek":"backward","second":1}"#);
		t(&Seek { seek: shukusai::audio::Seek::Absolute, second: 2 }, r#"{"seek":"absolute","second":2}"#);
	}

	#[test]
	fn search() {
		t(&Search { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(&Search { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(&Search { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
	}

	#[test]
	fn search_artist() {
		t(&SearchArtist { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(&SearchArtist { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(&SearchArtist { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
	}

	#[test]
	fn search_album() {
		t(&SearchAlbum { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(&SearchAlbum { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(&SearchAlbum { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
	}

	#[test]
	fn search_song() {
		t(&SearchSong { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(&SearchSong { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(&SearchSong { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
	}

	#[test]
	fn map_artist() {
		t(&MapArtist { artist: "hello".into() }, r#"{"artist":"hello"}"#);
	}

	#[test]
	fn map_album() {
		t(&MapAlbum { artist: "hello".into(), album: "hello2".into() }, r#"{"artist":"hello","album":"hello2"}"#);
	}

	#[test]
	fn map_song() {
		t(&MapSong { artist: "hello".into(), album: "hello2".into(), song: "hello3".into() }, r#"{"artist":"hello","album":"hello2","song":"hello3"}"#);
	}

	#[test]
	fn new_collection() {
		t(&NewCollection(vec![PathBuf::from("/path_1"), PathBuf::from("/path_2")]), r#"["/path_1","/path_2"]"#);
	}
}
