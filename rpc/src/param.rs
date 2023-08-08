//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use std::path::PathBuf;
use std::borrow::Cow;
use shukusai::{
	collection::{SongKey,AlbumKey,ArtistKey},
	audio::Append2,
	search::SearchKind,
};

use crate::{
	method::Method,
	impl_struct,
	impl_struct_lt,
	impl_rpc,
	impl_rpc_param,
};

//---------------------------------------------------------------------------------------------------- Collection
impl_rpc_param! {
	"Create a new Collection (and replace the current one)",
	"collection/collection_new",
	CollectionNew => Method::CollectionNew,
	"Filesystem PATH(s) `festivald`, to use multiple PATHs, use this flag per PATH",
	paths: Option<Vec<PathBuf>>
}
impl_rpc! {
	"Retrieve some brief metadata about the current Collection",
	"collection/collection_brief",
	CollectionBrief => Method::CollectionBrief
}
impl_rpc! {
	"Retrieve full metadata about the current Collection",
	"collection/collection_full",
	CollectionFull => Method::CollectionFull
}
impl_rpc! {
	"Retrieve an array of every Song in the current Collection, with its relational data",
	"collection/collection_relation",
	CollectionRelation => Method::CollectionRelation
}
impl_rpc! {
	"Retrieve an array of every Song in the current Collection, with its relational data",
	"collection/collection_relation_full",
	CollectionRelationFull => Method::CollectionRelationFull
}
impl_rpc! {
	"View some performance stats about the latest Collection construction",
	"collection/collection_perf",
	CollectionPerf => Method::CollectionPerf
}
impl_rpc! {
	"View the size of the current Collection's underlying resources (audio files and art)",
	"collection/collection_resource_size",
	CollectionResourceSize => Method::CollectionResourceSize
}

//---------------------------------------------------------------------------------------------------- State
impl_rpc! {
	"Retrieve state about the status of festivald itself",
	"state/state_daemon",
	StateDaemon => Method::StateDaemon
}
impl_rpc! {
	"Retrieve audio state",
	"state/state_audio",
	StateAudio => Method::StateAudio
}
impl_rpc! {
	"Retrieve the current state of a Collection reset",
	"state/state_reset",
	StateReset => Method::StateReset
}
impl_rpc! {
	"Retrieve the active configuration of festivald",
	"state/state_config",
	StateConfig => Method::StateConfig
}
impl_rpc! {
	"Retrieve an array of the IP addresses festivald has seen",
	"state/state_ip",
	StateIp => Method::StateIp
}

//---------------------------------------------------------------------------------------------------- Key
impl_rpc_param! {
	"Input an Artist key, retrieve an Artist",
	"key/key_artist",
	KeyArtist => Method::KeyArtist,
	"Artist key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Album key, retrieve an Album",
	"key/key_album",
	KeyAlbum => Method::KeyAlbum,
	"Album key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input a Song key, retrieve an Song",
	"key/key_song",
	KeySong => Method::KeySong,
	"Song key (unsigned integer)",
	key: usize
}

