//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use shukusai::{
	collection::{
		json::{
			CollectionJson,
			SongJson,
			AlbumJson,
			ArtistJson,
		},
		SongKey,
		AlbumKey,
		ArtistKey,
	},
	search::SearchKind,
	state::{
		PlaylistsJson,
		EntryJson,
	},
};
use crate::{
	impl_struct,
	impl_struct_lt,
	impl_struct_anon,
	impl_struct_anon_lt,
};
use std::{
	net::Ipv4Addr,
	path::{Path,PathBuf},
	borrow::Cow,
	collections::{
		VecDeque,
		HashSet,
		BTreeSet,
	},
};

//---------------------------------------------------------------------------------------------------- Response impl
// Generic response.
impl_struct_anon!(Status, ());

//---------------------------------------------------------------------------------------------------- Collection
impl_struct! {
	CollectionNew,
	time: f64,
	empty: bool,
	timestamp: u64,
	count_artist: u64,
	count_album: u64,
	count_song: u64,
	count_art: u64
}
impl_struct! {
	CollectionBrief,
	empty: bool,
	timestamp: u64,
	count_artist: u64,
	count_album: u64,
	count_song: u64,
	count_art: u64
}
impl_struct_anon_lt! {
	CollectionFull,
	CollectionJson<'a>
}
impl_struct_lt! {
	CollectionBriefArtists,
	len: usize,
	#[serde(borrow)]
	artists: Cow<'a, [Cow<'a, str>]>
}
impl_struct_lt! {
	CollectionBriefAlbums,
	len: usize,
	#[serde(borrow)]
	albums: Cow<'a, [Cow<'a, str>]>
}
impl_struct_lt! {
	CollectionBriefSongs,
	len: usize,
	#[serde(borrow)]
	songs: Cow<'a, [Cow<'a, str>]>
}
impl_struct_lt! {
	CollectionFullArtists,
	len: usize,
	#[serde(borrow)]
	artists: Cow<'a, [ArtistJson<'a>]>
}
impl_struct_lt! {
	CollectionFullAlbums,
	len: usize,
	#[serde(borrow)]
	albums: Cow<'a, [AlbumJson<'a>]>
}
impl_struct_lt! {
	CollectionFullSongs,
	len: usize,
	#[serde(borrow)]
	songs: Cow<'a, [SongJson<'a>]>
}
impl_struct_lt! {
	CollectionEntries,
	len: usize,
	#[serde(borrow)]
	entries: Cow<'a, [shukusai::collection::EntryJson<'a>]>
}
impl_struct! {
	CollectionPerf,
	bytes: u64,
	user: f32,
	sys: f32
}
impl_struct_lt! {
	CollectionHealth,
	all_ok: bool,
	song_len: usize,
	missing_len: usize,
	#[serde(borrow)]
	missing: Cow<'a, [shukusai::collection::EntryJson<'a>]>
}
impl_struct! {
	CollectionResourceSize,
	audio: u64,
	art: usize
}

