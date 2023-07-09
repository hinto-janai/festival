//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};

//---------------------------------------------------------------------------------------------------- __NAME__
/// HACK: until `std::mem::variant_count()` is stable.
pub const SEARCH_SORT_VARIANT_COUNT: usize = 3;
/// The table in the `Search` tab can show results
/// as the `Song` title, `Album` title, or `Artist` name.
///
/// This selects which one it is.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
pub enum SearchSort {
	Song,
	#[default]
	Album,
	Artist,
}

impl SearchSort {
	/// No [`String`] allocation.
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Song   => "Song",
			Self::Album  => "Album",
			Self::Artist => "Artist",
		}
	}

	/// Returns an iterator over all the variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		[
			Self::Song,
			Self::Album,
			Self::Artist,
		].iter()
	}

	/// Returns the next sequential [`SearchSort`] variant.
	///
	/// This returns the _first_ if at the _last_.
	pub fn next(&self) -> Self {
		match self {
			Self::Song   => Self::Album,
			Self::Album  => Self::Artist,
			Self::Artist => Self::Song,
		}
	}

	/// Returns the previous sequential [`SongSort`] variant.
	///
	/// This returns the _last_ if at the _first_.
	pub fn previous(&self) -> Self {
		match self {
			Self::Song   => Self::Artist,
			Self::Album  => Self::Song,
			Self::Artist => Self::Album,
		}
	}
}

impl std::fmt::Display for SearchSort {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	// Asserts `.iter()` covers all variants.
	fn iter_covers_all() {
		assert_eq!(SearchSort::iter().count(), SEARCH_SORT_VARIANT_COUNT);
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

		for i in SearchSort::iter() {
			assert!(set1.insert(i.as_str()));
			assert!(set2.insert(i.next()));
			assert!(set3.insert(i.previous()));
		}
	}
}
