//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use strum::{
	AsRefStr,
	Display,
	EnumCount,
	EnumIter,
	EnumString,
	EnumVariantNames,
	IntoStaticStr,
};

//---------------------------------------------------------------------------------------------------- Constants
/// [`Repeat::Song`]
const REPEAT_SONG: &str = "Repeat a single song after it finishes";
/// [`Repeat::Queue`]
const REPEAT_QUEUE: &str = "Repeat the entire queue after it finishes";
/// [`Repeat::QueuePause`]
const REPEAT_QUEUE_PAUSE: &str = "Repeat the entire queue after it finishes, but do not start immediately";
/// [`Repeat::Off`]
const REPEAT_OFF: &str = "Turn off all repeating";

//---------------------------------------------------------------------------------------------------- Repeat
#[derive(Copy,Clone,Debug,Hash,Eq,Ord,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
#[derive(AsRefStr,Display,EnumCount,EnumIter,EnumString,EnumVariantNames,IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
/// The different ways a "repeat" value
/// can be interpreted when playing audio.
pub enum Repeat {
	/// When finishing a [`Song`] in the queue, repeat it, forever.
	Song,
	/// When finishing the queue, repeat it, forever.
	Queue,
	/// When finishing the queue, repeat it, but paused.
	QueuePause,
	/// Turn off all repeating.
	Off,
}

impl Repeat {
	/// Returns the default, [`Self::QueuePause`].
	pub const fn new() -> Self {
		Self::QueuePause
	}

	#[inline]
	/// Returns formatted, human readable versions.
	pub const fn human(&self) -> &'static str {
		use Repeat::*;
		match self {
			Song        => REPEAT_SONG,
			Queue       => REPEAT_QUEUE,
			QueuePause  => REPEAT_QUEUE_PAUSE,
			Off         => REPEAT_OFF,
		}
	}

	/// Returns the next sequential [`Self`] variant.
	///
	/// This returns the _first_ if at the _last_.
	pub const fn next(&self) -> Self {
		match self {
			Self::Song  => Self::Queue,
			Self::Queue => Self::QueuePause,
			Self::QueuePause => Self::Off,
			Self::Off   => Self::Song,
		}
	}

	/// Returns the previous sequential [`Self`] variant.
	///
	/// This returns the _last_ if at the _first_.
	pub const fn previous(&self) -> Self {
		match self {
			Self::Song  => Self::Off,
			Self::Queue => Self::Song,
			Self::QueuePause => Self::Queue,
			Self::Off   => Self::QueuePause,
		}
	}
}

impl Default for Repeat {
	fn default() -> Self {
		Self::new()
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use strum::*;

	#[test]
	// Asserts each variant:
	// 1. Gives a different string
	// 2. `.next()` gives a different variant
	// 3. `.prev()` gives a different variant
	fn diff() {
		let mut set1 = std::collections::HashSet::new();
		let mut set2 = std::collections::HashSet::new();
		let mut set3 = std::collections::HashSet::new();

		for i in Repeat::iter() {
			assert!(set1.insert(i.human()));
			assert!(set2.insert(i.next()));
			assert!(set3.insert(i.previous()));
		}
	}
}
