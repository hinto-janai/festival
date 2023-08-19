//---------------------------------------------------------------------------------------------------- Use
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use std::sync::Arc;
use std::net::SocketAddrV4;
use hyper::{
	Request,
	Response,
	body::Body,
};
use http::request::Parts;
use serde_json::value::{
	RawValue,Value,
};
use shukusai::{
	kernel::{
		FrontendToKernel,
		KernelToFrontend,
	},
	state::{
		AUDIO_STATE,
		PLAYLISTS,
	},
	collection::{
		Collection,
		Artist,
		Album,
		Song,
		ArtistKey,
		AlbumKey,
		SongKey,
		json::{
			CollectionJson,
			ArtistJson,
			AlbumJson,
			SongJson,
		},
	},
	constants::{
		OS_ARCH,
		COMMIT,
	},
	search::SearchKind,
};
use crate::{
	resp,
	constants::FESTIVALD_VERSION,
	config::{AUTH,Config,config},
	statics::{
		RESETTING,
		TOTAL_CONNECTIONS,
		TOTAL_REQUESTS,
	},
	ptr::CollectionPtr,
};
use std::borrow::Cow;
use std::time::Duration;
use benri::{
	atomic_load,
	atomic_store,
	debug_panic,
	lock,send,recv,
	secs_f64,now,
};
use crossbeam::channel::{
	Sender,Receiver,
};
use json_rpc::{
	Id,
};
use disk::{Bincode2,Empty};
use std::collections::BTreeSet;

//---------------------------------------------------------------------------------------------------- Custom Method Error Codes/Messages
macro_rules! impl_err {
	($($name:ident, $code:literal, $msg:literal),*) => {
		const ERR_BASE_CODE: i32 = -32011;
		$(
			const $name: (i32, &str) = (ERR_BASE_CODE - $code, $msg);
		)*
	}
}

impl_err! {
	ERR_VOLUME,         0,  "Volume must be in between 0..100",
	ERR_KEY_ARTIST,     1,  "Artist key is invalid",
	ERR_KEY_ALBUM,      2,  "Album key is invalid",
	ERR_KEY_SONG,       3,  "Song key is invalid",
	ERR_MAP_ARTIST,     4,  "Artist does not exist",
	ERR_MAP_ALBUM,      5,  "Album does not exist",
	ERR_MAP_SONG,       6,  "Song does not exist",
	ERR_CURRENT,        7,  "No song is currently set",
	ERR_RAND,           8,  "The Collection is empty",
	ERR_RESETTING,      9,  "Currently resetting the Collection",
	ERR_PERF,           10, "Performance file does not exist",
	ERR_FS,             11, "Filesystem error",
	ERR_AUTH,           12, "Unauthorized",
	ERR_SERDE,          13, "(De)serialization error",
	ERR_APPEND,         14, "Index append was chosen, but no index was provided",
	ERR_INDEX,          15, "Bad index, greater or equal to queue length",
	ERR_OFFSET,         16, "Bad offset, greater or equal to amount of songs",
	ERR_PLAYLIST,       17, "Playlist doesn't exist",
	ERR_INDEX_PLAYLIST, 18, "Bad index, greater or equal to playlist length"
}

//---------------------------------------------------------------------------------------------------- Response "Cache"
// Instead of serializing heavy objects like the `Collection` each
// request, these mutable globals are so that we only have to do it
// _once_ per reset, then each request just uses these values.
//
// INVARIANT: These must be "updated" on a `Collection` reset.
pub static CACHE_COLLECTION_FULL:          tokio::sync::RwLock<String> = tokio::sync::RwLock::const_new(String::new());
pub static CACHE_COLLECTION_BRIEF_ARTISTS: tokio::sync::RwLock<String> = tokio::sync::RwLock::const_new(String::new());
pub static CACHE_COLLECTION_BRIEF_ALBUMS:  tokio::sync::RwLock<String> = tokio::sync::RwLock::const_new(String::new());
pub static CACHE_COLLECTION_BRIEF_SONGS:   tokio::sync::RwLock<String> = tokio::sync::RwLock::const_new(String::new());
pub static CACHE_COLLECTION_FULL_ARTISTS:  tokio::sync::RwLock<String> = tokio::sync::RwLock::const_new(String::new());
pub static CACHE_COLLECTION_FULL_ALBUMS:   tokio::sync::RwLock<String> = tokio::sync::RwLock::const_new(String::new());
pub static CACHE_COLLECTION_FULL_SONGS:    tokio::sync::RwLock<String> = tokio::sync::RwLock::const_new(String::new());
pub static CACHE_COLLECTION_ENTRIES:       tokio::sync::RwLock<String> = tokio::sync::RwLock::const_new(String::new());
pub static CACHE_PLAYLIST_BRIEF:           tokio::sync::RwLock<String> = tokio::sync::RwLock::const_new(String::new());
pub static CACHE_PLAYLIST_FULL:            tokio::sync::RwLock<String> = tokio::sync::RwLock::const_new(String::new());

// We pre-serialize the above response, but we
// need to pop off the `"id": null\n}` at the end.
// That string is `14` bytes.
const END_POP: u8 = 14;

// This sets all of the below.
pub async fn cache_set_all(collection: &Arc<Collection>) {
	cache_set_collection_full(collection).await;
	cache_set_collection_brief_artists(collection).await;
	cache_set_collection_brief_albums(collection).await;
	cache_set_collection_brief_songs(collection).await;
	cache_set_collection_full_artists(collection).await;
	cache_set_collection_full_albums(collection).await;
	cache_set_collection_full_songs(collection).await;
	cache_set_collection_entries(collection).await;
}

pub async fn cache_set_collection_full(collection: &Arc<Collection>) {
	// Instead of checking if the `Collection` -> `JSON String`
	// output is correct for every response, only check in debug builds.
	//
	// No need to do `Collection` -> `String` -> `CollectionJson` -> `String`
	// when all that is needed is `Collection` -> `String`
	#[cfg(debug_assertions)]
	{
		let string = serde_json::to_string(&*collection).unwrap();
		let c: CollectionJson = serde_json::from_str(&string).unwrap();
		assert_eq!(serde_json::to_string(&c).unwrap(), string);
	}

	let resp = json_rpc::Response::result(Cow::Borrowed(&collection), None);
	let mut resp = serde_json::to_string_pretty(&resp).expect(ERR_SERDE.1);
	for _ in 0..END_POP { resp.pop(); }
	*CACHE_COLLECTION_FULL.write().await = resp;
}

pub async fn cache_set_collection_brief_artists(collection: &Arc<Collection>) {
	let mut resp: Vec<&Arc<str>> = collection.artists
		.iter()
		.map(|x| &x.name)
		.collect();

	resp.sort();

	let resp = serde_json::json!({
		"len": resp.len(),
		"artists": resp,
	});

	let resp = json_rpc::Response::result(Cow::Borrowed(&resp), None);
	let mut resp = serde_json::to_string_pretty(&resp).expect(ERR_SERDE.1);
	for _ in 0..END_POP { resp.pop(); }
	*CACHE_COLLECTION_BRIEF_ARTISTS.write().await = resp;
}

pub async fn cache_set_collection_brief_albums(collection: &Arc<Collection>) {
	let mut resp: Vec<&Arc<str>> = collection.albums
		.iter()
		.map(|x| &x.title)
		.collect();

	resp.sort();

	let resp = serde_json::json!({
		"len": resp.len(),
		"albums": resp,
	});

	let resp = json_rpc::Response::result(Cow::Borrowed(&resp), None);
	let mut resp = serde_json::to_string_pretty(&resp).expect(ERR_SERDE.1);
	for _ in 0..END_POP { resp.pop(); }
	*CACHE_COLLECTION_BRIEF_ALBUMS.write().await = resp;
}

pub async fn cache_set_collection_brief_songs(collection: &Arc<Collection>) {
	let mut resp: Vec<&Arc<str>> = collection.songs
		.iter()
		.map(|x| &x.title)
		.collect();

	resp.sort();

	let resp = serde_json::json!({
		"len": resp.len(),
		"songs": resp,
	});

	let resp = json_rpc::Response::result(Cow::Borrowed(&resp), None);
	let mut resp = serde_json::to_string_pretty(&resp).expect(ERR_SERDE.1);
	for _ in 0..END_POP { resp.pop(); }
	*CACHE_COLLECTION_BRIEF_SONGS.write().await = resp;
}

pub async fn cache_set_collection_full_artists(collection: &Arc<Collection>) {
	let mut resp = Vec::<Value>::with_capacity(collection.artists.len());

	for artist in collection.artists.iter() {
		let artist = serde_json::to_value(artist).expect(ERR_SERDE.1);
		resp.push(artist);
	}

	let resp = serde_json::json!({
		"len": resp.len(),
		"artists": resp,
	});

	let resp = json_rpc::Response::result(Cow::Borrowed(&resp), None);
	let mut resp = serde_json::to_string_pretty(&resp).expect(ERR_SERDE.1);
	for _ in 0..END_POP { resp.pop(); }
	*CACHE_COLLECTION_FULL_ARTISTS.write().await = resp;
}

pub async fn cache_set_collection_full_albums(collection: &Arc<Collection>) {
	let mut resp = Vec::<Value>::with_capacity(collection.albums.len());

	for album in collection.albums.iter() {
		let album = serde_json::to_value(album).expect(ERR_SERDE.1);
		resp.push(album);
	}

	let resp = serde_json::json!({
		"len": resp.len(),
		"albums": resp,
	});

	let resp = json_rpc::Response::result(Cow::Borrowed(&resp), None);
	let mut resp = serde_json::to_string_pretty(&resp).expect(ERR_SERDE.1);
	for _ in 0..END_POP { resp.pop(); }
	*CACHE_COLLECTION_FULL_ALBUMS.write().await = resp;
}

