//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use crate::collection::{
	Collection,
	Artist,
	Album,
	Song,
};

//---------------------------------------------------------------------------------------------------- Sort Constants
const ARTIST_LEXI:                    &str = "Artists lexicographically";
const ARTIST_ALBUM_COUNT:             &str = "Artists per album count";
const ARTIST_SONG_COUNT:              &str = "Artists per song count";
const ALBUM_RELEASE_ARTIST_LEXI:      &str = "Artists lexicographically, albums in release order";
const ALBUM_LEXI_ARTIST_LEXI:         &str = "Artists lexicographically, albums lexicographically";
const ALBUM_LEXI:                     &str = "Albums lexicographically";
const ALBUM_RELEASE:                  &str = "Albums in release order";
const ALBUM_RUNTIME:                  &str = "Albums shortest to longest";
const SONG_ALBUM_RELEASE_ARTIST_LEXI: &str = "Artists lexicographically, albums in release order, songs in track order";
const SONG_ALBUM_LEXI_ARTIST_LEXI:    &str = "Artists lexicographically, albums lexicographically,, songs in track order";
const SONG_LEXI:                      &str = "Songs lexicographically";
const SONG_RELEASE:                   &str = "Songs in release order";
const SONG_RUNTIME:                   &str = "Songs shortest to longest";

//---------------------------------------------------------------------------------------------------- Sort
/// All the ways to sort the [`Collection`]'s [`Artist`]'s.
///
/// String sorting is done lexicographically as per the `std` [`Ord` implementation.](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord)
///
/// `lexi` is shorthand for `lexicographically`.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub enum ArtistSort {
	#[default]
	/// [`Artist`] in `lexi`. Field: [`Collection::sort_artist_lexi`].
	Lexi,
	/// [`Artist`] with most `Album`'s to least. Field: [`Collection::sort_artist_album_count`].
	AlbumCount,
	/// [`Artist`] with most `Song`'s to least. Field: [`Collection::sort_artist_song_count`].
	SongCount,
}

/// All the ways to sort the [`Collection`]'s [`Album`]'s.
///
/// String sorting is done lexicographically as per the `std` [`Ord` implementation.](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord)
///
/// `lexi` is shorthand for `lexicographically`.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub enum AlbumSort {
	#[default]
	/// [`Artist`] `lexi`, [`Album`]'s oldest release to latest. Field: [`Collection::sort_album_release_artist_lexi`].
	ReleaseArtistLexi,
	/// [`Artist`] `lexi`, [`Album`]'s `lexi`. Field: [`Collection::sort_album_lexi_artist_lexi`].
	LexiArtistLexi,
	/// [`Album`] lexi. Field: [`Collection::sort_album_lexi`].
	Lexi,
	/// [`Album`] oldest to latest. Field: [`Collection::sort_album_release`].
	Release,
	/// [`Album`] shortest to longest. Field: [`Collection::sort_album_runtime`].
	Runtime,
}

/// All the ways to sort the [`Collection`]'s [`Song`]'s.
///
/// String sorting is done lexicographically as per the `std` [`Ord` implementation.](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord)
///
/// `lexi` is shorthand for `lexicographically`.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub enum SongSort {
	#[default]
	/// [`Artist`] lexi, [`Album`] release, [`Song`] track_number. Field: [`Collection::sort_song_album_release_artist_lexi`].
	AlbumReleaseArtistLexi,
	/// [`Artist`] lexi, [`Album`] lexi, [`Song`] track_number. Field: [`Collection::sort_song_album_lexi_artist_lexi`].
	AlbumLexiArtistLexi,
	/// [`Song`] lexi. Field: [`Collection::sort_song_lexi`].
	Lexi,
	/// [`Song`] oldest to latest. Field: [`Collection::sort_song_release`].
	Release,
	/// [`Song`] shortest to longest. Field: [`Collection::sort_song_runtime`].
	Runtime,
}

impl ArtistSort {
	#[inline(always)]
	/// Returns formatted, human readable versions.
	///
	/// e.g: [`ArtistSort::AlbumCount`] is `Artists per album count`
	pub fn as_str(&self) -> &'static str {
		use ArtistSort::*;
		match self {
			Lexi       => ARTIST_LEXI,
			AlbumCount => ARTIST_ALBUM_COUNT,
			SongCount  => ARTIST_SONG_COUNT,
		}
	}
}

impl AlbumSort {
	#[inline(always)]
	/// Returns formatted, human readable versions.
	///
	/// e.g: [`AlbumSort::ReleaseArtistLexi`] is `Artists lexicographically, albums in release order`
	pub fn as_str(&self) -> &'static str {
		use AlbumSort::*;
		match self {
			ReleaseArtistLexi => ALBUM_RELEASE_ARTIST_LEXI,
			LexiArtistLexi    => ALBUM_LEXI_ARTIST_LEXI,
			Lexi              => ALBUM_LEXI,
			Release           => ALBUM_RELEASE,
			Runtime           => ALBUM_RUNTIME,
		}
	}
}

impl SongSort {
	#[inline(always)]
	/// Returns formatted, human readable versions.
	///
	/// e.g: [`SongSort::AlbumReleaseArtistLexi`] is `Artists lexicographically, albums in release order, songs in track order`
	pub fn as_str(&self) -> &'static str {
		use SongSort::*;
		match self {
			AlbumReleaseArtistLexi => SONG_ALBUM_RELEASE_ARTIST_LEXI,
			AlbumLexiArtistLexi    => SONG_ALBUM_LEXI_ARTIST_LEXI,
			Lexi                   => SONG_LEXI,
			Release                => SONG_RELEASE,
			Runtime                => SONG_RUNTIME,
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
