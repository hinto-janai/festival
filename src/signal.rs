//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::kernel::Kernel;
use crate::FESTIVAL;
use serde::{Serialize,Deserialize};
use std::ops::RangeInclusive;
use crate::collection::Song;

//---------------------------------------------------------------------------------------------------- Signals
macro_rules! impl_signal_empty {
	($($type:ident, $file_name:literal,)*) => {
		$(
			disk::empty!($type, disk::Dir::Data, FESTIVAL, "signal", $file_name);
			#[derive(Copy,Clone,Debug,PartialEq,Eq)]
			/// File representing a signal, whose existance acts as a boolean signal.
			pub struct $type;
		)*
	};
}

macro_rules! impl_signal_content {
	($($type:ident, $inner:ty, $file_name:literal, $doc:literal,)*) => {
		$(
			disk::plain!($type, disk::Dir::Data, FESTIVAL, "signal", $file_name);
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
	Previous,      "previous",
	ShuffleOn,     "shuffle_on",
	ShuffleOff,    "shuffle_off",
	ShuffleToggle, "shuffle_toggle",
	RepeatSong,    "repeat_song",
	RepeatQueue,   "repeat_queue",
	RepeatOff,     "repeat_off",
}

impl_signal_content! {
	Volume, crate::audio::Volume, "volume",
	"Contents should be a [`Volume`].",

	Seek, usize, "seek",
	"Contents should be a [`usize`] representing which second to seek to in the current [`Song`]. Will skip if the value if larger than the current [`Song`]'s runtime.",

	Skip, usize, "skip",
	"Contents should be a [`usize`]. This is similar to `Next`, although you can specify any amount of [`Song`]'s to skip in the queue.",

	Back, usize, "back",
	"Contents should be a [`back`]. This is the same as `Skip`, although it skips backwards.",
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
