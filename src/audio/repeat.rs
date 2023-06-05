//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use crate::kernel::Kernel;
use crate::collection::Song;

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
