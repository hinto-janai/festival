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
	audio::Repeat,
	search::SearchKind,
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
impl_struct! {
	CollectionPerf,
	bytes: u64,
	user: f32,
	sys: f32
}
impl_struct! {
	CollectionResourceSize,
	audio: u64,
	art: usize
}
impl_struct_lt! {
	CollectionRelationInner,
	#[serde(borrow)]
	artist: Cow<'a, str>,
	#[serde(borrow)]
	album: Cow<'a, str>,
	#[serde(borrow)]
	song: Cow<'a, str>,
	key_artist: ArtistKey,
	key_album: AlbumKey,
	key_song: SongKey
}
impl_struct_anon_lt! {
	CollectionRelation,
	Cow<'a, [CollectionRelationInner<'a>]>
}
impl_struct_lt! {
	CollectionRelationFullInner,
	#[serde(borrow)]
	artist: Cow<'a, str>,
	#[serde(borrow)]
	album: Cow<'a, str>,
	#[serde(borrow)]
	song: Cow<'a, str>,
	key_artist: ArtistKey,
	key_album: AlbumKey,
	key_song: SongKey,
	#[serde(borrow)]
	path: Cow<'a, std::path::Path>
}
impl_struct_anon_lt! {
	CollectionRelationFull,
	Cow<'a, [CollectionRelationFullInner<'a>]>
}

//---------------------------------------------------------------------------------------------------- State
impl_struct! {
	StateIpInner,
	ip: std::net::Ipv4Addr,
	count: u64
}
impl_struct_anon_lt!(StateIp, Cow<'a, [StateIpInner]>);
impl_struct_lt! {
	StateConfig,
	ip:                 std::net::Ipv4Addr,
	port:               u16,
	max_connections:    Option<u64>,
	exclusive_ips:      Option<Cow<'a, HashSet<Ipv4Addr>>>,
	sleep_on_fail:      Option<u64>,
	collection_paths:   Cow<'a, [PathBuf]>,
	tls:                bool,
	certificate:        Option<Cow<'a, Path>>,
	key:                Option<Cow<'a, Path>>,
	rest:               bool,
	docs:               bool,
	direct_download:    bool,
	filename_separator: Cow<'a, str>,
	log_level:          log::LevelFilter,
	watch:              bool,
	cache_time:         u64,
	media_controls:     bool,
	authorization:      bool,
	no_auth_rpc:        Option<Cow<'a, HashSet<crate::method::Method>>>,
	no_auth_rest:       Option<Cow<'a, HashSet<crate::resource::Resource>>>
}
impl_struct_lt! {
	StateDaemon,
	uptime:              u64,
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
	repeat:    Repeat,
	volume:    u8,
	song:      Option<SongJson<'a>>
}
impl_struct! {
	StateReset,
	resetting: bool,
	saving:    bool
}

//---------------------------------------------------------------------------------------------------- Key
impl_struct_lt! {
	Artist,
	#[serde(borrow)]
	artist: ArtistJson<'a>
}
impl_struct_lt! {
	Album,
	#[serde(borrow)]
	album: AlbumJson<'a>
}
impl_struct_lt! {
	Song,
	#[serde(borrow)]
	song: SongJson<'a>
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

//---------------------------------------------------------------------------------------------------- Playback
//impl_struct_anon!(Toggle, ());
//impl_struct_anon!(Play, ());
//impl_struct_anon!(Pause, ());
//impl_struct_anon!(Next, ());
//impl_struct_anon!(Stop, ());
//impl_struct_anon!(Shuffle, ());
//impl_struct_anon!(RepeatOff, ());
//impl_struct_anon!(RepeatSong, ());
//impl_struct_anon!(RepeatQueue, ());
//impl_struct_anon!(Previous, ());
//impl_struct_anon!(Volume, ());
//impl_struct_anon!(Clear, ());
//impl_struct_anon!(Seek, ());
//impl_struct_anon!(Skip, ());
//impl_struct_anon!(Back, ());

//---------------------------------------------------------------------------------------------------- Queue
//impl_struct_anon!(AddQueueKeyArtist, ());
//impl_struct_anon!(AddQueueKeyAlbum, ());
//impl_struct_anon!(AddQueueKeySong, ());
//impl_struct_anon!(AddQueueMapArtist, ());
//impl_struct_anon!(AddQueueMapAlbum, ());
//impl_struct_anon!(AddQueueMapSong, ());
impl_struct_lt!(AddQueueRandArtist, #[serde(borrow)] artist: Cow<'a, ArtistJson<'a>>);
impl_struct_lt!(AddQueueRandAlbum, #[serde(borrow)] album: Cow<'a, AlbumJson<'a>>);
impl_struct_lt!(AddQueueRandSong, #[serde(borrow)] song: Cow<'a, SongJson<'a>>);
impl_struct!(SetQueueIndex, out_of_bounds: bool);
impl_struct!(RemoveQueueRange, out_of_bounds: bool);


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
	fn status() {
		t(&Status { ok: true  }, r#"{"ok":true}"#);
		t(&Status { ok: false }, r#"{"ok":false}"#);
	}
}
