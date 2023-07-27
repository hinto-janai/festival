//---------------------------------------------------------------------------------------------------- Use
use anyhow::anyhow;
use std::sync::Arc;
use serde::{Serialize,Deserialize};
use crate::collection::{
	Collection,
	Artist,Album,Song,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use std::borrow::Cow;

//---------------------------------------------------------------------------------------------------- Collection
#[allow(missing_docs)]
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
/// A struct representation of `Collection`'s JSON serialization output.
pub struct CollectionJson<'a> {
	pub empty: bool,
	pub timestamp: u64,
	pub count_artist: u64,
	pub count_album: u64,
	pub count_song: u64,
	pub count_art: u64,

	#[serde(borrow)]
	pub artists: Cow<'a, [ArtistJson<'a>]>,
	#[serde(borrow)]
	pub albums: Cow<'a, [AlbumJson<'a>]>,
	#[serde(borrow)]
	pub songs: Cow<'a, [SongJson<'a>]>,

	#[serde(borrow)]
	pub sort_artist_lexi: Cow<'a, [ArtistKey]>,
	#[serde(borrow)]
	pub sort_artist_lexi_rev: Cow<'a, [ArtistKey]>,
	#[serde(borrow)]
	pub sort_artist_album_count: Cow<'a, [ArtistKey]>,
	#[serde(borrow)]
	pub sort_artist_album_count_rev: Cow<'a, [ArtistKey]>,
	#[serde(borrow)]
	pub sort_artist_song_count: Cow<'a, [ArtistKey]>,
	#[serde(borrow)]
	pub sort_artist_song_count_rev: Cow<'a, [ArtistKey]>,
	#[serde(borrow)]
	pub sort_artist_runtime: Cow<'a, [ArtistKey]>,
	#[serde(borrow)]
	pub sort_artist_runtime_rev: Cow<'a, [ArtistKey]>,
	#[serde(borrow)]
	pub sort_artist_name: Cow<'a, [ArtistKey]>,
	#[serde(borrow)]
	pub sort_artist_name_rev: Cow<'a, [ArtistKey]>,

	#[serde(borrow)]
	pub sort_album_release_artist_lexi: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_release_artist_lexi_rev: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_release_rev_artist_lexi: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_release_rev_artist_lexi_rev: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_lexi_artist_lexi: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_lexi_artist_lexi_rev: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_lexi_rev_artist_lexi: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_lexi_rev_artist_lexi_rev: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_lexi: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_lexi_rev: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_release: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_release_rev: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_runtime: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_runtime_rev: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_title: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub sort_album_title_rev: Cow<'a, [AlbumKey]>,

	#[serde(borrow)]
	pub sort_song_album_release_artist_lexi: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_album_release_artist_lexi_rev: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_album_release_rev_artist_lexi: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_album_release_rev_artist_lexi_rev: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_album_lexi_artist_lexi: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_album_lexi_artist_lexi_rev: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_album_lexi_rev_artist_lexi: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_album_lexi_rev_artist_lexi_rev: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_lexi: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_lexi_rev: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_release: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_release_rev: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_runtime: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_runtime_rev: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_title: Cow<'a, [SongKey]>,
	#[serde(borrow)]
	pub sort_song_title_rev: Cow<'a, [SongKey]>,
}

#[allow(missing_docs)]
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
/// A struct representation of `Artist`'s JSON serialization output.
pub struct ArtistJson<'a> {
	#[serde(borrow)]
	pub name: Cow<'a, str>,
	pub key: ArtistKey,
	pub runtime: u32,
	#[serde(borrow)]
	pub albums: Cow<'a, [AlbumKey]>,
	#[serde(borrow)]
	pub songs: Cow<'a, [SongKey]>,
}

#[allow(missing_docs)]
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
/// A struct representation of `Album`'s JSON serialization output.
pub struct AlbumJson<'a> {
	#[serde(borrow)]
	pub title: Cow<'a, str>,
	pub key: AlbumKey,
	pub artist: ArtistKey,
	#[serde(borrow)]
	pub release: Cow<'a, str>,
	pub runtime: u32,
	pub song_count: usize,
	#[serde(borrow)]
	pub songs: Cow<'a, [SongKey]>,
	pub discs: u32,
	pub art: Option<u64>,
	#[serde(borrow)]
	pub genre: Option<Cow<'a, str>>,
}

