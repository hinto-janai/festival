//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};

//----------------------------------------------------------------------------------------------------
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
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
