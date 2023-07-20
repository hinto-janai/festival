//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use std::path::PathBuf;
use shukusai::{
	audio::{Append,Seek,Repeat,Volume},
	search::SearchKind,
};
use strum::{
	AsRefStr,
	Display,
	EnumCount,
	EnumIter,
	EnumString,
	EnumVariantNames,
	IntoStaticStr,
};
use json_rpc::{
	Request,
};
use std::borrow::Cow;

//---------------------------------------------------------------------------------------------------- Method
#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[derive(AsRefStr,Display,EnumCount,EnumIter,EnumString,EnumVariantNames,IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
/// Enum of all the JSON-RPC 2.0 methods
///
/// This implements `From/Into` for `&'static str`, `&str`, and `String`.
///
/// It is (de)serialized directly from/into a `lower_snake_case` string.
///
/// If a method does not have any parameters, there will be no doc comment.
///
/// If a method _does_ have parameter(s), a doc comment will link to the struct representation of it, found in [`crate::param`].
pub enum Method {
	Stats,
	State,
	StateAudio,
	StateCollection,
	Uptime,
	Version,
	Toggle,
	Play,
	Pause,
	Next,
	Stop,
	RepeatOff,
	RepeatSong,
	RepeatQueue,
	Shuffle,
	Exit,

	/// [`crate::param::Previous`]
	Previous,
	/// [`crate::param::Volume`]
	Volume,
	/// [`crate::param::AddQueueSong`]
	AddQueueSong,
	/// [`crate::param::AddQueueAlbum`]
	AddQueueAlbum,
	/// [`crate::param::AddQueueArtist`]
	AddQueueArtist,
	/// [`crate::param::Clear`]
	Clear,
	/// [`crate::param::Seek`]
	Seek,
	/// [`crate::param::Skip`]
	Skip,
	/// [`crate::param::Back`]
	Back,
	/// [`crate::param::SetQueueIndex`]
	SetQueueIndex,
	/// [`crate::param::RemoveQueueRange`]
	RemoveQueueRange,
	/// [`crate::param::Search`]
	Search,
	/// [`crate::param::SearchArtist`]
	SearchArtist,
	/// [`crate::param::SearchAlbum`]
	SearchAlbum,
	/// [`crate::param::SearchSong`]
	SearchSong,
	/// [`crate::param::MapArtist`]
	MapArtist,
	/// [`crate::param::MapAlbum`]
	MapAlbum,
	/// [`crate::param::MapSong`]
	MapSong,
	/// [`crate::param::NewCollection`]
	NewCollection,
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::to_string;
	use strum::*;

	#[test]
	fn serde() {
		for i in Method::iter() {
			assert_eq!(format!("\"{}\"", i.as_ref()), to_string(&i).unwrap());
		}
	}

	#[test]
	fn from_str() {
		use std::str::FromStr;

		for i in Method::iter() {
			assert_eq!(Method::from_str(i.as_ref()).unwrap(), i);
		}
	}
}