//---------------------------------------------------------------------------------------------------- Daemon
impl_struct_lt! {
	DaemonConfig,
	ip:                  std::net::Ipv4Addr,
	port:                u16,
	max_connections:     Option<u64>,
	#[serde(borrow)]
	exclusive_ips:       Option<Cow<'a, BTreeSet<Ipv4Addr>>>,
	sleep_on_fail:       Option<u64>,
	#[serde(borrow)]
	collection_paths:    Cow<'a, [PathBuf]>,
	tls:                 bool,
	#[serde(borrow)]
	certificate:         Option<Cow<'a, Path>>,
	#[serde(borrow)]
	key:                 Option<Cow<'a, Path>>,
	rest:                bool,
	docs:                bool,
	direct_download:     bool,
	#[serde(borrow)]
	filename_separator:  Cow<'a, str>,
	log_level:           log::LevelFilter,
	watch:               bool,
	cache_time:          u64,
	restore_audio_state: bool,
	previous_threshold:  u32,
	media_controls:      bool,
	authorization:       bool,
	confirm_no_tls_auth: bool,
	#[serde(borrow)]
	no_auth_rpc:         Option<Cow<'a, BTreeSet<Cow<'a, str>>>>,
	#[serde(borrow)]
	no_auth_rest:        Option<Cow<'a, BTreeSet<Cow<'a, str>>>>,
	no_auth_docs:        bool
}
impl_struct_lt! {
	DaemonMethods,
	len: usize,
	#[serde(borrow)]
	methods: Cow<'a, BTreeSet<Cow<'a, str>>>
}
impl_struct_lt! {
	DaemonNoAuthRpc,
	len: usize,
	#[serde(borrow)]
	rpc: Cow<'a, BTreeSet<Cow<'a, str>>>
}
impl_struct_lt! {
	DaemonNoAuthRest,
	len: usize,
	#[serde(borrow)]
	rest: Cow<'a, BTreeSet<Cow<'a, str>>>
}
impl_struct_lt! {
	DaemonRemoveCacheInner,
	#[serde(borrow)]
	path: Cow<'a, Path>,
	bytes: u64
}
impl_struct_anon_lt!(DaemonRemoveCache, Cow<'a, [DaemonRemoveCacheInner<'a>]>);
impl_struct! {
	DaemonSeenIpsInner,
	ip: std::net::Ipv4Addr,
	count: u64
}
impl_struct_anon_lt!(DaemonSeenIps, Cow<'a, [DaemonSeenIpsInner]>);
impl_struct_lt! {
	DaemonShutdown,
	uptime: u64,
	#[serde(borrow)]
	uptime_readable: Cow<'a, str>,
	total_requests: u64,
	total_connections: u64
}
impl_struct_lt! {
	DaemonState,
	uptime:              u64,
	#[serde(borrow)]
	uptime_readable:     Cow<'a, str>,
	saving:              bool,
	total_requests:      u64,
	total_connections:   u64,
	current_connections: u64,
	rest:                bool,
	docs:                bool,
	direct_download:     bool,
	authorization:       bool,
	#[serde(borrow)]
	version: Cow<'a, str>,
	#[serde(borrow)]
	commit: Cow<'a, str>,
	#[serde(borrow)]
	os: Cow<'a, str>
}

//---------------------------------------------------------------------------------------------------- State
impl_struct_lt! {
	StateAudio,
	#[serde(borrow)]
	queue:     Cow<'a, [SongKey]>,
	queue_len: usize,
	queue_idx: Option<usize>,
	playing:   bool,
	song_key:  Option<SongKey>,
	elapsed:   u32,
	runtime:   u32,
	repeat:    shukusai::audio::Repeat,
	volume:    u8,
	#[serde(borrow)]
	song:      Option<SongJson<'a>>
}
impl_struct_lt! {
	StateQueueKey,
	len: usize,
	keys: Cow<'a, [SongKey]>
}
impl_struct_lt! {
	StateQueueSong,
	len: usize,
	#[serde(borrow)]
	songs: Cow<'a, [SongJson<'a>]>
}
impl_struct_lt! {
	StateQueueEntry,
	len: usize,
	#[serde(borrow)]
	entries: Cow<'a, [shukusai::collection::EntryJson<'a>]>
}
impl_struct! {
	StatePlaying,
	playing: bool
}
impl_struct! {
	StateRepeat,
	mode: shukusai::audio::Repeat
}
impl_struct_lt! {
	StateRuntime,
	elapsed: u32,
	runtime: u32,
	#[serde(borrow)]
	elapsed_readable: Cow<'a, str>,
	#[serde(borrow)]
	runtime_readable: Cow<'a, str>
}
impl_struct! {
	StateVolume,
	volume: shukusai::audio::Volume
}

//---------------------------------------------------------------------------------------------------- Key
impl_struct_lt! {
	KeyArtist,
	#[serde(borrow)]
	artist: ArtistJson<'a>
}
impl_struct_lt! {
	KeyAlbum,
	#[serde(borrow)]
	album: AlbumJson<'a>
}
impl_struct_lt! {
	KeySong,
	#[serde(borrow)]
	song: SongJson<'a>
}
impl_struct_lt! {
	KeyEntry,
	#[serde(borrow)]
	entry: shukusai::collection::EntryJson<'a>
}
impl_struct_lt! {
	KeyArtistAlbums,
	len: usize,
	#[serde(borrow)]
	albums: Cow<'a, [AlbumJson<'a>]>
}
impl_struct_lt! {
	KeyArtistSongs,
	len: usize,
	#[serde(borrow)]
	songs: Cow<'a, [SongJson<'a>]>
}
impl_struct_lt! {
	KeyArtistEntries,
	len: usize,
	#[serde(borrow)]
	entries: Cow<'a, [shukusai::collection::EntryJson<'a>]>
}
impl_struct_lt! {
	KeyAlbumArtist,
	#[serde(borrow)]
	artist: ArtistJson<'a>
}
impl_struct_lt! {
	KeyAlbumSongs,
	len: usize,
	#[serde(borrow)]
	songs: Cow<'a, [SongJson<'a>]>
}
impl_struct_lt! {
	KeyAlbumEntries,
	len: usize,
	#[serde(borrow)]
	entries: Cow<'a, [shukusai::collection::EntryJson<'a>]>
}
impl_struct_lt! {
	KeySongArtist,
	#[serde(borrow)]
	artist: ArtistJson<'a>
}
impl_struct_lt! {
	KeySongAlbum,
	#[serde(borrow)]
	album: AlbumJson<'a>
}
impl_struct_lt! {
	KeyOtherAlbums,
	len: usize,
	#[serde(borrow)]
	albums: Cow<'a, [AlbumJson<'a>]>
}
impl_struct_lt! {
	KeyOtherSongs,
	len: usize,
	#[serde(borrow)]
	songs: Cow<'a, [SongJson<'a>]>
}
impl_struct_lt! {
	KeyOtherEntries,
	len: usize,
	#[serde(borrow)]
	entries: Cow<'a, [shukusai::collection::EntryJson<'a>]>
}

