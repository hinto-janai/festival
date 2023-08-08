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
	($request:expr, $call:expr, $param:ty, $($extra_arg:expr),*) => {{
		let Some(value) = $request.params else {
			return Ok(crate::resp::invalid_params($request.id));
		};

		let Ok(param) = serde_json::from_str::<$param>(value.get()) else {
			return Ok(crate::resp::invalid_params($request.id));
		};

		$call(param, $request.id, $($extra_arg),*)
	}};
	($request:expr, $call:expr, $param:ty) => {{
		let Some(value) = $request.params else {
			return Ok(crate::resp::invalid_params($request.id));
		};

		let Ok(param) = serde_json::from_str::<$param>(value.get()) else {
			return Ok(crate::resp::invalid_params($request.id));
		};

		$call(param, $request.id)
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
	TO_ROUTER:   &'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
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
		CollectionNew          => ppacor!(request, collection_new, rpc::param::CollectionNew, collection.arc(), TO_KERNEL, FROM_KERNEL, TO_ROUTER).await,
		CollectionBrief        => collection_brief(request.id, collection.arc()).await,
		CollectionFull         => collection_full(request.id, collection.arc()).await,
		CollectionRelation     => collection_relation(request.id, collection.arc()).await,
		CollectionRelationFull => collection_relation_full(request.id, collection.arc()).await,
		CollectionPerf         => collection_perf(request.id).await,
		CollectionResourceSize => collection_resource_size(request.id, collection.arc()).await,

		//-------------------------------------------------- State
		StateIp                 => state_ip(request.id).await,
		StateConfig             => state_config(request.id).await,
		StateDaemon             => state_daemon(request.id).await,
		StateAudio              => state_audio(request.id, collection.arc()).await,
		StateReset              => state_reset(request.id).await,

		//-------------------------------------------------- Key
		KeyArtist => ppacor!(request, key_artist, rpc::param::KeyArtist, collection.arc()).await,
		KeyAlbum  => ppacor!(request, key_album, rpc::param::KeyAlbum, collection.arc()).await,
		KeySong   => ppacor!(request, key_song, rpc::param::KeySong, collection.arc()).await,

		//-------------------------------------------------- Map
		MapArtist => ppacor!(request, map_artist, rpc::param::MapArtist, collection.arc()).await,
		MapAlbum  => ppacor!(request, map_album, rpc::param::MapAlbum, collection.arc()).await,
		MapSong   => ppacor!(request, map_song, rpc::param::MapSong, collection.arc()).await,

		//-------------------------------------------------- Current
		CurrentArtist => current_artist(request.id, collection.arc()).await,
		CurrentAlbum  => current_album(request.id, collection.arc()).await,
		CurrentSong   => current_song(request.id, collection.arc()).await,

		//-------------------------------------------------- Rand
		RandArtist => rand_artist(request.id, collection.arc()).await,
		RandAlbum  => rand_album(request.id, collection.arc()).await,
		RandSong   => rand_song(request.id, collection.arc()).await,

		//-------------------------------------------------- Search
		Search       => ppacor!(request, search, rpc::param::Search, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchArtist => ppacor!(request, search_artist, rpc::param::SearchArtist, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchAlbum  => ppacor!(request, search_album, rpc::param::SearchAlbum, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchSong   => ppacor!(request, search_song, rpc::param::SearchSong, collection.arc(), TO_KERNEL, FROM_KERNEL).await,

		//-------------------------------------------------- Playback
		Toggle             => toggle(request.id, TO_KERNEL).await,
		Play               => play(request.id, TO_KERNEL).await,
		Pause              => pause(request.id, TO_KERNEL).await,
		Next               => next(request.id, TO_KERNEL).await,
		Stop               => stop(request.id, TO_KERNEL).await,
		RepeatOff          => repeat_off(request.id, TO_KERNEL).await,
		RepeatSong         => repeat_song(request.id, TO_KERNEL).await,
		RepeatQueue        => repeat_queue(request.id, TO_KERNEL).await,
		Shuffle            => shuffle(request.id, TO_KERNEL).await,
		Previous           => ppacor!(request, previous, rpc::param::Previous, TO_KERNEL).await,
		Volume             => ppacor!(request, volume, rpc::param::Volume, TO_KERNEL).await,
		Clear              => ppacor!(request, clear, rpc::param::Clear, TO_KERNEL).await,
		Seek               => ppacor!(request, seek, rpc::param::Seek, TO_KERNEL).await,
		Skip               => ppacor!(request, skip, rpc::param::Skip, TO_KERNEL).await,
		Back               => ppacor!(request, back, rpc::param::Back, TO_KERNEL).await,

		//-------------------------------------------------- Queue
		QueueAddKeyArtist  => ppacor!(request, queue_add_key_artist, rpc::param::QueueAddKeyArtist, collection.arc(), TO_KERNEL).await,
		QueueAddKeyAlbum   => ppacor!(request, queue_add_key_album, rpc::param::QueueAddKeyAlbum, collection.arc(), TO_KERNEL).await,
		QueueAddKeySong    => ppacor!(request, queue_add_key_song, rpc::param::QueueAddKeySong, collection.arc(), TO_KERNEL).await,
		QueueAddMapArtist  => ppacor!(request, queue_add_map_artist, rpc::param::QueueAddMapArtist, collection.arc(), TO_KERNEL).await,
		QueueAddMapAlbum   => ppacor!(request, queue_add_map_album, rpc::param::QueueAddMapAlbum, collection.arc(), TO_KERNEL).await,
		QueueAddMapSong    => ppacor!(request, queue_add_map_song, rpc::param::QueueAddMapSong, collection.arc(), TO_KERNEL).await,
		QueueAddRandArtist => ppacor!(request, queue_add_rand_artist, rpc::param::QueueAddRandArtist, collection.arc(), TO_KERNEL).await,
		QueueAddRandAlbum  => ppacor!(request, queue_add_rand_album, rpc::param::QueueAddRandAlbum, collection.arc(), TO_KERNEL).await,
		QueueAddRandSong   => ppacor!(request, queue_add_rand_song, rpc::param::QueueAddRandSong, collection.arc(), TO_KERNEL).await,
		QueueAddPlaylist   => ppacor!(request, queue_add_playlist, rpc::param::QueueAddPlaylist, collection.arc(), TO_KERNEL).await,
		QueueSetIndex      => ppacor!(request, queue_set_index, rpc::param::QueueSetIndex, TO_KERNEL).await,
		QueueRemoveRange   => ppacor!(request, queue_remove_range, rpc::param::QueueRemoveRange, TO_KERNEL).await,

		//-------------------------------------------------- Playlist
		PlaylistNew          => ppacor!(request, playlist_new, rpc::param::PlaylistNew, collection.arc(), TO_KERNEL).await,
		PlaylistRemove       => ppacor!(request, playlist_remove, rpc::param::PlaylistRemove, collection.arc(), TO_KERNEL).await,
		PlaylistClone        => ppacor!(request, playlist_clone, rpc::param::PlaylistClone, collection.arc(), TO_KERNEL).await,
		PlaylistRemoveEntry  => ppacor!(request, playlist_remove_entry, rpc::param::PlaylistRemoveEntry, collection.arc(), TO_KERNEL).await,
		PlaylistAddKeyArtist => ppacor!(request, playlist_add_key_artist, rpc::param::PlaylistAddKeyArtist, collection.arc(), TO_KERNEL).await,
		PlaylistAddKeyAlbum  => ppacor!(request, playlist_add_key_album, rpc::param::PlaylistAddKeyAlbum, collection.arc(), TO_KERNEL).await,
		PlaylistAddKeySong   => ppacor!(request, playlist_add_key_song, rpc::param::PlaylistAddKeySong, collection.arc(), TO_KERNEL).await,
		PlaylistAddMapArtist => ppacor!(request, playlist_add_map_artist, rpc::param::PlaylistAddMapArtist, collection.arc(), TO_KERNEL).await,
		PlaylistAddMapAlbum  => ppacor!(request, playlist_add_map_album, rpc::param::PlaylistAddMapAlbum, collection.arc(), TO_KERNEL).await,
		PlaylistAddMapSong   => ppacor!(request, playlist_add_map_song, rpc::param::PlaylistAddMapSong, collection.arc(), TO_KERNEL).await,
		PlaylistNames        => playlist_names(request.id).await,
		PlaylistCount        => playlist_count(request.id).await,
		PlaylistSingle       => ppacor!(request, playlist_single, rpc::param::PlaylistSingle, collection.arc(), TO_KERNEL).await,
		PlaylistAll          => playlist_all(request.id).await,
	}
}

//---------------------------------------------------------------------------------------------------- Collection
async fn collection_new<'a>(
	params:      rpc::param::CollectionNew,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
	FROM_KERNEL: &'static Receiver<KernelToFrontend>,
	TO_ROUTER:   &'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
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
				debug!("Task - collection_new(): strong count == {sc}, waiting...");
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
			debug!("Task - Collection Reset Path: {}", p.display());
		}

		send!(TO_KERNEL, FrontendToKernel::NewCollection(paths));

		// Wait until `Kernel` has given us `Arc<Collection>`.
		let mut collection = loop {
			match recv!(FROM_KERNEL) {
				KernelToFrontend::NewCollection(c) => break c,
				_ => (),
			}
		};

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
		TO_ROUTER.send(collection).await.unwrap();

		Ok(resp::result(r, id))
	}).await
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

