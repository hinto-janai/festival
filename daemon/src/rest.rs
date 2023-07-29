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
	hash::Hash,
	config::{Config,config},
	ptr::CollectionPtr,
};
use tokio::io::AsyncReadExt;
use std::path::Path;
use std::io::Write;

//---------------------------------------------------------------------------------------------------- Const
const ERR_END: &str =  "Unknown endpoint";

//---------------------------------------------------------------------------------------------------- REST Handler
pub async fn handle(
	parts:      Parts,
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
//		crate::router::sleep_on_fail_task(collection);
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
		if split.next().is_some() {
			return Ok(resp::not_found(ERR_END));
		}

		// Parse `usize` key.
		let Ok(key) = ep3.parse::<usize>() else {
			return Ok(resp::not_found("Key parse failure"));
		};

		match ep2 {
			"artist" => key_artist(key, collection.arc()).await,
			"album"  => key_album(key, collection.arc()).await,
			"song"   => key_song(key, collection.arc()).await,
			"art"    => key_art(key, collection.arc()).await,
			_        => Ok(resp::not_found(ERR_END)),
		}
	//-------------------------------------------------- `/map` endpoint.
	} else if ep1 == "map" {
		let Some(artist) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist]"));
		};

		let album = split.next();
		let song = split.next();

		// Return error if more than 4 endpoints.
		if split.next().is_some() {
			return Ok(resp::not_found(ERR_END));
		}

		match (album, song) {
			(Some(a), None)    => map_album(artist.as_ref(), a.as_ref(), collection.arc()).await,
			(Some(a), Some(s)) => map_song(artist.as_ref(), a.as_ref(), s.as_ref(), collection.arc()).await,
			_                  => map_artist(artist.as_ref(), collection.arc()).await,
		}
	//-------------------------------------------------- `/art` endpoint.
	} else if ep1 == "art" {
		let Some(artist) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist]"));
		};

		let Some(album) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [album]"));
		};

		// Return error if more than 3 endpoints.
		if split.next().is_some() {
			return Ok(resp::not_found(ERR_END));
		}

		art(artist.as_ref(), album.as_ref(), collection.arc()).await
	//-------------------------------------------------- `/current` endpoint.
	} else if ep1 == "current" {
		let Some(ep2) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist/album/song/art]"));
		};

		// Return error if more than 2 endpoints.
		if split.next().is_some() {
			return Ok(resp::not_found(ERR_END));
		}

		match ep2 {
			"artist" => current_artist(collection.arc()).await,
			"album"  => current_album(collection.arc()).await,
			"song"   => current_song(collection.arc()).await,
			"art"    => current_art(collection.arc()).await,
			_        => Ok(resp::not_found(ERR_END)),
		}
	//-------------------------------------------------- `/rand` endpoint.
	} else if ep1 == "rand" {
		let Some(ep2) = split.next() else {
			return Ok(resp::not_found("Missing endpoint: [artist/album/song/art]"));
		};

		// Return error if more than 2 endpoints.
		if split.next().is_some() {
			return Ok(resp::not_found(ERR_END));
		}

		match ep2 {
			"artist" => rand_artist(collection.arc()).await,
			"album"  => rand_album(collection.arc()).await,
			"song"   => rand_song(collection.arc()).await,
			"art"    => rand_art(collection.arc()).await,
			_        => Ok(resp::not_found(ERR_END)),
		}
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

// Takes in an already existing `ZipWriter`, and writes an album to it.
// This exists to de-dup code between `impl_artist()` and `impl_album()`.
async fn impl_album_inner(
	zip:        &mut zip::ZipWriter<std::io::Cursor<Vec<u8>>>,
	options:    &zip::write::FileOptions,
	album:      &Album,
	collection: &Arc<Collection>,
	folder:     Option<&str>,
) -> Option<Response<Body>> {
	let folder = match folder {
		Some(f) => format!("{f}/"),
		None    => "".to_string(),
	};

	// Write each `Song` into the `zip`.
	for song_key in &album.songs {
		let song = &collection.songs[song_key];

		let bytes = match read_file(&song.path).await {
			Ok(b)  => b,
			Err(r) => return Some(r),
		};

		let file_path = format!("{folder}{}.{}", song.title, song.extension);

		trace!("Task - impl_album_inner() song: {file_path}");

		if zip.start_file(&file_path, *options).is_err() {
			return Some(resp::server_err(ERR_SONG));
		}

		if zip.write(&bytes).is_err() {
			return Some(resp::server_err(ERR_ZIP));
		}
	}

	let artist = &collection.artists[album.artist];

	// Write `Art` if it exists.
	if let Art::Known { path, mime, len, extension } = &album.art {
		let bytes = match read_file(&path).await {
			Ok(b)  => b,
			Err(r) => return Some(r),
		};

		let file_path = format!("{folder}{}{}{}.{}", artist.name, config().filename_separator, album.title, extension);

		trace!("Task - impl_album_inner() art: {file_path}");

		if zip.start_file(&file_path, *options).is_err() {
			return Some(resp::server_err(ERR_ZIP));
		}

		if zip.write(&bytes).is_err() {
			return Some(resp::server_err(ERR_ZIP));
		}
	}

	None
}

async fn impl_artist(key: ArtistKey, artist: &Artist, collection: &Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	tokio::task::block_in_place(move || async move {
		trace!("Task - impl_artist(): {}", artist.name);

		let mut buf = vec![];
		let mut zip = zip::ZipWriter::new(std::io::Cursor::new(buf));
		let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

		for album_key in &artist.albums {
			let album = &collection.albums[album_key];

			trace!("Task - impl_artist() album: {}", album.title);

			if zip.add_directory(&*album.title, options).is_err() {
				return Ok(resp::server_err(ERR_ZIP));
			}

			if let Some(r) = impl_album_inner(&mut zip, &options, album, collection, Some(&*album.title)).await {
				return Ok(r);
			}
		}

		// Zip name.
		let name = format!("{}.zip", artist.name);

		let buf = zip.finish()?.into_inner();

		Ok(resp::rest_ok(buf, &name, MIME_ZIP))
	}).await
}

async fn impl_album(key: AlbumKey, album: &Album, collection: &Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	tokio::task::block_in_place(move || async move {
		trace!("Task - impl_album(): {}", album.title);

		let mut buf = vec![];
		let mut zip = zip::ZipWriter::new(std::io::Cursor::new(buf));
		let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

		if let Some(r) = impl_album_inner(&mut zip, &options, album, collection, None).await {
			return Ok(r);
		}

		let artist = &collection.artists[album.artist];

		// Zip name.
		let name = format!("{}{}{}.zip", artist.name, config().filename_separator, album.title);

		let Ok(buf) = zip.finish() else {
			return Ok(resp::server_err(ERR_ZIP));
		};

		Ok(resp::rest_ok(buf.into_inner(), &name, MIME_ZIP))
	}).await
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
pub async fn art(artist: &str, album: &str, collection: Arc<Collection>) -> Result<Response<Body>, anyhow::Error> {
	// If album exists...
	if let Some((album, key)) = collection.album(artist, album) {
		impl_art(key, album, &collection).await
	} else {
		Ok(resp::not_found("Album was not found"))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