pub async fn cache_set_collection_full_songs(collection: &Arc<Collection>) {
	let mut resp = Vec::<Value>::with_capacity(collection.songs.len());

	for song in collection.songs.iter() {
		let song = serde_json::to_value(song).expect(ERR_SERDE.1);
		resp.push(song);
	}

	let resp = serde_json::json!({
		"len": resp.len(),
		"songs": resp,
	});

	let resp = json_rpc::Response::result(Cow::Borrowed(&resp), None);
	let mut resp = serde_json::to_string_pretty(&resp).expect(ERR_SERDE.1);
	for _ in 0..END_POP { resp.pop(); }
	*CACHE_COLLECTION_FULL_SONGS.write().await = resp;
}

pub async fn cache_set_collection_entries(collection: &Arc<Collection>) {
	let mut vec = Vec::<shukusai::collection::EntryJson>::with_capacity(collection.songs.len());

	for song in collection.songs.iter() {
		let album  = &collection.albums[song.album];
		let artist = &collection.artists[album.artist];

		let r = shukusai::collection::EntryJson {
			artist: Cow::Borrowed(&artist.name),
			album: Cow::Borrowed(&album.title),
			song: Cow::Borrowed(&song.title),
			key_artist: ArtistKey::from(album.artist),
			key_album: AlbumKey::from(song.album),
			key_song: SongKey::from(song.key),
			path: Cow::Borrowed(song.path.as_path()),
		};

		vec.push(r);
	}

	let resp = serde_json::json!({
		"len": vec.len(),
		"entries": vec,
	});

	let resp = json_rpc::Response::result(Cow::Borrowed(&resp), None);
	let mut resp = serde_json::to_string_pretty(&resp).expect(ERR_SERDE.1);
	for _ in 0..END_POP { resp.pop(); }
	*CACHE_COLLECTION_ENTRIES.write().await = resp;
}

//---------------------------------------------------------------------------------------------------- Parse, call func, or return macro.
// Parse
// Params
// And
// Call
// Or
// Return
//
// We have the method, but we need to make sure the params
// are correct, so, attempt to parse, if it is correct, call the
// proper function, else return from the outer scope.
//
// This must be `.await`'ed.
macro_rules! ppacor {
	($method:expr, $request:expr, $call:expr, $param:ty, $($extra_arg:expr),*) => {{
		let Some(value) = $request.params else {
			return Ok(crate::resp::invalid_params($request.id));
		};

		let Ok(param) = serde_json::from_str::<$param>(value.get()) else {
			return Ok(crate::resp::invalid_params($request.id));
		};

		debug!("RPC - Method: {}, Params: {:?}", $method, param);

		$call(param, $request.id, $($extra_arg),*)
	}};
	($method:expr, $request:expr, $call:expr, $param:ty) => {{
		let Some(value) = $request.params else {
			return Ok(crate::resp::invalid_params($request.id));
		};

		let Ok(param) = serde_json::from_str::<$param>(value.get()) else {
			return Ok(crate::resp::invalid_params($request.id));
		};

		debug!("RPC - Method: {}, Params: {:?}", $method, param);

		$call(param, $request.id)
	}};
}

// Log
// And
// Call
//
// Log a method before calling the function (without params).
//
// This must be `.await`'ed.
macro_rules! lac {
	($method:expr, $request:expr, $call:expr $(,$extra_arg:expr),*) => {{
		debug!("RPC - Method: {}", $method);
		$call($request.id, $($extra_arg),*)
	}};
}

