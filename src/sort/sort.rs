//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use crate::collection::{
	Collection,
	Artist,
	Album,
	Song,
};

//---------------------------------------------------------------------------------------------------- Sort Constants
/// [`ArtistSort::Lexi`]
pub const ARTIST_LEXI:                    &str = "Artists lexicographically (A-Z)";
/// [`ArtistSort::LexiRev`]
pub const ARTIST_LEXI_REV:                &str = "Artists lexicographically (Z-A)";
/// [`ArtistSort::AlbumCount`]
pub const ARTIST_ALBUM_COUNT:             &str = "Artists per album count (least to most)";
/// [`ArtistSort::AlbumCountRev`]
pub const ARTIST_ALBUM_COUNT_REV:         &str = "Artists per album count (most to least)";
/// [`ArtistSort::SongCount`]
pub const ARTIST_SONG_COUNT:              &str = "Artists per song count (least to most)";
/// [`ArtistSort::SongCountRev`]
pub const ARTIST_SONG_COUNT_REV:          &str = "Artists per song count (most to least)";

/// [`AlbumSort::ReleaseArtistLexi`]
pub const ALBUM_RELEASE_ARTIST_LEXI:      &str = "Artists lexicographically (A-Z), albums in release order";
/// [`AlbumSort::ReleaseArtistLexiRev`]
pub const ALBUM_RELEASE_ARTIST_LEXI_REV:  &str = "Artists lexicographically (Z-A), albums in release order";
/// [`AlbumSort::LexiArtistLexi`]
pub const ALBUM_LEXI_ARTIST_LEXI:         &str = "Artists lexicographically (A-Z), albums lexicographically";
/// [`AlbumSort::LexiArtistLexiRev`]
pub const ALBUM_LEXI_ARTIST_LEXI_REV:     &str = "Artists lexicographically (Z-A), albums lexicographically";
/// [`AlbumSort::Lexi`]
pub const ALBUM_LEXI:                     &str = "Albums lexicographically (A-Z)";
/// [`AlbumSort::LexiRev`]
pub const ALBUM_LEXI_REV:                 &str = "Albums lexicographically (Z-A)";
/// [`AlbumSort::Release`]
pub const ALBUM_RELEASE:                  &str = "Albums in release order (oldest to latest)";
/// [`AlbumSort::ReleaseRev`]
pub const ALBUM_RELEASE_REV:              &str = "Albums in release order (latest to oldest)";
/// [`AlbumSort::Runtime`]
pub const ALBUM_RUNTIME:                  &str = "Albums shortest to longest";
/// [`AlbumSort::RuntimeRev`]
pub const ALBUM_RUNTIME_REV:              &str = "Albums longest to shortest";

/// [`SongSort::AlbumReleaseArtistLexi`]
pub const SONG_ALBUM_RELEASE_ARTIST_LEXI:     &str = "Artists lexicographically (A-Z), albums in release order, songs in track order";
/// [`SongSort::AlbumReleaseArtistLexiRev`]
pub const SONG_ALBUM_RELEASE_ARTIST_LEXI_REV: &str = "Artists lexicographically (Z-A), albums in release order, songs in track order";
/// [`SongSort::AlbumLexiArtistLexi`]
pub const SONG_ALBUM_LEXI_ARTIST_LEXI:        &str = "Artists lexicographically (A-Z), albums lexicographically,, songs in track order";
/// [`SongSort::AlbumLexiArtistLexiRev`]
pub const SONG_ALBUM_LEXI_ARTIST_LEXI_REV:    &str = "Artists lexicographically (Z-A), albums lexicographically,, songs in track order";
/// [`SongSort::Lexi`]
pub const SONG_LEXI:                          &str = "Songs lexicographically (A-Z)";
/// [`SongSort::Lexi`]
pub const SONG_LEXI_REV:                      &str = "Songs lexicographically (Z-A)";
/// [`SongSort::Release`]
pub const SONG_RELEASE:                       &str = "Songs in release order (oldest to latest)";
/// [`SongSort::ReleaseRev`]
pub const SONG_RELEASE_REV:                   &str = "Songs in release order (latest to oldest)";
/// [`SongSort::Runtime`]
pub const SONG_RUNTIME:                       &str = "Songs shortest to longest";
/// [`SongSort::RuntimeRev`]
pub const SONG_RUNTIME_REV:                   &str = "Songs longest to oldest";

