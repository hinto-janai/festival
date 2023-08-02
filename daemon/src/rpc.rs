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
const ERR_CURRENT:    (i32, &str) = (-32018, "No song is currently set");
const ERR_RAND:       (i32, &str) = (-32019, "The Collection is empty");
const ERR_RESETTING:  (i32, &str) = (-32020, "Currently resetting the Collection");
const ERR_PERF:       (i32, &str) = (-32021, "Performance file does not exist");
const ERR_FS:         (i32, &str) = (-32022, "Filesystem error");

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
		Previous          => ppacor!(request, previous, rpc::param::Previous, TO_KERNEL).await,
		Volume            => ppacor!(request, volume, rpc::param::Volume, TO_KERNEL).await,
		Clear             => ppacor!(request, clear, rpc::param::Clear, TO_KERNEL).await,
		Seek              => ppacor!(request, seek, rpc::param::Seek, TO_KERNEL).await,
		Skip              => ppacor!(request, skip, rpc::param::Skip, TO_KERNEL).await,
		Back              => ppacor!(request, back, rpc::param::Back, TO_KERNEL).await,
		AddQueueKeyArtist => ppacor!(request, add_queue_key_artist, rpc::param::AddQueueKeyArtist, collection.arc(), TO_KERNEL).await,
		AddQueueKeyAlbum  => ppacor!(request, add_queue_key_album, rpc::param::AddQueueKeyAlbum, collection.arc(), TO_KERNEL).await,
		AddQueueKeySong   => ppacor!(request, add_queue_key_song, rpc::param::AddQueueKeySong, collection.arc(), TO_KERNEL).await,
		AddQueueMapArtist => ppacor!(request, add_queue_map_artist, rpc::param::AddQueueMapArtist, collection.arc(), TO_KERNEL).await,
		AddQueueMapAlbum  => ppacor!(request, add_queue_map_album, rpc::param::AddQueueMapAlbum, collection.arc(), TO_KERNEL).await,
		AddQueueMapSong   => ppacor!(request, add_queue_map_song, rpc::param::AddQueueMapSong, collection.arc(), TO_KERNEL).await,
		AddQueueRandArtist => ppacor!(request, add_queue_rand_artist, rpc::param::AddQueueRandArtist, collection.arc(), TO_KERNEL).await,
		AddQueueRandAlbum  => ppacor!(request, add_queue_rand_album, rpc::param::AddQueueRandAlbum, collection.arc(), TO_KERNEL).await,
		AddQueueRandSong   => ppacor!(request, add_queue_rand_song, rpc::param::AddQueueRandSong, collection.arc(), TO_KERNEL).await,
		SetQueueIndex     => ppacor!(request, set_queue_index, rpc::param::SetQueueIndex, TO_KERNEL).await,
		RemoveQueueRange  => ppacor!(request, remove_queue_range, rpc::param::RemoveQueueRange, TO_KERNEL).await,

		// Key (exact key)
		KeyArtist => ppacor!(request, key_artist, rpc::param::KeyArtist, collection.arc()).await,
		KeyAlbum  => ppacor!(request, key_album, rpc::param::KeyAlbum, collection.arc()).await,
		KeySong   => ppacor!(request, key_song, rpc::param::KeySong, collection.arc()).await,

		// Map (exact hashmap)
		MapArtist => ppacor!(request, map_artist, rpc::param::MapArtist, collection.arc()).await,
		MapAlbum  => ppacor!(request, map_album, rpc::param::MapAlbum, collection.arc()).await,
		MapSong   => ppacor!(request, map_song, rpc::param::MapSong, collection.arc()).await,

		// Current (audio state)
		CurrentArtist => current_artist(request.id, collection.arc()).await,
		CurrentAlbum  => current_album(request.id, collection.arc()).await,
		CurrentSong   => current_song(request.id, collection.arc()).await,

		// Rand (rng)
		RandArtist => rand_artist(request.id, collection.arc()).await,
		RandAlbum  => rand_album(request.id, collection.arc()).await,
		RandSong   => rand_song(request.id, collection.arc()).await,

		// Search (fuzzy string)
		Search       => ppacor!(request, search, rpc::param::Search, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchArtist => ppacor!(request, search_artist, rpc::param::SearchArtist, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchAlbum  => ppacor!(request, search_album, rpc::param::SearchAlbum, collection.arc(), TO_KERNEL, FROM_KERNEL).await,
		SearchSong   => ppacor!(request, search_song, rpc::param::SearchSong, collection.arc(), TO_KERNEL, FROM_KERNEL).await,

		// Collection
		CollectionNew          => ppacor!(request, collection_new, rpc::param::CollectionNew, collection.arc(), TO_KERNEL, FROM_KERNEL, TO_ROUTER).await,
		CollectionPerf         => collection_perf(request.id).await,
		CollectionResourceSize => collection_resource_size(request.id, collection.arc()).await,
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

async fn add_queue_key_artist<'a>(
	params:     rpc::param::AddQueueKeyArtist,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(x) = collection.artists.get(params.key) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueArtist((params.key, params.append, params.clear, params.offset)));

		if params.offset != 0 && params.offset >= x.songs.len() {
			Ok(resp::result(rpc::resp::AddQueueKeyArtist { out_of_bounds: true }, id))
		} else {
			Ok(resp::result(rpc::resp::AddQueueKeyArtist { out_of_bounds: false }, id))
		}
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

		if params.offset != 0 && params.offset >= x.songs.len() {
			Ok(resp::result(rpc::resp::AddQueueKeyAlbum { out_of_bounds: true }, id))
		} else {
			Ok(resp::result(rpc::resp::AddQueueKeyAlbum { out_of_bounds: false }, id))
		}
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
	if let Some((x, key)) = collection.artist(params.artist) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueArtist((key, params.append, params.clear, params.offset)));

		if params.offset != 0 && params.offset >= x.songs.len() {
			Ok(resp::result(rpc::resp::AddQueueKeyArtist { out_of_bounds: true }, id))
		} else {
			Ok(resp::result(rpc::resp::AddQueueKeyArtist { out_of_bounds: false }, id))
		}
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
	if let Some((x, key)) = collection.album(params.artist, params.album) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueAlbum((key, params.append, params.clear, params.offset)));

		if params.offset != 0 && params.offset >= x.songs.len() {
			Ok(resp::result(rpc::resp::AddQueueKeyAlbum { out_of_bounds: true }, id))
		} else {
			Ok(resp::result(rpc::resp::AddQueueKeyAlbum { out_of_bounds: false }, id))
		}
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

