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
use mime::TEXT_PLAIN_UTF_8;
use hyper::header::{
	CONTENT_LENGTH,
	CONTENT_TYPE,
	CONTENT_DISPOSITION,
};
use crate::resp;
use http::request::Parts;
use shukusai::collection::{
	Art,
	Collection,
	ArtistKey,
	AlbumKey,
	SongKey,
	Artist,
	Album,
	Song,
};
use crate::{
	config::{Config,config},
	ptr::CollectionPtr,
};
use rpc::resource::Resource;
use tokio::io::AsyncReadExt;
use std::{
	path::{Path,PathBuf},
	io::Write,
};
use crate::config::AUTH;
use std::str::FromStr;
use benri::debug_panic;

//---------------------------------------------------------------------------------------------------- Const
pub const REST_ENDPOINTS: [&'static str; 6] = [
	"key",
	"map",
	"art",
	"current",
	"rand",
	"collection",
];

pub const ERR_END: &str = "Unknown endpoint";

//---------------------------------------------------------------------------------------------------- Auth.
// Check auth.
async fn rest_auth_ok(
	parts:    &Parts,
	addr:     &SocketAddrV4,
	resource: Resource,
) -> Option<Response<Body>> {
	if !config().no_auth_rest.as_ref().is_some_and(|h| h.contains(&resource)) {
		if let Some(hash) = AUTH.get() {
			if !crate::router::auth_ok(parts, hash).await {
				if crate::seen::seen(&addr).await {
					crate::router::sleep_on_fail().await;
				}
				return Some(resp::unauthorized("Unauthorized"));
			}
		}
	}

	None
}

