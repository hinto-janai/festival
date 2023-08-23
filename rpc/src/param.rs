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
	"Retrieve an array of every Artist name in the current Collection",
	"collection/collection_brief_artists",
	CollectionBriefArtists => Method::CollectionBriefArtists
}
impl_rpc! {
	"Retrieve an array of every Album title in the current Collection",
	"collection/collection_brief_albums",
	CollectionBriefAlbums => Method::CollectionBriefAlbums
}
impl_rpc! {
	"Retrieve an array of every Song title in the current Collection",
	"collection/collection_brief_songs",
	CollectionBriefSongs => Method::CollectionBriefSongs
}
impl_rpc! {
	"Retrieve an array of every Artist in the current Collection",
	"collection/collection_full_artists",
	CollectionFullArtists => Method::CollectionFullArtists
}
impl_rpc! {
	"Retrieve an array of every Album in the current Collection",
	"collection/collection_full_albums",
	CollectionFullAlbums => Method::CollectionFullAlbums
}
impl_rpc! {
	"Retrieve an array of every Song in the current Collection",
	"collection/collection_full_songs",
	CollectionFullSongs => Method::CollectionFullSongs
}
impl_rpc! {
	"Retrieve an array of every Song in the current Collection, with its relational data",
	"collection/collection_entries",
	CollectionEntries => Method::CollectionEntries
}
impl_rpc! {
	"View some performance stats about the latest Collection construction",
	"collection/collection_perf",
	CollectionPerf => Method::CollectionPerf
}
impl_rpc! {
	"View the health of the Collection (underlying files)",
	"collection/collection_health",
	CollectionHealth => Method::CollectionHealth
}
impl_rpc! {
	"View the size of the current Collection's underlying resources (audio files and art)",
	"collection/collection_resource_size",
	CollectionResourceSize => Method::CollectionResourceSize
}

//---------------------------------------------------------------------------------------------------- Daemon
impl_rpc! {
	"Retrieve the active configuration of `festivald`",
	"daemon/daemon_config",
	DaemonConfig => Method::DaemonConfig
}
impl_rpc! {
	"Retrieve all JSON-RPC methods this `festivald` knows about",
	"daemon/daemon_methods",
	DaemonMethods => Method::DaemonMethods
}
impl_rpc! {
	"Retrieve all no_auth_rpc JSON-RPC methods this `festivald` allows",
	"daemon/daemon_no_auth_rpc",
	DaemonNoAuthRpc => Method::DaemonNoAuthRpc
}
impl_rpc! {
	"Retrieve all no_auth_rest REST resources this `festivald` allows",
	"daemon/daemon_no_auth_rest",
	DaemonNoAuthRest => Method::DaemonNoAuthRest
}
impl_rpc! {
	"Remove `festivald` cache from disk",
	"daemon/daemon_remove_cache",
	DaemonRemoveCache => Method::DaemonRemoveCache
}
impl_rpc! {
	"Save `festivald` data to disk",
	"daemon/daemon_save",
	DaemonSave => Method::DaemonSave
}
impl_rpc! {
	"Retrieve an array of the IP addresses `festivald` has seen",
	"daemon/daemon_seen_ips",
	DaemonSeenIps => Method::DaemonSeenIps
}
impl_rpc! {
	"Shutdown `festivald`",
	"daemon/daemon_shutdown",
	DaemonShutdown => Method::DaemonShutdown
}
impl_rpc! {
	"Retrieve brief state of `festivald`",
	"daemon/daemon_state",
	DaemonState => Method::DaemonState
}

