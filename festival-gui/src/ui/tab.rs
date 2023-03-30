//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Tab Constants
// This is the text actually displayed in the `GUI`.
pub const ALBUMS:    &str = "Albums";
pub const ARTISTS:   &str = "Artists";
pub const SONGS:     &str = "Songs";
pub const QUEUE:     &str = "Queue";
pub const PLAYLISTS: &str = "Playlists";
pub const SEARCH:    &str = "Search";
pub const SETTINGS:  &str = "Settings";

//---------------------------------------------------------------------------------------------------- Tab Enum
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub enum Tab {
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
	pub fn as_str(&self) -> &'static str {
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
