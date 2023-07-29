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
	state::AUDIO_STATE,
	collection::{
		Collection,
		Artist,
		Album,
		Song,
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
	hash::Hash,
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
const ERR_VOLUME:     (i32, &str) = (-32011, "Volume must be in between 0..100");
const ERR_KEY_ARTIST: (i32, &str) = (-32012, "Artist key is invalid");
const ERR_KEY_ALBUM:  (i32, &str) = (-32013, "Album key is invalid");
const ERR_KEY_SONG:   (i32, &str) = (-32014, "Song key is invalid");
const ERR_MAP_ARTIST: (i32, &str) = (-32015, "Artist does not exist");
const ERR_MAP_ALBUM:  (i32, &str) = (-32016, "Album does not exist");
const ERR_MAP_SONG:   (i32, &str) = (-32017, "Song does not exist");
const ERR_RESETTING:  (i32, &str) = (-32018, "Currently resetting the Collection");

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
		return Ok(resp::invalid_request(None));
	};

	// If we're in the middle of a `Collection` reset, respond with "busy".
	if crate::statics::resetting() {
		return Ok(resp::resetting(ERR_RESETTING.0, ERR_RESETTING.1, request.id));
	}

	// Parse method.
	let Ok(method) = serde_json::from_str::<rpc::Method>(request.method.get()) else {
		return Ok(resp::method_not_found(request.id));
	};

	use rpc::Method::*;
	match method {
		// State retrieval.
		StateDaemon         => state_daemon(request.id).await,
		StateAudio          => state_audio(request.id, collection.arc()).await,
		StateReset          => state_reset(request.id).await,
		StateCollection     => state_collection(request.id, collection.arc()).await,
		StateCollectionFull => state_collection_full(request.id, collection.arc()).await,

		// Playback control, no params.
		Toggle      => toggle(request.id, TO_KERNEL).await,
		Play        => play(request.id, TO_KERNEL).await,
		Pause       => pause(request.id, TO_KERNEL).await,
		Next        => next(request.id, TO_KERNEL).await,
		Stop        => stop(request.id, TO_KERNEL).await,
		RepeatOff   => repeat_off(request.id, TO_KERNEL).await,
		RepeatSong  => repeat_song(request.id, TO_KERNEL).await,
		RepeatQueue => repeat_queue(request.id, TO_KERNEL).await,
		Shuffle     => shuffle(request.id, TO_KERNEL).await,
		// Playback control with params.
		Previous          => ppacor!(request, previous, rpc::param::Previous).await,
		Volume            => ppacor!(request, volume, rpc::param::Volume).await,
		Clear             => ppacor!(request, clear, rpc::param::Clear).await,
		Seek              => ppacor!(request, seek, rpc::param::Seek).await,
		Skip              => ppacor!(request, skip, rpc::param::Skip).await,
		Back              => ppacor!(request, back, rpc::param::Back).await,
		AddQueueKeyArtist => ppacor!(request, add_queue_key_artist, rpc::param::AddQueueKeyArtist, collection.arc(), TO_KERNEL).await,
		AddQueueKeyAlbum  => ppacor!(request, add_queue_key_album, rpc::param::AddQueueKeyAlbum, collection.arc(), TO_KERNEL).await,
		AddQueueKeySong   => ppacor!(request, add_queue_key_song, rpc::param::AddQueueKeySong, collection.arc(), TO_KERNEL).await,
		AddQueueMapArtist => ppacor!(request, add_queue_map_artist, rpc::param::AddQueueMapArtist, collection.arc(), TO_KERNEL).await,
		AddQueueMapAlbum  => ppacor!(request, add_queue_map_album, rpc::param::AddQueueMapAlbum, collection.arc(), TO_KERNEL).await,
		AddQueueMapSong   => ppacor!(request, add_queue_map_song, rpc::param::AddQueueMapSong, collection.arc(), TO_KERNEL).await,
		SetQueueIndex     => ppacor!(request, set_queue_index, rpc::param::SetQueueIndex).await,
		RemoveQueueRange  => ppacor!(request, remove_queue_range, rpc::param::RemoveQueueRange).await,

		// Key (exact key)
		KeyArtist => ppacor!(request, key_artist, rpc::param::KeyArtist, collection.arc()).await,
		KeyAlbum  => ppacor!(request, key_album, rpc::param::KeyAlbum, collection.arc()).await,
		KeySong   => ppacor!(request, key_song, rpc::param::KeySong, collection.arc()).await,

		// Map (exact hashmap)
		MapArtist => ppacor!(request, map_artist, rpc::param::MapArtist).await,
		MapAlbum  => ppacor!(request, map_album, rpc::param::MapAlbum).await,
		MapSong   => ppacor!(request, map_song, rpc::param::MapSong).await,

		// Search (fuzzy string)
		Search       => ppacor!(request, search, rpc::param::Search, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchArtist => ppacor!(request, search_artist, rpc::param::SearchArtist, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchAlbum  => ppacor!(request, search_album, rpc::param::SearchAlbum, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchSong   => ppacor!(request, search_song, rpc::param::SearchSong, collection.arc(), TO_KERNEL, FROM_KERNEL).await,

		// Collection
		NewCollection => ppacor!(request, new_collection, rpc::param::NewCollection, collection.arc(), TO_KERNEL, FROM_KERNEL, TO_ROUTER).await,
	}
}

//---------------------------------------------------------------------------------------------------- State retrieval.
async fn state_daemon<'a>(id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::StateDaemon {
		uptime:              shukusai::logger::uptime(),
		total_requests:      atomic_load!(TOTAL_REQUESTS),
		total_connections:   atomic_load!(TOTAL_CONNECTIONS),
		current_connections: crate::statics::connections(),
		rest:                config().rest,
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

async fn state_collection<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::StateCollection {
		empty: collection.empty,
		timestamp: collection.timestamp,
		count_artist: collection.count_artist.inner(),
		count_album: collection.count_album.inner(),
		count_song: collection.count_song.inner(),
		count_art: collection.count_art.inner(),
	};

	Ok(resp::result(resp, id))
}

async fn state_collection_full<'a>(id: Option<Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
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

	let x = crate::zip::TmpZip::new();

	Ok(resp::result(&*collection, id))
}