async fn collection_relation<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let mut vec = Vec::<rpc::resp::CollectionRelationInner>::with_capacity(collection.songs.len());

	for song in collection.songs.iter() {
		let album  = &collection.albums[song.album];
		let artist = &collection.artists[album.artist];

		let r = rpc::resp::CollectionRelationInner {
			artist: Cow::Borrowed(&artist.name),
			album: Cow::Borrowed(&album.title),
			song: Cow::Borrowed(&song.title),
			key_artist: ArtistKey::from(album.artist),
			key_album: AlbumKey::from(song.album),
			key_song: SongKey::from(song.key),
		};

		vec.push(r);
	}

	Ok(resp::result(rpc::resp::CollectionRelation(Cow::Owned(vec)), id))
}

async fn collection_relation_full<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let mut vec = Vec::<rpc::resp::CollectionRelationFullInner>::with_capacity(collection.songs.len());

	for song in collection.songs.iter() {
		let album  = &collection.albums[song.album];
		let artist = &collection.artists[album.artist];

		let r = rpc::resp::CollectionRelationFullInner {
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

	Ok(resp::result(rpc::resp::CollectionRelationFull(Cow::Owned(vec)), id))
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

async fn collection_full<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
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

	Ok(resp::result(&*collection, id))
}

//---------------------------------------------------------------------------------------------------- State
async fn state_ip<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let seen = crate::seen::SEEN_IPS.read().await.clone();

	let mut vec = Vec::with_capacity(seen.len());

	for (ip, count) in seen.into_iter() {
		let inner = rpc::resp::StateIpInner {
			ip,
			count,
		};
		vec.push(inner);
	}

	let resp = rpc::resp::StateIp(Cow::Owned(vec));

	Ok(resp::result(resp, id))
}

