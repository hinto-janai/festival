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
pub const ARTIST_LEXI:                    &str = "Artists A-Z";
/// [`ArtistSort::LexiRev`]
pub const ARTIST_LEXI_REV:                &str = "Artists Z-A";
/// [`ArtistSort::AlbumCount`]
pub const ARTIST_ALBUM_COUNT:             &str = "Artists per album count (least to most)";
/// [`ArtistSort::AlbumCountRev`]
pub const ARTIST_ALBUM_COUNT_REV:         &str = "Artists per album count (most to least)";
/// [`ArtistSort::SongCount`]
pub const ARTIST_SONG_COUNT:              &str = "Artists per song count (least to most)";
/// [`ArtistSort::SongCountRev`]
pub const ARTIST_SONG_COUNT_REV:          &str = "Artists per song count (most to least)";
/// [`ArtistSort::Runtime`]
pub const ARTIST_RUNTIME:                 &str = "Artists runtime shortest-longest";
/// [`ArtistSort::RuntimeRev`]
pub const ARTIST_RUNTIME_REV:             &str = "Artists runtime longest-shortest";
/// [`ArtistSort::Name`]
pub const ARTIST_NAME:                    &str = "Artist name shortest-longest";
/// [`ArtistSort::NameRev`]
pub const ARTIST_NAME_REV:                &str = "Artist name longest-shortest";

/// [`AlbumSort::ReleaseArtistLexi`]
pub const ALBUM_RELEASE_ARTIST_LEXI:          &str = "Artists A-Z, albums oldest-latest";
/// [`AlbumSort::ReleaseArtistLexiRev`]
pub const ALBUM_RELEASE_ARTIST_LEXI_REV:      &str = "Artists Z-A, albums oldest-latest";
/// [`AlbumSort::ReleaseRevArtistLexi`]
pub const ALBUM_RELEASE_REV_ARTIST_LEXI:      &str = "Artists A-Z, albums latest-oldest";
/// [`AlbumSort::ReleaseRevArtistLexiRev`]
pub const ALBUM_RELEASE_REV_ARTIST_LEXI_REV:  &str = "Artists Z-A, albums latest-oldest";
//---
/// [`AlbumSort::LexiArtistLexi`]
pub const ALBUM_LEXI_ARTIST_LEXI:         &str = "Artists A-Z, albums A-Z";
/// [`AlbumSort::LexiArtistLexiRev`]
pub const ALBUM_LEXI_ARTIST_LEXI_REV:     &str = "Artists Z-A, albums A-Z";
/// [`AlbumSort::LexiRevArtistLexi`]
pub const ALBUM_LEXI_REV_ARTIST_LEXI:         &str = "Artists A-Z, albums Z-A";
/// [`AlbumSort::LexiRevArtistLexiRev`]
pub const ALBUM_LEXI_REV_ARTIST_LEXI_REV:     &str = "Artists Z-A, albums Z-A";
//---
/// [`AlbumSort::Lexi`]
pub const ALBUM_LEXI:                     &str = "Albums A-Z";
/// [`AlbumSort::LexiRev`]
pub const ALBUM_LEXI_REV:                 &str = "Albums Z-A";
/// [`AlbumSort::Release`]
pub const ALBUM_RELEASE:                  &str = "Albums oldest-latest";
/// [`AlbumSort::ReleaseRev`]
pub const ALBUM_RELEASE_REV:              &str = "Albums latest-oldest";
/// [`AlbumSort::Runtime`]
pub const ALBUM_RUNTIME:                  &str = "Albums shortest-longest";
/// [`AlbumSort::RuntimeRev`]
pub const ALBUM_RUNTIME_REV:              &str = "Albums longest-shortest";
/// [`AlbumSort::Title`]
pub const ALBUM_TITLE:                    &str = "Album title shortest-longest";
/// [`AlbumSort::TitleRev`]
pub const ALBUM_TITLE_REV:                &str = "Album title longest-shortest";

/// [`SongSort::AlbumReleaseArtistLexi`]
pub const SONG_ALBUM_RELEASE_ARTIST_LEXI:         &str = "Artists A-Z, albums oldest-latest, songs in track order";
/// [`SongSort::AlbumReleaseArtistLexiRev`]
pub const SONG_ALBUM_RELEASE_ARTIST_LEXI_REV:     &str = "Artists Z-A, albums oldest-latest, songs in track order";
/// [`SongSort::AlbumReleaseRevArtistLexi`]
pub const SONG_ALBUM_RELEASE_REV_ARTIST_LEXI:     &str = "Artists A-Z, albums latest-oldest, songs in track order";
/// [`SongSort::AlbumReleaseRevArtistLexiRev`]
pub const SONG_ALBUM_RELEASE_REV_ARTIST_LEXI_REV: &str = "Artists Z-A, albums latest-oldest, songs in track order";

