//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};

//---------------------------------------------------------------------------------------------------- Constants
/// [`Repeat::Song`]
const REPEAT_SONG:         &str = "Repeat a single song after it finishes";
/// [`Repeat::Queue`]
const REPEAT_QUEUE:        &str = "Repeat the entire queue after it finishes";
/// [`Repeat::Off`]
const REPEAT_OFF:          &str = "Turn off all repeating";

//---------------------------------------------------------------------------------------------------- Repeat
/// The different ways a "repeat" value
/// can be interpreted when playing audio.
#[derive(Copy,Clone,Debug,Hash,Eq,Ord,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
pub enum Repeat {
	/// When finishing a [`Song`] in the queue, repeat it, forever.
	Song,
	/// When finishing the queue, repeat it, forever.
	Queue,
	/// Turn off all repeating.
	Off,
}

impl Repeat {
	/// Returns the default, [`Self::Off`].
	pub const fn new() -> Self {
		Self::Off
	}

	#[inline]
	/// Returns formatted, human readable versions.
	pub const fn as_str(&self) -> &'static str {
		use Repeat::*;
		match self {
			Song        => REPEAT_SONG,
			Queue       => REPEAT_QUEUE,
			Off         => REPEAT_OFF,
		}
	}

	/// Returns the next sequential [`Self`] variant.
	///
	/// This returns the _first_ if at the _last_.
	pub fn next(&self) -> Self {
		match self {
			Self::Song  => Self::Queue,
			Self::Queue => Self::Off,
			Self::Off   => Self::Song,
		}
	}

	/// Returns the previous sequential [`Self`] variant.
	///
	/// This returns the _last_ if at the _first_.
	pub fn previous(&self) -> Self {
		match self {
			Self::Song  => Self::Off,
			Self::Queue => Self::Song,
			Self::Off   => Self::Queue,
		}
	}

	#[inline]
	/// Returns an iterator over all [`Self`] variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		[
			Self::Song,
			Self::Queue,
			Self::Off,
		].iter()
	}

}

impl Default for Repeat {
	fn default() -> Self {
		Self::new()
	}
}
