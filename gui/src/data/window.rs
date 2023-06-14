//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use shukusai::constants::FESTIVAL;

//---------------------------------------------------------------------------------------------------- Constants
const ARTIST_ALBUM_SONG:    &str = "Artist name | Album title | Song title";
const ARTIST_ALBUM_SONG_RT: &str = "Artist name | Album title | Song title | Song runtime";
const ALBUM_SONG:           &str = "Album title | Song title";
const ALBUM_SONG_RT:        &str = "Album title | Song title | Song runtime";
const SONG:                 &str = "Song title";
const SONG_RT:              &str = "Song title | Song runtime";
const RT_SONG:              &str = "Song runtime | Song title";
const SONG_ALBUM:           &str = "Song title | Album title";
const RT_SONG_ALBUM:        &str = "Song runtime | Song title | Album title";
const SONG_ALBUM_ARTIST:    &str = "Song title | Album title | Artist name";
const RT_SONG_ALBUM_ARTIST: &str = "Song runtime | Song title | Album title | Artist name";
const QUEUE:                &str = "[Queue index/Queue length]";
const OFF:                  &str = "Off";

// The separator between the values.
const SEP: &str = "   |   ";

//----------------------------------------------------------------------------------------------------
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
/// Different ways the outer `Festival` window title can be set.
pub enum WindowTitle {
	ArtistAlbumSong,
	ArtistAlbumSongRuntime,
	AlbumSong,
	AlbumSongRuntime,
	Song,
	SongRuntime,
	RuntimeSong,
	SongAlbum,
	RuntimeSongAlbum,
	#[default]
	SongAlbumArtist,
	RuntimeSongAlbumArtist,
	Queue,
	Off,
}

impl WindowTitle {
	/// Formats and returns a [`String`] according to [`Self`].
	pub fn format(
		&self,
		queue_idx: usize,
		queue_len: usize,
		runtime:   &str,
		artist:    &str,
		album:     &str,
		song:      &str,
	) -> String {
		use WindowTitle::*;
		match self {
			ArtistAlbumSong        => format!("{FESTIVAL}{SEP}{artist}{SEP}{album}{SEP}{song}"),
			ArtistAlbumSongRuntime => format!("{FESTIVAL}{SEP}{artist}{SEP}{album}{SEP}{song}{SEP}{runtime}"),
			AlbumSong              => format!("{FESTIVAL}{SEP}{album}{SEP}{song}"),
			AlbumSongRuntime       => format!("{FESTIVAL}{SEP}{album}{SEP}{song}{SEP}{runtime}"),
			Song                   => format!("{FESTIVAL}{SEP}{song}"),
			SongRuntime            => format!("{FESTIVAL}{SEP}{song}{SEP}{runtime}"),
			RuntimeSong            => format!("{FESTIVAL}{SEP}{runtime}{SEP}{song}"),
			SongAlbum              => format!("{FESTIVAL}{SEP}{song}{SEP}{album}"),
			RuntimeSongAlbum       => format!("{FESTIVAL}{SEP}{runtime}{SEP}{song}{SEP}{album}"),
			SongAlbumArtist        => format!("{FESTIVAL}{SEP}{song}{SEP}{album}{SEP}{artist}"),
			RuntimeSongAlbumArtist => format!("{FESTIVAL}{SEP}{runtime}{SEP}{song}{SEP}{album}{SEP}{artist}"),
			Queue                  => format!("{FESTIVAL}{SEP}[{queue_idx}/{queue_len}]"),
			Off                    => format!("{FESTIVAL}"),
		}
	}

	/// No [`String`] allocation.
	pub fn as_str(&self) -> &'static str {
		use WindowTitle::*;
		match self {
			ArtistAlbumSong        => ARTIST_ALBUM_SONG,
			ArtistAlbumSongRuntime => ARTIST_ALBUM_SONG_RT,
			AlbumSong              => ALBUM_SONG,
			AlbumSongRuntime       => ALBUM_SONG_RT,
			Song                   => SONG,
			SongRuntime            => SONG_RT,
			RuntimeSong            => RT_SONG,
			SongAlbum              => SONG_ALBUM,
			RuntimeSongAlbum       => RT_SONG_ALBUM,
			SongAlbumArtist        => SONG_ALBUM_ARTIST,
			RuntimeSongAlbumArtist => RT_SONG_ALBUM_ARTIST,
			Queue                  => QUEUE,
			Off                    => OFF,
		}
	}

	#[inline]
	/// Returns an iterator over all [`Self`] variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		use WindowTitle::*;
		[
			ArtistAlbumSong,
			ArtistAlbumSongRuntime,
			AlbumSong,
			AlbumSongRuntime,
			Song,
			SongRuntime,
			RuntimeSong,
			SongAlbum,
			RuntimeSongAlbum,
			SongAlbumArtist,
			RuntimeSongAlbumArtist,
			Queue,
			Off,
		].iter()
	}

	#[inline]
	/// Returns the next sequential [`Self`] variant.
	///
	/// This returns the _first_ tab if at the _last_ tab.
	pub fn next(&self) -> Self {
		use WindowTitle::*;
		match self {
			ArtistAlbumSong        => ArtistAlbumSongRuntime,
			ArtistAlbumSongRuntime => AlbumSong,
			AlbumSong              => AlbumSongRuntime,
			AlbumSongRuntime       => Song,
			Song                   => SongRuntime,
			SongRuntime            => RuntimeSong,
			RuntimeSong            => SongAlbum,
			SongAlbum              => RuntimeSongAlbum,
			RuntimeSongAlbum       => SongAlbumArtist,
			SongAlbumArtist        => RuntimeSongAlbumArtist,
			RuntimeSongAlbumArtist => Queue,
			Queue                  => Off,
			Off                    => ArtistAlbumSong,
		}
	}

	#[inline]
	/// Returns the previous sequential [`Self`] variant.
	///
	/// This returns the _last_ tab if at the _first_ tab.
	pub fn previous(&self) -> Self {
		use WindowTitle::*;
		match self {
			ArtistAlbumSong        => Off,
			ArtistAlbumSongRuntime => ArtistAlbumSong,
			AlbumSong              => ArtistAlbumSongRuntime,
			AlbumSongRuntime       => AlbumSong,
			Song                   => AlbumSongRuntime,
			SongRuntime            => Song,
			RuntimeSong            => SongRuntime,
			SongAlbum              => RuntimeSong,
			RuntimeSongAlbum       => SongAlbum,
			SongAlbumArtist        => RuntimeSongAlbum,
			RuntimeSongAlbumArtist => SongAlbumArtist,
			Queue                  => RuntimeSongAlbumArtist,
			Off                    => Queue,
		}
	}
}

impl std::fmt::Display for WindowTitle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
