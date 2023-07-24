//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
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

//---------------------------------------------------------------------------------------------------- Constants
// Tells browsers to view files.
const VIEW_IN_BROWSER: &str = "inline";
// Tells browsers to download files.
const DOWNLOAD_IN_BROWSER: &str = "attachment";

//---------------------------------------------------------------------------------------------------- REST Handler
pub async fn handle(
	parts:  Parts,
) -> Result<Response<Body>, anyhow::Error> {
	let mut split = parts.uri.path().split('/');

	split.next();

	let Some(ep1) = split.next() else {
//		crate::router::sleep_on_fail_task(collection);
		return Ok(resp::not_found("missing endpoint 1"));
	};

	//-------------------------------------------------- `key` endpoint.
	if ep1 == "key" {
		let Some(ep2) = split.next() else {
			return Ok(resp::not_found("missing endpoint: [artist/album/song]"));
		};

		let Some(ep3) = split.next() else {
			return Ok(resp::not_found("missing endpoint: [key]"));
		};

		// Return error if more than 3 endpoints.
		if split.next().is_some() {
			return Ok(resp::not_found("unknown endpoint"));
		}

		// Parse `usize` key.
		let Ok(key) = ep3.parse::<usize>() else {
			return Ok(resp::not_found("key parse failure"));
		};

		match ep2 {
			"artist" => key_artist(key).await,
			"album"  => key_album(key).await,
			"song"   => key_song(key).await,
			"art"    => key_art(key).await,
			_        => Ok(resp::not_found("unknown endpoint")),
		}
	//-------------------------------------------------- `string` endpoint.
	} else if ep1 == "string" {
		let Some(ep2) = split.next() else {
			return Ok(resp::not_found("missing endpoint: [artist]"));
		};

		let artist = match urlencoding::decode(ep2) {
			Ok(a)  => a,
			Err(e) => return Ok(resp::not_found("artist parse failure")),
		};

		let album = if let Some(a) = split.next() {
			match urlencoding::decode(a) {
				Ok(a)  => Some(a),
				Err(e) => return Ok(resp::not_found("album parse failure")),
			}
		} else {
			None
		};

		let song = if let Some(s) = split.next() {
			match urlencoding::decode(s) {
				Ok(a)  => Some(a),
				Err(e) => return Ok(resp::not_found("song parse failure")),
			}
		} else {
			None
		};

		// Return error if more than 4 endpoints.
		if split.next().is_some() {
			return Ok(resp::not_found("unknown endpoint"));
		}

		match (album, song) {
			(Some(a), None)    => string_album(artist.as_ref(), a.as_ref()).await,
			(Some(a), Some(s)) => string_song(artist.as_ref(), a.as_ref(), s.as_ref()).await,
			_                  => string_artist(artist.as_ref()).await,
		}
	//-------------------------------------------------- `art` endpoint.
	} else if ep1 == "art" {
		let Some(artist) = split.next() else {
			return Ok(resp::not_found("missing endpoint: [artist]"));
		};

		let Ok(artist) = urlencoding::decode(artist) else {
			return Ok(resp::not_found("artist parse failure"));
		};

		let Some(album) = split.next() else {
			return Ok(resp::not_found("missing endpoint: [album]"));
		};

		let Ok(album) = urlencoding::decode(album) else {
			return Ok(resp::not_found("album parse failure"));
		};

		// Return error if more than 3 endpoints.
		if split.next().is_some() {
			return Ok(resp::not_found("unknown endpoint"));
		}

		art(artist.as_ref(), album.as_ref()).await
	//-------------------------------------------------- unknown endpoint.
	} else {
		Ok(resp::not_found("unknown endpoint"))
	}
}

//---------------------------------------------------------------------------------------------------- Artist
pub async fn key_artist(key: usize) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from(format!("key_artist: {key}")))) }
pub async fn key_album(key: usize) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from(format!("key_album: {key}")))) }
pub async fn key_song(key: usize) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from(format!("key_song: {key}")))) }
pub async fn key_art(key: usize) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from(format!("key_art: {key}")))) }

pub async fn string_artist(artist: &str) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from(format!("string_artist: {artist}")))) }
pub async fn string_album(artist: &str, album: &str) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from(format!("string_album: {artist}, {album}")))) }
pub async fn string_song(artist: &str, album: &str, song: &str) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from(format!("string_song: {artist}, {album}, {song}")))) }

pub async fn art(artist: &str, album: &str) -> Result<Response<Body>, anyhow::Error> { Ok(Response::new(Body::from(format!("art: {artist}, {album}")))) }

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