//---------------------------------------------------------------------------------------------------- JSON-RPC Handler
pub async fn handle(
	parts:       Parts,
	body:        Body,
	addr:        SocketAddrV4,
	collection:  &'static CollectionPtr,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
	FROM_KERNEL: &'static Receiver<KernelToFrontend>,
	TO_ROUTER_S: &'static tokio::sync::mpsc::Sender::<()>,
	TO_ROUTER_C: &'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
) -> Result<Response<Body>, anyhow::Error> {
	// Body to bytes.
	let body = hyper::body::to_bytes(body).await?;

	// Check if JSON.
	if serde_json::from_slice::<&RawValue>(&body).is_err() {
		return Ok(resp::parse_error(None));
	};

	// Parse request.
	let Ok(request) = serde_json::from_slice::<json_rpc::Request<&RawValue, &RawValue>>(&body) else {
		return Ok(resp::parse_error(None));
	};

	// If we're in the middle of a `Collection` reset, respond with "busy".
	if crate::statics::resetting() {
		return Ok(resp::resetting(ERR_RESETTING.0, ERR_RESETTING.1, request.id));
	}

	// Parse method.
	let Ok(method) = serde_json::from_str::<rpc::Method>(request.method.get()) else {
		return Ok(resp::method_not_found(request.id));
	};

	// Check auth.
	if !config().no_auth_rpc.as_ref().is_some_and(|h| h.contains(&method)) {
		if let Some(hash) = AUTH.get() {
			if !crate::router::auth_ok(&parts, hash).await {
				if crate::seen::seen(&addr).await {
					crate::router::sleep_on_fail().await;
				}
				return Ok(resp::unauth_rpc(ERR_AUTH.0, ERR_AUTH.1, request.id));
			}
		}
	}

	use rpc::Method::*;
	match method {
		//-------------------------------------------------- Collection
		//                               method
		//                        (in-scope) variable
		//                                  |
		//                                  |     request
		//                                  | (in-scope) variable
		//                                  |        |
		//                                  |        |    function to call if
		//                                  |        |   the parameters are ok
		//                                  |        | (defined somewhere below)     expected                     additional function arguments
		//                                  |        |          |                 parameter type                               |
		//                                  |        |          |                       |             |------------------------|--------------------------|
		//                                  v        v          v                       v             v                                                   v
		CollectionNew          => ppacor!(method, request, collection_new, rpc::param::CollectionNew, collection.arc(), TO_KERNEL, FROM_KERNEL, TO_ROUTER_C).await,
		CollectionBrief        => lac!(method, request, collection_brief, collection.arc()).await,
		CollectionFull         => lac!(method, request, collection_full).await,
		CollectionBriefArtists => lac!(method, request, collection_brief_artists).await,
		CollectionBriefAlbums  => lac!(method, request, collection_brief_albums).await,
		CollectionBriefSongs   => lac!(method, request, collection_brief_songs).await,
		CollectionFullArtists  => lac!(method, request, collection_full_artists).await,
		CollectionFullAlbums   => lac!(method, request, collection_full_albums).await,
		CollectionFullSongs    => lac!(method, request, collection_full_songs).await,
		CollectionEntries      => lac!(method, request, collection_entries).await,
		CollectionPerf         => lac!(method, request, collection_perf).await,
		CollectionHealth       => lac!(method, request, collection_health, collection.arc()).await,
		CollectionResourceSize => lac!(method, request, collection_resource_size, collection.arc()).await,

		//-------------------------------------------------- Daemon
		DaemonConfig      => lac!(method, request, daemon_config).await,
		DaemonMethods     => lac!(method, request, daemon_methods).await,
		DaemonNoAuthRpc   => lac!(method, request, daemon_no_auth_rpc).await,
		DaemonNoAuthRest  => lac!(method, request, daemon_no_auth_rest).await,
		DaemonRemoveCache => lac!(method, request, daemon_remove_cache).await,
		DaemonSave        => lac!(method, request, daemon_save, TO_KERNEL).await,
		DaemonSeenIps     => lac!(method, request, daemon_seen_ips).await,
		DaemonShutdown    => lac!(method, request, daemon_shutdown, TO_ROUTER_S).await,
		DaemonState       => lac!(method, request, daemon_state).await,

		//-------------------------------------------------- State
		StateAudio      => lac!(method, request, state_audio, collection.arc()).await,
		StateQueueKey   => lac!(method, request, state_queue_key, collection.arc()).await,
		StateQueueSong  => lac!(method, request, state_queue_song, collection.arc()).await,
		StateQueueEntry => lac!(method, request, state_queue_entry, collection.arc()).await,
		StatePlaying    => lac!(method, request, state_playing).await,
		StateRepeat     => lac!(method, request, state_repeat).await,
		StateRuntime    => lac!(method, request, state_runtime).await,
		StateVolume     => lac!(method, request, state_volume).await,

		//-------------------------------------------------- Key
		KeyArtist        => ppacor!(method, request, key_artist, rpc::param::KeyArtist, collection.arc()).await,
		KeyAlbum         => ppacor!(method, request, key_album, rpc::param::KeyAlbum, collection.arc()).await,
		KeySong          => ppacor!(method, request, key_song, rpc::param::KeySong, collection.arc()).await,
		KeyEntry         => ppacor!(method, request, key_entry, rpc::param::KeyEntry, collection.arc()).await,
		KeyArtistAlbums  => ppacor!(method, request, key_artist_albums, rpc::param::KeyArtistAlbums, collection.arc()).await,
		KeyArtistSongs   => ppacor!(method, request, key_artist_songs, rpc::param::KeyArtistSongs, collection.arc()).await,
		KeyArtistEntries => ppacor!(method, request, key_artist_entries, rpc::param::KeyArtistEntries, collection.arc()).await,
		KeyAlbumArtist   => ppacor!(method, request, key_album_artist, rpc::param::KeyAlbumArtist, collection.arc()).await,
		KeyAlbumSongs    => ppacor!(method, request, key_album_songs, rpc::param::KeyAlbumSongs, collection.arc()).await,
		KeyAlbumEntries  => ppacor!(method, request, key_album_entries, rpc::param::KeyAlbumEntries, collection.arc()).await,
		KeySongArtist    => ppacor!(method, request, key_song_artist, rpc::param::KeySongArtist, collection.arc()).await,
		KeySongAlbum     => ppacor!(method, request, key_song_album, rpc::param::KeySongAlbum, collection.arc()).await,
		KeyOtherAlbums   => ppacor!(method, request, key_other_albums, rpc::param::KeyOtherAlbums, collection.arc()).await,
		KeyOtherSongs    => ppacor!(method, request, key_other_songs, rpc::param::KeyOtherSongs, collection.arc()).await,
		KeyOtherEntries  => ppacor!(method, request, key_other_entries, rpc::param::KeyOtherEntries, collection.arc()).await,

		//-------------------------------------------------- Map
		MapArtist        => ppacor!(method, request, map_artist, rpc::param::MapArtist, collection.arc()).await,
		MapAlbum         => ppacor!(method, request, map_album, rpc::param::MapAlbum, collection.arc()).await,
		MapSong          => ppacor!(method, request, map_song, rpc::param::MapSong, collection.arc()).await,
		MapEntry         => ppacor!(method, request, map_entry, rpc::param::MapEntry, collection.arc()).await,
		MapArtistAlbums  => ppacor!(method, request, map_artist_albums, rpc::param::MapArtistAlbums, collection.arc()).await,
		MapArtistSongs   => ppacor!(method, request, map_artist_songs, rpc::param::MapArtistSongs, collection.arc()).await,
		MapArtistEntries => ppacor!(method, request, map_artist_entries, rpc::param::MapArtistEntries, collection.arc()).await,
		MapAlbumSongs    => ppacor!(method, request, map_album_songs, rpc::param::MapAlbumSongs, collection.arc()).await,
		MapAlbumEntries  => ppacor!(method, request, map_album_entries, rpc::param::MapAlbumEntries, collection.arc()).await,

		//-------------------------------------------------- Current
		CurrentArtist => lac!(method, request, current_artist, collection.arc()).await,
		CurrentAlbum  => lac!(method, request, current_album, collection.arc()).await,
		CurrentSong   => lac!(method, request, current_song, collection.arc()).await,
		CurrentEntry  => lac!(method, request, current_entry, collection.arc()).await,

		//-------------------------------------------------- Rand
		RandArtist => lac!(method, request, rand_artist, collection.arc()).await,
		RandAlbum  => lac!(method, request, rand_album, collection.arc()).await,
		RandSong   => lac!(method, request, rand_song, collection.arc()).await,
		RandEntry  => lac!(method, request, rand_entry, collection.arc()).await,

		//-------------------------------------------------- Search
		Search       => ppacor!(method, request, search, rpc::param::Search, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchArtist => ppacor!(method, request, search_artist, rpc::param::SearchArtist, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchAlbum  => ppacor!(method, request, search_album, rpc::param::SearchAlbum, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchSong   => ppacor!(method, request, search_song, rpc::param::SearchSong, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchEntry  => ppacor!(method, request, search_entry, rpc::param::SearchEntry, collection.arc(), TO_KERNEL, FROM_KERNEL).await,

		//-------------------------------------------------- Playback
		Toggle             => lac!(method, request, toggle, TO_KERNEL).await,
		Play               => lac!(method, request, play, TO_KERNEL).await,
		Pause              => lac!(method, request, pause, TO_KERNEL).await,
		Next               => lac!(method, request, next, TO_KERNEL).await,
		Stop               => lac!(method, request, stop, TO_KERNEL).await,
		Previous           => ppacor!(method, request, previous, rpc::param::Previous, TO_KERNEL).await,
		Shuffle            => lac!(method, request, shuffle, TO_KERNEL).await,
		Clear              => ppacor!(method, request, clear, rpc::param::Clear, TO_KERNEL).await,
		Seek               => ppacor!(method, request, seek, rpc::param::Seek, TO_KERNEL).await,
		Skip               => ppacor!(method, request, skip, rpc::param::Skip, TO_KERNEL).await,
		Back               => ppacor!(method, request, back, rpc::param::Back, TO_KERNEL).await,
		Repeat             => ppacor!(method, request, repeat, rpc::param::Repeat, TO_KERNEL).await,
		Volume             => ppacor!(method, request, volume, rpc::param::Volume, TO_KERNEL).await,
		VolumeUp           => ppacor!(method, request, volume_up, rpc::param::VolumeUp, TO_KERNEL).await,
		VolumeDown         => ppacor!(method, request, volume_down, rpc::param::VolumeDown, TO_KERNEL).await,

		//-------------------------------------------------- Queue
		QueueAddKeyArtist  => ppacor!(method, request, queue_add_key_artist, rpc::param::QueueAddKeyArtist, collection.arc(), TO_KERNEL).await,
		QueueAddKeyAlbum   => ppacor!(method, request, queue_add_key_album, rpc::param::QueueAddKeyAlbum, collection.arc(), TO_KERNEL).await,
		QueueAddKeySong    => ppacor!(method, request, queue_add_key_song, rpc::param::QueueAddKeySong, collection.arc(), TO_KERNEL).await,
		QueueAddMapArtist  => ppacor!(method, request, queue_add_map_artist, rpc::param::QueueAddMapArtist, collection.arc(), TO_KERNEL).await,
		QueueAddMapAlbum   => ppacor!(method, request, queue_add_map_album, rpc::param::QueueAddMapAlbum, collection.arc(), TO_KERNEL).await,
		QueueAddMapSong    => ppacor!(method, request, queue_add_map_song, rpc::param::QueueAddMapSong, collection.arc(), TO_KERNEL).await,
		QueueAddRandArtist => ppacor!(method, request, queue_add_rand_artist, rpc::param::QueueAddRandArtist, collection.arc(), TO_KERNEL).await,
		QueueAddRandAlbum  => ppacor!(method, request, queue_add_rand_album, rpc::param::QueueAddRandAlbum, collection.arc(), TO_KERNEL).await,
		QueueAddRandSong   => ppacor!(method, request, queue_add_rand_song, rpc::param::QueueAddRandSong, collection.arc(), TO_KERNEL).await,
		QueueAddRandEntry  => ppacor!(method, request, queue_add_rand_entry, rpc::param::QueueAddRandEntry, collection.arc(), TO_KERNEL).await,
		QueueAddPlaylist   => ppacor!(method, request, queue_add_playlist, rpc::param::QueueAddPlaylist, collection.arc(), TO_KERNEL).await,
		QueueSetIndex      => ppacor!(method, request, queue_set_index, rpc::param::QueueSetIndex, TO_KERNEL).await,
		QueueRemoveRange   => ppacor!(method, request, queue_remove_range, rpc::param::QueueRemoveRange, TO_KERNEL).await,

		//-------------------------------------------------- Playlist
		PlaylistNew          => ppacor!(method, request, playlist_new, rpc::param::PlaylistNew, collection.arc()).await,
		PlaylistRemove       => ppacor!(method, request, playlist_remove, rpc::param::PlaylistRemove, collection.arc()).await,
		PlaylistClone        => ppacor!(method, request, playlist_clone, rpc::param::PlaylistClone, collection.arc()).await,
		PlaylistGetIndex     => ppacor!(method, request, playlist_get_index, rpc::param::PlaylistGetIndex, collection.arc()).await,
		PlaylistRemoveIndex  => ppacor!(method, request, playlist_remove_index, rpc::param::PlaylistRemoveIndex, collection.arc()).await,
		PlaylistAddKeyArtist => ppacor!(method, request, playlist_add_key_artist, rpc::param::PlaylistAddKeyArtist, collection.arc()).await,
		PlaylistAddKeyAlbum  => ppacor!(method, request, playlist_add_key_album, rpc::param::PlaylistAddKeyAlbum, collection.arc()).await,
		PlaylistAddKeySong   => ppacor!(method, request, playlist_add_key_song, rpc::param::PlaylistAddKeySong, collection.arc()).await,
		PlaylistAddMapArtist => ppacor!(method, request, playlist_add_map_artist, rpc::param::PlaylistAddMapArtist, collection.arc()).await,
		PlaylistAddMapAlbum  => ppacor!(method, request, playlist_add_map_album, rpc::param::PlaylistAddMapAlbum, collection.arc()).await,
		PlaylistAddMapSong   => ppacor!(method, request, playlist_add_map_song, rpc::param::PlaylistAddMapSong, collection.arc()).await,
		PlaylistSingle       => ppacor!(method, request, playlist_single, rpc::param::PlaylistSingle, collection.arc()).await,
		PlaylistBrief        => lac!(method, request, playlist_brief).await,
		PlaylistFull         => lac!(method, request, playlist_full).await,
	}
}

//---------------------------------------------------------------------------------------------------- AUDIO_STATE Lock
// Some tasks aren't too important, and it's much
// more important that `Audio` has quick access to
// `AUDIO_STATE`, so this function is for accessing
// it in a more low priority way.
async fn audio_state_low_priority_lock() -> std::sync::RwLockReadGuard<'static, shukusai::state::AudioState> {
	loop {
		if let Ok(lock) = AUDIO_STATE.try_read() {
			return lock;
		}

		tokio::time::sleep(Duration::from_millis(1)).await;
	}
}

//---------------------------------------------------------------------------------------------------- Collection
async fn collection_new<'a>(
	params:      rpc::param::CollectionNew,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
	FROM_KERNEL: &'static Receiver<KernelToFrontend>,
	TO_ROUTER_C: &'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
) -> Result<Response<Body>, anyhow::Error> {
	tokio::task::block_in_place(move || async move {
		let now = now!();

		// Compare and set `RESETTING`.
		// If it was `true` already, there might be another `task`
		// attempting a `Collection` reset, if so, exit out.
		use std::sync::atomic::Ordering;
		if RESETTING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) != Ok(false) {
			return Ok(resp::resetting(ERR_RESETTING.0, ERR_RESETTING.1, id));
		}

		// `RESETTING` should be `true` from this point onward.
		debug_assert_eq!(atomic_load!(RESETTING), true);

		// Wait until we are the last `task`
		// with this current `Arc<Collection>`
		//
		// Due to some `hyper` closure move stuff,
		// a `task` will have 2 `Arc<Collection>`.
		// Something about `service_fn` maybe being
		// called multiple times, so you can't "move"
		// things into it (even though it's 1 connection
		// per service, so it should only called once? idk).
		//
		// Regardless, wait until we're close enough.
		// `CCD` doesn't deconstruct for `festivald` anyway.
		//
		// `Kernel` + `Audio` + `Search` + `task` + `task` == 5
		loop {
			let sc = Arc::strong_count(&collection);

			if sc > 5 {
				debug!("RPC - collection_new(): strong count == {sc}, waiting...");
				tokio::time::sleep(Duration::from_millis(10)).await;
			} else {
				break;
			}
		}

		// Priority goes to parameter PATHs, then fallback to `collection_paths`,
		// else send empty `Vec`, `shukusai` will handle it and use default the Music directory.
		let paths = match params.paths {
			Some(p) => p,
			None    => config().collection_paths.clone(),
		};

		for p in paths.iter() {
			debug!("RPC - Collection Reset Path: {}", p.display());
		}

		send!(TO_KERNEL, FrontendToKernel::NewCollection(paths));

		// Wait until `Kernel` has given us `Arc<Collection>`.
		let collection = loop {
			match recv!(FROM_KERNEL) {
				KernelToFrontend::NewCollection(c) => break c,
				_ => (),
			}
		};

		// Set RPC cache.
		cache_set_all(&collection).await;

		// We're done resetting.
		atomic_store!(RESETTING, false);

		// Respond to user.
		let r = rpc::resp::CollectionNew {
			time: secs_f64!(now),
			empty: collection.empty,
			timestamp: collection.timestamp,
			count_artist: collection.count_artist.inner(),
			count_album: collection.count_album.inner(),
			count_song: collection.count_song.inner(),
			count_art: collection.count_art.inner(),
		};

		// Send to `Router`.
		// SAFETY: should never panic since the `Receiver` lives forever.
		TO_ROUTER_C.send(collection).await.unwrap();

		Ok(resp::result(r, id))
	}).await
}

async fn collection_brief<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::CollectionBrief {
		empty: collection.empty,
		timestamp: collection.timestamp,
		count_artist: collection.count_artist.inner(),
		count_album: collection.count_album.inner(),
		count_song: collection.count_song.inner(),
		count_art: collection.count_art.inner(),
	};

	Ok(resp::result(resp, id))
}