//---------------------------------------------------------------------------------------------------- Sort
/// All the ways to sort the [`Collection`]'s [`Artist`]'s.
///
/// String sorting is done lexicographically as per the `std` [`Ord` implementation.](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord)
///
/// `lexi` is shorthand for `lexicographically`.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
pub enum ArtistSort {
	/// [`Artist`] in `lexi`. Field: [`Collection::sort_artist_lexi`].
	Lexi,
	/// [`Artist`] in `lexi`. Field: [`Collection::sort_artist_lexi`] (reversed).
	LexiRev,
	/// [`Artist`] with most `Album`'s to least. Field: [`Collection::sort_artist_album_count`].
	AlbumCount,
	#[default]
	/// [`Artist`] with most `Album`'s to least. Field: [`Collection::sort_artist_album_count`] (reversed).
	AlbumCountRev,
	/// [`Artist`] with most `Song`'s to least. Field: [`Collection::sort_artist_song_count`].
	SongCount,
	/// [`Artist`] with most `Song`'s to least. Field: [`Collection::sort_artist_song_count`] (reversed).
	SongCountRev,
}

/// All the ways to sort the [`Collection`]'s [`Album`]'s.
///
/// String sorting is done lexicographically as per the `std` [`Ord` implementation.](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord)
///
/// `lexi` is shorthand for `lexicographically`.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
pub enum AlbumSort {
	#[default]
	/// [`Artist`] `lexi`, [`Album`]'s oldest release to latest. Field: [`Collection::sort_album_release_artist_lexi`].
	ReleaseArtistLexi,
	/// [`Artist`] `lexi`, [`Album`]'s oldest release to latest. Field: [`Collection::sort_album_release_artist_lexi`] (reversed).
	ReleaseArtistLexiRev,
	/// [`Artist`] `lexi`, [`Album`]'s `lexi`. Field: [`Collection::sort_album_lexi_artist_lexi`].
	LexiArtistLexi,
	/// [`Artist`] `lexi`, [`Album`]'s `lexi`. Field: [`Collection::sort_album_lexi_artist_lexi`] (reversed).
	LexiArtistLexiRev,
	/// [`Album`] lexi. Field: [`Collection::sort_album_lexi`].
	Lexi,
	/// [`Album`] lexi. Field: [`Collection::sort_album_lexi`] (reversed).
	LexiRev,
	/// [`Album`] oldest to latest. Field: [`Collection::sort_album_release`].
	Release,
	/// [`Album`] latest to oldest. Field: [`Collection::sort_album_release`].
	ReleaseRev,
	/// [`Album`] shortest to longest. Field: [`Collection::sort_album_runtime`].
	Runtime,
	/// [`Album`] longest to shortest. Field: [`Collection::sort_album_runtime`].
	RuntimeRev,
}

/// All the ways to sort the [`Collection`]'s [`Song`]'s.
///
/// String sorting is done lexicographically as per the `std` [`Ord` implementation.](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord)
///
/// `lexi` is shorthand for `lexicographically`.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
pub enum SongSort {
	/// [`Artist`] lexi, [`Album`] release, [`Song`] track_number. Field: [`Collection::sort_song_album_release_artist_lexi`].
	AlbumReleaseArtistLexi,
	/// [`Artist`] lexi, [`Album`] release, [`Song`] track_number. Field: [`Collection::sort_song_album_release_artist_lexi`] (reversed).
	AlbumReleaseArtistLexiRev,
	/// [`Artist`] lexi, [`Album`] lexi, [`Song`] track_number. Field: [`Collection::sort_song_album_lexi_artist_lexi`].
	AlbumLexiArtistLexi,
	/// [`Artist`] lexi, [`Album`] lexi, [`Song`] track_number. Field: [`Collection::sort_song_album_lexi_artist_lexi`] (reversed).
	AlbumLexiArtistLexiRev,
	/// [`Song`] lexi. Field: [`Collection::sort_song_lexi`].
	#[default]
	Lexi,
	/// [`Song`] lexi. Field: [`Collection::sort_song_lexi`] (reversed).
	LexiRev,
	/// [`Song`] oldest to latest. Field: [`Collection::sort_song_release`].
	Release,
	/// [`Song`] latest to oldest. Field: [`Collection::sort_song_release`].
	ReleaseRev,
	/// [`Song`] shortest to longest. Field: [`Collection::sort_song_runtime`].
	Runtime,
	/// [`Song`] longest to shortest. Field: [`Collection::sort_song_runtime`].
	RuntimeRev,
}

