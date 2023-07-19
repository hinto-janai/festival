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

//----------------------------------------------------------------------------------------------------
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[derive(AsRefStr,Display,EnumCount,EnumIter,EnumString,EnumVariantNames,IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
/// The sub-tabs in the `Artists` tab.
pub enum ArtistSubTab {
	#[default]
	/// Show all `Artists`.
	All,

	/// Show a specific `Artist` from our `State`'s `ArtistKey`.
	View,
}

impl ArtistSubTab {
	/// No [`String`] allocation.
	pub fn human(&self) -> &'static str {
		match self {
			Self::All  => "All",
			Self::View => "View",
		}
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

		for i in ArtistSubTab::iter() {
			assert!(set1.insert(i.human()));
			assert!(set2.insert(i.next()));
			assert!(set3.insert(i.previous()));
		}
	}
}
