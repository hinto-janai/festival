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
/// Each method has:
///   - A struct representation of the expected response, found in [`resp`] with exact same name (or an `Owned` variant)
///   - (Optionally) a struct representation of the associated parameters, found in [`crate::param`]
pub enum Method {
	// Collection
	CollectionNew,
	CollectionBrief,
	CollectionFull,
	CollectionBriefArtists,
	CollectionBriefAlbums,
	CollectionBriefSongs,
	CollectionFullArtists,
	CollectionFullAlbums,
	CollectionFullSongs,
	CollectionEntries,
	CollectionPerf,
	CollectionHealth,
	CollectionResourceSize,

	DaemonSave,
	DaemonRemoveCache,
	DaemonShutdown,

	// State retrieval.
	StateAudio,
	StateConfig,
	StateDaemon,
	StateIp,
	StateQueue,
	StateQueueEntry,
	StateVolume,

	// Key (exact key)
	KeyArtist,
	KeyAlbum,
	KeySong,
	KeyEntry,
	KeyArtistAlbums,
	KeyArtistSongs,
	KeyArtistEntries,
	KeyAlbumArtist,
	KeyAlbumSongs,
	KeyAlbumEntries,
	KeySongArtist,
	KeySongAlbum,
	KeyOtherAlbums,
	KeyOtherSongs,
	KeyOtherEntries,

	// Map (exact hashmap)
	MapArtist,
	MapAlbum,
	MapSong,
	MapEntry,
	MapArtistAlbums,
	MapArtistSongs,
	MapArtistEntries,
	MapAlbumSongs,
	MapAlbumEntries,

	// Current (audio state)
	CurrentArtist,
	CurrentAlbum,
	CurrentSong,
	CurrentEntry,

	// Rand (audio state)
	RandArtist,
	RandAlbum,
	RandSong,
	RandEntry,

	// Search (fuzzy keys)
	Search,
	SearchArtist,
	SearchAlbum,
	SearchSong,
	SearchEntry,

	// Playback control.
	Toggle,
	Play,
	Pause,
	Next,
	Stop,
	Previous,
	Clear,
	Seek,
	Skip,
	Back,
	Shuffle,
	Repeat,
	Volume,
	VolumeUp,
	VolumeDown,

	QueueAddKeyArtist,
	QueueAddKeyAlbum,
	QueueAddKeySong,
	QueueAddMapArtist,
	QueueAddMapAlbum,
	QueueAddMapSong,
	QueueAddRandArtist,
	QueueAddRandAlbum,
	QueueAddRandSong,
	QueueAddRandEntry,
	QueueAddPlaylist,
	QueueSetIndex,
	QueueRemoveRange,

	// Playlists.
	PlaylistNew,
	PlaylistRemove,
	PlaylistClone,
	PlaylistRemoveEntry,
	PlaylistAddKeyArtist,
	PlaylistAddKeyAlbum,
	PlaylistAddKeySong,
	PlaylistAddMapArtist,
	PlaylistAddMapAlbum,
	PlaylistAddMapSong,
	PlaylistSingle,
	PlaylistBrief,
	PlaylistFull,
}

impl Method {
	/// Print each method, separated by a newline.
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

	DaemonSave(crate::param::DaemonSave),
	DaemonRemoveCache(crate::param::DaemonRemoveCache),
	DaemonShutdown(crate::param::DaemonShutdown),

	StateAudio(crate::param::StateAudio),
	StateConfig(crate::param::StateConfig),
	StateDaemon(crate::param::StateDaemon),
	StateIp(crate::param::StateIp),
	StateQueue(crate::param::StateQueue),
	StateQueueEntry(crate::param::StateQueueEntry),
	StateVolume(crate::param::StateVolume),

	KeyArtist(crate::param::KeyArtist),
	KeyAlbum(crate::param::KeyAlbum),
	KeySong(crate::param::KeySong),
	KeyEntry(crate::param::KeyEntry),
	KeyArtistAlbums(crate::param::KeyArtistAlbums),
	KeyArtistSongs(crate::param::KeyArtistSongs),
	KeyArtistEntries(crate::param::KeyArtistEntries),
	KeyAlbumArtist(crate::param::KeyAlbumArtist),
	KeyAlbumSongs(crate::param::KeyAlbumSongs),
	KeyAlbumEntries(crate::param::KeyAlbumEntries),
	KeySongArtist(crate::param::KeySongArtist),
	KeySongAlbum(crate::param::KeySongAlbum),
	KeyOtherAlbums(crate::param::KeyOtherAlbums),
	KeyOtherSongs(crate::param::KeyOtherSongs),
	KeyOtherEntries(crate::param::KeyOtherEntries),

	MapArtist(crate::param::MapArtistOwned),
	MapAlbum(crate::param::MapAlbumOwned),
	MapSong(crate::param::MapSongOwned),
	MapEntry(crate::param::MapEntryOwned),
	MapArtistAlbums(crate::param::MapArtistAlbumsOwned),
	MapArtistSongs(crate::param::MapArtistSongsOwned),
	MapArtistEntries(crate::param::MapArtistEntriesOwned),
	MapAlbumSongs(crate::param::MapAlbumSongsOwned),
	MapAlbumEntries(crate::param::MapAlbumEntriesOwned),

	CurrentArtist(crate::param::CurrentArtist),
	CurrentAlbum(crate::param::CurrentAlbum),
	CurrentSong(crate::param::CurrentSong),
	CurrentEntry(crate::param::CurrentEntry),

	RandArtist(crate::param::RandArtist),
	RandAlbum(crate::param::RandAlbum),
	RandSong(crate::param::RandSong),
	RandEntry(crate::param::RandEntry),

	Search(crate::param::SearchOwned),
	SearchArtist(crate::param::SearchArtistOwned),
	SearchAlbum(crate::param::SearchAlbumOwned),
	SearchSong(crate::param::SearchSongOwned),
	SearchEntry(crate::param::SearchEntryOwned),

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
	Repeat(crate::param::Repeat),
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
	QueueAddRandEntry(crate::param::QueueAddRandEntry),
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
