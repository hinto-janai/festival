//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};

//---------------------------------------------------------------------------------------------------- Constants
/// [`Repeat::Song`]
const REPEAT_SONG:  &str = "Repeat a single song after it finishes";
/// [`Repeat::Queue`]
const REPEAT_QUEUE: &str = "Repeat the entire queue after it finishes";
/// [`Repeat::Off`]
const REPEAT_OFF:   &str = "Turn off all repeating";

//---------------------------------------------------------------------------------------------------- Repeat
/// HACK: until `std::mem::variant_count()` is stable.
pub const REPEAT_VARIANT_COUNT: usize = 3;
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
	pub const fn next(&self) -> Self {
		match self {
			Self::Song  => Self::Queue,
			Self::Queue => Self::Off,
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

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	// Asserts `.iter()` covers all variants.
	fn iter_covers_all() {
		assert_eq!(Repeat::iter().count(), REPEAT_VARIANT_COUNT);
	}

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
			assert!(set1.insert(i.as_str()));
			assert!(set2.insert(i.next()));
			assert!(set3.insert(i.previous()));
		}
	}
}