async fn collection_full<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	Ok(resp::result_cache(&*CACHE_COLLECTION_FULL.read().await, id))
}

async fn collection_brief_artists<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	Ok(resp::result_cache(&*CACHE_COLLECTION_BRIEF_ARTISTS.read().await, id))
}

async fn collection_brief_albums<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	Ok(resp::result_cache(&*CACHE_COLLECTION_BRIEF_ALBUMS.read().await, id))
}

async fn collection_brief_songs<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	Ok(resp::result_cache(&*CACHE_COLLECTION_BRIEF_SONGS.read().await, id))
}

async fn collection_full_artists<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	Ok(resp::result_cache(&*CACHE_COLLECTION_FULL_ARTISTS.read().await, id))
}

async fn collection_full_albums<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	Ok(resp::result_cache(&*CACHE_COLLECTION_FULL_ALBUMS.read().await, id))
}

async fn collection_full_songs<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	Ok(resp::result_cache(&*CACHE_COLLECTION_FULL_SONGS.read().await, id))
}

async fn collection_entries<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	Ok(resp::result_cache(&*CACHE_COLLECTION_ENTRIES.read().await, id))
}

async fn collection_perf<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	use disk::Json;

	let Ok(perf) = shukusai::perf::Perf::from_file() else {
		return Ok(resp::error(ERR_PERF.0, ERR_PERF.1, id));
	};

	let resp = rpc::resp::CollectionPerf {
		bytes: perf.total.bytes,
		user: perf.total.user,
		sys: perf.total.sys,
	};

	Ok(resp::result(resp, id))
}

async fn collection_health<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let mut vec = vec![];

	for song in collection.songs.iter() {
		if !song.path.exists() {
			let album  = &collection.albums[song.album];
			let artist = &collection.artists[album.artist];

			let json = shukusai::collection::EntryJson {
				artist: Cow::Borrowed(&artist.name),
				album: Cow::Borrowed(&album.title),
				song: Cow::Borrowed(&song.title),
				key_artist: ArtistKey::from(album.artist),
				key_album: AlbumKey::from(song.album),
				key_song: SongKey::from(song.key),
				path: Cow::Borrowed(song.path.as_path()),
			};
			vec.push(json);
		}
	}

	let resp = serde_json::json!({
		"all_ok": vec.is_empty(),
		"song_len": collection.songs.len(),
		"missing_len": vec.len(),
		"missing": vec,
	});

	Ok(resp::result(resp, id))
}

async fn collection_resource_size<'a>(
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	tokio::task::block_in_place(move || async move {
		// Audio size.
		let mut audio = 0;
		for song in collection.songs.iter() {
			let Ok(metadata) = tokio::fs::metadata(&song.path).await else {
				return Ok(resp::error(ERR_FS.0, ERR_FS.1, id));
			};

			audio += metadata.len();
		}

		// Art size.
		let mut art = 0;
		for album in collection.albums.iter() {
			if let shukusai::collection::Art::Known { len, .. } = &album.art {
				art += len;
			}
		}

		let resp = rpc::resp::CollectionResourceSize {
			audio,
			art,
		};

		Ok(resp::result(resp, id))
	}).await
}

//---------------------------------------------------------------------------------------------------- State
async fn state_audio<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let shukusai::state::AudioState {
		queue,
		queue_idx,
		playing,
		song,
		elapsed,
		runtime,
		repeat,
		volume,
	} = audio_state_low_priority_lock().await.clone();

	let song_key = song;
	let song = if let Some(key) = song_key {
		let song = &collection.songs[key];
		Some(song)
	} else {
		None
	};

	let resp = serde_json::json!({
		"queue": queue,
		"queue_len": queue.len(),
		"queue_idx": queue_idx,
		"playing": playing,
		"song_key": song_key,
		"elapsed": elapsed.inner(),
		"runtime": runtime.inner(),
		"repeat": repeat,
		"volume": volume.inner(),
		"song": song,
	});

	Ok(resp::result(resp, id))
}

async fn state_queue_key<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let queue = audio_state_low_priority_lock().await.queue.clone();

	let resp = serde_json::json!({
		"len": queue.len(),
		"keys": queue,
	});

	Ok(resp::result(resp, id))
}

async fn state_queue_song<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let queue = audio_state_low_priority_lock().await.queue.clone();

	let vec: Vec<&Song> = queue
		.iter()
		.map(|key| &collection.songs[key])
		.collect();

	let resp = serde_json::json!({
		"len": vec.len(),
		"songs": vec,
	});

	Ok(resp::result(resp, id))
}

async fn state_queue_entry<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let queue = audio_state_low_priority_lock().await.queue.clone();

	let vec: Vec<shukusai::collection::EntryJson> = queue
		.iter()
		.map(|key| {
			let (artist, album, song) = collection.walk(key);
			shukusai::collection::EntryJson {
				path: Cow::Borrowed(&song.path),
				key_artist: artist.key,
				key_album: album.key,
				key_song: song.key,
				artist: Cow::Borrowed(&*artist.name),
				album: Cow::Borrowed(&*album.title),
				song: Cow::Borrowed(&*song.title),
			}
		})
		.collect();

	let resp = serde_json::json!({
		"len": vec.len(),
		"entries": vec,
	});

	Ok(resp::result(resp, id))
}

async fn state_playing<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::StatePlaying {
		playing: audio_state_low_priority_lock().await.playing,
	};

	Ok(resp::result(resp, id))
}

async fn state_repeat<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::StateRepeat {
		mode: audio_state_low_priority_lock().await.repeat,
	};

	Ok(resp::result(resp, id))
}

async fn state_runtime<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let lock = audio_state_low_priority_lock().await;
	let elapsed = lock.elapsed;
	let runtime = lock.runtime;
	drop(lock);

	let resp = rpc::resp::StateRuntime {
		elapsed: elapsed.inner(),
		runtime: runtime.inner(),
		elapsed_readable: Cow::Borrowed(elapsed.as_str()),
		runtime_readable: Cow::Borrowed(runtime.as_str()),
	};

	Ok(resp::result(resp, id))
}

async fn state_volume<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::StateVolume {
		volume: shukusai::audio::Volume::new(atomic_load!(shukusai::state::VOLUME)),
	};

	Ok(resp::result(resp, id))
}

//---------------------------------------------------------------------------------------------------- Daemon
async fn daemon_config<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let c = config();

	let resp = serde_json::json!({
		"ip":                  c.ip,
		"port":                c.port,
		"max_connections":     c.max_connections,
		"exclusive_ips":       c.exclusive_ips.as_ref().map(|h| Cow::Borrowed(h)),
		"sleep_on_fail":       c.sleep_on_fail.clone(),
		"collection_paths":    Cow::Borrowed(&c.collection_paths),
		"tls":                 c.tls,
		"certificate":         c.certificate.as_ref().map(|p| Cow::Borrowed(p.as_path())),
		"key":                 c.key.as_ref().map(|p| Cow::Borrowed(p.as_path())),
		"rest":                c.rest,
		"docs":                c.docs,
		"direct_download":     c.direct_download,
		"filename_separator":  Cow::Borrowed(&c.filename_separator),
		"log_level":           c.log_level.clone(),
		"watch":               c.watch,
		"cache_clean":         c.cache_clean,
		"cache_time":          c.cache_time,
		"restore_audio_state": c.restore_audio_state,
		"media_controls":      c.media_controls,
		"authorization":       AUTH.get().is_some(),
		"confirm_no_tls_auth": c.confirm_no_tls_auth,
		"no_auth_rpc":         c.no_auth_rpc.as_ref().map(|h| Cow::Borrowed(h)),
		"no_auth_rest":        c.no_auth_rest.as_ref().map(|h| Cow::Borrowed(h)),
		"no_auth_docs":        c.no_auth_docs,
	});

	Ok(resp::result(resp, id))
}

async fn daemon_save<'a>(
	id:        Option<Id<'a>>,
	TO_KERNEL: &'static Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	let volume = shukusai::audio::Volume::new(atomic_load!(shukusai::state::VOLUME));
	send!(TO_KERNEL, shukusai::kernel::FrontendToKernel::Volume(volume));

	tokio::task::block_in_place(|| async move {
		// Save `Playlists`.
		if PLAYLISTS.read().save_atomic().is_err() {
			return Ok(resp::error(ERR_FS.0, ERR_FS.1, id));
		}

		// Save `AudioState`.
		if AUDIO_STATE.read().save_atomic().is_err() {
			return Ok(resp::error(ERR_FS.0, ERR_FS.1, id));
		}

		Ok(resp::result_ok(id))
	}).await
}

