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
impl_param!(Previous, threshold: Option<u32>);
impl_param!(Volume, volume: u8);
impl_param!(Clear, playback: bool);
impl_param!(Skip, skip: usize);
impl_param!(Back, back: usize);
impl_param!(SetQueueIndex, index: usize);
impl_param!(RemoveQueueRange, start: usize, end: usize, skip: bool);
impl_param_lt!(AddQueueSong, key: usize, append: Cow<'a, str>, clear: bool);
impl_param_lt!(AddQueueAlbum, key: usize, append: Cow<'a, str>, clear: bool, offset: usize);
impl_param_lt!(AddQueueArtist, key: usize, append: Cow<'a, str>, clear: bool, offset: usize);
impl_param_lt!(Seek, seek: Cow<'a, str>, second: u64);
impl_param_lt!(Search, input: Cow<'a, str>, kind: Cow<'a, str>);
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

//	#[test]
//	fn add_queue_song() {
//		t(&AddQueueSong { key: 0, append: , clear: },          r#"{"index":0}"#);
//	}

	#[test]
	fn add_queue_song() {
		t(&AddQueueSong { key: 0, append: "front".into(), clear: true }, r#"{"key":0,"append":"front","clear":true}"#);
	}
}
