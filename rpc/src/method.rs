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
	/// [`crate::resp::CollectionBriefArtists`]
	CollectionBriefArtists,
	/// [`crate::resp::CollectionBriefAlbums`]
	CollectionBriefAlbums,
	/// [`crate::resp::CollectionBriefSongs`]
	CollectionBriefSongs,
	/// [`crate::resp::CollectionFullArtists`]
	CollectionFullArtists,
	/// [`crate::resp::CollectionFullAlbums`]
	CollectionFullAlbums,
	/// [`crate::resp::CollectionFullSongs`]
	CollectionFullSongs,
	/// [`crate::resp::CollectionEntries`]
	CollectionEntries,
	/// [`crate::resp::CollectionPerf`]
	CollectionPerf,
	/// [`crate::resp::CollectionHealth`]
	CollectionHealth,
	/// [`crate::resp::CollectionResourceSize`]
	CollectionResourceSize,

	// State retrieval.
	/// [`crate::resp::StateAudio`]
	StateAudio,
	/// [`crate::resp::StateConfig`]
	StateConfig,
	/// [`crate::resp::StateDaemon`]
	StateDaemon,
	/// [`crate::resp::StateIp`] (inner type is [`crate::resp::StateIpInner`])
	StateIp,
	/// [`crate::resp::StateReset`]
	StateReset,
	/// [`crate::resp::StateVolume`]
	StateVolume,

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
	/// [`crate::resp::Status`] & [`crate::param::Previous`]
	Previous,
	/// [`crate::resp::Status`] & [`crate::param::Clear`]
	Clear,
	/// [`crate::resp::Status`] & [`crate::param::Seek`]
	Seek,
	/// [`crate::resp::Status`] & [`crate::param::Skip`]
	Skip,
	/// [`crate::resp::Status`] & [`crate::param::Back`]
	Back,
	/// [`crate::resp::Status`]
	Shuffle,
	/// [`crate::resp::Status`]
	RepeatOff,
	/// [`crate::resp::Status`]
	RepeatSong,
	/// [`crate::resp::Status`]
	RepeatQueue,
	/// [`crate::resp::Volume`] & [`crate::param::Volume`]
	Volume,
	/// [`crate::resp::VolumeUp`] & [`crate::param::VolumeUp`]
	VolumeUp,
	/// [`crate::resp::VolumeDown`] & [`crate::param::VolumeDown`]
	VolumeDown,

	/// [`crate::resp::QueueAddKeyArtist`] & [`crate::param::QueueAddKeyArtist`]
	QueueAddKeyArtist,
	/// [`crate::resp::QueueAddKeyAlbum`] & [`crate::param::QueueAddKeyAlbum`]
	QueueAddKeyAlbum,
	/// [`crate::resp::QueueAddKeySong`] & [`crate::param::QueueAddKeySong`]
	QueueAddKeySong,
	/// [`crate::resp::QueueAddMapArtist`] & [`crate::param::QueueAddMapArtist`]
	QueueAddMapArtist,
	/// [`crate::resp::QueueAddMapAlbum`] & [`crate::param::QueueAddMapAlbum`]
	QueueAddMapAlbum,
	/// [`crate::resp::QueueAddMapSong`] & [`crate::param::QueueAddMapSong`]
	QueueAddMapSong,
	/// [`crate::resp::QueueAddRandArtist`] & [`crate::param::QueueAddRandArtist`]
	QueueAddRandArtist,
	/// [`crate::resp::QueueAddRandAlbum`] & [`crate::param::QueueAddRandAlbum`]
	QueueAddRandAlbum,
	/// [`crate::resp::QueueAddRandSong`] & [`crate::param::QueueAddRandSong`]
	QueueAddRandSong,
	/// [`crate::resp::QueueAddPlaylist`] & [`crate::param::QueueAddPlaylist`]
	QueueAddPlaylist,
	/// [`crate::resp::SetQueueIndex`] & [`crate::param::QueueSetIndex`]
	QueueSetIndex,
	/// [`crate::resp::Status`] & [`crate::param::QueueRemoveRange`]
	QueueRemoveRange,

	// Playlists.
	/// [`crate::resp::PlaylistNew`] & [`crate::param::PlaylistNew`]
	PlaylistNew,
	/// [`crate::resp::PlaylistRemove`] & [`crate::param::PlaylistRemove`]
	PlaylistRemove,
	/// [`crate::resp::PlaylistClone`] & [`crate::param::PlaylistClone`]
	PlaylistClone,
	/// [`crate::resp::PlaylistRemoveEntry`] & [`crate::param::PlaylistRemoveEntry`]
	PlaylistRemoveEntry,
	/// [`crate::resp::PlaylistAddKeyArtist`] & [`crate::param::PlaylistAddKeyArtist`]
	PlaylistAddKeyArtist,
	/// [`crate::resp::PlaylistAddKeyAlbum`] & [`crate::param::PlaylistAddKeyAlbum`]
	PlaylistAddKeyAlbum,
	/// [`crate::resp::PlaylistAddKeySong`] & [`crate::param::PlaylistAddKeySong`]
	PlaylistAddKeySong,
	/// [`crate::resp::PlaylistAddMapArtist`] & [`crate::param::PlaylistAddMapArtist`]
	PlaylistAddMapArtist,
	/// [`crate::resp::PlaylistAddMapAlbum`] & [`crate::param::PlaylistAddMapAlbum`]
	PlaylistAddMapAlbum,
	/// [`crate::resp::PlaylistAddMapSong`] & [`crate::param::PlaylistAddMapSong`]
	PlaylistAddMapSong,
	/// [`crate::resp::PlaylistSingle`] & [`crate::param::PlaylistSingle`]
	PlaylistSingle,
	/// [`crate::resp::PlaylistBrief`] & [`crate::param::PlaylistBrief`]
	PlaylistBrief,
	/// [`crate::resp::PlaylistFull`] & [`crate::param::PlaylistFull`]
	PlaylistFull,
}