/// [`SongSort::AlbumLexiArtistLexi`]
pub const SONG_ALBUM_LEXI_ARTIST_LEXI:            &str = "Artists A-Z, albums A-Z, songs in track order";
/// [`SongSort::AlbumLexiArtistLexiRev`]
pub const SONG_ALBUM_LEXI_ARTIST_LEXI_REV:        &str = "Artists Z-A, albums A-Z, songs in track order";
/// [`SongSort::AlbumLexiRevArtistLexi`]
pub const SONG_ALBUM_LEXI_REV_ARTIST_LEXI:        &str = "Artists A-Z, albums Z-A, songs in track order";
/// [`SongSort::AlbumLexiArtistLexiRev`]
pub const SONG_ALBUM_LEXI_REV_ARTIST_LEXI_REV:    &str = "Artists Z-A, albums Z-A, songs in track order";

/// [`SongSort::Lexi`]
pub const SONG_LEXI:                          &str = "Songs A-Z";
/// [`SongSort::Lexi`]
pub const SONG_LEXI_REV:                      &str = "Songs Z-A";
/// [`SongSort::Release`]
pub const SONG_RELEASE:                       &str = "Songs oldest-latest";
/// [`SongSort::ReleaseRev`]
pub const SONG_RELEASE_REV:                   &str = "Songs latest-oldest";
/// [`SongSort::Runtime`]
pub const SONG_RUNTIME:                       &str = "Songs shortest-longest";
/// [`SongSort::RuntimeRev`]
pub const SONG_RUNTIME_REV:                   &str = "Songs longest-oldest";
/// [`SongSort::Title`]
pub const SONG_TITLE:                         &str = "Song title shortest-longest";
/// [`SongSort::TitleRev`]
pub const SONG_TITLE_REV:                     &str = "Song title longest-shortest";

//---------------------------------------------------------------------------------------------------- Sort
/// All the ways to sort the [`Collection`]'s [`Artist`]'s.
///
/// String sorting is done lexicographically as per the `std` [`Ord` implementation.](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord)
///
/// `lexi` is shorthand for `lexicographically`.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
pub enum ArtistSort {
	/// [`Artist`] A-Z. Field: [`Collection::sort_artist_lexi`].
	Lexi,
	/// [`Artist`] Z-A. Field: [`Collection::sort_artist_lexi_rev`].
	LexiRev,
	/// [`Artist`] with most `Album`'s to least. Field: [`Collection::sort_artist_album_count`].
	AlbumCount,
	#[default]
	/// [`Artist`] with least `Album`'s to most. Field: [`Collection::sort_artist_album_count_rev`].
	AlbumCountRev,
	/// [`Artist`] with most `Song`'s to least. Field: [`Collection::sort_artist_song_count`].
	SongCount,
	/// [`Artist`] with least `Song`'s to most. Field: [`Collection::sort_artist_song_count_rev`].
	SongCountRev,
	/// [`Artist`] with least runtime. Field: [`Collection::sort_artist_runtime`].
	Runtime,
	/// [`Artist`] with most runtime. Field: [`Collection::sort_artist_runtime_rev`].
	RuntimeRev,
	/// [`Artist`] name shortest-longest. Field: [`Collection::sort_artist_name`].
	Name,
	/// [`Artist`] name longest-shortest. Field: [`Collection::sort_artist_name_rev`].
	NameRev,
}

