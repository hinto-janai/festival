//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use crate::kernel::Kernel;

//---------------------------------------------------------------------------------------------------- Constants
/// [`Shuffle::On`]
const SHUFFLE_ON:     &str = "Shuffle new songs added to the queue";
/// [`Shuffle::Off`]
const SHUFFLE_OFF:    &str = "Turn off shuffle";
/// [`Shuffle::Toggle`]
const SHUFFLE_TOGGLE: &str = "Toggle shuffle";

//---------------------------------------------------------------------------------------------------- Repeat
/// The different ways a "shuffle" value
/// can be interpreted when playing audio.
#[derive(Copy,Clone,Debug,Hash,Eq,Ord,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
pub enum Shuffle {
	/// Turn on `Shuffle`: Randomize the songs added to the queue.
	On,
	/// Turn off `Shuffle`.
	Off,
	/// Toggle `Shuffle`.
	Toggle,
}

impl Shuffle {
	#[inline]
	/// Returns formatted, human readable versions.
	pub const fn as_str(&self) -> &'static str {
		use Shuffle::*;
		match self {
			On     => SHUFFLE_ON,
			Off    => SHUFFLE_OFF,
			Toggle => SHUFFLE_TOGGLE,
		}
	}

	#[inline]
	/// Returns an iterator over all [`Self`] variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		[
			Self::On,
			Self::Off,
			Self::Toggle,
		].iter()
	}

}