//---------------------------------------------------------------------------------------------------- Map
impl_struct_lt! {
	MapArtist,
	#[serde(borrow)]
	artist: ArtistJson<'a>
}
impl_struct_lt! {
	MapAlbum,
	#[serde(borrow)]
	album: AlbumJson<'a>
}
impl_struct_lt! {
	MapSong,
	#[serde(borrow)]
	song: SongJson<'a>
}
impl_struct_lt! {
	MapEntry,
	#[serde(borrow)]
	entry: shukusai::collection::EntryJson<'a>
}
impl_struct_lt! {
	MapArtistAlbums,
	len: usize,
	#[serde(borrow)]
	albums: Cow<'a, [AlbumJson<'a>]>
}
impl_struct_lt! {
	MapArtistSongs,
	len: usize,
	#[serde(borrow)]
	songs: Cow<'a, [SongJson<'a>]>
}
impl_struct_lt! {
	MapArtistEntries,
	len: usize,
	#[serde(borrow)]
	entries: Cow<'a, [shukusai::collection::EntryJson<'a>]>
}
impl_struct_lt! {
	MapAlbumSongs,
	len: usize,
	#[serde(borrow)]
	songs: Cow<'a, [SongJson<'a>]>
}
impl_struct_lt! {
	MapAlbumEntries,
	len: usize,
	#[serde(borrow)]
	entries: Cow<'a, [shukusai::collection::EntryJson<'a>]>
}

//---------------------------------------------------------------------------------------------------- Current
impl_struct_lt! {
	CurrentArtist,
	#[serde(borrow)]
	artist: ArtistJson<'a>
}
impl_struct_lt! {
	CurrentAlbum,
	#[serde(borrow)]
	album: AlbumJson<'a>
}
impl_struct_lt! {
	CurrentSong,
	#[serde(borrow)]
	song: SongJson<'a>
}
impl_struct_lt! {
	CurrentEntry,
	#[serde(borrow)]
	entry: shukusai::collection::EntryJson<'a>
}

//---------------------------------------------------------------------------------------------------- Rand
impl_struct_lt! {
	RandArtist,
	#[serde(borrow)]
	artist: ArtistJson<'a>
}
impl_struct_lt! {
	RandAlbum,
	#[serde(borrow)]
	album: AlbumJson<'a>
}
impl_struct_lt! {
	RandSong,
	#[serde(borrow)]
	song: SongJson<'a>
}
impl_struct_lt! {
	RandEntry,
	#[serde(borrow)]
	entry: shukusai::collection::EntryJson<'a>
}

//---------------------------------------------------------------------------------------------------- Search
impl_struct_lt! {
	Search,
	#[serde(borrow)]
	artists: Cow<'a, [ArtistJson<'a>]>,
	#[serde(borrow)]
	albums: Cow<'a, [AlbumJson<'a>]>,
	#[serde(borrow)]
	songs: Cow<'a, [SongJson<'a>]>
}
impl_struct_lt! {
	SearchArtist,
	#[serde(borrow)]
	artists: Cow<'a, [ArtistJson<'a>]>
}
impl_struct_lt! {
	SearchAlbum,
	#[serde(borrow)]
	albums: Cow<'a, [AlbumJson<'a>]>
}
impl_struct_lt! {
	SearchSong,
	#[serde(borrow)]
	songs: Cow<'a, [SongJson<'a>]>
}
impl_struct_lt! {
	SearchEntry,
	#[serde(borrow)]
	entries: Cow<'a, [shukusai::collection::EntryJson<'a>]>
}