/// All the ways to sort the [`Collection`]'s [`Album`]'s.
///
/// String sorting is done lexicographically as per the `std` [`Ord` implementation.](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord)
///
/// `lexi` is shorthand for `lexicographically`.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
pub enum AlbumSort {
	#[default]
	/// [`Artist`] A-Z, [`Album`] oldest-latest. Field: [`Collection::sort_album_release_artist_lexi`].
	ReleaseArtistLexi,
	/// [`Artist`] Z-A, [`Album`] oldest-latest. Field: [`Collection::sort_album_release_artist_lexi_rev`].
	ReleaseArtistLexiRev,
	/// [`Artist`] A-Z, [`Album`] latest-oldest. Field: [`Collection::sort_album_release_rev_artist_lexi`].
	ReleaseRevArtistLexi,
	/// [`Artist`] Z-A, [`Album`] latest-oldest. Field: [`Collection::sort_album_release_rev_artist_lexi_rev`].
	ReleaseRevArtistLexiRev,
	/// [`Artist`] A-Z, [`Album`]'s A-Z. Field: [`Collection::sort_album_lexi_artist_lexi`].
	LexiArtistLexi,
	/// [`Artist`] Z-A, [`Album`]'s A-Z. Field: [`Collection::sort_album_lexi_artist_lexi_rev`].
	LexiArtistLexiRev,
	/// [`Artist`] A-Z, [`Album`]'s Z-A. Field: [`Collection::sort_album_lexi_rev_artist_lexi`].
	LexiRevArtistLexi,
	/// [`Artist`] Z-A, [`Album`]'s Z-A. Field: [`Collection::sort_album_lexi_rev_artist_lexi_rev`].
	LexiRevArtistLexiRev,
	/// [`Album`] A-Z. Field: [`Collection::sort_album_lexi`].
	Lexi,
	/// [`Album`] Z-A. Field: [`Collection::sort_album_lexi_rev`].
	LexiRev,
	/// [`Album`] oldest-latest. Field: [`Collection::sort_album_release`].
	Release,
	/// [`Album`] latest-oldest. Field: [`Collection::sort_album_release_rev`].
	ReleaseRev,
	/// [`Album`] shortest-longest. Field: [`Collection::sort_album_runtime`].
	Runtime,
	/// [`Album`] longest-shortest. Field: [`Collection::sort_album_runtime_rev`].
	RuntimeRev,
	/// [`Album`] shortest title, longest title. Field: [`Collection::sort_album_title`].
	Title,
	/// [`Album`] longest title, shortest title. Field: [`Collection::sort_album_title_rev`].
	TitleRev,
}