async fn state_config<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let c = config();

	let resp = rpc::resp::StateConfig {
		ip:                 c.ip,
		port:               c.port,
		max_connections:    c.max_connections,
		exclusive_ips:      c.exclusive_ips.as_ref().map(|h| Cow::Borrowed(h)),
		sleep_on_fail:      c.sleep_on_fail.clone(),
		collection_paths:   Cow::Borrowed(&c.collection_paths),
		tls:                c.tls,
		certificate:        c.certificate.as_ref().map(|p| Cow::Borrowed(p.as_path())),
		key:                c.key.as_ref().map(|p| Cow::Borrowed(p.as_path())),
		rest:               c.rest,
		docs:               c.docs,
		direct_download:    c.direct_download,
		filename_separator: Cow::Borrowed(&c.filename_separator),
		log_level:          c.log_level.clone(),
		watch:              c.watch,
		cache_time:         c.cache_time,
		media_controls:     c.media_controls,
		authorization:      AUTH.get().is_some(),
		no_auth_rpc:        c.no_auth_rpc.as_ref().map(|h| Cow::Borrowed(h)),
		no_auth_rest:       c.no_auth_rest.as_ref().map(|h| Cow::Borrowed(h)),
	};

	Ok(resp::result(resp, id))
}

async fn state_daemon<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::StateDaemon {
		uptime:              shukusai::logger::uptime(),
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
	} = AUDIO_STATE.read().clone();

	let song_key = song;
	let song = if let Some(key) = song_key {
		Some(&collection.songs[key])
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

async fn state_reset<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::StateReset {
		resetting: crate::statics::resetting(),
		saving: shukusai::state::saving(),
	};

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

//---------------------------------------------------------------------------------------------------- Current (audio state)
async fn current_artist<'a>(
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let song = AUDIO_STATE.read().song.clone();

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
	let song = AUDIO_STATE.read().song.clone();

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
	let song = AUDIO_STATE.read().song.clone();

	if let Some(key) = song {
		let r = &collection.songs[key];
		Ok(resp::result(serde_json::json!({ "song": r }), id))
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
	stop,         FrontendToKernel::Stop,
	shuffle,      FrontendToKernel::Shuffle,
	repeat_off,   FrontendToKernel::Repeat(shukusai::audio::Repeat::Off),
	repeat_song,  FrontendToKernel::Repeat(shukusai::audio::Repeat::Song),
	repeat_queue, FrontendToKernel::Repeat(shukusai::audio::Repeat::Queue)
}

async fn previous<'a>(
	params:    rpc::param::Previous,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	send!(TO_KERNEL, FrontendToKernel::Previous(params.threshold));
	Ok(resp::result_ok(id))
}

