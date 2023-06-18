//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};

//---------------------------------------------------------------------------------------------------- __NAME__
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
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
