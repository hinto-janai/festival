//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use crate::collection::{
	Collection,
	Artist,Album,Song,
	ArtistKey,
	AlbumKey,
	SongKey,
};

//---------------------------------------------------------------------------------------------------- Collection
#[allow(missing_docs)]
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
/// A struct representation of `Collection`'s JSON serialization output.
pub struct CollectionJson {
	pub empty: bool,
	pub timestamp: u64,
	pub count_artist: u64,
	pub count_album: u64,
	pub count_song: u64,
	pub count_art: u64,

	pub artists: Box<[ArtistJson]>,
	pub albums: Box<[AlbumJson]>,
	pub songs: Box<[SongJson]>,

	pub sort_artist_lexi: Box<[ArtistKey]>,
	pub sort_artist_lexi_rev: Box<[ArtistKey]>,
	pub sort_artist_album_count: Box<[ArtistKey]>,
	pub sort_artist_album_count_rev: Box<[ArtistKey]>,
	pub sort_artist_song_count: Box<[ArtistKey]>,
	pub sort_artist_song_count_rev: Box<[ArtistKey]>,
	pub sort_artist_runtime: Box<[ArtistKey]>,
	pub sort_artist_runtime_rev: Box<[ArtistKey]>,
	pub sort_artist_name: Box<[ArtistKey]>,
	pub sort_artist_name_rev: Box<[ArtistKey]>,

	pub sort_album_release_artist_lexi: Box<[AlbumKey]>,
	pub sort_album_release_artist_lexi_rev: Box<[AlbumKey]>,
	pub sort_album_release_rev_artist_lexi: Box<[AlbumKey]>,
	pub sort_album_release_rev_artist_lexi_rev: Box<[AlbumKey]>,
	pub sort_album_lexi_artist_lexi: Box<[AlbumKey]>,
	pub sort_album_lexi_artist_lexi_rev: Box<[AlbumKey]>,
	pub sort_album_lexi_rev_artist_lexi: Box<[AlbumKey]>,
	pub sort_album_lexi_rev_artist_lexi_rev: Box<[AlbumKey]>,
	pub sort_album_lexi: Box<[AlbumKey]>,
	pub sort_album_lexi_rev: Box<[AlbumKey]>,
	pub sort_album_release: Box<[AlbumKey]>,
	pub sort_album_release_rev: Box<[AlbumKey]>,
	pub sort_album_runtime: Box<[AlbumKey]>,
	pub sort_album_runtime_rev: Box<[AlbumKey]>,
	pub sort_album_title: Box<[AlbumKey]>,
	pub sort_album_title_rev: Box<[AlbumKey]>,

	pub sort_song_album_release_artist_lexi: Box<[SongKey]>,
	pub sort_song_album_release_artist_lexi_rev: Box<[SongKey]>,
	pub sort_song_album_release_rev_artist_lexi: Box<[SongKey]>,
	pub sort_song_album_release_rev_artist_lexi_rev: Box<[SongKey]>,
	pub sort_song_album_lexi_artist_lexi: Box<[SongKey]>,
	pub sort_song_album_lexi_artist_lexi_rev: Box<[SongKey]>,
	pub sort_song_album_lexi_rev_artist_lexi: Box<[SongKey]>,
	pub sort_song_album_lexi_rev_artist_lexi_rev: Box<[SongKey]>,
	pub sort_song_lexi: Box<[SongKey]>,
	pub sort_song_lexi_rev: Box<[SongKey]>,
	pub sort_song_release: Box<[SongKey]>,
	pub sort_song_release_rev: Box<[SongKey]>,
	pub sort_song_runtime: Box<[SongKey]>,
	pub sort_song_runtime_rev: Box<[SongKey]>,
	pub sort_song_title: Box<[SongKey]>,
	pub sort_song_title_rev: Box<[SongKey]>,
}

#[allow(missing_docs)]
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
/// A struct representation of `Artist`'s JSON serialization output.
pub struct ArtistJson {
	pub name: String,
	pub key: ArtistKey,
	pub runtime: u32,
	pub albums: Box<[AlbumKey]>,
	pub songs: Box<[SongKey]>,
}

#[allow(missing_docs)]
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
/// A struct representation of `Album`'s JSON serialization output.
pub struct AlbumJson {
	pub title: String,
	pub key: AlbumKey,
	pub artist: ArtistKey,
	pub release: String,
	pub runtime: u32,
	pub song_count: usize,
	pub songs: Box<[SongKey]>,
	pub discs: u32,
	pub art: Option<String>,
	pub genre: Option<String>,
}

#[allow(missing_docs)]
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
/// A struct representation of `Song`'s JSON serialization output.
pub struct SongJson {
	pub title: String,
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