async fn daemon_remove_cache<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let Ok(project) = crate::zip::CollectionZip::project_dir_path() else {
		return Ok(resp::error(ERR_FS.0, ERR_FS.1, id));
	};

	let mut vec: Vec::<rpc::resp::DaemonRemoveCacheInner> = vec![];

	for entry in walkdir::WalkDir::new(project).into_iter().filter_map(|e| e.ok()) {
		if !entry.file_type().is_file() {
			continue;
		}

		let path = entry.into_path();

		let Ok(metadata) = tokio::fs::metadata(&path).await else {
			warn!("RPC - disk_remove_cache(): metadata error, skipping {}", path.display());
			continue;
		};

		let bytes = metadata.len();

		if tokio::fs::remove_file(&path).await.is_err() {
			warn!("RPC - disk_remove_cache(): remove error, skipping {}", path.display());
		}

		let resp = rpc::resp::DaemonRemoveCacheInner {
			path: Cow::Owned(path),
			bytes,
		};

		vec.push(resp);
	}

	Ok(resp::result(vec, id))
}

async fn daemon_shutdown<'a>(
	id:          Option<Id<'a>>,
	TO_ROUTER_S: &'static tokio::sync::mpsc::Sender::<()>,
) -> Result<Response<Body>, anyhow::Error> {
	match TO_ROUTER_S.send(()).await {
		Ok(_) => {
			let resp = rpc::resp::DaemonShutdown {
				uptime:            shukusai::logger::uptime(),
				uptime_readable:   Cow::Owned(readable::Time::from(shukusai::logger::uptime()).into_string()),
				total_requests:    atomic_load!(TOTAL_REQUESTS),
				total_connections: atomic_load!(TOTAL_CONNECTIONS),
			};
			Ok(resp::result(resp, id))
		},
		Err(_) => Ok(resp::internal_error(id)),
	}
}

async fn daemon_state<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::DaemonState {
		uptime:              shukusai::logger::uptime(),
		uptime_readable:     Cow::Owned(readable::Time::from(shukusai::logger::uptime()).into_string()),
		saving:              shukusai::state::saving(),
		total_requests:      atomic_load!(TOTAL_REQUESTS),
		total_connections:   atomic_load!(TOTAL_CONNECTIONS),
		current_connections: crate::statics::connections(),
		rest:                config().rest,
		docs:                config().docs,
		direct_download:     config().direct_download,
		authorization:       AUTH.get().is_some(),
		version:             Cow::Borrowed(FESTIVALD_VERSION),
		commit:              Cow::Borrowed(COMMIT),
		os:                  Cow::Borrowed(OS_ARCH),
	};

	Ok(resp::result(resp, id))
}

async fn daemon_seen_ips<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let seen = crate::seen::SEEN_IPS.read().await.clone();

	let mut vec = Vec::with_capacity(seen.len());

	for (ip, count) in seen.into_iter() {
		let inner = rpc::resp::DaemonSeenIpsInner {
			ip,
			count,
		};
		vec.push(inner);
	}

	let resp = rpc::resp::DaemonSeenIps(Cow::Owned(vec));

	Ok(resp::result(resp, id))
}

async fn daemon_methods<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	use strum::{EnumCount,VariantNames};

	let resp = serde_json::json!({
		"len": rpc::Method::COUNT,
		"methods": rpc::Method::VARIANTS,
	});

	Ok(resp::result(resp, id))
}

async fn daemon_no_auth_rpc<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let rpc = &config().no_auth_rpc;
	let len = match rpc {
		Some(r) => r.len(),
		None    => 0,
	};

	let btree = BTreeSet::new();

	let resp = serde_json::json!({
		"len": len,
		"rpc": rpc.as_ref().unwrap_or_else(|| &btree),
	});

	Ok(resp::result(resp, id))
}

async fn daemon_no_auth_rest<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let rest = &config().no_auth_rest;
	let len = match rest {
		Some(r) => r.len(),
		None    => 0,
	};

	let btree = BTreeSet::new();

	let resp = serde_json::json!({
		"len": len,
		"rest": rest.as_ref().unwrap_or_else(|| &btree),
	});

	Ok(resp::result(resp, id))
}

