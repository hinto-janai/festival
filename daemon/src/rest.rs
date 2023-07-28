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
use mime::TEXT_PLAIN_UTF_8;
use hyper::header::{
	CONTENT_LENGTH,
	CONTENT_TYPE,
	CONTENT_DISPOSITION,
};
use crate::resp;
use http::request::Parts;
use shukusai::collection::{
	Collection,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use crate::config::config;
use tokio::io::AsyncReadExt;

//---------------------------------------------------------------------------------------------------- REST Handler
pub async fn handle(
	parts:      Parts,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	// If we're in the middle of a `Collection` reset, respond with "busy".
	if crate::statics::resetting() {
		return Ok(resp::resetting_rest());
	}

	let mut split = parts.uri.path().split('/');

	split.next();

	let Some(ep1) = split.next() else {
//		crate::router::sleep_on_fail_task(collection);
		return Ok(resp::not_found("Missing endpoint 1"));
	};

	//-------------------------------------------------- `key` endpoint.
	if ep1 == "key" {
		let Some(ep2) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist/album/song]"));
		};

		let Some(ep3) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [key]"));
		};

		// Return error if more than 3 endpoints.
		if split.next().is_some() {
			return Ok(resp::not_found("Unknown endpoint"));
		}

		// Parse `usize` key.
		let Ok(key) = ep3.parse::<usize>() else {
			return Ok(resp::not_found("Key parse failure"));
		};

		match ep2 {
			"artist" => key_artist(key).await,
			"album"  => key_album(key).await,
			"song"   => key_song(key, collection).await,
			"art"    => key_art(key).await,
			_        => Ok(resp::not_found("Unknown endpoint")),
		}
	//-------------------------------------------------- `map` endpoint.
	} else if ep1 == "map" {
		let Some(ep2) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist]"));
		};

		let artist = match urlencoding::decode(ep2) {
			Ok(a)  => a,
			Err(e) => return Ok(resp::not_found("Artist parse failure")),
		};

		let album = if let Some(a) = split.next() {
			match urlencoding::decode(a) {
				Ok(a)  => Some(a),
				Err(e) => return Ok(resp::not_found("Album parse failure")),
			}
		} else {
			None
		};

		let song = if let Some(s) = split.next() {
			match urlencoding::decode(s) {
				Ok(a)  => Some(a),
				Err(e) => return Ok(resp::not_found("Song parse failure")),
			}
		} else {
			None
		};

		// Return error if more than 4 endpoints.
		if split.next().is_some() {
			return Ok(resp::not_found("Unknown endpoint"));
		}

		match (album, song) {
			(Some(a), None)    => map_album(artist.as_ref(), a.as_ref()).await,
			(Some(a), Some(s)) => map_song(artist.as_ref(), a.as_ref(), s.as_ref()).await,
			_                  => map_artist(artist.as_ref()).await,
		}
	//-------------------------------------------------- `art` endpoint.
	} else if ep1 == "art" {
		let Some(artist) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist]"));
		};

		let Ok(artist) = urlencoding::decode(artist) else {
			return Ok(resp::not_found("Artist parse failure"));
		};

		let Some(album) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [album]"));
		};

		let Ok(album) = urlencoding::decode(album) else {
			return Ok(resp::not_found("Album parse failure"));
		};

		// Return error if more than 3 endpoints.
		if split.next().is_some() {
			return Ok(resp::not_found("Unknown endpoint"));
		}

		art(artist.as_ref(), album.as_ref()).await
	//-------------------------------------------------- unknown endpoint.
	} else {
		Ok(resp::not_found("Unknown endpoint"))
	}
}

//---------------------------------------------------------------------------------------------------- `/key`
pub async fn key_artist(key: usize) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

pub async fn key_album(key: usize) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

pub async fn key_song(key: usize, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let key = SongKey::from(key);

	// If key exists...
	if let Some(song) = collection.songs.get(key) {
		// Open the file.
		let Ok(mut file) = tokio::fs::File::open(&song.path).await else {
			return Ok(resp::not_found("Song not found"));
		};

		let cap = match file.metadata().await {
			Ok(m) => m.len() as usize,
			_ => 1_000_000 // 1 megabyte,
		};

		// Copy the bytes into owned buffer.
		let mut dst: Vec<u8> = Vec::with_capacity(cap);
		if file.read_to_end(&mut dst).await.is_err() {
			return Ok(resp::server_err("Failed to copy song bytes"));
		};

		// Format the file name.
		let (artist, album, _) = collection.walk(key);
		let name = format!(
			"{}{}{}{}{}.{}",
			artist.name,
			config().filename_separator,
			album.title,
			config().filename_separator,
			song.title,
			song.extension,
		);

		Ok(resp::rest_ok(dst, &name, &song.mime))
	} else {
		Ok(resp::not_found("Song key is invalid"))
	}
}

pub async fn key_art(key: usize) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

//---------------------------------------------------------------------------------------------------- `/map`
pub async fn map_artist(artist: &str) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

pub async fn map_album(artist: &str, album: &str) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

pub async fn map_song(artist: &str, album: &str, song: &str) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

//---------------------------------------------------------------------------------------------------- `/art`
pub async fn art(artist: &str, album: &str) -> Result<Response<Body>, anyhow::Error> {
	todo!()
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