async fn volume<'a>(
	params:    rpc::param::Volume,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	if params.volume > 100 {
		Ok(resp::error(ERR_VOLUME.0, ERR_VOLUME.1, id))
	} else {
		let v = shukusai::audio::Volume::new(params.volume);
		send!(TO_KERNEL, FrontendToKernel::Volume(v));
		Ok(resp::result_ok(id))
	}
}

async fn clear<'a>(
	params:    rpc::param::Clear,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	send!(TO_KERNEL, FrontendToKernel::Clear(params.playback));
	Ok(resp::result_ok(id))
}

async fn seek<'a>(
	params:    rpc::param::Seek,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	send!(TO_KERNEL, FrontendToKernel::Seek((params.seek, params.second)));
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

		send!(TO_KERNEL, FrontendToKernel::QueueAddArtist((key, append, params.clear, offset)));

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

		send!(TO_KERNEL, FrontendToKernel::QueueAddAlbum((key, append, params.clear, offset)));

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

		send!(TO_KERNEL, FrontendToKernel::QueueAddSong((key, append, params.clear)));

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

		send!(TO_KERNEL, FrontendToKernel::QueueAddArtist((key, append, params.clear, offset)));

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

		send!(TO_KERNEL, FrontendToKernel::QueueAddAlbum((key, append, params.clear, offset)));

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

		send!(TO_KERNEL, FrontendToKernel::QueueAddSong((key, append, params.clear)));

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

		send!(TO_KERNEL, FrontendToKernel::QueueAddArtist((key, append, params.clear, offset)));

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

		send!(TO_KERNEL, FrontendToKernel::QueueAddAlbum((key, append, params.clear, offset)));

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

		send!(TO_KERNEL, FrontendToKernel::QueueAddSong((key, append, params.clear)));

		Ok(resp::result(serde_json::json!({ "song": x }), id))
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

		send!(TO_KERNEL, FrontendToKernel::QueueAddPlaylist((playlist, append, params.clear, offset)));

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
	if params.index >= AUDIO_STATE.read().queue.len() {
		Ok(resp::result(rpc::resp::QueueSetIndex { out_of_bounds: true }, id))
	} else {
		send!(TO_KERNEL, FrontendToKernel::QueueSetIndex(params.index));
		Ok(resp::result(rpc::resp::QueueSetIndex { out_of_bounds: false }, id))
	}
}

async fn queue_remove_range<'a>(
	params:    rpc::param::QueueRemoveRange,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let len = AUDIO_STATE.read().queue.len();

	if params.start > params.end ||  params.start >= len || params.end > len {
		Ok(resp::result(rpc::resp::QueueRemoveRange { out_of_bounds: true }, id))
	} else {
		send!(TO_KERNEL, FrontendToKernel::QueueRemoveRange((params.start..params.end, params.skip)));
		Ok(resp::result(rpc::resp::QueueRemoveRange { out_of_bounds: false }, id))
	}
}

//---------------------------------------------------------------------------------------------------- Playlists
async fn playlist_new<'a>(
	params:      rpc::param::PlaylistNew<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	match PLAYLISTS.write().playlist_new(&params.playlist) {
		Some(_) => Ok(resp::result(rpc::resp::PlaylistNew { existed: true }, id)),
		None    => Ok(resp::result(rpc::resp::PlaylistNew { existed: false }, id)),
	}
}

async fn playlist_remove<'a>(
	params:      rpc::param::PlaylistRemove<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	match PLAYLISTS.write().playlist_remove(params.playlist.into()) {
		Some(_) => Ok(resp::result(rpc::resp::PlaylistRemove { existed: true }, id)),
		None    => Ok(resp::result(rpc::resp::PlaylistRemove { existed: false }, id)),
	}
}

async fn playlist_clone<'a>(
	params:      rpc::param::PlaylistClone<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	match PLAYLISTS.write().playlist_clone(params.from.into(), &params.to) {
		Ok(Some(_)) => Ok(resp::result(rpc::resp::PlaylistClone { existed: true }, id)),
		Ok(None)    => Ok(resp::result(rpc::resp::PlaylistClone { existed: false }, id)),
		Err(_)      => Ok(resp::error(ERR_PLAYLIST.0, ERR_PLAYLIST.1, id)),
	}
}