impl Method {
	/// Print each method, seperated by a newline.
	pub fn println_all() {
		use strum::IntoEnumIterator;
		for i in Self::iter() {
			println!("{i}");
		}
	}
}

#[derive(clap::Subcommand,Clone,Debug,Serialize,Deserialize)]
#[derive(AsRefStr,Display,EnumCount,EnumVariantNames,IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[command(rename_all = "snake_case")]
pub enum Rpc {
	CollectionNew(crate::param::CollectionNew),
	CollectionBrief(crate::param::CollectionBrief),
	CollectionFull(crate::param::CollectionFull),
	CollectionBriefArtists(crate::param::CollectionBriefArtists),
	CollectionBriefAlbums(crate::param::CollectionBriefAlbums),
	CollectionBriefSongs(crate::param::CollectionBriefSongs),
	CollectionFullArtists(crate::param::CollectionFullArtists),
	CollectionFullAlbums(crate::param::CollectionFullAlbums),
	CollectionFullSongs(crate::param::CollectionFullSongs),
	CollectionEntries(crate::param::CollectionEntries),
	CollectionPerf(crate::param::CollectionPerf),
	CollectionHealth(crate::param::CollectionHealth),
	CollectionResourceSize(crate::param::CollectionResourceSize),

	StateAudio(crate::param::StateAudio),
	StateConfig(crate::param::StateConfig),
	StateDaemon(crate::param::StateDaemon),
	StateIp(crate::param::StateIp),
	StateReset(crate::param::StateReset),
	StateVolume(crate::param::StateVolume),

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
	Previous(crate::param::Previous),
	Clear(crate::param::Clear),
	Seek(crate::param::Seek),
	Skip(crate::param::Skip),
	Back(crate::param::Back),
	Shuffle(crate::param::Shuffle),
	RepeatOff(crate::param::RepeatOff),
	RepeatSong(crate::param::RepeatSong),
	RepeatQueue(crate::param::RepeatQueue),
	Volume(crate::param::Volume),
	VolumeUp(crate::param::VolumeUp),
	VolumeDown(crate::param::VolumeDown),

	QueueAddKeyArtist(crate::param::QueueAddKeyArtist),
	QueueAddKeyAlbum(crate::param::QueueAddKeyAlbum),
	QueueAddKeySong(crate::param::QueueAddKeySong),
	QueueAddMapArtist(crate::param::QueueAddMapArtistOwned),
	QueueAddMapAlbum(crate::param::QueueAddMapAlbumOwned),
	QueueAddMapSong(crate::param::QueueAddMapSongOwned),
	QueueAddRandArtist(crate::param::QueueAddRandArtist),
	QueueAddRandAlbum(crate::param::QueueAddRandAlbum),
	QueueAddRandSong(crate::param::QueueAddRandSong),
	QueueAddPlaylist(crate::param::QueueAddPlaylistOwned),
	QueueSetIndex(crate::param::QueueSetIndex),
	QueueRemoveRange(crate::param::QueueRemoveRange),

	PlaylistNew(crate::param::PlaylistNewOwned),
	PlaylistRemove(crate::param::PlaylistRemoveOwned),
	PlaylistClone(crate::param::PlaylistCloneOwned),
	PlaylistRemoveEntry(crate::param::PlaylistRemoveEntryOwned),
	PlaylistAddKeyArtist(crate::param::PlaylistAddKeyArtist),
	PlaylistAddKeyAlbum(crate::param::PlaylistAddKeyAlbum),
	PlaylistAddKeySong(crate::param::PlaylistAddKeySong),
	PlaylistAddMapArtist(crate::param::PlaylistAddMapArtistOwned),
	PlaylistAddMapAlbum(crate::param::PlaylistAddMapAlbumOwned),
	PlaylistAddMapSong(crate::param::PlaylistAddMapSongOwned),
	PlaylistSingle(crate::param::PlaylistSingleOwned),
	PlaylistBrief(crate::param::PlaylistBrief),
	PlaylistFull(crate::param::PlaylistFull),
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