#[allow(missing_docs)]
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
/// A struct representation of `Song`'s JSON serialization output.
pub struct SongJson<'a> {
	#[serde(borrow)]
	pub title: Cow<'a, str>,
	pub key: SongKey,
	pub album: AlbumKey,
	pub runtime: u32,
	pub sample_rate: u32,
	pub track: Option<u32>,
	pub disc: Option<u32>,
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
#[cfg(feature = "daemon")]
mod tests {
	use super::*;

	const EXPECTED_COLLECTION: &str =
r#"{
  empty: true,
  timestamp: 0,
  count_artist: 0,
  count_album: 0,
  count_song: 0,
  count_art: 0,
  artists: [],
  albums: [],
  songs: [],
  sort_artist_lexi: [],
  sort_artist_lexi_rev: [],
  sort_artist_album_count: [],
  sort_artist_album_count_rev: [],
  sort_artist_song_count: [],
  sort_artist_song_count_rev: [],
  sort_artist_runtime: [],
  sort_artist_runtime_rev: [],
  sort_artist_name: [],
  sort_artist_name_rev: [],
  sort_album_release_artist_lexi: [],
  sort_album_release_artist_lexi_rev: [],
  sort_album_release_rev_artist_lexi: [],
  sort_album_release_rev_artist_lexi_rev: [],
  sort_album_lexi_artist_lexi: [],
  sort_album_lexi_artist_lexi_rev: [],
  sort_album_lexi_rev_artist_lexi: [],
  sort_album_lexi_rev_artist_lexi_rev: [],
  sort_album_lexi: [],
  sort_album_lexi_rev: [],
  sort_album_release: [],
  sort_album_release_rev: [],
  sort_album_runtime: [],
  sort_album_runtime_rev: [],
  sort_album_title: [],
  sort_album_title_rev: [],
  sort_song_album_release_artist_lexi: [],
  sort_song_album_release_artist_lexi_rev: [],
  sort_song_album_release_rev_artist_lexi: [],
  sort_song_album_release_rev_artist_lexi_rev: [],
  sort_song_album_lexi_artist_lexi: [],
  sort_song_album_lexi_artist_lexi_rev: [],
  sort_song_album_lexi_rev_artist_lexi: [],
  sort_song_album_lexi_rev_artist_lexi_rev: [],
  sort_song_lexi: [],
  sort_song_lexi_rev: [],
  sort_song_release: [],
  sort_song_release_rev: [],
  sort_song_runtime: [],
  sort_song_runtime_rev: [],
  sort_song_title: [],
  sort_song_title_rev: []
}"#;

	const EXPECTED_ARTIST: &str =
r#"{
  "name": "",
  "key": 0,
  "runtime": 0,
  "albums": [],
  "songs": []
}"#;

	const EXPECTED_ALBUM: &str =
r#"{
  "title": "",
  "key": 0,
  "artist": 0,
  "release": "????-??-??",
  "runtime": 0,
  "song_count": 0,
  "songs": [],
  "discs": 0,
  "art": null,
  "genre": null
}"#;

	const EXPECTED_SONG: &str =
r#"{
  "title": "",
  "key": 0,
  "album": 0,
  "runtime": 0,
  "sample_rate": 0,
  "track": null,
  "disc": null
}"#;

	fn serde_json_collection() {
		let s: String = serde_json::to_string_pretty(&Collection::new()).unwrap();
		assert_eq!(EXPECTED_COLLECTION, s);
		let d: CollectionJson = serde_json::from_str(&s).unwrap();
		assert_eq!(EXPECTED_COLLECTION, serde_json::to_string_pretty(&d).unwrap());
	}

	#[test]
	fn serde_json_artist() {
		let s: String = serde_json::to_string_pretty(&Artist::default()).unwrap();
		assert_eq!(EXPECTED_ARTIST, s);
		let d: ArtistJson = serde_json::from_str(&s).unwrap();
		assert_eq!(EXPECTED_ARTIST, serde_json::to_string_pretty(&d).unwrap());
	}

	#[test]
	fn serde_json_album() {
		let s: String = serde_json::to_string_pretty(&Album::default()).unwrap();
		assert_eq!(EXPECTED_ALBUM, s);
		let d: AlbumJson = serde_json::from_str(&s).unwrap();
		assert_eq!(EXPECTED_ALBUM, serde_json::to_string_pretty(&d).unwrap());
	}

	#[test]
	fn serde_json_song() {
		let s: String = serde_json::to_string_pretty(&Song::default()).unwrap();
		assert_eq!(EXPECTED_SONG, s);
		let d: SongJson = serde_json::from_str(&s).unwrap();
		assert_eq!(EXPECTED_SONG, serde_json::to_string_pretty(&d).unwrap());
	}
}