//---------------------------------------------------------------------------------------------------- Key (exact key)
async fn key_artist<'a>(
	params:     rpc::param::KeyArtist,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(r) = collection.artists.get(params.key.into()) {
		Ok(resp::result(serde_json::json!({ "artist": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_ARTIST.0, ERR_KEY_ARTIST.1, id))
	}
}

async fn key_album<'a>(
	params:     rpc::param::KeyAlbum,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(r) = collection.albums.get(params.key.into()) {
		Ok(resp::result(serde_json::json!({ "album": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_ALBUM.0, ERR_KEY_ALBUM.1, id))
	}
}

async fn key_song<'a>(
	params:     rpc::param::KeySong,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(r) = collection.songs.get(params.key.into()) {
		Ok(resp::result(serde_json::json!({ "song": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id))
	}
}

async fn key_entry<'a>(
	params:     rpc::param::KeyEntry,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(s) = collection.songs.get(params.key.into()) {
		let r = shukusai::collection::EntryJson::from_song(s.key, &collection);
		Ok(resp::result(serde_json::json!({ "entry": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id))
	}
}

async fn key_artist_albums<'a>(
	params:     rpc::param::KeyArtistAlbums,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = ArtistKey::from(params.key);
	if collection.artists.get(key).is_some() {
		let r = collection.artist_albums(key);
		Ok(resp::result(serde_json::json!({ "len": r.len(), "albums": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_ARTIST.0, ERR_KEY_ARTIST.1, id))
	}
}

async fn key_artist_songs<'a>(
	params:     rpc::param::KeyArtistSongs,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = ArtistKey::from(params.key);
	if collection.artists.get(key).is_some() {
		let r = collection.artist_songs(key);
		Ok(resp::result(serde_json::json!({ "len": r.len(), "songs": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_ARTIST.0, ERR_KEY_ARTIST.1, id))
	}
}

async fn key_artist_entries<'a>(
	params:     rpc::param::KeyArtistEntries,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = ArtistKey::from(params.key);
	if let Some(artist) = collection.artists.get(key) {
		let r: Box<[shukusai::collection::EntryJson]> = artist.songs
			.iter()
			.map(|k| shukusai::collection::EntryJson::from_song(*k, &collection))
			.collect();
		Ok(resp::result(serde_json::json!({ "len": r.len(), "entries": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_ARTIST.0, ERR_KEY_ARTIST.1, id))
	}
}

async fn key_album_artist<'a>(
	params:     rpc::param::KeyAlbumArtist,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = AlbumKey::from(params.key);
	if let Some(album) = collection.albums.get(key) {
		let r = &collection.artists[album.artist];
		Ok(resp::result(serde_json::json!({ "artist": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_ALBUM.0, ERR_KEY_ALBUM.1, id))
	}
}

async fn key_album_songs<'a>(
	params:     rpc::param::KeyAlbumSongs,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = AlbumKey::from(params.key);
	if collection.albums.get(key).is_some() {
		let r = collection.album_songs(key);
		Ok(resp::result(serde_json::json!({ "len": r.len(), "songs": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_ALBUM.0, ERR_KEY_ALBUM.1, id))
	}
}

async fn key_album_entries<'a>(
	params:     rpc::param::KeyAlbumEntries,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = AlbumKey::from(params.key);
	if let Some(album) = collection.albums.get(key) {
		let r: Box<[shukusai::collection::EntryJson]> = album.songs
			.iter()
			.map(|k| shukusai::collection::EntryJson::from_song(*k, &collection))
			.collect();
		Ok(resp::result(serde_json::json!({ "len": r.len(), "entries": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_ALBUM.0, ERR_KEY_ALBUM.1, id))
	}
}

async fn key_song_artist<'a>(
	params:     rpc::param::KeySongArtist,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = SongKey::from(params.key);
	if let Some(song) = collection.songs.get(key) {
		let (artist, _, _) = &collection.walk(key);
		Ok(resp::result(serde_json::json!({ "artist": artist }), id))
	} else {
		Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id))
	}
}

async fn key_song_album<'a>(
	params:     rpc::param::KeySongAlbum,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = SongKey::from(params.key);
	if let Some(song) = collection.songs.get(key) {
		let album = &collection.albums[song.album];
		Ok(resp::result(serde_json::json!({ "album": album }), id))
	} else {
		Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id))
	}
}

async fn key_other_albums<'a>(
	params:     rpc::param::KeyOtherAlbums,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = AlbumKey::from(params.key);
	if let Some(album) = collection.albums.get(key) {
		let r: Box<[&Album]> = collection
			.other_albums(key)
			.iter()
			.map(|k| &collection.albums[*k])
			.collect();
		Ok(resp::result(serde_json::json!({ "len": r.len(), "albums": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_ALBUM.0, ERR_KEY_ALBUM.1, id))
	}
}

async fn key_other_songs<'a>(
	params:     rpc::param::KeyOtherSongs,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = SongKey::from(params.key);
	if let Some(song) = collection.songs.get(key) {
		let r: Box<[&Song]> = collection
			.other_songs(key)
			.iter()
			.map(|k| &collection.songs[*k])
			.collect();
		Ok(resp::result(serde_json::json!({ "len": r.len(), "songs": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id))
	}
}

async fn key_other_entries<'a>(
	params:     rpc::param::KeyOtherEntries,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = SongKey::from(params.key);
	if let Some(song) = collection.songs.get(key) {
		let r: Box<[shukusai::collection::EntryJson]> = collection
			.other_songs(key)
			.iter()
			.map(|k| shukusai::collection::EntryJson::from_song(*k, &collection))
			.collect();
		Ok(resp::result(serde_json::json!({ "len": r.len(), "entries": r }), id))
	} else {
		Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id))
	}
}

//---------------------------------------------------------------------------------------------------- Map (exact hashmap)
async fn map_artist<'a>(
	params:     rpc::param::MapArtist<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((r, _)) = collection.artist(params.artist) {
		Ok(resp::result(serde_json::json!({ "artist": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_ARTIST.0, ERR_MAP_ARTIST.1, id))
	}
}

async fn map_album<'a>(
	params:     rpc::param::MapAlbum<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((r, _)) = collection.album(params.artist, params.album) {
		Ok(resp::result(serde_json::json!({ "album": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_ALBUM.0, ERR_MAP_ALBUM.1, id))
	}
}

async fn map_song<'a>(
	params:     rpc::param::MapSong<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((r, _)) = collection.song(params.artist, params.album, params.song) {
		Ok(resp::result(serde_json::json!({ "song": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_SONG.0, ERR_MAP_SONG.1, id))
	}
}

async fn map_entry<'a>(
	params:     rpc::param::MapEntry<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((r, _)) = collection.song(params.artist, params.album, params.song) {
		let r = shukusai::collection::EntryJson::from_song(r.key, &collection);
		Ok(resp::result(serde_json::json!({ "entry": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_SONG.0, ERR_MAP_SONG.1, id))
	}
}

async fn map_artist_albums<'a>(
	params:     rpc::param::MapArtistAlbums<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((artist, _)) = collection.artist(params.artist) {
		let r: Box<[&Album]> = artist.albums
			.iter()
			.map(|k| &collection.albums[*k])
			.collect();
		Ok(resp::result(serde_json::json!({ "len": r.len(), "albums": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_ARTIST.0, ERR_MAP_ARTIST.1, id))
	}
}

async fn map_artist_songs<'a>(
	params:     rpc::param::MapArtistSongs<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((artist, _)) = collection.artist(params.artist) {
		let r: Box<[&Song]> = artist.songs
			.iter()
			.map(|k| &collection.songs[*k])
			.collect();
		Ok(resp::result(serde_json::json!({ "len": r.len(), "songs": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_ARTIST.0, ERR_MAP_ARTIST.1, id))
	}
}

async fn map_artist_entries<'a>(
	params:     rpc::param::MapArtistEntries<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((artist, _)) = collection.artist(params.artist) {
		let r: Box<[shukusai::collection::EntryJson]> = artist.songs
			.iter()
			.map(|k| shukusai::collection::EntryJson::from_song(*k, &collection))
			.collect();
		Ok(resp::result(serde_json::json!({ "len": r.len(), "entries": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_ARTIST.0, ERR_MAP_ARTIST.1, id))
	}
}

async fn map_album_songs<'a>(
	params:     rpc::param::MapAlbumSongs<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((album, _)) = collection.album(params.artist, params.album) {
		let r: Box<[&Song]> = album.songs
			.iter()
			.map(|k| &collection.songs[*k])
			.collect();
		Ok(resp::result(serde_json::json!({ "len": r.len(), "songs": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_ALBUM.0, ERR_MAP_ALBUM.1, id))
	}
}

async fn map_album_entries<'a>(
	params:     rpc::param::MapAlbumEntries<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((album, _)) = collection.album(params.artist, params.album) {
		let r: Box<[shukusai::collection::EntryJson]> = album.songs
			.iter()
			.map(|k| shukusai::collection::EntryJson::from_song(*k, &collection))
			.collect();
		Ok(resp::result(serde_json::json!({ "len": r.len(), "entries": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_ALBUM.0, ERR_MAP_ALBUM.1, id))
	}
}

//---------------------------------------------------------------------------------------------------- Current (audio state)
async fn current_artist<'a>(
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let song = audio_state_low_priority_lock().await.song.clone();

	if let Some(key) = song {
		let (r, _) = collection.artist_from_song(key);
		Ok(resp::result(serde_json::json!({ "artist": r }), id))
	} else {
		Ok(resp::error(ERR_CURRENT.0, ERR_CURRENT.1, id))
	}
}

async fn current_album<'a>(
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let song = audio_state_low_priority_lock().await.song.clone();

	if let Some(key) = song {
		let (r, _) = collection.album_from_song(key);
		Ok(resp::result(serde_json::json!({ "album": r }), id))
	} else {
		Ok(resp::error(ERR_CURRENT.0, ERR_CURRENT.1, id))
	}
}

async fn current_song<'a>(
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let song = audio_state_low_priority_lock().await.song.clone();

	if let Some(key) = song {
		let r = &collection.songs[key];
		Ok(resp::result(serde_json::json!({ "song": r }), id))
	} else {
		Ok(resp::error(ERR_CURRENT.0, ERR_CURRENT.1, id))
	}
}

async fn current_entry<'a>(
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let song = audio_state_low_priority_lock().await.song.clone();

	if let Some(key) = song {
		let r = &collection.songs[key];
		let r = shukusai::collection::EntryJson::from_song(r.key, &collection);
		Ok(resp::result(serde_json::json!({ "entry": r }), id))
	} else {
		Ok(resp::error(ERR_CURRENT.0, ERR_CURRENT.1, id))
	}
}

//---------------------------------------------------------------------------------------------------- Rand (rng)
async fn rand_artist<'a>(
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_artist(None) {
		let r = &collection.artists[key];
		Ok(resp::result(serde_json::json!({ "artist": r }), id))
	} else {
		Ok(resp::error(ERR_RAND.0, ERR_RAND.1, id))
	}
}

async fn rand_album<'a>(
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_album(None) {
		let r = &collection.albums[key];
		Ok(resp::result(serde_json::json!({ "album": r }), id))
	} else {
		Ok(resp::error(ERR_RAND.0, ERR_RAND.1, id))
	}
}

async fn rand_song<'a>(
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_song(None) {
		let r = &collection.songs[key];
		Ok(resp::result(serde_json::json!({ "song": r }), id))
	} else {
		Ok(resp::error(ERR_RAND.0, ERR_RAND.1, id))
	}
}

async fn rand_entry<'a>(
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_song(None) {
		let r = &collection.songs[key];
		let r = shukusai::collection::EntryJson::from_song(r.key, &collection);
		Ok(resp::result(serde_json::json!({ "entry": r }), id))
	} else {
		Ok(resp::error(ERR_RAND.0, ERR_RAND.1, id))
	}
}

//---------------------------------------------------------------------------------------------------- Search (fuzzy string)
// Implement the generic part of `search`.
// Acquires and holds onto a `kernel_lock` for the entire time,
// returns the search `Keychain`.
macro_rules! impl_search {
	($params:expr, $id:expr, $to_kernel:expr, $from_kernel:expr) => {{
		// Acquire `Kernel` lock.
		let kernel_lock = loop {
			match crate::statics::KERNEL_LOCK.try_lock() {
				Ok(lock) => break lock,
				_ => tokio::time::sleep(Duration::from_millis(1)).await,
			}
		};

		// Send `Search` signal to `Kernel`.
		send!($to_kernel, FrontendToKernel::Search(($params.input.into(), $params.kind)));

		// Receive from `Kernel`.
		let keychain = 'outer: loop {
			let msg = 'inner: loop {
				match $from_kernel.try_recv() {
					Ok(msg) => break 'inner msg,
					_ => tokio::time::sleep(Duration::from_millis(1)).await,
				}
			};

			// INVARIANT: This _must_ be `SearchResp` or our `KERNEL_LOCK` workaround isn't working.
			if let KernelToFrontend::SearchResp(keychain) = msg {
				break 'outer keychain;
			};
		};

		keychain
	}}
}

async fn search<'a>(
	params:      rpc::param::Search<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
	FROM_KERNEL: &'static Receiver<KernelToFrontend>,
) -> Result<Response<Body>, anyhow::Error> {
	let keychain = impl_search!(params, id, TO_KERNEL, FROM_KERNEL);

	// Collect objects.
	// FIXME: Maybe we can serialize directly off iter instead of boxing?
	let artists: Box<[&Artist]> = keychain.artists.iter().map(|k| &collection.artists[k]).collect();
	let albums:  Box<[&Album]>  = keychain.albums.iter().map(|k| &collection.albums[k]).collect();
	let songs:   Box<[&Song]>   = keychain.songs.iter().map(|k| &collection.songs[k]).collect();

	// Turn in response.
	let resp = serde_json::json!({
		"artists": artists,
		"albums": albums,
		"songs": songs,
	});

	Ok(resp::result(resp, id))
}

async fn search_artist<'a>(
	params:      rpc::param::SearchArtist<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
	FROM_KERNEL: &'static Receiver<KernelToFrontend>,
) -> Result<Response<Body>, anyhow::Error> {
	let keychain = impl_search!(params, id, TO_KERNEL, FROM_KERNEL);

	let slice: Box<[&Artist]> = keychain.artists.iter().map(|k| &collection.artists[k]).collect();

	let resp = serde_json::json!({"artists": slice});

	Ok(resp::result(resp, id))
}

async fn search_album<'a>(
	params:      rpc::param::SearchAlbum<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
	FROM_KERNEL: &'static Receiver<KernelToFrontend>,
) -> Result<Response<Body>, anyhow::Error> {
	let keychain = impl_search!(params, id, TO_KERNEL, FROM_KERNEL);

	let slice: Box<[&Album]> = keychain.albums.iter().map(|k| &collection.albums[k]).collect();

	let resp = serde_json::json!({"albums": slice});

	Ok(resp::result(resp, id))
}

async fn search_song<'a>(
	params:      rpc::param::SearchSong<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
	FROM_KERNEL: &'static Receiver<KernelToFrontend>,
) -> Result<Response<Body>, anyhow::Error> {
	let keychain = impl_search!(params, id, TO_KERNEL, FROM_KERNEL);

	let slice: Box<[&Song]> = keychain.songs.iter().map(|k| &collection.songs[k]).collect();

	let resp = serde_json::json!({"songs": slice});

	Ok(resp::result(resp, id))
}