//---------------------------------------------------------------------------------------------------- REST Handler
pub async fn handle(
	parts:  Parts,
	addr:       SocketAddrV4,
	collection: &'static CollectionPtr,
) -> Result<Response<Body>, anyhow::Error> {
	// If we're in the middle of a `Collection` reset, respond with "busy".
	if crate::statics::resetting() {
		return Ok(resp::resetting_rest());
	}

	let Ok(uri) = urlencoding::decode(parts.uri.path()) else {
		return Ok(resp::server_err("URI parse failure"));
	};

	trace!("Task - REST URI Full: {uri}");
	for (i, s) in uri.split('/').enumerate() {
		trace!("Task - REST URI Split [{i}]: {s}");
	}

	let mut split = uri.split('/');

	split.next();

	let Some(ep1) = split.next() else {
		return Ok(resp::not_found("Missing endpoint 1"));
	};

	//-------------------------------------------------- `/key` endpoint.
	if ep1 == "key" {
		let Some(ep2) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist/album/song/art]"));
		};

		let Some(ep3) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [key]"));
		};

		// Return error if more than 3 endpoints.
		match split.next() {
			Some(s) if !s.is_empty() => return Ok(resp::not_found(ERR_END)),
			_ => (),
		}

		// Parse `usize` key.
		let Ok(key) = ep3.parse::<usize>() else {
			return Ok(resp::not_found("Key parse failure"));
		};

		let Some(resource) = Resource::from_str_not_c(ep2) else {
			return Ok(resp::not_found(ERR_END));
		};

		if let Some(resp) = rest_auth_ok(&parts, &addr, resource).await {
			return Ok(resp);
		}

		match resource {
			Resource::Artist => key_artist(key, collection.arc()).await,
			Resource::Album  => key_album(key, collection.arc()).await,
			Resource::Song   => key_song(key, collection.arc()).await,
			Resource::Art    => key_art(key, collection.arc()).await,
			_ => {
				debug_panic!("parsed resource {resource:?}, but reached unreachable");
				Ok(resp::server_err("Unknown resource"))
			},
		}
	//-------------------------------------------------- `/map` endpoint.
	} else if ep1 == "map" {
		let Some(artist) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist]"));
		};

		let album = split.next();
		let song = split.next();

		// Return error if more than 3 endpoints.
		match split.next() {
			Some(s) if !s.is_empty() => return Ok(resp::not_found(ERR_END)),
			_ => (),
		}

		match (album, song) {
			// Album.
			(Some(a), None) if !a.is_empty() => {
				if let Some(resp) = rest_auth_ok(&parts, &addr, Resource::Album).await {
					return Ok(resp);
				}
				map_album(artist.as_ref(), a.as_ref(), collection.arc()).await
			},

			// Song.
			(Some(a), Some(s)) if !a.is_empty() && !s.is_empty() => {
				if let Some(resp) = rest_auth_ok(&parts, &addr, Resource::Song).await {
					return Ok(resp);
				}
				map_song(artist.as_ref(), a.as_ref(), s.as_ref(), collection.arc()).await
			},

			// Artist
			_ => {
				if let Some(resp) = rest_auth_ok(&parts, &addr, Resource::Artist).await {
					return Ok(resp);
				}
				map_artist(artist.as_ref(), collection.arc()).await
			},
		}
	//-------------------------------------------------- `/art` endpoint.
	} else if ep1 == "art" {
		// Art auth.
		if let Some(resp) = rest_auth_ok(&parts, &addr, Resource::Art).await {
			return Ok(resp);
		}

		let Some(artist) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist]"));
		};

		let album = split.next();

		// Return error if more than 3 endpoints.
		match split.next() {
			Some(s) if !s.is_empty() => return Ok(resp::not_found(ERR_END)),
			_ => (),
		}

		if let Some(album) = album {
			art_album(artist.as_ref(), album.as_ref(), collection.arc()).await
		} else {
			art_artist(artist.as_ref(), collection.arc()).await
		}
	//-------------------------------------------------- `/current` endpoint.
	} else if ep1 == "current" {
		let Some(ep2) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist/album/song/art]"));
		};

		// Return error if more than 2 endpoints.
		match split.next() {
			Some(s) if !s.is_empty() => return Ok(resp::not_found(ERR_END)),
			_ => (),
		}

		let Some(resource) = Resource::from_str_not_c(ep2) else {
			return Ok(resp::not_found(ERR_END));
		};

		// Auth.
		if let Some(resp) = rest_auth_ok(&parts, &addr, resource).await {
			return Ok(resp);
		}

		match resource {
			Resource::Artist => current_artist(collection.arc()).await,
			Resource::Album  => current_album(collection.arc()).await,
			Resource::Song   => current_song(collection.arc()).await,
			Resource::Art    => current_art(collection.arc()).await,
			_ => {
				debug_panic!("parsed resource {resource:?}, but reached unreachable");
				Ok(resp::server_err("Unknown resource"))
			},
		}
	//-------------------------------------------------- `/rand` endpoint.
	} else if ep1 == "rand" {
		let Some(ep2) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist/album/song/art]"));
		};

		// Return error if more than 2 endpoints.
		match split.next() {
			Some(s) if !s.is_empty() => return Ok(resp::not_found(ERR_END)),
			_ => (),
		}

		let Some(resource) = Resource::from_str_not_c(ep2) else {
			return Ok(resp::not_found(ERR_END));
		};

		// Auth.
		if let Some(resp) = rest_auth_ok(&parts, &addr, resource).await {
			return Ok(resp);
		}

		match resource {
			Resource::Artist => rand_artist(collection.arc()).await,
			Resource::Album  => rand_album(collection.arc()).await,
			Resource::Song   => rand_song(collection.arc()).await,
			Resource::Art    => rand_art(collection.arc()).await,
			_ => {
				debug_panic!("parsed resource {resource:?}, but reached unreachable");
				Ok(resp::server_err("Unknown resource"))
			},
		}
	//-------------------------------------------------- `/collection` endpoint.
	} else if ep1 == "collection" {
		// Auth.
		if let Some(resp) = rest_auth_ok(&parts, &addr, Resource::Collection).await {
			return Ok(resp);
		}

		match split.next() {
			Some(s) if !s.is_empty() => return Ok(resp::not_found(ERR_END)),
			_ => (),
		}

		collection_fn(collection.arc()).await
	//-------------------------------------------------- unknown endpoint.
	} else {
		Ok(resp::not_found(ERR_END))
	}
}

//---------------------------------------------------------------------------------------------------- Open a `File` and read, async
const ERR_FILE: &str = "File not found";
const ERR_BYTE: &str = "Failed to read file bytes";

