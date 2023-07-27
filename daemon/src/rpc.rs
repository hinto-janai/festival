//---------------------------------------------------------------------------------------------------- Use
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use crate::hash::Hash;
use std::sync::Arc;
use std::net::SocketAddrV4;
use crate::config::Config;
use hyper::{
	Request,
	Response,
	body::Body,
};
use http::request::Parts;
use serde_json::value::{
	RawValue,Value,
};
use crate::resp;
use crate::constants::{
	FESTIVALD_VERSION,
};
use shukusai::{
	state::AUDIO_STATE,
	collection::{
		Collection,
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
};
use crate::config::{
	AUTH,config,
};
use std::borrow::Cow;
use benri::lock;

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
	($request:expr, $call:expr, $param:ty) => {{
		let Some(value) = $request.params else {
			return Ok(crate::resp::invalid_params($request.id));
		};

		let Ok(param) = serde_json::from_str::<$param>(value.get()) else {
			return Ok(crate::resp::invalid_params($request.id));
		};

		$call(param)
	}}
}

//---------------------------------------------------------------------------------------------------- JSON-RPC Handler
pub async fn handle(
	parts:      Parts,
	body:       Body,
	addr:       SocketAddrV4,
	collection: Arc<Collection>,
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

	// Parse method.
	let Ok(method) = serde_json::from_str::<rpc::Method>(request.method.get()) else {
		return Ok(resp::method_not_found(request.id));
	};

	use rpc::Method::*;
	match method {
		// State retrieval.
		StateDaemon         => state_daemon(request.id).await,
		StateAudio          => state_audio(request.id).await,
		StateReset          => state_reset(request.id).await,
		StateCollection     => state_collection(request.id, collection).await,
		StateCollectionFull => state_collection_full(request.id, collection).await,

		// Playback control.
		Toggle      => toggle(request.id).await,
		Play        => play(request.id).await,
		Pause       => pause(request.id).await,
		Next        => next(request.id).await,
		Stop        => stop(request.id).await,
		RepeatOff   => repeat_off(request.id).await,
		RepeatSong  => repeat_song(request.id).await,
		RepeatQueue => repeat_queue(request.id).await,
		Shuffle     => shuffle(request.id).await,

		Previous         => ppacor!(request, previous, rpc::param::Previous).await,
		Volume           => ppacor!(request, volume, rpc::param::Volume).await,
		AddQueueSong     => ppacor!(request, add_queue_song, rpc::param::AddQueueSong).await,
		AddQueueAlbum    => ppacor!(request, add_queue_album, rpc::param::AddQueueAlbum).await,
		AddQueueArtist   => ppacor!(request, add_queue_artist, rpc::param::AddQueueArtist).await,
		Clear            => ppacor!(request, clear, rpc::param::Clear).await,
		Seek             => ppacor!(request, seek, rpc::param::Seek).await,
		Skip             => ppacor!(request, skip, rpc::param::Skip).await,
		Back             => ppacor!(request, back, rpc::param::Back).await,
		SetQueueIndex    => ppacor!(request, set_queue_index, rpc::param::SetQueueIndex).await,
		RemoveQueueRange => ppacor!(request, remove_queue_range, rpc::param::RemoveQueueRange).await,

		// Key (exact key).await,
		KeyArtist => ppacor!(request, key_artist, rpc::param::KeyArtist).await,
		KeyAlbum  => ppacor!(request, key_album, rpc::param::KeyAlbum).await,
		KeySong   => ppacor!(request, key_song, rpc::param::KeySong).await,

		// Map (exact hashmap).await,
		MapArtist => ppacor!(request, map_artist, rpc::param::MapArtist).await,
		MapAlbum  => ppacor!(request, map_album, rpc::param::MapAlbum).await,
		MapSong   => ppacor!(request, map_song, rpc::param::MapSong).await,

		// Search (fuzzy keys).await,
		Search       => ppacor!(request, search, rpc::param::Search).await,
		SearchArtist => ppacor!(request, search_artist, rpc::param::SearchArtist).await,
		SearchAlbum  => ppacor!(request, search_album, rpc::param::SearchAlbum).await,
		SearchSong   => ppacor!(request, search_song, rpc::param::SearchSong).await,

		// Collection
		NewCollection => ppacor!(request, new_collection, rpc::param::NewCollection).await,
	}
}