async fn playlist_remove_entry<'a>(
	params:      rpc::param::PlaylistRemoveEntry<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	match PLAYLISTS.write().playlist_remove_entry(params.index, params.playlist.into()) {
		Ok(Some(_)) => Ok(resp::result(rpc::resp::PlaylistRemoveEntry { existed: true }, id)),
		Ok(None)    => Ok(resp::result(rpc::resp::PlaylistRemoveEntry { existed: false }, id)),
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
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = ArtistKey::from(params.key);
	if collection.artists.get(key).is_none() {
		return Ok(resp::error(ERR_KEY_ARTIST.0, ERR_KEY_ARTIST.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let existed = p.playlist_add_artist(playlist, key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddKeyArtist { existed }, id))
}

async fn playlist_add_key_album<'a>(
	params:      rpc::param::PlaylistAddKeyAlbum,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = AlbumKey::from(params.key);
	if collection.albums.get(key).is_none() {
		return Ok(resp::error(ERR_KEY_ALBUM.0, ERR_KEY_ALBUM.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let existed = p.playlist_add_album(playlist, key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddKeyAlbum { existed }, id))
}

async fn playlist_add_key_song<'a>(
	params:      rpc::param::PlaylistAddKeySong,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let key = SongKey::from(params.key);
	if collection.songs.get(key).is_none() {
		return Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let existed = p.playlist_add_song(playlist, key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddKeySong { existed }, id))
}

async fn playlist_add_map_artist<'a>(
	params:      rpc::param::PlaylistAddMapArtist<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let Some((_, key)) = collection.artist(&params.artist) else {
		return Ok(resp::error(ERR_MAP_ARTIST.0, ERR_MAP_ARTIST.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let existed = p.playlist_add_artist(playlist, key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddMapArtist { existed }, id))
}

async fn playlist_add_map_album<'a>(
	params:      rpc::param::PlaylistAddMapAlbum<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let Some((_, key)) = collection.album(&params.artist, &params.album) else {
		return Ok(resp::error(ERR_MAP_ALBUM.0, ERR_MAP_ALBUM.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let existed = p.playlist_add_album(playlist, key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddMapAlbum { existed }, id))
}

async fn playlist_add_map_song<'a>(
	params:      rpc::param::PlaylistAddMapSong<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let Some((song, _)) = collection.song(&params.artist, &params.album, &params.song) else {
		return Ok(resp::error(ERR_MAP_SONG.0, ERR_MAP_SONG.1, id));
	};

	let playlist: Arc<str> = params.playlist.into();
	let mut p = PLAYLISTS.write();

	let append = get_append_playlist!(params, id, p, playlist);

	let existed = p.playlist_add_song(playlist, song.key, append, &collection);
	Ok(resp::result(rpc::resp::PlaylistAddMapSong { existed }, id))
}

async fn playlist_names<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	Ok(resp::result(PLAYLISTS.read().name_arcs(), id))
}

async fn playlist_count<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let count = PLAYLISTS.read().len();
	Ok(resp::result(rpc::resp::PlaylistCount { count }, id))
}

async fn playlist_single<'a>(
	params:      rpc::param::PlaylistSingle<'a>,
	id:          Option<Id<'a>>,
	collection:  Arc<Collection>,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let v = PLAYLISTS.read().get(&*params.playlist).map(|v| v.clone());
	if let Some(v) = v {
		let mut valid   = 0;
		let mut invalid = 0;

		for i in v.iter() {
			match i {
				shukusai::state::PlaylistEntry::Valid { .. }   => valid +=1,
				shukusai::state::PlaylistEntry::Invalid { .. } => invalid +=1,
			}
		}

		let resp = serde_json::json!({
			"playlist": Cow::Borrowed(&*params.playlist),
			"all_valid": invalid == 0,
			"len": v.len(),
			"valid": valid,
			"invalid": invalid,
			"entries": v,
		});

		Ok(resp::result(resp, id))
	} else {
		Ok(resp::error(ERR_PLAYLIST.0, ERR_PLAYLIST.1, id))
	}
}

async fn playlist_all<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let playlists = PLAYLISTS.read().clone();

	let playlist_len = playlists.len();
	let mut entry_len = 0;
	let mut valid     = 0;
	let mut invalid   = 0;

	for vec_deque in playlists.values() {
		for i in vec_deque.iter() {
			entry_len += 1;

			match i {
				shukusai::state::PlaylistEntry::Valid { .. }   => valid +=1,
				shukusai::state::PlaylistEntry::Invalid { .. } => invalid +=1,
			}
		}
	}

	let resp = serde_json::json!({
		"all_valid":    invalid == 0,
		"playlist_len": playlist_len,
		"entry_len":    entry_len,
		"valid":        valid,
		"invalid":      invalid,
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