async fn read_file(path: &Path) -> Result<Vec<u8>, Response<Body>> {
	// Open the file.
	let Ok(mut file) = tokio::fs::File::open(&path).await else {
		return Err(resp::not_found(ERR_FILE));
	};

	let cap = match file.metadata().await {
		Ok(m) => m.len() as usize,
		_ => 1_000_000 // 1 megabyte,
	};

	// Copy the bytes into owned buffer.
	let mut dst: Vec<u8> = Vec::with_capacity(cap);
	if file.read_to_end(&mut dst).await.is_err() {
		return Err(resp::server_err(ERR_BYTE));
	};

	Ok(dst)
}

//---------------------------------------------------------------------------------------------------- The inner "impl", re-used in all other endpoints.
const MIME_ZIP: &str = "application/zip";
const ERR_ZIP:  &str = "Failed to create zip file";
const ERR_SONG: &str = "Song file not found";

use tokio_util::codec::{BytesCodec, FramedRead};
use crate::zip::{
	CollectionZip,
	ArtistZip,
	AlbumZip,
	ArtZip,
};

// Attempts to get file size.
async fn file_len(file: &tokio::fs::File) -> Option<u64> {
	if let Ok(md) = file.metadata().await {
		Some(md.len())
	} else {
		None
	}
}

// Takes in an already existing `ZipWriter`, and writes an artist to it.
// This exists to de-dup code between `impl_artist()` and `collection()`.
async fn impl_artist_inner(
	zip:        &mut zip::ZipWriter<std::fs::File>,
	options:    zip::write::FileOptions,
	artist:     &Artist,
	collection: &Arc<Collection>,
	folder:     &str,
) -> Option<Response<Body>> {
	for album_key in &artist.albums {
		let album = &collection.albums[album_key];

		if let Some(r) = impl_album_inner(zip, options, album, &collection, &format!("{folder}/{}/{}", artist.name, album.title)).await {
			return Some(r);
		}
	}

	None
}

// Takes in an already existing `ZipWriter`, and writes an album to it.
// This exists to de-dup code between `impl_artist()` and `impl_album()`.
async fn impl_album_inner(
	zip:        &mut zip::ZipWriter<std::fs::File>,
	options:    zip::write::FileOptions,
	album:      &Album,
	collection: &Arc<Collection>,
	folder:     &str,
) -> Option<Response<Body>> {
	// Write each `Song` into the `zip`.
	for song_key in &album.songs {
		let song = &collection.songs[song_key];

		trace!("Task - impl_album_inner() song: {}", song.path.display());

		let bytes = match read_file(&song.path).await {
			Ok(b)  => b,
			Err(r) => return Some(r),
		};

		let file_path = format!("{folder}/{}.{}", song.title, song.extension);

		let r = tokio::task::block_in_place(|| {
			if zip.start_file_from_path(&PathBuf::from(file_path), options).is_err() {
				return Some(resp::server_err(ERR_SONG));
			}

			if zip.write(&bytes).is_err() {
				return Some(resp::server_err(ERR_SONG));
			}

			None
		});

		if r.is_some() {
			return r;
		}
	}

	let artist = &collection.artists[album.artist];

	// Write `Art` if it exists.
	if let Art::Known { path, mime, len, extension } = &album.art {
		trace!("Task - impl_album_inner() art: {}", path.display());

		let bytes = match read_file(&path).await {
			Ok(b)  => b,
			Err(r) => return Some(r),
		};

		let file_path = format!("{folder}/{}{}{}.{}", artist.name, config().filename_separator, album.title, extension);

		if zip.start_file_from_path(&PathBuf::from(file_path), options).is_err() {
			return Some(resp::server_err(ERR_SONG));
		}

		if zip.write(&bytes).is_err() {
			return Some(resp::server_err(ERR_SONG));
		}
	}

	None
}

