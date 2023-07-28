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
/// Each method is commented with:
///   - A struct representation of the expected response, found in [`resp`] with exact same name
///   - (Optionally) a struct representation of the associated parameters, found in [`crate::param`]
pub enum Method {
	// State retrieval.
	/// [`crate::resp::StateDaemon`]
	StateDaemon,
	/// [`crate::resp::StateAudio`]
	StateAudio,
	/// [`crate::resp::StateReset`]
	StateReset,
	/// [`crate::resp::StateCollection`]
	StateCollection,
	/// [`crate::resp::StateCollectionFull`]
	StateCollectionFull,

	// Playback control.
	/// [`crate::resp::Toggle`]
	Toggle,
	/// [`crate::resp::Status`]
	Play,
	/// [`crate::resp::Status`]
	Pause,
	/// [`crate::resp::Status`]
	Next,
	/// [`crate::resp::Status`]
	Stop,
	/// [`crate::resp::Status`]
	Shuffle,
	/// [`crate::resp::Status`]
	RepeatOff,
	/// [`crate::resp::Status`]
	RepeatSong,
	/// [`crate::resp::Status`]
	RepeatQueue,
	/// [`crate::resp::Status`] & [`crate::param::Previous`]
	Previous,
	/// [`crate::resp::Status`] & [`crate::param::Volume`]
	Volume,
	/// [`crate::resp::Status`] & [`crate::param::Clear`]
	Clear,
	/// [`crate::resp::Status`] & [`crate::param::Seek`]
	Seek,
	/// [`crate::resp::Status`] & [`crate::param::Skip`]
	Skip,
	/// [`crate::resp::Status`] & [`crate::param::Back`]
	Back,
	/// [`crate::resp::Status`] & [`crate::param::AddQueueKeyArtist`]
	AddQueueKeyArtist,
	/// [`crate::resp::Status`] & [`crate::param::AddQueueKeyAlbum`]
	AddQueueKeyAlbum,
	/// [`crate::resp::Status`] & [`crate::param::AddQueueKeySong`]
	AddQueueKeySong,
	/// [`crate::resp::Status`] & [`crate::param::AddQueueMapArtist`]
	AddQueueMapArtist,
	/// [`crate::resp::Status`] & [`crate::param::AddQueueMapAlbum`]
	AddQueueMapAlbum,
	/// [`crate::resp::Status`] & [`crate::param::AddQueueMapSong`]
	AddQueueMapSong,
	/// [`crate::resp::Status`] & [`crate::param::SetQueueIndex`]
	SetQueueIndex,
	/// [`crate::resp::Status`] & [`crate::param::RemoveQueueRange`]
	RemoveQueueRange,

	// Key (exact key)
	/// [`crate::resp::KeyArtist`] & [`crate::param::KeyArtist`]
	KeyArtist,
	/// [`crate::resp::KeyAlbum`] & [`crate::param::KeyAlbum`]
	KeyAlbum,
	/// [`crate::resp::KeySong`] & [`crate::param::KeySong`]
	KeySong,

	// Map (exact hashmap)
	/// [`crate::resp::MapArtist`] & [`crate::param::MapArtist`]
	MapArtist,
	/// [`crate::resp::MapAlbum`] & [`crate::param::MapAlbum`]
	MapAlbum,
	/// [`crate::resp::MapSong`] & [`crate::param::MapSong`]
	MapSong,

	// Search (fuzzy keys)
	/// [`crate::resp::Search`] & [`crate::param::Search`]
	Search,
	/// [`crate::resp::SearchArtist`] &  [`crate::param::SearchArtist`]
	SearchArtist,
	/// [`crate::resp::SearchAlbum`] & [`crate::param::SearchAlbum`]
	SearchAlbum,
	/// [`crate::resp::SearchSong`] & [`crate::param::SearchSong`]
	SearchSong,

	// Collection
	/// [`crate::resp::NewCollection`] & [`crate::param::NewCollection`]
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
