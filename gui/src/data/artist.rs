//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};

//----------------------------------------------------------------------------------------------------
/// HACK: until `std::mem::variant_count()` is stable.
pub const ARTIST_SUB_TAB_VARIANT_COUNT: usize = 2;
/// The sub-tabs in the `Artists` tab.
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
pub enum ArtistSubTab {
	#[default]
	/// Show all `Artists`.
	All,

	/// Show a specific `Artist` from our `State`'s `ArtistKey`.
	View,
}

impl ArtistSubTab {
	/// No [`String`] allocation.
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::All  => "All",
			Self::View => "View",
		}
	}

	/// Returns an iterator over all the variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		[
			Self::All,
			Self::View,
		].iter()
	}

	/// Returns the next sequential [`ArtistSubTab`] variant.
	///
	/// This returns the _first_ if at the _last_.
	pub fn next(&self) -> Self {
		match self {
			Self::All  => Self::View,
			Self::View => Self::All,
		}
	}

	/// Returns the previous sequential [`ArtistSubTab`] variant.
	///
	/// This returns the _last_ if at the _first_.
	pub fn previous(&self) -> Self {
		match self {
			Self::All  => Self::View,
			Self::View => Self::All,
		}
	}
}

impl std::fmt::Display for ArtistSubTab {
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
		assert_eq!(ArtistSubTab::iter().count(), ARTIST_SUB_TAB_VARIANT_COUNT);
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

		for i in ArtistSubTab::iter() {
			assert!(set1.insert(i.as_str()));
			assert!(set2.insert(i.next()));
			assert!(set3.insert(i.previous()));
		}
	}
}
