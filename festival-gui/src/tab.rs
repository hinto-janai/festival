//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Tab Constants
// This is the text actually displayed in the `GUI`.
pub(super) const ALBUMS:    &str = "Albums";
pub(super) const ARTISTS:   &str = "Artists";
pub(super) const SONGS:     &str = "Songs";
pub(super) const QUEUE:     &str = "Queue";
pub(super) const PLAYLISTS: &str = "Playlists";
pub(super) const SEARCH:    &str = "Search";
pub(super) const SETTINGS:  &str = "Settings";

//---------------------------------------------------------------------------------------------------- Tab Enum
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub(super) enum Tab {
	#[default]
	Albums,
	Artists,
	Songs,
	Queue,
	Playlists,
	Search,
	Settings,
}

impl Tab {
	/// No [`String`] allocation.
	pub(super) fn as_str(&self) -> &'static str {
		match self {
			Self::Albums    => ALBUMS,
			Self::Artists   => ARTISTS,
			Self::Songs     => SONGS,
			Self::Queue     => QUEUE,
			Self::Playlists => PLAYLISTS,
			Self::Search    => SEARCH,
			Self::Settings  => SETTINGS,
		}
	}
}

impl std::fmt::Display for Tab {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
