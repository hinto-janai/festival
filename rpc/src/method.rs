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
	// Collection
	/// [`crate::resp::CollectionNew`] & [`crate::param::CollectionNew`]
	CollectionNew,
	/// [`crate::resp::CollectionBrief`]
	CollectionBrief,
	/// [`crate::resp::CollectionFull`]
	CollectionFull,
	/// [`crate::resp::CollectionRelation`] (inner type is [`crate::resp::CollectionRelationInner`])
	CollectionRelation,
	/// [`crate::resp::CollectionRelationFull`] (inner type is [`crate::resp::CollectionRelationFullInner`])
	CollectionRelationFull,
	/// [`crate::resp::CollectionPerf`]
	CollectionPerf,
	/// [`crate::resp::CollectionResourceSize`]
	CollectionResourceSize,

	// State retrieval.
	/// [`crate::resp::StateIp`] (inner type is [`crate::resp::StateIpInner`])
	StateIp,
	/// [`crate::resp::StateConfig`]
	StateConfig,
	/// [`crate::resp::StateDaemon`]
	StateDaemon,
	/// [`crate::resp::StateAudio`]
	StateAudio,
	/// [`crate::resp::StateReset`]
	StateReset,


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

	// Current (audio state)
	/// [`crate::resp::CurrentArtist`]
	CurrentArtist,
	/// [`crate::resp::CurrentAlbum`]
	CurrentAlbum,
	/// [`crate::resp::CurrentSong`]
	CurrentSong,

	// Rand (audio state)
	/// [`crate::resp::RandArtist`]
	RandArtist,
	/// [`crate::resp::RandAlbum`]
	RandAlbum,
	/// [`crate::resp::RandSong`]
	RandSong,

	// Search (fuzzy keys)
	/// [`crate::resp::Search`] & [`crate::param::Search`]
	Search,
	/// [`crate::resp::SearchArtist`] &  [`crate::param::SearchArtist`]
	SearchArtist,
	/// [`crate::resp::SearchAlbum`] & [`crate::param::SearchAlbum`]
	SearchAlbum,
	/// [`crate::resp::SearchSong`] & [`crate::param::SearchSong`]
	SearchSong,

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
	/// [`crate::resp::AddQueueKeyArtist`] & [`crate::param::AddQueueKeyArtist`]
	AddQueueKeyArtist,
	/// [`crate::resp::AddQueueKeyAlbum`] & [`crate::param::AddQueueKeyAlbum`]
	AddQueueKeyAlbum,
	/// [`crate::resp::AddQueueKeySong`] & [`crate::param::AddQueueKeySong`]
	AddQueueKeySong,
	/// [`crate::resp::AddQueueMapArtist`] & [`crate::param::AddQueueMapArtist`]
	AddQueueMapArtist,
	/// [`crate::resp::AddQueueMapAlbum`] & [`crate::param::AddQueueMapAlbum`]
	AddQueueMapAlbum,
	/// [`crate::resp::AddQueueMapSong`] & [`crate::param::AddQueueMapSong`]
	AddQueueMapSong,
	/// [`crate::resp::AddQueueRandArtist`] & [`crate::param::AddQueueRandArtist`]
	AddQueueRandArtist,
	/// [`crate::resp::AddQueueRandAlbum`] & [`crate::param::AddQueueRandAlbum`]
	AddQueueRandAlbum,
	/// [`crate::resp::AddQueueRandSong`] & [`crate::param::AddQueueRandSong`]
	AddQueueRandSong,
	/// [`crate::resp::SetQueueIndex`] & [`crate::param::SetQueueIndex`]
	SetQueueIndex,
	/// [`crate::resp::Status`] & [`crate::param::RemoveQueueRange`]
	RemoveQueueRange,
}

#[derive(clap::Subcommand,Clone,Debug,Serialize,Deserialize)]
#[derive(AsRefStr,Display,EnumCount,EnumVariantNames,IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[command(rename_all = "snake_case")]
/// Combined method + param.
pub enum Rpc {
	CollectionNew(crate::param::CollectionNew),
	CollectionBrief(crate::param::CollectionBrief),
	CollectionFull(crate::param::CollectionFull),
	CollectionRelation(crate::param::CollectionRelation),
	CollectionRelationFull(crate::param::CollectionRelationFull),
	CollectionPerf(crate::param::CollectionPerf),
	CollectionResourceSize(crate::param::CollectionResourceSize),

	StateIp(crate::param::StateIp),
	StateConfig(crate::param::StateConfig),
	StateDaemon(crate::param::StateDaemon),
	StateAudio(crate::param::StateAudio),
	StateReset(crate::param::StateReset),

	KeyArtist(crate::param::KeyArtist),
	KeyAlbum(crate::param::KeyAlbum),
	KeySong(crate::param::KeySong),

	MapArtist(crate::param::MapArtistOwned),
	MapAlbum(crate::param::MapAlbumOwned),
	MapSong(crate::param::MapSongOwned),

	CurrentArtist(crate::param::CurrentArtist),
	CurrentAlbum(crate::param::CurrentAlbum),
	CurrentSong(crate::param::CurrentSong),

	RandArtist(crate::param::RandArtist),
	RandAlbum(crate::param::RandAlbum),
	RandSong(crate::param::RandSong),

	Search(crate::param::SearchOwned),
	SearchArtist(crate::param::SearchArtistOwned),
	SearchAlbum(crate::param::SearchAlbumOwned),
	SearchSong(crate::param::SearchSongOwned),

	Toggle(crate::param::Toggle),
	Play(crate::param::Play),
	Pause(crate::param::Pause),
	Next(crate::param::Next),
	Stop(crate::param::Stop),
	Shuffle(crate::param::Shuffle),
	RepeatOff(crate::param::RepeatOff),
	RepeatSong(crate::param::RepeatSong),
	RepeatQueue(crate::param::RepeatQueue),
	Previous(crate::param::Previous),
	Volume(crate::param::Volume),
	Clear(crate::param::Clear),
	Seek(crate::param::Seek),
	Skip(crate::param::Skip),
	Back(crate::param::Back),

	AddQueueKeyArtist(crate::param::AddQueueKeyArtist),
	AddQueueKeyAlbum(crate::param::AddQueueKeyAlbum),
	AddQueueKeySong(crate::param::AddQueueKeySong),
	AddQueueMapArtist(crate::param::AddQueueMapArtistOwned),
	AddQueueMapAlbum(crate::param::AddQueueMapAlbumOwned),
	AddQueueMapSong(crate::param::AddQueueMapSongOwned),
	AddQueueRandArtist(crate::param::AddQueueRandArtist),
	AddQueueRandAlbum(crate::param::AddQueueRandAlbum),
	AddQueueRandSong(crate::param::AddQueueRandSong),
	SetQueueIndex(crate::param::SetQueueIndex),
	RemoveQueueRange(crate::param::RemoveQueueRange),
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