//---------------------------------------------------------------------------------------------------- Playback control, no params.
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

//---------------------------------------------------------------------------------------------------- Playback control with params.
async fn previous<'a>(params: rpc::param::Previous, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

async fn volume<'a>(params: rpc::param::Volume, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

async fn clear<'a>(params: rpc::param::Clear, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

async fn seek<'a>(params: rpc::param::Seek, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

async fn skip<'a>(params: rpc::param::Skip, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

async fn back<'a>(params: rpc::param::Back, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

async fn add_queue_key_artist<'a>(
	params:     rpc::param::AddQueueKeyArtist,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(x) = collection.artists.get(params.key) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueArtist((params.key, params.append, params.clear, params.offset)));
		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_KEY_ARTIST.0, ERR_KEY_ARTIST.1, id))
	}
}

async fn add_queue_key_album<'a>(
	params:     rpc::param::AddQueueKeyAlbum,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(x) = collection.albums.get(params.key) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueAlbum((params.key, params.append, params.clear, params.offset)));
		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_KEY_ALBUM.0, ERR_KEY_ALBUM.1, id))
	}
}

async fn add_queue_key_song<'a>(
	params:     rpc::param::AddQueueKeySong,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(x) = collection.songs.get(params.key) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueSong((params.key, params.append, params.clear)));
		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id))
	}
}

async fn add_queue_map_artist<'a>(
	params:     rpc::param::AddQueueMapArtist<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((_, key)) = collection.artist(params.artist) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueArtist((key, params.append, params.clear, params.offset)));
		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_MAP_ARTIST.0, ERR_MAP_ARTIST.1, id))
	}
}

async fn add_queue_map_album<'a>(
	params:     rpc::param::AddQueueMapAlbum<'a>,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((_, key)) = collection.album(params.artist, params.album) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueAlbum((key, params.append, params.clear, params.offset)));
		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_MAP_ALBUM.0, ERR_MAP_ALBUM.1, id))
	}
}

async fn add_queue_map_song<'a>(
	params: rpc::param::AddQueueMapSong<'a>,
	id: Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some((_, key)) = collection.song(params.artist, params.album, params.song) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueSong((key, params.append, params.clear)));
		Ok(resp::result_ok(id))
	} else {
		Ok(resp::error(ERR_MAP_SONG.0, ERR_MAP_SONG.1, id))
	}
}

async fn set_queue_index<'a>(params: rpc::param::SetQueueIndex, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

async fn remove_queue_range<'a>(params: rpc::param::RemoveQueueRange, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

//---------------------------------------------------------------------------------------------------- Key (exact key)
async fn key_artist<'a>(
	params:     rpc::param::KeyArtist,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(r) = collection.artists.get(params.key) {
		Ok(resp::result(r, id))
	} else {
		Ok(resp::error(ERR_KEY_ARTIST.0, ERR_KEY_ARTIST.1, id))
	}
}

async fn key_album<'a>(
	params:     rpc::param::KeyAlbum,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(r) = collection.albums.get(params.key) {
		Ok(resp::result(r, id))
	} else {
		return Ok(resp::error(ERR_KEY_ALBUM.0, ERR_KEY_ALBUM.1, id))
	}
}

async fn key_song<'a>(
	params:     rpc::param::KeySong,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(r) = collection.songs.get(params.key) {
		Ok(resp::result(r, id))
	} else {
		return Ok(resp::error(ERR_KEY_SONG.0, ERR_KEY_SONG.1, id))
	}
}

//---------------------------------------------------------------------------------------------------- Map (exact hashmap)
async fn map_artist<'a>(params: rpc::param::MapArtist<'a>, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

async fn map_album<'a>(params: rpc::param::MapAlbum<'a>, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

async fn map_song<'a>(params: rpc::param::MapSong<'a>, id: Option<Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	todo!()
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
		let msg = loop {
			match $from_kernel.try_recv() {
				Ok(msg) => break msg,
				_ => tokio::time::sleep(Duration::from_millis(1)).await,
			}
		};

		// INVARIANT: This _must_ be `SearchResp` or our `KERNEL_LOCK` workaround isn't working.
		let KernelToFrontend::SearchResp(keychain) = msg else {
			debug_panic!("search method but not search resp");
			return Ok(resp::internal_error($id));
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

//---------------------------------------------------------------------------------------------------- Collection
async fn new_collection<'a>(
	params:      rpc::param::NewCollection,
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
				debug!("Task - new_collection(): strong count == {sc}, waiting...");
				tokio::time::sleep(Duration::from_millis(10)).await;
			} else {
				break;
			}
		}

		send!(TO_KERNEL, FrontendToKernel::NewCollection(params.paths.unwrap_or_else(|| vec![])));

		// Wait until `Kernel` has given us `Arc<Collection>`.
		let mut collection = loop {
			match recv!(FROM_KERNEL) {
				KernelToFrontend::NewCollection(c) => break c,
				_ => debug_panic!("wrong kernel msg"),
			}
		};

		// We're done resetting.
		atomic_store!(RESETTING, false);

		// Respond to user.
		let r = rpc::resp::NewCollection {
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

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