//---------------------------------------------------------------------------------------------------- State
impl_rpc! {
	"Retrieve audio state",
	"state/state_audio",
	StateAudio => Method::StateAudio
}
impl_rpc! {
	"Retrieve state of the queue as Keys",
	"state/state_queue_key",
	StateQueueKey => Method::StateQueueKey
}
impl_rpc! {
	"Retrieve state of the queue as Song objects",
	"state/state_queue_song",
	StateQueueSong => Method::StateQueueSong
}
impl_rpc! {
	"Retrieve state of the queue as Entry objects",
	"state/state_queue_entry",
	StateQueueEntry => Method::StateQueueEntry
}
impl_rpc! {
	"Retrieve the current playback status",
	"state/state_playing",
	StatePlaying => Method::StatePlaying
}
impl_rpc! {
	"Retrieve the currently set Repeat mode",
	"state/state_repeat",
	StateRepeat => Method::StateRepeat
}
impl_rpc! {
	"Retrieve the elapsed runtime & total runtime of the currently set Song",
	"state/state_runtime",
	StateRuntime => Method::StateRuntime
}
impl_rpc! {
	"Retrieve the current volume level",
	"state/state_volume",
	StateVolume => Method::StateVolume
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
	"Input a Song key, retrieve a Song",
	"key/key_song",
	KeySong => Method::KeySong,
	"Song key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input a Song key, retrieve an Entry",
	"key/key_entry",
	KeyEntry => Method::KeyEntry,
	"Song key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Artist key, retrieve all their Albums",
	"key/key_artist_albums",
	KeyArtistAlbums => Method::KeyArtistAlbums,
	"Artist key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Artist key, retrieve all their Songs",
	"key/key_artist_songs",
	KeyArtistSongs => Method::KeyArtistSongs,
	"Artist key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Artist key, retrieve all their Songs in Entry form",
	"key/key_artist_entries",
	KeyArtistEntries => Method::KeyArtistEntries,
	"Artist key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Album key, retrieve its Artist",
	"key/key_album_artist",
	KeyAlbumArtist => Method::KeyAlbumArtist,
	"Album key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Album key, retrieve all its Songs",
	"key/key_album_songs",
	KeyAlbumSongs => Method::KeyAlbumSongs,
	"Album key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Album key, retrieve all its Songs in Entry form",
	"key/key_album_entries",
	KeyAlbumEntries => Method::KeyAlbumEntries,
	"Album key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Song key, retrieve its Artist",
	"key/key_song_artist",
	KeySongArtist => Method::KeySongArtist,
	"Song key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Song key, retrieve its Album",
	"key/key_song_album",
	KeySongAlbum => Method::KeySongAlbum,
	"Song key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Album key, retrieve all Albums by the same Artist",
	"key/key_other_albums",
	KeyOtherAlbums => Method::KeyOtherAlbums,
	"Album key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Song key, retrieve all Songs by the same Artist",
	"key/key_other_songs",
	KeyOtherSongs => Method::KeyOtherSongs,
	"Song key (unsigned integer)",
	key: usize
}
impl_rpc_param! {
	"Input an Song key, retrieve all Songs by the same Artist in Entry form",
	"key/key_other_entries",
	KeyOtherEntries => Method::KeyOtherEntries,
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
impl_struct_lt!(MapEntry, #[serde(borrow)] artist: Cow<'a, str>, #[serde(borrow)] album: Cow<'a, str>, #[serde(borrow)] song: Cow<'a, str>);
impl_rpc_param! {
	"Input an Artist name, Album title, and Song title, retrieve an Entry",
	"map/map_entry",
	MapEntryOwned => Method::MapEntry,
	"Artist name",
	artist: String,
	"Album title",
	album: String,
	"Song title",
	song: String
}
impl_struct_lt!(MapArtistAlbums, #[serde(borrow)] artist: Cow<'a, str>);
impl_rpc_param! {
	"Input an Artist name, retrieve all their Albums",
	"map/map_artist_albums",
	MapArtistAlbumsOwned => Method::MapArtistAlbums,
	"Artist name",
	artist: String
}
impl_struct_lt!(MapArtistSongs, #[serde(borrow)] artist: Cow<'a, str>);
impl_rpc_param! {
	"Input an Artist name, retrieve all their Songs",
	"map/map_artist_songs",
	MapArtistSongsOwned => Method::MapArtistSongs,
	"Artist name",
	artist: String
}
impl_struct_lt!(MapArtistEntries, #[serde(borrow)] artist: Cow<'a, str>);
impl_rpc_param! {
	"Input an Artist name, retrieve all their Songs in Entry form",
	"map/map_artist_entries",
	MapArtistEntriesOwned => Method::MapArtistEntries,
	"Artist name",
	artist: String
}
impl_struct_lt!(MapAlbumSongs, #[serde(borrow)] artist: Cow<'a, str>, #[serde(borrow)] album: Cow<'a, str>);
impl_rpc_param! {
	"Input an Artist name and Album title, retrieve all its Songs",
	"map/map_album_songs",
	MapAlbumSongsOwned => Method::MapAlbumSongs,
	"Artist name",
	artist: String,
	"Album title",
	album: String
}
impl_struct_lt!(MapAlbumEntries, #[serde(borrow)] artist: Cow<'a, str>, #[serde(borrow)] album: Cow<'a, str>);
impl_rpc_param! {
	"Input an Artist name and Album title, retrieve all its Songs in Entry form",
	"map/map_album_entries",
	MapAlbumEntriesOwned => Method::MapAlbumEntries,
	"Artist name",
	artist: String,
	"Album title",
	album: String
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
impl_rpc! {
	"Access the currently set Song, as an Entry",
	"current/current_entry",
	CurrentEntry => Method::CurrentEntry
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
impl_rpc! {
	"Access a random Song, as an Entry",
	"rand/rand_entry",
	RandEntry => Method::RandEntry
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
	#[arg(value_name = "all|sim60|sim70|sim80|top25|top5|top1")]
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
	#[arg(value_name = "all|sim60|sim70|sim80|top25|top5|top1")]
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
	#[arg(value_name = "all|sim60|sim70|sim80|top25|top5|top1")]
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
	#[arg(value_name = "all|sim60|sim70|sim80|top25|top5|top1")]
	kind: SearchKind
}
impl_struct_lt!(SearchEntry, #[serde(borrow)] input: Cow<'a, str>, kind: SearchKind);
impl_rpc_param! {
	"Input a string, retrieve an array of Song's (in Entry form), sorted by how similar their titles are to the input",
	"search/search_entry",
	SearchEntryOwned => Method::SearchEntry,
	"The string to match against, to use as input",
	input: String,
	"Type of search",
	#[arg(value_name = "all|sim60|sim70|sim80|top25|top5|top1")]
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
impl_rpc_param! {
	"Set a repeat mode",
	"playback/repeat",
	Repeat => Method::Repeat,
	"The repeat mode to set.",
	#[arg(value_name = "off|song|queue")]
	mode: shukusai::audio::Repeat
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
	"Raise the playback volume",
	"playback/volume_up",
	VolumeUp => Method::VolumeUp,
	"The number to raise the volume by. Must be in-between 0..100.",
	#[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
	up: u8
}
impl_rpc_param! {
	"Lower the playback volume",
	"playback/volume_down",
	VolumeDown => Method::VolumeDown,
	"The number to lower the volume by. Must be in-between 0..100.",
	#[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
	down: u8
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
	#[arg(value_name = "forward|backward|absolute")]
	kind: shukusai::audio::Seek,
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
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Artist?",
	offset: Option<usize>
}
impl_rpc_param! {
	"Add an Album to the queue with an Album key",
	"queue/queue_add_key_album",
	QueueAddKeyAlbum => Method::QueueAddKeyAlbum,
	"Album key",
	key: usize,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Album?",
	offset: Option<usize>
}
impl_rpc_param! {
	"Add an Song to the queue with an Song key",
	"queue/queue_add_key_song",
	QueueAddKeySong => Method::QueueAddKeySong,
	"Song key",
	key: usize,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_struct_lt!(QueueAddMapArtist, #[serde(borrow)] artist: Cow<'a, str>, append: Append2, clear: bool, play: bool, index: Option<usize>, offset: Option<usize>);
impl_struct_lt!(QueueAddMapAlbum, #[serde(borrow)] artist: Cow<'a, str>, #[serde(borrow)] album: Cow<'a, str>, append: Append2, clear: bool, play: bool, index: Option<usize>, offset: Option<usize>);
impl_struct_lt!(QueueAddMapSong, #[serde(borrow)] artist: Cow<'a, str>, #[serde(borrow)] album: Cow<'a, str>, #[serde(borrow)] song: Cow<'a, str>, append: Append2, clear: bool, play: bool, index: Option<usize>);
impl_rpc_param! {
	"Add an Artist to the queue with an Artist name",
	"queue/queue_add_map_artist",
	QueueAddMapArtistOwned => Method::QueueAddMapArtist,
	"Artist name",
	artist: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Artist?",
	offset: Option<usize>
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
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Album?",
	offset: Option<usize>
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
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_rpc_param! {
	"Add a random Artist to the queue",
	"queue/queue_add_rand_artist",
	QueueAddRandArtist => Method::QueueAddRandArtist,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Artist?",
	offset: Option<usize>
}
impl_rpc_param! {
	"Add a random Album to the queue",
	"queue/queue_add_rand_album",
	QueueAddRandAlbum => Method::QueueAddRandAlbum,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the Album?",
	offset: Option<usize>
}
impl_rpc_param! {
	"Add a random Song to the queue",
	"queue/queue_add_rand_song",
	QueueAddRandSong => Method::QueueAddRandSong,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_rpc_param! {
	"Add a random Song to the queue, receive it back in Entry form",
	"queue/queue_add_rand_entry",
	QueueAddRandEntry => Method::QueueAddRandEntry,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_struct_lt!(QueueAddPlaylist, #[serde(borrow)] playlist: Cow<'a, str>, append: Append2, clear: bool, play: bool, index: Option<usize>, offset: Option<usize>);
impl_rpc_param! {
	"Add a playlist to the queue",
	"queue/queue_add_playlist",
	QueueAddPlaylistOwned => Method::QueueAddPlaylist,
	"The name of the playlist",
	playlist: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"Should the queue be cleared before adding?",
	clear: bool,
	"Should we start playing?",
	play: bool,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>,
	"Should we start at an offset within the playlist?",
	offset: Option<usize>
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
impl_struct_lt!(PlaylistRemove, #[serde(borrow)] playlist: Cow<'a, str>);
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
impl_struct_lt!(PlaylistGetIndex, #[serde(borrow)] playlist: Cow<'a, str>, index: usize);
impl_rpc_param! {
	"Get a Playlist Entry in a Playlist, using its index number",
	"playlist/playlist_get_index",
	PlaylistGetIndexOwned => Method::PlaylistGetIndex,
	"The name of the playlist",
	playlist: String,
	"The index of the entry in the playlist",
	index: usize
}
impl_struct_lt!(PlaylistRemoveIndex, #[serde(borrow)] playlist: Cow<'a, str>, index: usize);
impl_rpc_param! {
	"Remove a Playlist Entry in a Playlist, using its index number",
	"playlist/playlist_remove_index",
	PlaylistRemoveIndexOwned => Method::PlaylistRemoveIndex,
	"The name of the playlist",
	playlist: String,
	"The index of the entry in the playlist",
	index: usize
}
impl_struct_lt!(PlaylistAddKeyArtist, #[serde(borrow)] playlist: Cow<'a, str>, key: usize, append: Append2, index: Option<usize>);
impl_rpc_param! {
	"Add an artist to a playlist",
	"playlist/playlist_add_key_artist",
	PlaylistAddKeyArtistOwned => Method::PlaylistAddKeyArtist,
	"The name of the playlist",
	playlist: String,
	"The artist key",
	key: usize,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_struct_lt!(PlaylistAddKeyAlbum, #[serde(borrow)] playlist: Cow<'a, str>, key: usize, append: Append2, index: Option<usize>);
impl_rpc_param! {
	"Add an album to a playlist",
	"playlist/playlist_add_key_album",
	PlaylistAddKeyAlbumOwned => Method::PlaylistAddKeyAlbum,
	"The name of the playlist",
	playlist: String,
	"The album key",
	key: usize,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_struct_lt!(PlaylistAddKeySong, #[serde(borrow)] playlist: Cow<'a, str>, key: usize, append: Append2, index: Option<usize>);
impl_rpc_param! {
	"Add a song to a playlist",
	"playlist/playlist_add_key_song",
	PlaylistAddKeySongOwned => Method::PlaylistAddKeySong,
	"The name of the playlist",
	playlist: String,
	"The song key",
	key: usize,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_struct_lt!(PlaylistAddMapArtist, #[serde(borrow)] playlist: Cow<'a, str>, artist: Cow<'a, str>, append: Append2, index: Option<usize>);
impl_rpc_param! {
	"Add an artist to a playlist",
	"playlist/playlist_add_map_artist",
	PlaylistAddMapArtistOwned => Method::PlaylistAddMapArtist,
	"The name of the playlist",
	playlist: String,
	"The name of the artist",
	artist: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_struct_lt!(PlaylistAddMapAlbum, #[serde(borrow)] playlist: Cow<'a, str>, artist: Cow<'a, str>, album: Cow<'a, str>, append: Append2, index: Option<usize>);
impl_rpc_param! {
	"Add an album to a playlist",
	"playlist/playlist_add_map_album",
	PlaylistAddMapAlbumOwned => Method::PlaylistAddMapAlbum,
	"The name of the playlist",
	playlist: String,
	"The name of the artist",
	artist: String,
	"The name of the album",
	album: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
}
impl_struct_lt!(PlaylistAddMapSong, #[serde(borrow)] playlist: Cow<'a, str>, artist: Cow<'a, str>, album: Cow<'a, str>, song: Cow<'a, str>, append: Append2, index: Option<usize>);
impl_rpc_param! {
	"Add a song to a playlist",
	"playlist/playlist_add_map_song",
	PlaylistAddMapSongOwned => Method::PlaylistAddMapSong,
	"The name of the playlist",
	playlist: String,
	"The name of the artist",
	artist: String,
	"The name of the album",
	album: String,
	"The name of the song",
	song: String,
	"In which way should we add to the queue?",
	#[arg(value_name = "front|back|index")]
	append: Append2,
	"If the `index` append option was picked, this will be index used",
	index: Option<usize>
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
	"Retrieve all playlist names",
	"playlist/playlist_brief",
	PlaylistBrief => Method::PlaylistBrief
}
impl_rpc! {
	"Retrieve full data of all playlists",
	"playlist/playlist_full",
	PlaylistFull => Method::PlaylistFull
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	//------------------------------------- Serde sanity tests.
	// Testing function.
	fn t<T>(value: T, expected: &'static str)
		where
			T: Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
	{
		let string = serde_json::to_string(&value).unwrap();
		assert_eq!(string, expected);
		let t: T = serde_json::from_str(&string).unwrap();
		assert_eq!(t, value);
		let e: T = serde_json::from_str(expected).unwrap();
		assert_eq!(e, value);
	}

	//---------------------------------------------------------------------------------------------------- Collection
	#[test]
	fn collection_new() {
		t(CollectionNew { paths: vec![PathBuf::from("/path_1"), PathBuf::from("/path_2")].into() }, r#"{"paths":["/path_1","/path_2"]}"#);
	}

	//---------------------------------------------------------------------------------------------------- Key
	#[test]
	fn key_artist() {
		t(KeyArtist { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyArtist { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_album() {
		t(KeyAlbum { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyAlbum { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_song() {
		t(KeySong { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeySong { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_entry() {
		t(KeyEntry { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyEntry { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_artist_albums() {
		t(KeyArtistAlbums { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyArtistAlbums { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_artist_songs() {
		t(KeyArtistSongs { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyArtistSongs { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_artist_entries() {
		t(KeyArtistEntries { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyArtistEntries { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_album_artist() {
		t(KeyAlbumArtist { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyAlbumArtist { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_album_songs() {
		t(KeyAlbumSongs { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyAlbumSongs { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_album_entries() {
		t(KeyAlbumEntries { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyAlbumEntries { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_song_artist() {
		t(KeySongArtist { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeySongArtist { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_song_album() {
		t(KeySongAlbum { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeySongAlbum { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_other_albums() {
		t(KeyOtherAlbums { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyOtherAlbums { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_other_songs() {
		t(KeyOtherSongs { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyOtherSongs { key: 0 }, r#"{"key":0}"#);
	}

	#[test]
	fn key_other_entries() {
		t(KeyOtherEntries { key: usize::MAX }, r#"{"key":18446744073709551615}"#);
		t(KeyOtherEntries { key: 0 }, r#"{"key":0}"#);
	}

	//---------------------------------------------------------------------------------------------------- Map
	#[test]
	fn map_artist() {
		t(MapArtistOwned { artist: "hello".into() }, r#"{"artist":"hello"}"#);
	}

	#[test]
	fn map_album() {
		t(MapAlbumOwned { artist: "hello".into(), album: "hello2".into() }, r#"{"artist":"hello","album":"hello2"}"#);
	}

	#[test]
	fn map_song() {
		t(MapSongOwned { artist: "hello".into(), album: "hello2".into(), song: "hello3".into() }, r#"{"artist":"hello","album":"hello2","song":"hello3"}"#);
	}

	#[test]
	fn map_entry() {
		t(MapEntryOwned { artist: "hello".into(), album: "hello2".into(), song: "hello3".into() }, r#"{"artist":"hello","album":"hello2","song":"hello3"}"#);
	}

	#[test]
	fn map_artist_albums() {
		t(MapArtistAlbumsOwned { artist: "hello".into() }, r#"{"artist":"hello"}"#);
	}

	#[test]
	fn map_artist_songs() {
		t(MapArtistSongsOwned { artist: "hello".into() }, r#"{"artist":"hello"}"#);
	}

	#[test]
	fn map_artist_entries() {
		t(MapArtistEntriesOwned { artist: "hello".into() }, r#"{"artist":"hello"}"#);
	}

	#[test]
	fn map_album_songs() {
		t(MapAlbumSongsOwned { artist: "hello".into(), album: "hello2".into() }, r#"{"artist":"hello","album":"hello2"}"#);
	}

	#[test]
	fn map_album_entries() {
		t(MapAlbumEntriesOwned { artist: "hello".into(), album: "hello2".into() }, r#"{"artist":"hello","album":"hello2"}"#);
	}

	//---------------------------------------------------------------------------------------------------- Search
	#[test]
	fn search() {
		t(SearchOwned { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(SearchOwned { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(SearchOwned { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
		t(SearchOwned { input: "hello4".into(), kind: shukusai::search::SearchKind::Top1 }, r#"{"input":"hello4","kind":"top1"}"#);
	}

	#[test]
	fn search_artist() {
		t(SearchArtistOwned { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(SearchArtistOwned { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(SearchArtistOwned { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
		t(SearchArtistOwned { input: "hello4".into(), kind: shukusai::search::SearchKind::Top1 }, r#"{"input":"hello4","kind":"top1"}"#);
	}

	#[test]
	fn search_album() {
		t(SearchAlbumOwned { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(SearchAlbumOwned { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(SearchAlbumOwned { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
		t(SearchAlbumOwned { input: "hello4".into(), kind: shukusai::search::SearchKind::Top1 }, r#"{"input":"hello4","kind":"top1"}"#);
	}

	#[test]
	fn search_song() {
		t(SearchSongOwned { input: "hello1".into(), kind: shukusai::search::SearchKind::All }, r#"{"input":"hello1","kind":"all"}"#);
		t(SearchSongOwned { input: "hello2".into(), kind: shukusai::search::SearchKind::Sim70 }, r#"{"input":"hello2","kind":"sim70"}"#);
		t(SearchSongOwned { input: "hello3".into(), kind: shukusai::search::SearchKind::Top25 }, r#"{"input":"hello3","kind":"top25"}"#);
		t(SearchSongOwned { input: "hello4".into(), kind: shukusai::search::SearchKind::Top1 }, r#"{"input":"hello4","kind":"top1"}"#);
	}

	//---------------------------------------------------------------------------------------------------- Playback
	#[test]
	fn repeat() {
		t(Repeat { mode: shukusai::audio::Repeat::Off }, r#"{"mode":"off"}"#);
		t(Repeat { mode: shukusai::audio::Repeat::Song }, r#"{"mode":"song"}"#);
		t(Repeat { mode: shukusai::audio::Repeat::Queue }, r#"{"mode":"queue"}"#);
	}

	#[test]
	fn previous() {
		t(Previous { threshold: Some(u32::MAX) }, r#"{"threshold":4294967295}"#);
		t(Previous { threshold: Some(0) },        r#"{"threshold":0}"#);
		t(Previous { threshold: None },           r#"{"threshold":null}"#);
	}

	#[test]
	fn volume() {
		t(Volume { volume: u8::MAX }, r#"{"volume":255}"#);
		t(Volume { volume: 0 },       r#"{"volume":0}"#);
	}

	#[test]
	fn volume_up() {
		t(VolumeUp { up: u8::MAX }, r#"{"up":255}"#);
		t(VolumeUp { up: 0 },       r#"{"up":0}"#);
	}

	#[test]
	fn volume_down() {
		t(VolumeDown { down: u8::MAX }, r#"{"down":255}"#);
		t(VolumeDown { down: 0 },       r#"{"down":0}"#);
	}

	#[test]
	fn clear() {
		t(Clear { playback: true }, r#"{"playback":true}"#);
		t(Clear { playback: false }, r#"{"playback":false}"#);
	}

	#[test]
	fn seek() {
		t(Seek { kind: shukusai::audio::Seek::Forward, second: 0 }, r#"{"kind":"forward","second":0}"#);
		t(Seek { kind: shukusai::audio::Seek::Backward, second: 1 }, r#"{"kind":"backward","second":1}"#);
		t(Seek { kind: shukusai::audio::Seek::Absolute, second: u64::MAX }, r#"{"kind":"absolute","second":18446744073709551615}"#);
	}

	#[test]
	fn skip() {
		t(Skip { skip: usize::MAX }, r#"{"skip":18446744073709551615}"#);
		t(Skip { skip: 0 },          r#"{"skip":0}"#);
	}

	#[test]
	fn back() {
		t(Back { back: usize::MAX }, r#"{"back":18446744073709551615}"#);
		t(Back { back: 0 },          r#"{"back":0}"#);
	}

	//---------------------------------------------------------------------------------------------------- Queue
	#[test]
	fn queue_add_key_artist() {
		t(QueueAddKeyArtist { key: 0, append: shukusai::audio::Append2::Back, clear: false, play: false, index: None, offset: None },
			r#"{"key":0,"append":"back","clear":false,"play":false,"index":null,"offset":null}"#
		);
		t(QueueAddKeyArtist { key: 0, append: shukusai::audio::Append2::Back, clear: false, play: false, index: None, offset: Some(0) },
			r#"{"key":0,"append":"back","clear":false,"play":false,"index":null,"offset":0}"#
		);
		t(QueueAddKeyArtist { key: 0, append: shukusai::audio::Append2::Back, clear: true, play: true, index: None, offset: Some(1) },
			r#"{"key":0,"append":"back","clear":true,"play":true,"index":null,"offset":1}"#
		);
	}

	#[test]
	fn queue_add_key_album() {
		t(QueueAddKeyAlbum { key: 0, append: shukusai::audio::Append2::Back, clear: false, play: false, index: None, offset: None },
			r#"{"key":0,"append":"back","clear":false,"play":false,"index":null,"offset":null}"#
		);
		t(QueueAddKeyAlbum { key: 0, append: shukusai::audio::Append2::Back, clear: false, play: false, index: None, offset: Some(0) },
			r#"{"key":0,"append":"back","clear":false,"play":false,"index":null,"offset":0}"#
		);
		t(QueueAddKeyAlbum { key: 0, append: shukusai::audio::Append2::Back, clear: true, play: true, index: None, offset: Some(1) },
			r#"{"key":0,"append":"back","clear":true,"play":true,"index":null,"offset":1}"#
		);
	}

	#[test]
	fn queue_add_key_song() {
		t(QueueAddKeySong { key: 0, append: shukusai::audio::Append2::Back, clear: false, play: false, index: None },
			r#"{"key":0,"append":"back","clear":false,"play":false,"index":null}"#
		);
		t(QueueAddKeySong { key: 0, append: shukusai::audio::Append2::Back, clear: false, play: false, index: None},
			r#"{"key":0,"append":"back","clear":false,"play":false,"index":null}"#
		);
		t(QueueAddKeySong { key: 0, append: shukusai::audio::Append2::Back, clear: true, play: true, index: None},
			r#"{"key":0,"append":"back","clear":true,"play":true,"index":null}"#
		);
	}

	#[test]
	fn queue_add_map_artist() {
		t(QueueAddMapArtistOwned { artist: "hello".into(), append: shukusai::audio::Append2::Back, clear: false, play: false, index: None, offset: None },
			r#"{"artist":"hello","append":"back","clear":false,"play":false,"index":null,"offset":null}"#
		);
		t(QueueAddMapArtistOwned { artist: "hello".into(), append: shukusai::audio::Append2::Back, clear: false, play: false, index: None, offset: Some(0) },
			r#"{"artist":"hello","append":"back","clear":false,"play":false,"index":null,"offset":0}"#
		);
		t(QueueAddMapArtistOwned { artist: "hello".into(), append: shukusai::audio::Append2::Back, clear: true, play: true, index: None, offset: Some(1) },
			r#"{"artist":"hello","append":"back","clear":true,"play":true,"index":null,"offset":1}"#
		);
	}

	#[test]
	fn queue_add_map_album() {
		t(QueueAddMapAlbumOwned {
			artist: "hello".into(),
			album: "hello2".into(),
			append: shukusai::audio::Append2::Back,
			clear: false,
			play: false,
			index: None,
			offset: None
			},
			r#"{"artist":"hello","album":"hello2","append":"back","clear":false,"play":false,"index":null,"offset":null}"#
		);
		t(QueueAddMapAlbumOwned {
			artist: "hello".into(),
			album: "hello2".into(),
			append: shukusai::audio::Append2::Back,
			clear: false,
			play: false,
			index: None,
			offset: Some(0)
			},
			r#"{"artist":"hello","album":"hello2","append":"back","clear":false,"play":false,"index":null,"offset":0}"#
		);
		t(QueueAddMapAlbumOwned {
			artist: "hello".into(),
			album: "hello2".into(),
			append: shukusai::audio::Append2::Back,
			clear: true,
			play: true,
			index: None,
			offset: Some(1)
			},
			r#"{"artist":"hello","album":"hello2","append":"back","clear":true,"play":true,"index":null,"offset":1}"#
		);
	}

	#[test]
	fn queue_add_map_song() {
		t(QueueAddMapSongOwned {
			artist: "hello".into(),
			album: "hello2".into(),
			song: "hello3".into(),
			append: shukusai::audio::Append2::Back,
			clear: false,
			play: false,
			index: None,
			},
			r#"{"artist":"hello","album":"hello2","song":"hello3","append":"back","clear":false,"play":false,"index":null}"#
		);
		t(QueueAddMapSongOwned {
			artist: "hello".into(),
			album: "hello2".into(),
			song: "hello3".into(),
			append: shukusai::audio::Append2::Back,
			clear: false,
			play: false,
			index: None,
			},
			r#"{"artist":"hello","album":"hello2","song":"hello3","append":"back","clear":false,"play":false,"index":null}"#
		);
		t(QueueAddMapSongOwned {
			artist: "hello".into(),
			album: "hello2".into(),
			song: "hello3".into(),
			append: shukusai::audio::Append2::Back,
			clear: true,
			play: true,
			index: None,
			},
			r#"{"artist":"hello","album":"hello2","song":"hello3","append":"back","clear":true,"play":true,"index":null}"#
		);
	}

	#[test]
	fn queue_add_rand_artist() {
		t(QueueAddRandArtist { append: shukusai::audio::Append2::Back, clear: false, play: false, index: None, offset: None },
			r#"{"append":"back","clear":false,"play":false,"index":null,"offset":null}"#
		);
		t(QueueAddRandArtist { append: shukusai::audio::Append2::Back, clear: false, play: false, index: None, offset: Some(0) },
			r#"{"append":"back","clear":false,"play":false,"index":null,"offset":0}"#
		);
		t(QueueAddRandArtist { append: shukusai::audio::Append2::Back, clear: true, play: true, index: None, offset: Some(1) },
			r#"{"append":"back","clear":true,"play":true,"index":null,"offset":1}"#
		);
	}

	#[test]
	fn queue_add_rand_album() {
		t(QueueAddRandAlbum {
			append: shukusai::audio::Append2::Back,
			clear: false,
			play: false,
			index: None,
			offset: None
			},
			r#"{"append":"back","clear":false,"play":false,"index":null,"offset":null}"#
		);
		t(QueueAddRandAlbum {
			append: shukusai::audio::Append2::Back,
			clear: false,
			play: false,
			index: None,
			offset: Some(0)
			},
			r#"{"append":"back","clear":false,"play":false,"index":null,"offset":0}"#
		);
		t(QueueAddRandAlbum {
			append: shukusai::audio::Append2::Back,
			clear: true,
			play: true,
			index: None,
			offset: Some(1)
			},
			r#"{"append":"back","clear":true,"play":true,"index":null,"offset":1}"#
		);
	}

	#[test]
	fn queue_add_rand_song() {
		t(QueueAddRandSong {
			append: shukusai::audio::Append2::Back,
			clear: false,
			play: false,
			index: None,
			},
			r#"{"append":"back","clear":false,"play":false,"index":null}"#
		);
		t(QueueAddRandSong {
			append: shukusai::audio::Append2::Back,
			clear: false,
			play: false,
			index: None,
			},
			r#"{"append":"back","clear":false,"play":false,"index":null}"#
		);
		t(QueueAddRandSong {
			append: shukusai::audio::Append2::Back,
			clear: true,
			play: true,
			index: None,
			},
			r#"{"append":"back","clear":true,"play":true,"index":null}"#
		);
	}

	#[test]
	fn queue_add_rand_entry() {
		t(QueueAddRandEntry {
			append: shukusai::audio::Append2::Back,
			clear: false,
			play: false,
			index: None,
			},
			r#"{"append":"back","clear":false,"play":false,"index":null}"#
		);
		t(QueueAddRandEntry {
			append: shukusai::audio::Append2::Back,
			clear: false,
			play: false,
			index: None,
			},
			r#"{"append":"back","clear":false,"play":false,"index":null}"#
		);
		t(QueueAddRandEntry {
			append: shukusai::audio::Append2::Back,
			clear: true,
			play: true,
			index: None,
			},
			r#"{"append":"back","clear":true,"play":true,"index":null}"#
		);
	}

	#[test]
	fn queue_add_map_playlist() {
		t(QueueAddPlaylistOwned { playlist: "hello".into(), append: shukusai::audio::Append2::Back, clear: false, play: false, index: None, offset: None },
			r#"{"playlist":"hello","append":"back","clear":false,"play":false,"index":null,"offset":null}"#
		);
		t(QueueAddPlaylistOwned { playlist: "hello".into(), append: shukusai::audio::Append2::Back, clear: false, play: false, index: None, offset: Some(0) },
			r#"{"playlist":"hello","append":"back","clear":false,"play":false,"index":null,"offset":0}"#
		);
		t(QueueAddPlaylistOwned { playlist: "hello".into(), append: shukusai::audio::Append2::Back, clear: true, play: true, index: None, offset: Some(1) },
			r#"{"playlist":"hello","append":"back","clear":true,"play":true,"index":null,"offset":1}"#
		);
	}

	#[test]
	fn queue_set_index() {
		t(QueueSetIndex { index: usize::MAX }, r#"{"index":18446744073709551615}"#);
		t(QueueSetIndex { index: 0 },          r#"{"index":0}"#);
	}

	#[test]
	fn queue_remove_range() {
		t(QueueRemoveRange { start: usize::MAX, end: usize::MAX, skip: true }, r#"{"start":18446744073709551615,"end":18446744073709551615,"skip":true}"#);
		t(QueueRemoveRange { start: 0, end: 0, skip: false },                  r#"{"start":0,"end":0,"skip":false}"#);
	}

	//---------------------------------------------------------------------------------------------------- Queue
	#[test]
	fn playlist_new() {
		t(PlaylistNewOwned { playlist: "hello".into() }, r#"{"playlist":"hello"}"#);
	}

	#[test]
	fn playlist_remove() {
		t(PlaylistRemoveOwned { playlist: "hello".into() }, r#"{"playlist":"hello"}"#);
	}

	#[test]
	fn playlist_clone() {
		t(PlaylistCloneOwned { from: "hello".into(), to: "hello2".into() }, r#"{"from":"hello","to":"hello2"}"#);
	}

	#[test]
	fn playlist_get_index() {
		t(PlaylistGetIndexOwned { playlist: "hello".into(), index: 0 }, r#"{"playlist":"hello","index":0}"#);
	}

	#[test]
	fn playlist_remove_index() {
		t(PlaylistRemoveIndexOwned { playlist: "hello".into(), index: 0 }, r#"{"playlist":"hello","index":0}"#);
	}

	#[test]
	fn playlist_add_key_artist() {
		t(PlaylistAddKeyArtistOwned { playlist: "hello".into(), key: 0, append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","key":0,"append":"back","index":null}"#
		);
		t(PlaylistAddKeyArtistOwned { playlist: "hello".into(), key: 0, append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","key":0,"append":"back","index":null}"#
		);
		t(PlaylistAddKeyArtistOwned { playlist: "hello".into(), key: 0, append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","key":0,"append":"back","index":null}"#
		);
	}

	#[test]
	fn playlist_add_key_album() {
		t(PlaylistAddKeyAlbumOwned { playlist: "hello".into(), key: 0, append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","key":0,"append":"back","index":null}"#
		);
		t(PlaylistAddKeyAlbumOwned { playlist: "hello".into(), key: 0, append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","key":0,"append":"back","index":null}"#
		);
		t(PlaylistAddKeyAlbumOwned { playlist: "hello".into(), key: 0, append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","key":0,"append":"back","index":null}"#
		);
	}

	#[test]
	fn playlist_add_key_song() {
		t(PlaylistAddKeySongOwned { playlist: "hello".into(), key: 0, append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","key":0,"append":"back","index":null}"#
		);
		t(PlaylistAddKeySongOwned { playlist: "hello".into(), key: 0, append: shukusai::audio::Append2::Back, index: None},
			r#"{"playlist":"hello","key":0,"append":"back","index":null}"#
		);
		t(PlaylistAddKeySongOwned { playlist: "hello".into(), key: 0, append: shukusai::audio::Append2::Back, index: None},
			r#"{"playlist":"hello","key":0,"append":"back","index":null}"#
		);
	}

	#[test]
	fn playlist_add_map_artist() {
		t(PlaylistAddMapArtistOwned { playlist: "hello".into(), artist: "hello".into(), append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","artist":"hello","append":"back","index":null}"#
		);
		t(PlaylistAddMapArtistOwned { playlist: "hello".into(), artist: "hello".into(), append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","artist":"hello","append":"back","index":null}"#
		);
		t(PlaylistAddMapArtistOwned { playlist: "hello".into(), artist: "hello".into(), append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","artist":"hello","append":"back","index":null}"#
		);
	}

	#[test]
	fn playlist_add_map_album() {
		t(PlaylistAddMapAlbumOwned { playlist: "hello".into(), artist: "hello".into(), album: "hello".into(), append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","artist":"hello","album":"hello","append":"back","index":null}"#
		);
		t(PlaylistAddMapAlbumOwned { playlist: "hello".into(), artist: "hello".into(), album: "hello".into(), append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","artist":"hello","album":"hello","append":"back","index":null}"#
		);
		t(PlaylistAddMapAlbumOwned { playlist: "hello".into(), artist: "hello".into(), album: "hello".into(), append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","artist":"hello","album":"hello","append":"back","index":null}"#
		);
	}

	#[test]
	fn playlist_add_map_song() {
		t(PlaylistAddMapSongOwned { playlist: "hello".into(), artist: "hello".into(), album: "hello".into(), song: "hello".into(), append: shukusai::audio::Append2::Back, index: None },
			r#"{"playlist":"hello","artist":"hello","album":"hello","song":"hello","append":"back","index":null}"#
		);
		t(PlaylistAddMapSongOwned { playlist: "hello".into(), artist: "hello".into(), album: "hello".into(), song: "hello".into(), append: shukusai::audio::Append2::Back, index: None},
			r#"{"playlist":"hello","artist":"hello","album":"hello","song":"hello","append":"back","index":null}"#
		);
		t(PlaylistAddMapSongOwned { playlist: "hello".into(), artist: "hello".into(), album: "hello".into(), song: "hello".into(), append: shukusai::audio::Append2::Back, index: None},
			r#"{"playlist":"hello","artist":"hello","album":"hello","song":"hello","append":"back","index":null}"#
		);
	}

	#[test]
	fn playlist_single() {
		t(PlaylistSingleOwned { playlist: "hello".into() }, r#"{"playlist":"hello"}"#);
	}
}
