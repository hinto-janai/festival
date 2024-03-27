//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{FESTIVAL, FRONTEND_SUB_DIR, SIGNAL_SUB_DIR};
use anyhow::anyhow;
use const_format::formatcp;
use serde::{Deserialize, Serialize};

//---------------------------------------------------------------------------------------------------- Signals
macro_rules! impl_signal_empty {
	($($type:ident, $file_name:literal,)*) => {
		$(
			disk::empty!($type, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/{SIGNAL_SUB_DIR}"), $file_name);
			#[derive(Copy,Clone,Debug,PartialEq,Eq)]
			/// File representing a signal, whose existence acts as a boolean signal.
			pub struct $type;
		)*
	};
}

macro_rules! impl_signal_content {
	($($type:ident, $inner:ty, $file_name:literal, $doc:literal,)*) => {
		$(
			disk::plain!($type, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/{SIGNAL_SUB_DIR}"), $file_name);
			#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
			/// File representing a signal, which has contents inside.
			///
			#[doc = $doc ]
			pub struct $type(pub $inner);

			impl $type {
				/// HACK:
				/// `serde_plain` & `std::parse` don't parse `\n` automatically so
				/// - `printf "123" > file` works but
				/// - `echo "123" > file` doesn't
				///
				/// For now, use this function over `disk`'s version.
				///
				/// It manually parses using `.lines()`.
				pub fn from_file() -> Result<Self, anyhow::Error> {
					let s = std::fs::read(Self::absolute_path()?)?;

					let s = std::str::from_utf8(&s)?;
					use disk::Plain;

					match s.lines().next() {
						Some(s) => Self::from_bytes(s.as_bytes()),
						None => Err(anyhow!("`None` on `.lines()`")),
					}
				}
			}
		)*
	};
}

impl_signal_empty! {
    Toggle,        "toggle",
    Pause,         "pause",
    Play,          "play",
    Next,          "next",
    Stop,          "stop",
    Previous,      "previous",
    Shuffle,       "shuffle",
    RepeatSong,    "repeat_song",
    RepeatQueue,   "repeat_queue",
    RepeatOff,     "repeat_off",
}

impl_signal_content! {
    Volume, crate::audio::Volume, "volume",
    "Contents should be a [`Volume`]. Sets the volume to the given value.",

    Seek, u64, "seek",
    "Contents should be a [`u64`] representing the absolute second to seek to in the current [`Song`]. Will skip if the value if larger than the current [`Song`]'s runtime.",

    SeekForward, u64, "seek_forward",
    "Contents should be a [`u64`] representing how many seconds to seek forwards in the current [`Song`]. Will skip if the value if larger than the current [`Song`]'s runtime.",

    SeekBackward, u64, "seek_backward",
    "Contents should be a [`u64`] representing how many seconds to seek backwards in the current [`Song`]. Will reset the song if under-bounds",

    Index, usize, "index",
    "Contents should be a [`usize`]. This skips to an index in the queue starting from 1. Will end the queue if the index is out of bounds.",

    Clear, bool, "clear",
    "Contents should be a [`bool`], either `true` or `false`. This signal clears the queue. The [`bool`] represents if we should continue playback.",

    Skip, usize, "skip",
    "Contents should be a [`usize`]. This is similar to `Next`, although you can specify any amount of [`Song`]'s to skip in the queue.",

    Back, usize, "back",
    "Contents should be a [`usize`]. This is the same as `Skip`, although it skips backwards.",

    // SOMEDAY
//	ArtistKey, usize, "artist_key",
//	"Contents should be a [`usize`] representing an [`ArtistKey`]. This will add the [`Artist`]'s songs to the queue",
//
//	AlbumKey, usize, "album_key",
//	"Contents should be a [`usize`] representing an [`AlbumKey`]. This will add the [`Album`] to the queue",
//
//	SongKey, usize, "song_key",
//	"Contents should be a [`usize`] representing a [`SongKey`]. This will add the [`Song`] to the queue",
//
//	Artist, usize, "artist",
//	"Contents should be a [`String`] representing an [`Artist`] name. This will add the [`Artist`]'s songs to the queue",
//
//	Album, usize, "album",
//	"Contents should be a [`String`] representing an [`Album`] title. This will add the [`Album`] to the queue",
//
//	Song, usize, "song",
//	"Contents should be a [`String`] representing a [`Song`] title. This will add the [`Song`] to the queue",
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