async fn search_entry<'a>(
	params:      rpc::param::SearchEntry<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
	FROM_KERNEL: &'static Receiver<KernelToFrontend>,
) -> Result<Response<Body>, anyhow::Error> {
	let keychain = impl_search!(params, id, TO_KERNEL, FROM_KERNEL);

	let entries: Box<[shukusai::collection::EntryJson]> = keychain.songs
		.iter()
		.map(|k| shukusai::collection::EntryJson::from_song(*k, &collection))
		.collect();

	let resp = serde_json::json!({"entries": entries});

	Ok(resp::result(resp, id))
}

//---------------------------------------------------------------------------------------------------- Playback
// Implement the function <-> `FrontendToKernel` signal mappings with no params.
macro_rules! impl_signal {
	($($func:ident, $signal:expr),*) => {
		$(
			async fn $func<'a>(
				id: Option<Id<'a>>,
				TO_KERNEL: &Sender<FrontendToKernel>
			) -> Result<Response<Body>, anyhow::Error> {
				send!(TO_KERNEL, $signal);
				Ok(resp::result_ok(id))
			}
		)*
	}
}

impl_signal! {
	toggle,       FrontendToKernel::Toggle,
	play,         FrontendToKernel::Play,
	pause,        FrontendToKernel::Pause,
	next,         FrontendToKernel::Next,
	shuffle,      FrontendToKernel::Shuffle
}

async fn previous<'a>(
	params:    rpc::param::Previous,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	send!(TO_KERNEL, FrontendToKernel::Previous(params.threshold));
	Ok(resp::result_ok(id))
}

async fn clear<'a>(
	params:    rpc::param::Clear,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let len = audio_state_low_priority_lock().await.queue.len();
	let resp = rpc::resp::Clear { len };
	if len != 0 {
		send!(TO_KERNEL, FrontendToKernel::Clear(params.playback));
	}
	Ok(resp::result(resp, id))
}

async fn stop<'a>(
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let len = audio_state_low_priority_lock().await.queue.len();
	let resp = rpc::resp::Stop { len };
	if len != 0 {
		send!(TO_KERNEL, FrontendToKernel::Stop);
	}
	Ok(resp::result(resp, id))
}

async fn seek<'a>(
	params:    rpc::param::Seek,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	send!(TO_KERNEL, FrontendToKernel::Seek((params.kind, params.second)));
	Ok(resp::result_ok(id))
}

async fn skip<'a>(
	params:   rpc::param::Skip,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	send!(TO_KERNEL, FrontendToKernel::Skip(params.skip));
	Ok(resp::result_ok(id))
}

async fn back<'a>(
	params:    rpc::param::Back,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	send!(TO_KERNEL, FrontendToKernel::Back(params.back));
	Ok(resp::result_ok(id))
}

async fn repeat<'a>(
	params:    rpc::param::Repeat,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	let current  = params.mode;
	let previous = audio_state_low_priority_lock().await.repeat;
	let resp = rpc::resp::Repeat {
		previous,
		current,
	};
	if previous != current {
		send!(TO_KERNEL, FrontendToKernel::Repeat(current));
	}
	Ok(resp::result(resp, id))
}

async fn volume<'a>(
	params:    rpc::param::Volume,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let current  = shukusai::audio::Volume::new(if params.volume > 100 { 100 } else { params.volume });
	let previous = shukusai::audio::Volume::new(atomic_load!(shukusai::state::VOLUME));
	send!(TO_KERNEL, FrontendToKernel::Volume(current));
	let resp = rpc::resp::Volume {
		previous,
		current,
	};
	Ok(resp::result(resp, id))
}

async fn volume_up<'a>(
	params:    rpc::param::VolumeUp,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let previous = shukusai::audio::Volume::new(atomic_load!(shukusai::state::VOLUME));
	let current  = previous.inner() + params.up;
	let current  = shukusai::audio::Volume::new(if current > 100 { 100 } else { current });
	send!(TO_KERNEL, FrontendToKernel::Volume(current));
	let resp = rpc::resp::VolumeUp {
		previous,
		current,
	};
	Ok(resp::result(resp, id))
}

async fn volume_down<'a>(
	params:    rpc::param::VolumeDown,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let previous = shukusai::audio::Volume::new(atomic_load!(shukusai::state::VOLUME));
	let current  = shukusai::audio::Volume::new(previous.inner().saturating_sub(params.down));
	send!(TO_KERNEL, FrontendToKernel::Volume(current));
	let resp = rpc::resp::VolumeDown {
		previous,
		current,
	};
	Ok(resp::result(resp, id))
}

//---------------------------------------------------------------------------------------------------- Queue
macro_rules! get_append {
	($params:expr, $id:expr) => {
		match $params.append {
			shukusai::audio::Append2::Index => {
				let Some(i) = $params.index else {
					return Ok(resp::error(ERR_APPEND.0, ERR_APPEND.1, $id));
				};

				if i != 0 && i >= AUDIO_STATE.read().queue.len() {
					return Ok(resp::error(ERR_INDEX.0, ERR_INDEX.1, $id));
				}

				shukusai::audio::Append::Index(i)
			},
			shukusai::audio::Append2::Front => shukusai::audio::Append::Front,
			shukusai::audio::Append2::Back => shukusai::audio::Append::Back,
		}
	}
}

macro_rules! get_offset {
	($offset:expr, $len:expr, $id:expr) => {
		match $offset {
			None => 0,
			Some(o) => {
				if o != 0 && o >= $len {
					return Ok(resp::error(ERR_OFFSET.0, ERR_OFFSET.1, $id));
				}
				o
			},
		}
	}
}

// Some
// Or
// False
//
// Unwrap a `Option<bool>`, else `false`.
fn sof(o: Option<bool>) -> bool {
	o.unwrap_or_else(|| false)
}

async fn queue_add_key_artist<'a>(
	params:     rpc::param::QueueAddKeyArtist,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	let key = ArtistKey::from(params.key);
	if let Some(x) = collection.artists.get(key) {
		let append = get_append!(params, id);
		let offset = get_offset!(params.offset, x.songs.len(), id);

		send!(TO_KERNEL, FrontendToKernel::QueueAddArtist((key, append, sof(params.clear), sof(params.play), offset)));

		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_KEY_ARTIST.0, ERR_KEY_ARTIST.1, id))
	}
}

async fn queue_add_key_album<'a>(
	params:     rpc::param::QueueAddKeyAlbum,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	let key = AlbumKey::from(params.key);
	if let Some(x) = collection.albums.get(key) {
		let append = get_append!(params, id);
		let offset = get_offset!(params.offset, x.songs.len(), id);

		send!(TO_KERNEL, FrontendToKernel::QueueAddAlbum((key, append, sof(params.clear), sof(params.play), offset)));

		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_KEY_ALBUM.0, ERR_KEY_ALBUM.1, id))
	}
}

async fn queue_add_key_song<'a>(
	params:     rpc::param::QueueAddKeySong,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	let key = SongKey::from(params.key);
	if let Some(x) = collection.songs.get(key) {
		let append = get_append!(params, id);

		send!(TO_KERNEL, FrontendToKernel::QueueAddSong((key, append, sof(params.clear), sof(params.play))));

		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id))
	}
}

async fn queue_add_map_artist<'a>(
	params:     rpc::param::QueueAddMapArtist<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((x, key)) = collection.artist(params.artist) {
		let append = get_append!(params, id);
		let offset = get_offset!(params.offset, x.songs.len(), id);

		send!(TO_KERNEL, FrontendToKernel::QueueAddArtist((key, append, sof(params.clear), sof(params.play), offset)));

		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_MAP_ARTIST.0, ERR_MAP_ARTIST.1, id))
	}
}

async fn queue_add_map_album<'a>(
	params:     rpc::param::QueueAddMapAlbum<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((x, key)) = collection.album(params.artist, params.album) {
		let append = get_append!(params, id);
		let offset = get_offset!(params.offset, x.songs.len(), id);

		send!(TO_KERNEL, FrontendToKernel::QueueAddAlbum((key, append, sof(params.clear), sof(params.play), offset)));

		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_MAP_ALBUM.0, ERR_MAP_ALBUM.1, id))
	}
}

async fn queue_add_map_song<'a>(
	params: rpc::param::QueueAddMapSong<'a>,
	id: Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((_, key)) = collection.song(params.artist, params.album, params.song) {
		let append = get_append!(params, id);

		send!(TO_KERNEL, FrontendToKernel::QueueAddSong((key, append, sof(params.clear), sof(params.play))));

		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_MAP_SONG.0, ERR_MAP_SONG.1, id))
	}
}

async fn queue_add_rand_artist<'a>(
	params:     rpc::param::QueueAddRandArtist,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_artist(None) {
		let x = &collection.artists[key];

		let append = get_append!(params, id);
		let offset = get_offset!(params.offset, x.songs.len(), id);

		send!(TO_KERNEL, FrontendToKernel::QueueAddArtist((key, append, sof(params.clear), sof(params.play), offset)));

		Ok(resp::result(serde_json::json!({ "artist": x }), id))
	} else {
		Ok(resp::error(ERR_MAP_ARTIST.0, ERR_MAP_ARTIST.1, id))
	}
}

async fn queue_add_rand_album<'a>(
	params:     rpc::param::QueueAddRandAlbum,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_album(None) {
		let x = &collection.albums[key];

		let append = get_append!(params, id);
		let offset = get_offset!(params.offset, x.songs.len(), id);

		send!(TO_KERNEL, FrontendToKernel::QueueAddAlbum((key, append, sof(params.clear), sof(params.play), offset)));

		Ok(resp::result(serde_json::json!({ "album": x }), id))
	} else {
		Ok(resp::error(ERR_MAP_ALBUM.0, ERR_MAP_ALBUM.1, id))
	}
}

async fn queue_add_rand_song<'a>(
	params: rpc::param::QueueAddRandSong,
	id: Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_song(None) {
		let x = &collection.songs[key];

		let append = get_append!(params, id);

		send!(TO_KERNEL, FrontendToKernel::QueueAddSong((key, append, sof(params.clear), sof(params.play))));

		Ok(resp::result(serde_json::json!({ "song": x }), id))
	} else {
		Ok(resp::error(ERR_RAND.0, ERR_RAND.1, id))
	}
}