/// All the ways to sort the [`Collection`]'s [`Song`]'s.
///
/// String sorting is done lexicographically as per the `std` [`Ord` implementation.](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord)
///
/// `lexi` is shorthand for `lexicographically`.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
pub enum SongSort {
	/// [`Artist`] A-Z, [`Album`] oldest-latest, [`Song`] track_number. Field: [`Collection::sort_song_album_release_artist_lexi`].
	AlbumReleaseArtistLexi,
	/// [`Artist`] Z-A, [`Album`] oldest-latest, [`Song`] track_number. Field: [`Collection::sort_song_album_release_artist_lexi_rev`].
	AlbumReleaseArtistLexiRev,
	/// [`Artist`] A-Z, [`Album`] latest-oldest, [`Song`] track_number. Field: [`Collection::sort_song_album_release_rev_artist_lexi`].
	AlbumReleaseRevArtistLexi,
	/// [`Artist`] Z-A, [`Album`] oldest-latest, [`Song`] track_number. Field: [`Collection::sort_song_album_release_rev_artist_lexi_rev`].
	AlbumReleaseRevArtistLexiRev,
	/// [`Artist`] A-Z, [`Album`] A-Z, [`Song`] track_number. Field: [`Collection::sort_song_album_lexi_artist_lexi`].
	AlbumLexiArtistLexi,
	/// [`Artist`] Z-A, [`Album`] A-Z, [`Song`] track_number. Field: [`Collection::sort_song_album_lexi_artist_lexi_rev`].
	AlbumLexiArtistLexiRev,
	/// [`Artist`] A-Z, [`Album`] Z-A, [`Song`] track_number. Field: [`Collection::sort_song_album_lexi_rev_artist_lexi`].
	AlbumLexiRevArtistLexi,
	/// [`Artist`] Z-A, [`Album`] Z-A, [`Song`] track_number. Field: [`Collection::sort_song_album_lexi_rev_artist_lexi_rev`].
	AlbumLexiRevArtistLexiRev,
	/// [`Song`] A-Z. Field: [`Collection::sort_song_lexi`].
	#[default]
	Lexi,
	/// [`Song`] Z-A. Field: [`Collection::sort_song_lexi_rev`].
	LexiRev,
	/// [`Song`] oldest-latest. Field: [`Collection::sort_song_release`].
	Release,
	/// [`Song`] latest-oldest. Field: [`Collection::sort_song_release`].
	ReleaseRev,
	/// [`Song`] shortest-longest. Field: [`Collection::sort_song_runtime`].
	Runtime,
	/// [`Song`] longest-shortest. Field: [`Collection::sort_song_runtime`].
	RuntimeRev,
	/// [`Song`] shortest title, longest title. Field: [`Collection::sort_song_title`].
	Title,
	/// [`Song`] longest title, shortest title. Field: [`Collection::sort_song_title_rev`].
	TitleRev,
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
			Runtime       => ARTIST_RUNTIME,
			RuntimeRev    => ARTIST_RUNTIME_REV,
			Name          => ARTIST_NAME,
			NameRev       => ARTIST_NAME_REV,
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
			Self::Runtime,
			Self::RuntimeRev,
			Self::Name,
			Self::NameRev,
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
			ReleaseArtistLexi       => ALBUM_RELEASE_ARTIST_LEXI,
			ReleaseArtistLexiRev    => ALBUM_RELEASE_ARTIST_LEXI_REV,
			ReleaseRevArtistLexi    => ALBUM_RELEASE_REV_ARTIST_LEXI,
			ReleaseRevArtistLexiRev => ALBUM_RELEASE_REV_ARTIST_LEXI_REV,
			LexiArtistLexi          => ALBUM_LEXI_ARTIST_LEXI,
			LexiArtistLexiRev       => ALBUM_LEXI_ARTIST_LEXI_REV,
			LexiRevArtistLexi       => ALBUM_LEXI_REV_ARTIST_LEXI,
			LexiRevArtistLexiRev    => ALBUM_LEXI_REV_ARTIST_LEXI_REV,
			Lexi                    => ALBUM_LEXI,
			LexiRev                 => ALBUM_LEXI_REV,
			Release                 => ALBUM_RELEASE,
			ReleaseRev              => ALBUM_RELEASE_REV,
			Runtime                 => ALBUM_RUNTIME,
			RuntimeRev              => ALBUM_RUNTIME_REV,
			Title                   => ALBUM_TITLE,
			TitleRev                => ALBUM_TITLE_REV,
		}
	}

	#[inline]
	/// Returns an iterator over all [`AlbumSort`] variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		[
			Self::ReleaseArtistLexi,
			Self::ReleaseArtistLexiRev,
			Self::ReleaseRevArtistLexi,
			Self::ReleaseRevArtistLexiRev,
			Self::LexiArtistLexi,
			Self::LexiArtistLexiRev,
			Self::LexiRevArtistLexi,
			Self::LexiRevArtistLexiRev,
			Self::Lexi,
			Self::LexiRev,
			Self::Release,
			Self::ReleaseRev,
			Self::Runtime,
			Self::RuntimeRev,
			Self::Title,
			Self::TitleRev,
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
			AlbumReleaseArtistLexi       => SONG_ALBUM_RELEASE_ARTIST_LEXI,
			AlbumReleaseArtistLexiRev    => SONG_ALBUM_RELEASE_ARTIST_LEXI_REV,
			AlbumReleaseRevArtistLexi    => SONG_ALBUM_RELEASE_REV_ARTIST_LEXI,
			AlbumReleaseRevArtistLexiRev => SONG_ALBUM_RELEASE_REV_ARTIST_LEXI_REV,
			AlbumLexiArtistLexi          => SONG_ALBUM_LEXI_ARTIST_LEXI,
			AlbumLexiArtistLexiRev       => SONG_ALBUM_LEXI_ARTIST_LEXI_REV,
			AlbumLexiRevArtistLexi       => SONG_ALBUM_LEXI_REV_ARTIST_LEXI,
			AlbumLexiRevArtistLexiRev    => SONG_ALBUM_LEXI_REV_ARTIST_LEXI_REV,
			Lexi                         => SONG_LEXI,
			LexiRev                      => SONG_LEXI_REV,
			Release                      => SONG_RELEASE,
			ReleaseRev                   => SONG_RELEASE_REV,
			Runtime                      => SONG_RUNTIME,
			RuntimeRev                   => SONG_RUNTIME_REV,
			Title                        => SONG_TITLE,
			TitleRev                     => SONG_TITLE_REV,
		}
	}

	#[inline]
	/// Returns an iterator over all [`SongSort`] variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		[
			Self::AlbumReleaseArtistLexi,
			Self::AlbumReleaseArtistLexiRev,
			Self::AlbumReleaseRevArtistLexi,
			Self::AlbumReleaseRevArtistLexiRev,
			Self::AlbumLexiArtistLexi,
			Self::AlbumLexiArtistLexiRev,
			Self::AlbumLexiRevArtistLexi,
			Self::AlbumLexiRevArtistLexiRev,
			Self::Lexi,
			Self::LexiRev,
			Self::Release,
			Self::ReleaseRev,
			Self::Runtime,
			Self::RuntimeRev,
			Self::Title,
			Self::TitleRev,
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
