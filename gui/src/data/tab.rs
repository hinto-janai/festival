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

//---------------------------------------------------------------------------------------------------- Tab Constants
// This is the text actually displayed in the `GUI`.
pub const VIEW:      &str = "View";
pub const ALBUMS:    &str = "Albums";
pub const ARTISTS:   &str = "Artists";
pub const SONGS:     &str = "Songs";
pub const QUEUE:     &str = "Queue";
pub const PLAYLISTS: &str = "Playlists";
pub const SEARCH:    &str = "Search";
pub const SETTINGS:  &str = "Settings";

//---------------------------------------------------------------------------------------------------- Tab Enum
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[derive(AsRefStr,Display,EnumCount,EnumIter,EnumString,EnumVariantNames,IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Tab {
	/// The tab that represents a full-view of a
	/// particular `Album`, showing the full art,
	/// the track list, all other `Album`'s by the
	/// `Artist`, etc.
	///
	/// This data in this tab is mutated by the user
	/// clicking on `Album` buttons and "setting"
	/// (`Option::Some(AlbumKey)`) an `Album` to look at.
	View,

	#[default]
	Albums,
	Artists,
	Songs,
	Queue,
	Search,
	Settings,
}

impl Tab {
	/// No [`String`] allocation.
	pub fn human(&self) -> &'static str {
		match self {
			Self::View      => VIEW,
			Self::Albums    => ALBUMS,
			Self::Artists   => ARTISTS,
			Self::Songs     => SONGS,
			Self::Queue     => QUEUE,
			Self::Search    => SEARCH,
			Self::Settings  => SETTINGS,
		}
	}

	#[inline]
	/// Returns the next sequential [`Tab`] variant.
	///
	/// This returns the _first_ tab if at the _last_ tab.
	pub fn next(&self) -> Self {
		match self {
			Self::View      => Self::Albums,
			Self::Albums    => Self::Artists,
			Self::Artists   => Self::Songs,
			Self::Songs     => Self::Queue,
			Self::Queue     => Self::Search,
			Self::Search    => Self::Settings,
			Self::Settings  => Self::View,
		}
	}

	#[inline]
	/// Returns the previous sequential [`Tab`] variant.
	///
	/// This returns the _last_ tab if at the _first_ tab.
	pub fn previous(&self) -> Self {
		match self {
			Self::View      => Self::Settings,
			Self::Albums    => Self::View,
			Self::Artists   => Self::Albums,
			Self::Songs     => Self::Artists,
			Self::Queue     => Self::Songs,
			Self::Search    => Self::Queue,
			Self::Settings  => Self::Search,
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

		for i in Tab::iter() {
			assert!(set1.insert(i.human()));
			assert!(set2.insert(i.next()));
			assert!(set3.insert(i.previous()));
		}
	}
}