//---------------------------------------------------------------------------------------------------- No-Param methods.
async fn state_daemon<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::StateDaemon {
		uptime:          shukusai::logger::uptime(),
		rest:            config().rest,
		direct_download: config().direct_download,
		authorization:   AUTH.get().is_some(),
		version:         Cow::Borrowed(FESTIVALD_VERSION),
		commit:          Cow::Borrowed(COMMIT),
		os:              Cow::Borrowed(OS_ARCH),
	};

	Ok(resp::result(resp, id))
}

async fn state_audio<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
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

	let resp = rpc::resp::StateAudio {
		queue: Cow::Owned(queue.into()),
		queue_idx,
		playing,
		song,
		elapsed: elapsed.inner(),
		runtime: runtime.inner(),
		repeat,
		volume: volume.inner(),
	};

	Ok(resp::result(resp, id))
}

async fn state_reset<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> {
	let resp = rpc::resp::StateReset {
		resetting: shukusai::state::resetting(),
		saving: shukusai::state::saving(),
	};

	Ok(resp::result(resp, id))
}

async fn state_collection<'a>(id: Option<json_rpc::Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
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

async fn state_collection_full<'a>(id: Option<json_rpc::Id<'a>>, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	// Instead of checking if the `Collection` -> `JSON String`
	// output is correct for every response, only check in debug builds.
	//
	// No need to do `Collection` -> `String` -> `CollectionJson` -> `String`
	// when all that is needed is `Collection` -> `String`
	#[cfg(debug_assertions)]
	{
		let string = serde_json::to_string(&collection).unwrap();
		let c: CollectionJson = serde_json::from_str(&string).unwrap();
		assert_eq!(serde_json::to_string(&c).unwrap(), string);
	}

	Ok(resp::result(collection, id))
}

async fn toggle<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("toggle"))) }
async fn play<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("play"))) }
async fn pause<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("pause"))) }
async fn next<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("next"))) }
async fn stop<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("stop"))) }
async fn repeat_off<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("repeat_off"))) }
async fn repeat_song<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("repeat_song"))) }
async fn repeat_queue<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("repeat_queue"))) }
async fn shuffle<'a>(id: Option<json_rpc::Id<'a>>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("shuffle"))) }

//---------------------------------------------------------------------------------------------------- Param methods.
async fn previous(params: rpc::param::Previous) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("previous"))) }
async fn volume(params: rpc::param::Volume) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("volume"))) }
async fn add_queue_song(params: rpc::param::AddQueueSong) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("add_queue_song"))) }
async fn add_queue_album(params: rpc::param::AddQueueAlbum) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("add_queue_album"))) }
async fn add_queue_artist(params: rpc::param::AddQueueArtist) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("add_queue_artist"))) }
async fn clear(params: rpc::param::Clear) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("clear"))) }
async fn seek(params: rpc::param::Seek) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("seek"))) }
async fn skip(params: rpc::param::Skip) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("skip"))) }
async fn back(params: rpc::param::Back) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("back"))) }
async fn set_queue_index(params: rpc::param::SetQueueIndex) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("set_queue_index"))) }
async fn remove_queue_range(params: rpc::param::RemoveQueueRange) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("remove_queue_range"))) }
async fn key_artist(params: rpc::param::KeyArtist) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("key_artist"))) }
async fn key_album(params: rpc::param::KeyAlbum) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("key_album"))) }
async fn key_song(params: rpc::param::KeySong) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("key_song"))) }
async fn map_artist<'a>(params: rpc::param::MapArtist<'a>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("map_artist"))) }
async fn map_album<'a>(params: rpc::param::MapAlbum<'a>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("map_album"))) }
async fn map_song<'a>(params: rpc::param::MapSong<'a>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("map_song"))) }
async fn search<'a>(params: rpc::param::Search<'a>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("search"))) }
async fn search_artist<'a>(params: rpc::param::SearchArtist<'a>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("search_artist"))) }
async fn search_album<'a>(params: rpc::param::SearchAlbum<'a>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("search_album"))) }
async fn search_song<'a>(params: rpc::param::SearchSong<'a>) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("search_song"))) }
async fn new_collection(params: rpc::param::NewCollection) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from("new_collection"))) }

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