impl ArtistSort {
	#[inline]
	/// Returns formatted, human readable versions.
	///
	/// e.g: [`ArtistSort::AlbumCount`] returns [`ARTIST_ALBUM_COUNT`]
	pub const fn as_str(&self) -> &'static str {
		use ArtistSort::*;
		match self {
			Lexi          => ARTIST_LEXI,
			LexiRev       => ARTIST_LEXI_REV,
			AlbumCount    => ARTIST_ALBUM_COUNT,
			AlbumCountRev => ARTIST_ALBUM_COUNT_REV,
			SongCount     => ARTIST_SONG_COUNT,
			SongCountRev  => ARTIST_SONG_COUNT_REV,
		}
	}

	#[inline]
	/// Returns an iterator over all [`ArtistSort`] variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		[
			Self::Lexi,
			Self::LexiRev,
			Self::AlbumCount,
			Self::AlbumCountRev,
			Self::SongCount,
			Self::SongCountRev,
		].iter()
	}
}

impl AlbumSort {
	#[inline]
	/// Returns formatted, human readable versions.
	///
	/// e.g: [`AlbumSort::ReleaseArtistLexi`] returns [`ALBUM_RELEASE_ARTIST_LEXI`]
	pub const fn as_str(&self) -> &'static str {
		use AlbumSort::*;
		match self {
			ReleaseArtistLexi    => ALBUM_RELEASE_ARTIST_LEXI,
			ReleaseArtistLexiRev => ALBUM_RELEASE_ARTIST_LEXI_REV,
			LexiArtistLexi       => ALBUM_LEXI_ARTIST_LEXI,
			LexiArtistLexiRev    => ALBUM_LEXI_ARTIST_LEXI_REV,
			Lexi                 => ALBUM_LEXI,
			LexiRev              => ALBUM_LEXI_REV,
			Release              => ALBUM_RELEASE,
			ReleaseRev           => ALBUM_RELEASE_REV,
			Runtime              => ALBUM_RUNTIME,
			RuntimeRev           => ALBUM_RUNTIME_REV,
		}
	}

	#[inline]
	/// Returns an iterator over all [`AlbumSort`] variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		[
			Self::ReleaseArtistLexi,
			Self::ReleaseArtistLexiRev,
			Self::LexiArtistLexi,
			Self::LexiArtistLexiRev,
			Self::Lexi,
			Self::LexiRev,
			Self::Release,
			Self::ReleaseRev,
			Self::Runtime,
			Self::RuntimeRev,
		].iter()
	}
}

impl SongSort {
	#[inline]
	/// Returns formatted, human readable versions.
	///
	/// e.g: [`SongSort::AlbumReleaseArtistLexi`] returns [`SONG_ALBUM_RELEASE_ARTIST_LEXI`]
	pub const fn as_str(&self) -> &'static str {
		use SongSort::*;
		match self {
			AlbumReleaseArtistLexi    => SONG_ALBUM_RELEASE_ARTIST_LEXI,
			AlbumReleaseArtistLexiRev => SONG_ALBUM_RELEASE_ARTIST_LEXI_REV,
			AlbumLexiArtistLexi       => SONG_ALBUM_LEXI_ARTIST_LEXI,
			AlbumLexiArtistLexiRev    => SONG_ALBUM_LEXI_ARTIST_LEXI_REV,
			Lexi                      => SONG_LEXI,
			LexiRev                   => SONG_LEXI_REV,
			Release                   => SONG_RELEASE,
			ReleaseRev                => SONG_RELEASE_REV,
			Runtime                   => SONG_RUNTIME,
			RuntimeRev                => SONG_RUNTIME_REV,
		}
	}

	#[inline]
	/// Returns an iterator over all [`SongSort`] variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		[
			Self::AlbumReleaseArtistLexi,
			Self::AlbumReleaseArtistLexiRev,
			Self::AlbumLexiArtistLexi,
			Self::AlbumLexiArtistLexiRev,
			Self::Lexi,
			Self::LexiRev,
			Self::Release,
			Self::ReleaseRev,
			Self::Runtime,
			Self::RuntimeRev,
		].iter()
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