async fn queue_add_rand_entry<'a>(
	params: rpc::param::QueueAddRandEntry,
	id: Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_song(None) {
		let x = &collection.songs[key];

		let append = get_append!(params, id);

		send!(TO_KERNEL, FrontendToKernel::QueueAddSong((key, append, sof(params.clear), sof(params.play))));

		Ok(resp::result(serde_json::json!({ "entry": shukusai::collection::EntryJson::from_song(key, &collection) }), id))
	} else {
		Ok(resp::error(ERR_RAND.0, ERR_RAND.1, id))
	}
}

async fn queue_add_playlist<'a>(
	params: rpc::param::QueueAddPlaylist<'a>,
	id: Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(playlist) = PLAYLISTS.read().get(&*params.playlist) {
		let append = get_append!(params, id);
		let offset = get_offset!(params.offset, playlist.len(), id);

		let playlist: Arc<str> = params.playlist.into();

		send!(TO_KERNEL, FrontendToKernel::QueueAddPlaylist((playlist, append, sof(params.clear), sof(params.play), offset)));

		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_PLAYLIST.0, ERR_PLAYLIST.1, id))
	}
}

async fn queue_set_index<'a>(
	params:    rpc::param::QueueSetIndex,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let queue_len = audio_state_low_priority_lock().await.queue.len();

	if params.index >= queue_len {
		Ok(resp::result(rpc::resp::QueueSetIndex { out_of_bounds: true, index: params.index, queue_len, }, id))
	} else {
		send!(TO_KERNEL, FrontendToKernel::QueueSetIndex(params.index));
		Ok(resp::result(rpc::resp::QueueSetIndex { out_of_bounds: false, index: params.index, queue_len, }, id))
	}
}

async fn queue_remove_range<'a>(
	params:    rpc::param::QueueRemoveRange,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let queue_len = audio_state_low_priority_lock().await.queue.len();

	if params.start > params.end ||  params.start >= queue_len || params.end > queue_len {
		Ok(resp::result(rpc::resp::QueueRemoveRange { out_of_bounds: true, start: params.start, end: params.end, queue_len }, id))
	} else {
		send!(TO_KERNEL, FrontendToKernel::QueueRemoveRange((params.start..params.end, params.skip)));
		Ok(resp::result(rpc::resp::QueueRemoveRange { out_of_bounds: false, start: params.start, end: params.end, queue_len }, id))
	}
}

//---------------------------------------------------------------------------------------------------- Playlists
async fn playlist_new<'a>(
	params:      rpc::param::PlaylistNew<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	match PLAYLISTS.write().playlist_new(&params.playlist) {
		Some(v) => Ok(resp::result(serde_json::json!({ "len": v.len(), "entries": v }), id)),
		None    => Ok(resp::result(rpc::resp::PlaylistNew { len: None, entries: None }, id)),
	}
}

async fn playlist_remove<'a>(
	params:      rpc::param::PlaylistRemove<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	match PLAYLISTS.write().playlist_remove(params.playlist.into()) {
		Some(v) => Ok(resp::result(serde_json::json!({ "len": v.len(), "entries": v }), id)),
		None    => Ok(resp::error(ERR_PLAYLIST.0, ERR_PLAYLIST.1, id)),
	}
}

async fn playlist_clone<'a>(
	params:      rpc::param::PlaylistClone<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	match PLAYLISTS.write().playlist_clone(params.from.into(), &params.to) {
		Ok(Some(v)) => Ok(resp::result(serde_json::json!({ "len": v.len(), "entries": v }), id)),
		Ok(None)    => Ok(resp::result(rpc::resp::PlaylistClone { len: None, entries: None }, id)),
		Err(_)      => Ok(resp::error(ERR_PLAYLIST.0, ERR_PLAYLIST.1, id)),
	}
}

async fn playlist_get_index<'a>(
	params:      rpc::param::PlaylistGetIndex<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	match PLAYLISTS.read().playlist_get_index(params.index, params.playlist.into()) {
		Ok(Some(v)) => Ok(resp::result(serde_json::json!({ "entry": v }), id)),
		Ok(None)    => Ok(resp::error(ERR_INDEX_PLAYLIST.0, ERR_INDEX_PLAYLIST.1, id)),
		Err(_)      => Ok(resp::error(ERR_PLAYLIST.0, ERR_PLAYLIST.1, id)),
	}
}

async fn playlist_remove_index<'a>(
	params:      rpc::param::PlaylistRemoveIndex<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	match PLAYLISTS.write().playlist_remove_index(params.index, params.playlist.into()) {
		Ok(Some(v)) => Ok(resp::result(serde_json::json!({ "entry": v }), id)),
		Ok(None)    => Ok(resp::error(ERR_INDEX_PLAYLIST.0, ERR_INDEX_PLAYLIST.1, id)),
		Err(_)      => Ok(resp::error(ERR_PLAYLIST.0, ERR_PLAYLIST.1, id)),
	}
}

macro_rules! get_append_playlist {
	($params:expr, $id:expr, $playlist_lock:expr, $playlist:expr) => {
		match $params.append {
			shukusai::audio::Append2::Index => {
				let Some(i) = $params.index else {
					return Ok(resp::error(ERR_APPEND.0, ERR_APPEND.1, $id));
				};

				let p = $playlist_lock.get(&$playlist);

				if i != 0 && (p.is_none() || p.is_some_and(|v| i >= v.len())) {
					return Ok(resp::error(ERR_INDEX_PLAYLIST.0, ERR_INDEX_PLAYLIST.1, $id));
				}

				shukusai::audio::Append::Index(i)
			},
			shukusai::audio::Append2::Front => shukusai::audio::Append::Front,
			shukusai::audio::Append2::Back => shukusai::audio::Append::Back,
		}
	}
}

async fn playlist_add_key_artist<'a>(
	params:      rpc::param::PlaylistAddKeyArtist,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = ArtistKey::from(params.key);
	if collection.artists.get(key).is_none() {
		return Ok(resp::error(ERR_KEY_ARTIST.0, ERR_KEY_ARTIST.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let (existed, old_len, new_len) = p.playlist_add_artist(playlist, key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddKeyArtist { existed, old_len, new_len }, id))
}

async fn playlist_add_key_album<'a>(
	params:      rpc::param::PlaylistAddKeyAlbum,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = AlbumKey::from(params.key);
	if collection.albums.get(key).is_none() {
		return Ok(resp::error(ERR_KEY_ALBUM.0, ERR_KEY_ALBUM.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let (existed, old_len, new_len) = p.playlist_add_album(playlist, key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddKeyAlbum { existed, old_len, new_len }, id))
}

async fn playlist_add_key_song<'a>(
	params:      rpc::param::PlaylistAddKeySong,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = SongKey::from(params.key);
	if collection.songs.get(key).is_none() {
		return Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let (existed, old_len, new_len) = p.playlist_add_song(playlist, key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddKeySong { existed, old_len, new_len }, id))
}

async fn playlist_add_map_artist<'a>(
	params:      rpc::param::PlaylistAddMapArtist<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let Some((_, key)) = collection.artist(&params.artist) else {
		return Ok(resp::error(ERR_MAP_ARTIST.0, ERR_MAP_ARTIST.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let (existed, old_len, new_len) = p.playlist_add_artist(playlist, key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddMapArtist { existed, old_len, new_len }, id))
}

async fn playlist_add_map_album<'a>(
	params:      rpc::param::PlaylistAddMapAlbum<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let Some((_, key)) = collection.album(&params.artist, &params.album) else {
		return Ok(resp::error(ERR_MAP_ALBUM.0, ERR_MAP_ALBUM.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let (existed, old_len, new_len) = p.playlist_add_album(playlist, key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddMapAlbum { existed, old_len, new_len }, id))
}

async fn playlist_add_map_song<'a>(
	params:      rpc::param::PlaylistAddMapSong<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let Some((song, _)) = collection.song(&params.artist, &params.album, &params.song) else {
		return Ok(resp::error(ERR_MAP_SONG.0, ERR_MAP_SONG.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let (existed, old_len, new_len) = p.playlist_add_song(playlist, song.key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddMapSong { existed, old_len, new_len }, id))
}

async fn playlist_single<'a>(
	params:      rpc::param::PlaylistSingle<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let v = PLAYLISTS.read().get(&*params.playlist).map(|v| v.clone());
	if let Some(v) = v {
		let mut valid   = 0;
		let mut invalid = 0;

		for i in v.iter() {
			match i {
				shukusai::state::Entry::Valid { .. }   => valid +=1,
				shukusai::state::Entry::Invalid { .. } => invalid +=1,
			}
		}

		let resp = serde_json::json!({
			"playlist": Cow::Borrowed(&*params.playlist),
			"all_valid": invalid == 0,
			"entry_len": v.len(),
			"valid_len": valid,
			"invalid_len": invalid,
			"entries": v,
		});

		Ok(resp::result(resp, id))
	} else {
		Ok(resp::error(ERR_PLAYLIST.0, ERR_PLAYLIST.1, id))
	}
}

async fn playlist_brief<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let playlists: Cow<Vec<Arc<str>>> = Cow::Owned(PLAYLISTS.read().name_arcs());

	let resp = serde_json::json!({
		"len": playlists.len(),
		"playlists": playlists,
	});

	Ok(resp::result(resp, id))
}

async fn playlist_full<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let playlists = PLAYLISTS.read().clone();

	let playlist_len = playlists.len();
	let mut entry_len = 0;
	let mut valid     = 0;
	let mut invalid   = 0;

	for vec_deque in playlists.values() {
		for i in vec_deque.iter() {
			entry_len += 1;

			match i {
				shukusai::state::Entry::Valid { .. }   => valid +=1,
				shukusai::state::Entry::Invalid { .. } => invalid +=1,
			}
		}
	}

	let resp = serde_json::json!({
		"all_valid":    invalid == 0,
		"playlist_len": playlist_len,
		"entry_len":    entry_len,
		"valid_len":        valid,
		"invalid_len":      invalid,
		"playlists":    playlists,
	});

	Ok(resp::result(resp, id))
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