async fn impl_artist(key: ArtistKey, artist: &Artist, collection: &Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	trace!("Task - impl_artist(): {}", artist.name);

	// Zip name.
	let zip_name = format!("{}.zip", artist.name);

	// Create temporary `PATH` for a `ZIP`.
	let Ok(cache) = ArtistZip::new(&zip_name) else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// If the file exists already, serve it.
	if cache.exists() {
		if let Ok(file) = tokio::fs::File::open(&cache.real).await {
			trace!("Task - ArtistZip Cache hit: {zip_name}");
			let len    = file_len(&file).await;
			let stream = FramedRead::new(file, BytesCodec::new());
			let body   = Body::wrap_stream(stream);
			return Ok(resp::rest_zip(body, &zip_name, len));
		}
	}

	// Else, create file.
	let Ok(file) = std::fs::File::create(&cache.tmp) else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// Create `ZIP`.
	let mut zip = zip::ZipWriter::new(file);
	let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

	if let Some(r) = impl_artist_inner(&mut zip, options, artist, collection, &artist.name).await {
		return Ok(r);
	}

	if zip.finish().is_err() {
		return Ok(resp::server_err(ERR_ZIP));
	}

	if cache.tmp_to_real().is_err() {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// Re-open as `async`.
	let Ok(file) = tokio::fs::File::open(&cache.real).await else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	let len    = file_len(&file).await;
	let stream = FramedRead::new(file, BytesCodec::new());
	let body   = Body::wrap_stream(stream);

	Ok(resp::rest_zip(body, &zip_name, len))
}

async fn impl_album(key: AlbumKey, album: &Album, collection: &Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	trace!("Task - impl_album(): {}", album.title);

	let artist = &collection.artists[album.artist];

	// Zip name.
	let zip_name = format!("{}{}{}.zip", artist.name, config().filename_separator, album.title);

	// Create temporary `PATH` for a `ZIP`.
	let Ok(cache) = AlbumZip::new(&zip_name) else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// If the file exists already, serve it.
	if cache.exists() {
		if let Ok(file) = tokio::fs::File::open(&cache.real).await {
			trace!("Task - AlbumZip Cache hit: {zip_name}");
			let len    = file_len(&file).await;
			let stream = FramedRead::new(file, BytesCodec::new());
			let body   = Body::wrap_stream(stream);
			return Ok(resp::rest_zip(body, &zip_name, len));
		}
	}

	// Else, create file.
	let Ok(file) = std::fs::File::create(&cache.tmp) else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// Create `ZIP`.
	let mut zip = zip::ZipWriter::new(file);
	let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

	if let Some(r) = impl_album_inner(&mut zip, options, album, collection, &album.title).await {
		return Ok(r);
	}

	if zip.finish().is_err() {
		return Ok(resp::server_err(ERR_ZIP));
	}

	if cache.tmp_to_real().is_err() {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// Re-open.
	let Ok(file) = tokio::fs::File::open(&cache.real).await else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	let len    = file_len(&file).await;
	let stream = FramedRead::new(file, BytesCodec::new());
	let body   = Body::wrap_stream(stream);

	Ok(resp::rest_zip(body, &zip_name, len))
}

async fn impl_song(key: SongKey, song: &Song, collection: &Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	trace!("Task - impl_song(): {}", song.title);

	// Open the file.
	let Ok(mut file) = tokio::fs::File::open(&song.path).await else {
		return Ok(resp::not_found(ERR_SONG));
	};

	let cap = match file.metadata().await {
		Ok(m) => m.len() as usize,
		_ => 1_000_000 // 1 megabyte,
	};

	// Copy the bytes into owned buffer.
	let mut dst: Vec<u8> = Vec::with_capacity(cap);
	if file.read_to_end(&mut dst).await.is_err() {
		return Ok(resp::server_err(ERR_BYTE));
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
}

async fn impl_art(key: AlbumKey, album: &Album, collection: &Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	// If art exists...
	let Art::Known { path, mime, len, extension } = &album.art  else {
		return Ok(resp::not_found("No album art available"));
	};

	// Open the file.
	let Ok(mut file) = tokio::fs::File::open(&path).await else {
		return Ok(resp::not_found("Album art not found on filesystem"));
	};

	// Copy the bytes into owned buffer.
	let mut dst: Vec<u8> = Vec::with_capacity(*len);
	if file.read_to_end(&mut dst).await.is_err() {
		return Ok(resp::server_err("Failed to copy album art bytes"));
	};

	// Format the file name.
	let artist = &collection.artists[album.artist];
	let name = format!(
		"{}{}{}.{}",
		artist.name,
		config().filename_separator,
		album.title,
		extension,
	);

	Ok(resp::rest_ok(dst, &name, mime))
}

//---------------------------------------------------------------------------------------------------- `/key`
pub async fn key_artist(key: usize, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let key = ArtistKey::from(key);

	if let Some(artist) = collection.artists.get(key) {
		impl_artist(key, artist, &collection).await
	} else {
		Ok(resp::not_found("Artist key is invalid"))
	}
}

pub async fn key_album(key: usize, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let key = AlbumKey::from(key);

	if let Some(album) = collection.albums.get(key) {
		impl_album(key, album, &collection).await
	} else {
		Ok(resp::not_found("Album key is invalid"))
	}
}

pub async fn key_song(key: usize, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let key = SongKey::from(key);

	if let Some(song) = collection.songs.get(key) {
		impl_song(key, song, &collection).await
	} else {
		Ok(resp::not_found("Song key is invalid"))
	}
}

pub async fn key_art(key: usize, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let key = AlbumKey::from(key);

	// If key exists...
	if let Some(album) = collection.albums.get(key) {
		impl_art(key, album, &collection).await
	} else {
		Ok(resp::not_found("Album key is invalid"))
	}
}

//---------------------------------------------------------------------------------------------------- `/map`
pub async fn map_artist(artist: &str, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	if let Some((artist, key)) = collection.artist(artist) {
		impl_artist(key, artist, &collection).await
	} else {
		Ok(resp::not_found("Artist not found"))
	}
}

pub async fn map_album(artist: &str, album: &str, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	if let Some((album, key)) = collection.album(artist, album) {
		impl_album(key, album, &collection).await
	} else {
		Ok(resp::not_found("Artist/Album not found"))
	}
}

pub async fn map_song(artist: &str, album: &str, song: &str, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	if let Some((song, key)) = collection.song(artist, album, song) {
		impl_song(key, song, &collection).await
	} else {
		Ok(resp::not_found("Artist/Album/Song not found"))
	}
}

//---------------------------------------------------------------------------------------------------- `/current`
// These RPC calls aren't important enough
// to block `Audio`, so just wait until
// the lock is uncontended.
async fn impl_current_song() -> Option<SongKey> {
	loop {
		if let Ok(a) = shukusai::state::AUDIO_STATE.try_read() {
			return a.song.clone();
		}

		tokio::time::sleep(std::time::Duration::from_millis(5)).await;
	}
}

pub async fn current_artist(collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let song = impl_current_song().await;

	if let Some(key) = song {
		let (artist, key) = collection.artist_from_song(key);
		impl_artist(key, artist, &collection).await
	} else {
		Ok(resp::not_found("No current song"))
	}
}

pub async fn current_album(collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let song = impl_current_song().await;

	if let Some(key) = song {
		let (album, key) = collection.album_from_song(key);
		impl_album(key, album, &collection).await
	} else {
		Ok(resp::not_found("No current song"))
	}
}

pub async fn current_song(collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let song = impl_current_song().await;

	if let Some(key) = song {
		let song = &collection.songs[key];
		impl_song(key, song, &collection).await
	} else {
		Ok(resp::not_found("No current song"))
	}
}

pub async fn current_art(collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let song = impl_current_song().await;

	if let Some(key) = song {
		let (album, key) = collection.album_from_song(key);
		impl_art(key, album, &collection).await
	} else {
		Ok(resp::not_found("No current song"))
	}
}

//---------------------------------------------------------------------------------------------------- `/rand`
pub async fn rand_artist(collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let Some(key) = collection.rand_artist(None) else {
		return Ok(resp::not_found("No artists"));
	};

	impl_artist(key, &collection.artists[key], &collection).await
}

pub async fn rand_album(collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let Some(key) = collection.rand_album(None) else {
		return Ok(resp::not_found("No albums"));
	};

	impl_album(key, &collection.albums[key], &collection).await
}

pub async fn rand_song(collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let Some(key) = collection.rand_song(None) else {
		return Ok(resp::not_found("No songs"));
	};

	impl_song(key, &collection.songs[key], &collection).await
}

pub async fn rand_art(collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let Some(key) = collection.rand_album(None) else {
		return Ok(resp::not_found("No art"));
	};

	impl_art(key, &collection.albums[key], &collection).await
}

//---------------------------------------------------------------------------------------------------- `/art`
pub async fn art_artist(artist: &str, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	let Some((artist, key)) = collection.artist(artist) else {
		return Ok(resp::not_found("Artist was not found"));
	};

	// Zip name.
	let zip_name = format!("Art{}{}.zip", config().filename_separator, artist.name);

	// Create temporary `PATH` for a `ZIP`.
	let Ok(cache) = ArtZip::new(&zip_name) else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// If the file exists already, serve it.
	if cache.exists() {
		if let Ok(file) = tokio::fs::File::open(&cache.real).await {
			trace!("Task - ArtZip Cache hit: {zip_name}");
			let len    = file_len(&file).await;
			let stream = FramedRead::new(file, BytesCodec::new());
			let body   = Body::wrap_stream(stream);
			return Ok(resp::rest_zip(body, &zip_name, len));
		}
	}

	// Else, create file.
	let Ok(file) = std::fs::File::create(&cache.tmp) else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// Create `ZIP`.
	let mut zip = zip::ZipWriter::new(file);
	let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

	for album_key in artist.albums.iter() {
		let album = &collection.albums[album_key];

		// Write `Art` if it exists.
		if let Art::Known { path, mime, len, extension } = &album.art {
			trace!("Task - art_artist() art: {}", path.display());

			let bytes = match read_file(&path).await {
				Ok(b)  => b,
				Err(r) => return Ok(resp::not_found("Album art was not found")),
			};

			let file_path = format!("{}.{}", album.title, extension);

			let r = tokio::task::block_in_place(|| {
				if zip.start_file_from_path(&PathBuf::from(file_path), options).is_err() {
					return Some(resp::server_err(ERR_SONG));
				}

				if zip.write(&bytes).is_err() {
					return Some(resp::server_err(ERR_SONG));
				}

				None
			});

			if let Some(r) = r {
				return Ok(r);
			}
		}
	}

	if zip.finish().is_err() {
		return Ok(resp::server_err(ERR_ZIP));
	}

	if cache.tmp_to_real().is_err() {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// Re-open.
	let Ok(file) = tokio::fs::File::open(&cache.real).await else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	let len    = file_len(&file).await;
	let stream = FramedRead::new(file, BytesCodec::new());
	let body   = Body::wrap_stream(stream);

	Ok(resp::rest_zip(body, &zip_name, len))
}

pub async fn art_album(artist: &str, album: &str, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	// If album exists...
	if let Some((album, key)) = collection.album(artist, album) {
		impl_art(key, album, &collection).await
	} else {
		Ok(resp::not_found("Album was not found"))
	}
}

//---------------------------------------------------------------------------------------------------- `/collection`
pub async fn collection_fn(collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	// Zip name.
	let file_path = format!("Collection{}{}", config().filename_separator, collection.timestamp);
	let zip_name  = format!("{file_path}.zip");

	// Create temporary `PATH` for a `ZIP`.
	let Ok(cache) = CollectionZip::new(&zip_name) else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// If the file exists already, serve it.
	if cache.exists() {
		if let Ok(file) = tokio::fs::File::open(&cache.real).await {
			trace!("Task - CollectionZip Cache hit: {zip_name}");
			let len    = file_len(&file).await;
			let stream = FramedRead::new(file, BytesCodec::new());
			let body   = Body::wrap_stream(stream);
			return Ok(resp::rest_zip(body, &zip_name, len));
		}
	}

	// Else, create file.
	let Ok(file) = std::fs::File::create(&cache.tmp) else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// Create `ZIP`.
	let mut zip = zip::ZipWriter::new(file);
	let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

	for artist in collection.artists.iter() {
		if let Some(r) = impl_artist_inner(&mut zip, options, artist, &collection, &file_path).await {
			return Ok(r);
		}
	}

	let Ok(state_collection_full) = serde_json::to_vec_pretty(&collection) else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	if zip.start_file_from_path(&PathBuf::from(format!("{file_path}/state_collection_full.json")), options).is_err() {
		return Ok(resp::server_err(ERR_ZIP));
	}

	if zip.write(&state_collection_full).is_err() {
		return Ok(resp::server_err(ERR_ZIP));
	}

	if zip.finish().is_err() {
		return Ok(resp::server_err(ERR_ZIP));
	}

	if cache.tmp_to_real().is_err() {
		return Ok(resp::server_err(ERR_ZIP));
	};

	// Re-open.
	let Ok(file) = tokio::fs::File::open(&cache.real).await else {
		return Ok(resp::server_err(ERR_ZIP));
	};

	let len    = file_len(&file).await;
	let stream = FramedRead::new(file, BytesCodec::new());
	let body   = Body::wrap_stream(stream);

	Ok(resp::rest_zip(body, &zip_name, len))
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
