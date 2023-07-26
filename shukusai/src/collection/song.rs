//---------------------------------------------------------------------------------------------------- Use
use serde::Serialize;
use bincode::{Encode,Decode};
use std::path::PathBuf;
use readable::Runtime;
use std::marker::PhantomData;
use crate::collection::{
	SongKey,
	AlbumKey,
};
use std::sync::Arc;

//----------------------------------------------------------------------------------------------------
#[derive(Clone,Debug,Hash,PartialEq,PartialOrd,Serialize,Encode,Decode)]
/// Struct holding [`Song`] metadata, with a pointer to the [`Album`] it belongs to
///
/// This struct holds all the metadata about a particular [`Song`].
///
/// It contains a [`SongKey`] that is the index of the owning [`Album`], in the [`Collection`].
pub struct Song {
	/// This [`Song`]'s [`SongKey`].
	pub key: SongKey,
	/// Title of the [`Song`].
	pub title: Arc<str>,
	/// Title of the [`Song`] in "Unicode Derived Core Property" lowercase.
	pub title_lowercase: Arc<str>,
	/// Key to the [`Album`].
	pub album: AlbumKey,
	#[serde(serialize_with = "crate::collection::serde::runtime")]
	/// Total runtime of this [`Song`].
	pub runtime: Runtime,
	/// Sample rate of this [`Song`].
	pub sample_rate: u32,
	/// The track number of this [`Song`].
	pub track: Option<u32>,
	/// The disc number of this [`Song`].
	pub disc: Option<u32>,
	#[serde(skip)]
	/// The [`PathBuf`] this [`Song`] is located at.
	pub path: PathBuf,
}

impl Default for Song {
	fn default() -> Self {
		Self {
			key: SongKey::zero(),
			title: "".into(),
			title_lowercase: "".into(),
			album: Default::default(),
			runtime: Default::default(),
			sample_rate: Default::default(),
			track: Default::default(),
			disc: Default::default(),
			path: Default::default(),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	const EXPECTED: &str =
r#"{
  "title": "",
  "title_lowercase": "",
  "album": 0,
  "runtime": 0,
  "sample_rate": 0,
  "track": null,
  "disc": null
}"#;

	#[test]
	fn serde_json() {
		let d: String = serde_json::to_string_pretty(&Song::default()).unwrap();
		assert_eq!(EXPECTED, d);
	}
}