//---------------------------------------------------------------------------------------------------- Map
// `clap` + `lifetimes` == super fun macro error hell, so define 2 types, one borrowed (for no-copy deserialization), one owned (for clap).
impl_struct_lt!(MapArtist, #[serde(borrow)] artist: Cow<'a, str>);
impl_rpc_param! {
	"Input an Artist name, retrieve an Artist",
	"map/map_artist",
	MapArtistOwned => Method::MapArtist,
	"Artist name",
	artist: String
}
impl_struct_lt!(MapAlbum, #[serde(borrow)] artist: Cow<'a, str>, #[serde(borrow)] album: Cow<'a, str>);
impl_rpc_param! {
	"Input an Artist name and Album title, retrieve an Album",
	"map/map_album",
	MapAlbumOwned => Method::MapAlbum,
	"Artist name",
	artist: String,
	"Album title",
	album: String
}
impl_struct_lt!(MapSong, #[serde(borrow)] artist: Cow<'a, str>, #[serde(borrow)] album: Cow<'a, str>, #[serde(borrow)] song: Cow<'a, str>);
impl_rpc_param! {
	"Input an Artist name, Album title, and Song title, retrieve a Song",
	"map/map_song",
	MapSongOwned => Method::MapSong,
	"Artist name",
	artist: String,
	"Album title",
	album: String,
	"Song title",
	song: String
}

//---------------------------------------------------------------------------------------------------- Current
impl_rpc! {
	"Access the Artist of the currently set Song",
	"current/current_artist",
	CurrentArtist => Method::CurrentArtist
}
impl_rpc! {
	"Access the Album of the currently set Song",
	"current/current_album",
	CurrentAlbum => Method::CurrentAlbum
}
impl_rpc! {
	"Access the currently set Song",
	"current/current_song",
	CurrentSong => Method::CurrentSong
}

//---------------------------------------------------------------------------------------------------- Rand
impl_rpc! {
	"Access a random Artist",
	"rand/rand_artist",
	RandArtist => Method::RandArtist
}
impl_rpc! {
	"Access a random Album",
	"rand/rand_album",
	RandAlbum => Method::RandAlbum
}
impl_rpc! {
	"Access a random Song",
	"rand/rand_song",
	RandSong => Method::RandSong
}

//---------------------------------------------------------------------------------------------------- Search
impl_struct_lt!(Search, #[serde(borrow)] input: Cow<'a, str>, kind: SearchKind);
impl_rpc_param! {
	"Input a string, retrieve arrays of Artist's, Album's, and Song's, sorted by how similar their names/titles are to the input",
	"search/search",
	SearchOwned => Method::Search,
	"The string to match against, to use as input",
	input: String,
	"Type of search",
	#[arg(value_name = "ALL|SIM70|TOP25|TOP1")]
	kind: SearchKind
}
impl_struct_lt!(SearchArtist, #[serde(borrow)] input: Cow<'a, str>, kind: SearchKind);
impl_rpc_param! {
	"Input a string, retrieve an array of Artist's, sorted by how similar their names are to the input",
	"search/search_artist",
	SearchArtistOwned => Method::SearchArtist,
	"The string to match against, to use as input",
	input: String,
	"Type of search",
	#[arg(value_name = "ALL|SIM70|TOP25|TOP1")]
	kind: SearchKind
}
impl_struct_lt!(SearchAlbum, #[serde(borrow)] input: Cow<'a, str>, kind: SearchKind);
impl_rpc_param! {
	"Input a string, retrieve an array of Album's, sorted by how similar their titles are to the input",
	"search/search_album",
	SearchAlbumOwned => Method::SearchAlbum,
	"The string to match against, to use as input",
	input: String,
	"Type of search",
	#[arg(value_name = "ALL|SIM70|TOP25|TOP1")]
	kind: SearchKind
}
impl_struct_lt!(SearchSong, #[serde(borrow)] input: Cow<'a, str>, kind: SearchKind);
impl_rpc_param! {
	"Input a string, retrieve an array of Song's, sorted by how similar their titles are to the input",
	"search/search_song",
	SearchSongOwned => Method::SearchSong,
	"The string to match against, to use as input",
	input: String,
	"Type of search",
	#[arg(value_name = "ALL|SIM70|TOP25|TOP1")]
	kind: SearchKind
}

//---------------------------------------------------------------------------------------------------- Playback
impl_rpc! {
	"Toggle playback",
	"playback/toggle",
	Toggle => Method::Toggle
}
impl_rpc! {
	"Start playback",
	"playback/play",
	Play => Method::Play
}
impl_rpc! {
	"Pause playback",
	"playback/pause",
	Pause => Method::Pause
}
impl_rpc! {
	"Skip to the next song in the queue",
	"playback/next",
	Next => Method::Next
}
impl_rpc! {
	"Clear the queue and stop playback",
	"playback/stop",
	Stop => Method::Stop
}
impl_rpc! {
	"Shuffle the current queue, then start playing from the 1st Song in the queue",
	"playback/shuffle",
	Shuffle => Method::Shuffle
}
impl_rpc! {
	"Turn off repeating",
	"playback/repeat_off",
	RepeatOff => Method::RepeatOff
}
impl_rpc! {
	"Turn on song repeating",
	"playback/repeat_song",
	RepeatSong => Method::RepeatSong
}
impl_rpc! {
	"Turn on queue repeating",
	"playback/repeat_queue",
	RepeatQueue => Method::RepeatQueue
}
impl_rpc_param! {
	"Set the current Song to the previous in the queue",
	"playback/previous",
	Previous => Method::Previous,
	"Reset current Song if the current Song runtime (seconds) has passed this number",
	#[arg(value_name = "SECONDS")]
	threshold: Option<u32>
}
impl_rpc_param! {
	"Set the playback volume",
	"playback/volume",
	Volume => Method::Volume,
	"The volume % to set. Must be in-between 0..100.",
	#[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
	volume: u8
}
impl_rpc_param! {
	"Clear the queue",
	"playback/clear",
	Clear => Method::Clear,
	"If there is a Song currently set and playing, this flag continues playback",
	playback: bool
}
impl_rpc_param! {
	"Seek forwards/backwards or to an absolute second in the current Song",
	"playback/seek",
	Seek => Method::Seek,
	r#"The "type" of seeking we should do"#,
	#[arg(value_name = "FORWARD|BACKWARD|ABSOLUTE")]
	seek: shukusai::audio::Seek,
	"The second to seek forward/backwards/to",
	second: u64
}
impl_rpc_param! {
	"Skip forwards a variable amount of Song's in the current queue",
	"playback/skip",
	Skip => Method::Skip,
	"How many Song's to skip",
	skip: usize
}
impl_rpc_param! {
	"Go backwards a variable amount of Song's in the current queue",
	"playback/back",
	Back => Method::Back,
	"How many Song's to go backwards",
	back: usize
}

//---------------------------------------------------------------------------------------------------- Queue
impl_rpc_param! {
	"Add an Artist to the queue with an Artist key",
	"queue/queue_add_key_artist",
	QueueAddKeyArtist => Method::QueueAddKeyArtist,
	"Artist key",
	key: usize,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Artist?",
	offset: Option<usize>,
	"Should the queue be cleared before adding?",
	clear: bool
}
impl_rpc_param! {
	"Add an Album to the queue with an Album key",
	"queue/queue_add_key_album",
	QueueAddKeyAlbum => Method::QueueAddKeyAlbum,
	"Album key",
	key: usize,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Album?",
	offset: Option<usize>,
	"Should the queue be cleared before adding?",
	clear: bool
}
impl_rpc_param! {
	"Add an Song to the queue with an Song key",
	"queue/queue_add_key_song",
	QueueAddKeySong => Method::QueueAddKeySong,
	"Song key",
	key: usize,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should the queue be cleared before adding?",
	clear: bool
}
impl_struct_lt!(QueueAddMapArtist, #[serde(borrow)] artist: Cow<'a, str>, append: Append2, index: Option<usize>, offset: Option<usize>, clear: bool);
impl_struct_lt!(QueueAddMapAlbum, #[serde(borrow)] artist: Cow<'a, str>, #[serde(borrow)] album: Cow<'a, str>, append: Append2, index: Option<usize>, offset: Option<usize>, clear: bool);
impl_struct_lt!(QueueAddMapSong, #[serde(borrow)] artist: Cow<'a, str>, #[serde(borrow)] album: Cow<'a, str>, #[serde(borrow)] song: Cow<'a, str>, append: Append2, index: Option<usize>, clear: bool);
impl_rpc_param! {
	"Add an Artist to the queue with an Artist name",
	"queue/queue_add_map_artist",
	QueueAddMapArtistOwned => Method::QueueAddMapArtist,
	"Artist name",
	artist: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Artist?",
	offset: Option<usize>,
	"Should the queue be cleared before adding?",
	clear: bool
}
impl_rpc_param! {
	"Add an Album to the queue with an Artist name and Album title",
	"queue/queue_add_map_album",
	QueueAddMapAlbumOwned => Method::QueueAddMapAlbum,
	"Artist name",
	artist: String,
	"Album title",
	album: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Album?",
	offset: Option<usize>,
	"Should the queue be cleared before adding?",
	clear: bool
}
impl_rpc_param! {
	"Add a Song to the queue with an Artist name Album title, and Song title",
	"queue/queue_add_map_song",
	QueueAddMapSongOwned => Method::QueueAddMapSong,
	"Artist name",
	artist: String,
	"Album title",
	album: String,
	"Song title",
	song: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should the queue be cleared before adding?",
	clear: bool
}
impl_rpc_param! {
	"Add a random Artist to the queue",
	"queue/queue_add_rand_artist",
	QueueAddRandArtist => Method::QueueAddRandArtist,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Artist?",
	offset: Option<usize>,
	"Should the queue be cleared before adding?",
	clear: bool
}
impl_rpc_param! {
	"Add a random Album to the queue",
	"queue/queue_add_rand_album",
	QueueAddRandAlbum => Method::QueueAddRandAlbum,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Album?",
	offset: Option<usize>,
	"Should the queue be cleared before adding?",
	clear: bool
}
impl_rpc_param! {
	"Add a random Song to the queue",
	"queue/queue_add_rand_song",
	QueueAddRandSong => Method::QueueAddRandSong,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should the queue be cleared before adding?",
	clear: bool
}
impl_struct_lt!(QueueAddPlaylist, #[serde(borrow)] playlist: Cow<'a, str>, append: Append2, index: Option<usize>, clear: bool);
impl_rpc_param! {
	"Add a playlist to the queue",
	"queue/queue_add_playlist",
	QueueAddPlaylistOwned => Method::QueueAddPlaylist,
	"The name of the playlist",
	playlist: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should the queue be cleared before adding?",
	clear: bool
}
impl_rpc_param! {
	"Set the current Song to a queue index",
	"queue/queue_set_index",
	QueueSetIndex => Method::QueueSetIndex,
	"An index in the queue (1st Song is index 0, 2nd Song is index 1, etc)",
	index: usize
}
impl_rpc_param! {
	"Remove a range of queue indices",
	"queue/queue_remove_range",
	QueueRemoveRange => Method::QueueRemoveRange,
	"The beginning index to start removing from",
	start: usize,
	"The index to stop at",
	end: usize,
	"This flag will skip to the next song if the range includes the current one",
	skip: bool
}

//---------------------------------------------------------------------------------------------------- Playlists
impl_struct_lt!(PlaylistNew, #[serde(borrow)] playlist: Cow<'a, str>);
impl_rpc_param! {
	"Create a new empty playlist",
	"playlist/playlist_new",
	PlaylistNewOwned => Method::PlaylistNew,
	"The name of the new playlist",
	playlist: String
}
impl_struct_lt!(PlaylistRemove, #[serde(borrow)] name: Cow<'a, str>);
impl_rpc_param! {
	"Remove a playlist",
	"playlist/playlist_remove",
	PlaylistRemoveOwned => Method::PlaylistRemove,
	"The name of the playlist to remove",
	playlist: String
}
impl_struct_lt!(PlaylistClone, #[serde(borrow)] from: Cow<'a, str>, to: Cow<'a, str>);
impl_rpc_param! {
	"Clone a playlist into a new one",
	"playlist/playlist_clone",
	PlaylistCloneOwned => Method::PlaylistClone,
	"The name of the playlist to clone",
	from: String,
	"The name of the new playlist",
	to: String
}
impl_struct_lt!(PlaylistRemoveSong, #[serde(borrow)] playlist: Cow<'a, str>, index: usize);
impl_rpc_param! {
	"Remove a song in a playlist",
	"playlist/playlist_remove_song",
	PlaylistRemoveSongOwned => Method::PlaylistRemoveSong,
	"The name of the playlist",
	playlist: String,
	"The index of the song in the playlist",
	index: usize
}
impl_struct_lt!(PlaylistAddArtist, #[serde(borrow)] playlist: Cow<'a, str>, artist: Cow<'a, str>);
impl_rpc_param! {
	"Add an artist to a playlist",
	"playlist/playlist_add_artist",
	PlaylistAddArtistOwned => Method::PlaylistAddArtist,
	"The name of the playlist",
	playlist: String,
	"The name of the artist",
	artist: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_struct_lt!(PlaylistAddAlbum, #[serde(borrow)] playlist: Cow<'a, str>, artist: Cow<'a, str>, album: Cow<'a, str>);
impl_rpc_param! {
	"Add an album to a playlist",
	"playlist/playlist_add_album",
	PlaylistAddAlbumOwned => Method::PlaylistAddAlbum,
	"The name of the playlist",
	playlist: String,
	"The name of the artist",
	artist: String,
	"The name of the album",
	album: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_struct_lt!(PlaylistAddSong, #[serde(borrow)] playlist: Cow<'a, str>, artist: Cow<'a, str>, album: Cow<'a, str>, song: Cow<'a, str>);
impl_rpc_param! {
	"Add a song to a playlist",
	"playlist/playlist_add_song",
	PlaylistAddSongOwned => Method::PlaylistAddSong,
	"The name of the playlist",
	playlist: String,
	"The name of the artist",
	artist: String,
	"The name of the album",
	album: String,
	"The name of the song",
	song: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "FRONT|BACK|INDEX")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_rpc! {
	"Retrieve all playlist names",
	"playlist/playlist_names",
	PlaylistNames => Method::PlaylistNames
}
impl_rpc! {
	"Retrieve how many playlists there are",
	"playlist/playlist_count",
	PlaylistCount => Method::PlaylistCount
}
impl_struct_lt!(PlaylistSingle, #[serde(borrow)] playlist: Cow<'a, str>);
impl_rpc_param! {
	"Retrieve a single playlist",
	"playlist/playlist_single",
	PlaylistSingleOwned => Method::PlaylistSingle,
	"The name of the playlist",
	playlist: String
}
impl_rpc! {
	"Retrieve all playlists",
	"playlist/playlist_all",
	PlaylistAll => Method::PlaylistAll
}

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
		t(&QueueAddSong { key: SongKey::from(0_u8), append: shukusai::audio::Append::Front, clear: true }, r#"{"key":0,"append":"front","clear":true}"#);
		t(&QueueAddSong { key: SongKey::from(1_u8), append: shukusai::audio::Append::Back, clear: false }, r#"{"key":1,"append":"back","clear":false}"#);
		t(&QueueAddSong { key: SongKey::from(2_u8), append: shukusai::audio::Append::Index(0), clear: true }, r#"{"key":2,"append":{"index":0},"clear":true}"#);
	}

	#[test]
	fn add_queue_album() {
		t(&QueueAddAlbum { key: AlbumKey::from(0_u8), append: shukusai::audio::Append::Front, clear: true, offset: 0 }, r#"{"key":0,"append":"front","clear":true,"offset":0}"#);
		t(&QueueAddAlbum { key: AlbumKey::from(1_u8), append: shukusai::audio::Append::Back, clear: false, offset: 1 }, r#"{"key":1,"append":"back","clear":false,"offset":1}"#);
		t(&QueueAddAlbum { key: AlbumKey::from(2_u8), append: shukusai::audio::Append::Index(0), clear: true, offset: 2 }, r#"{"key":2,"append":{"index":0},"clear":true,"offset":2}"#);
	}

	#[test]
	fn add_queue_artist() {
		t(&QueueAddArtist { key: ArtistKey::from(0_u8), append: shukusai::audio::Append::Front, clear: true, offset: 0 }, r#"{"key":0,"append":"front","clear":true,"offset":0}"#);
		t(&QueueAddArtist { key: ArtistKey::from(1_u8), append: shukusai::audio::Append::Back, clear: false, offset: 1 }, r#"{"key":1,"append":"back","clear":false,"offset":1}"#);
		t(&QueueAddArtist { key: ArtistKey::from(2_u8), append: shukusai::audio::Append::Index(0), clear: true, offset: 2 }, r#"{"key":2,"append":{"index":0},"clear":true,"offset":2}"#);
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
		t(&Search { input: "hello4".into(), kind: shukusai::search::SearchKind::Top1 }, r#"{"input":"hello4","kind":"top1"}"#);
	}

	#[test]
	fn search_artist() {
		t(&SearchArtist { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(&SearchArtist { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(&SearchArtist { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
		t(&SearchArtist { input: "hello4".into(), kind: shukusai::search::SearchKind::Top1 }, r#"{"input":"hello4","kind":"top1"}"#);
	}

	#[test]
	fn search_album() {
		t(&SearchAlbum { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(&SearchAlbum { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(&SearchAlbum { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
		t(&SearchAlbum { input: "hello4".into(), kind: shukusai::search::SearchKind::Top1 }, r#"{"input":"hello4","kind":"top1"}"#);
	}

	#[test]
	fn search_song() {
		t(&SearchSong { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(&SearchSong { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(&SearchSong { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
		t(&SearchSong { input: "hello4".into(), kind: shukusai::search::SearchKind::Top1 }, r#"{"input":"hello4","kind":"top1"}"#);
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
		t(&CollectionNew { paths: vec![PathBuf::from("/path_1"), PathBuf::from("/path_2")] }, r#"{"paths":["/path_1","/path_2"]}"#);
	}
}