async fn add_queue_rand_artist<'a>(
	params:     rpc::param::AddQueueRandArtist,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_artist(None) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueArtist((key, params.append, params.clear, params.offset)));
		let r = &collection.artists[key];
		Ok(resp::result(serde_json::json!({ "artist": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_ARTIST.0, ERR_MAP_ARTIST.1, id))
	}
}

async fn add_queue_rand_album<'a>(
	params:     rpc::param::AddQueueRandAlbum,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_album(None) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueAlbum((key, params.append, params.clear, params.offset)));
		let r = &collection.albums[key];
		Ok(resp::result(serde_json::json!({ "album": r }), id))
	} else {
		Ok(resp::error(ERR_MAP_ALBUM.0, ERR_MAP_ALBUM.1, id))
	}
}

async fn add_queue_rand_song<'a>(
	params: rpc::param::AddQueueRandSong,
	id: Option<Id<'a>>,
	collection: Arc<Collection>,
	TO_KERNEL:  &Sender<FrontendToKernel>
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(key) = collection.rand_song(None) {
		send!(TO_KERNEL, FrontendToKernel::AddQueueSong((key, params.append, params.clear)));
		let r = &collection.songs[key];
		Ok(resp::result(serde_json::json!({ "song": r }), id))
	} else {
		Ok(resp::error(ERR_RAND.0, ERR_RAND.1, id))
	}
}

async fn set_queue_index<'a>(
	params:    rpc::param::SetQueueIndex,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	send!(TO_KERNEL, FrontendToKernel::SetQueueIndex(params.index));

	if params.index >= shukusai::state::AUDIO_STATE.read().queue.len() {
		Ok(resp::result(rpc::resp::SetQueueIndex { out_of_bounds: true }, id))
	} else {
		Ok(resp::result(rpc::resp::SetQueueIndex { out_of_bounds: false }, id))
	}
}

async fn remove_queue_range<'a>(
	params:    rpc::param::RemoveQueueRange,
	id:        Option<Id<'a>>,
	TO_KERNEL: &Sender<FrontendToKernel>,
) -> Result<Response<Body>, anyhow::Error> {
	let len = shukusai::state::AUDIO_STATE.read().queue.len();
	send!(TO_KERNEL, FrontendToKernel::RemoveQueueRange((params.start..params.end, params.skip)));

	if params.start > params.end ||  params.start >= len || params.end > len {
		Ok(resp::result(rpc::resp::RemoveQueueRange { out_of_bounds: true }, id))
	} else {
		Ok(resp::result(rpc::resp::RemoveQueueRange { out_of_bounds: false }, id))
	}
}

//---------------------------------------------------------------------------------------------------- Key (exact key)
async fn key_artist<'a>(
	params:     rpc::param::KeyArtist,
	id:         Option<Id<'a>>,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	if let Some(r) = collection.artists.get(params.key) {
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
	if let Some(r) = collection.albums.get(params.key) {
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
	if let Some(r) = collection.songs.get(params.key) {
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
	let song = shukusai::state::AUDIO_STATE.read().song.clone();

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
	let song = shukusai::state::AUDIO_STATE.read().song.clone();

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
	let song = shukusai::state::AUDIO_STATE.read().song.clone();

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

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