//---------------------------------------------------------------------------------------------------- Playback
//impl_struct_anon!(Toggle, ());
//impl_struct_anon!(Play, ());
//impl_struct_anon!(Pause, ());
impl_struct! {
	Clear,
	len: usize
}
impl_struct! {
	Stop,
	len: usize
}
//impl_struct_anon!(Next, ());
//impl_struct_anon!(Stop, ());
//impl_struct_anon!(Shuffle, ());
impl_struct! {
	Repeat,
	previous: shukusai::audio::Repeat,
	current: shukusai::audio::Repeat
}
//impl_struct_anon!(Previous, ());
impl_struct! {
	Volume,
	previous: shukusai::audio::Volume,
	current: shukusai::audio::Volume
}
impl_struct! {
	VolumeUp,
	previous: shukusai::audio::Volume,
	current: shukusai::audio::Volume
}
impl_struct! {
	VolumeDown,
	previous: shukusai::audio::Volume,
	current: shukusai::audio::Volume
}
//impl_struct_anon!(Clear, ());
//impl_struct_anon!(Seek, ());
//impl_struct_anon!(Skip, ());
//impl_struct_anon!(Back, ());

//---------------------------------------------------------------------------------------------------- Queue
//impl_struct_anon!(QueueAddKeyArtist, ());
//impl_struct_anon!(QueueAddKeyAlbum, ());
//impl_struct_anon!(QueueAddKeySong, ());
//impl_struct_anon!(QueueAddMapArtist, ());
//impl_struct_anon!(QueueAddMapAlbum, ());
//impl_struct_anon!(QueueAddMapSong, ());
impl_struct_lt!(QueueAddRandArtist, #[serde(borrow)] artist: Cow<'a, ArtistJson<'a>>);
impl_struct_lt!(QueueAddRandAlbum, #[serde(borrow)] album: Cow<'a, AlbumJson<'a>>);
impl_struct_lt!(QueueAddRandSong, #[serde(borrow)] song: Cow<'a, SongJson<'a>>);
impl_struct_lt!(QueueAddRandEntry, #[serde(borrow)] entry: Cow<'a, shukusai::collection::EntryJson<'a>>);
impl_struct!(QueueSetIndex, out_of_bounds: bool, index: usize, queue_len: usize);
impl_struct!(QueueRemoveRange, out_of_bounds: bool, start: usize, end: usize, queue_len: usize);

//---------------------------------------------------------------------------------------------------- Playlist
impl_struct_lt!(PlaylistNew, len: Option<usize>, #[serde(borrow)] entries: Option<Cow<'a, [EntryJson<'a>]>>);
impl_struct_lt!(PlaylistRemove, len: usize, #[serde(borrow)] entries: Cow<'a, [EntryJson<'a>]>);
impl_struct_lt!(PlaylistClone, len: Option<usize>, #[serde(borrow)] entries: Option<Cow<'a, [EntryJson<'a>]>>);
impl_struct_lt!(PlaylistGetIndex, #[serde(borrow)] entry: Option<Cow<'a, EntryJson<'a>>>);
impl_struct_lt!(PlaylistRemoveIndex, #[serde(borrow)] entry: Option<Cow<'a, EntryJson<'a>>>);
impl_struct!(PlaylistAddKeyArtist, existed: bool, old_len: usize, new_len: usize);
impl_struct!(PlaylistAddKeyAlbum, existed: bool, old_len: usize, new_len: usize);
impl_struct!(PlaylistAddKeySong, existed: bool, old_len: usize, new_len: usize);
impl_struct!(PlaylistAddMapArtist, existed: bool, old_len: usize, new_len: usize);
impl_struct!(PlaylistAddMapAlbum, existed: bool, old_len: usize, new_len: usize);
impl_struct!(PlaylistAddMapSong, existed: bool, old_len: usize, new_len: usize);
impl_struct_lt! {
	PlaylistSingle,
	#[serde(borrow)]
	playlist: Cow<'a, str>,
	all_valid: bool,
	entry_len: usize,
	valid_len: usize,
	invalid_len: usize,
	#[serde(borrow)]
	entries: Cow<'a, [EntryJson<'a>]>
}
impl_struct_lt! {
	PlaylistBrief,
	len: usize,
	playlists: Cow<'a, [Cow<'a, str>]>
}
impl_struct_lt! {
	PlaylistFull,
	all_valid: bool,
	playlist_len: usize,
	entry_len: usize,
	valid_len: usize,
	invalid_len: usize,
	#[serde(borrow)]
	playlists: Cow<'a, PlaylistsJson<'a>>
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	// Responses are tested in `/cli/src/rpc.rs`.

//	//------------------------------------- Serde sanity tests.
//	// Testing function.
//	fn t<T>(value: &T, expected: &'static str)
//		where
//			T: Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
//	{
//		let string = serde_json::to_string(value).unwrap();
//		assert_eq!(string, expected);
//		let t: T = serde_json::from_str(&string).unwrap();
//		assert_eq!(t, *value);
//		let e: T = serde_json::from_str(expected).unwrap();
//		assert_eq!(e, *value);
//	}
//
//	#[test]
//	fn status() {
//		t(&Status { ok: true  }, r#"{"ok":true}"#);
//		t(&Status { ok: false }, r#"{"ok":false}"#);
//	}
}
