//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Sort Constants
/// [`SearchKind::All`]
pub const ALL:    &str = "View all the results, sorted from most similar to least";
/// [`SearchKind::Sim70`]
pub const SIM_70: &str = "View only the results that are at least 70% similar";
/// [`SearchKind::Top25`]
pub const TOP_25: &str = "View only the top 25 similar results";

//---------------------------------------------------------------------------------------------------- SearchKind
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
/// The different kinds of searches you can request from `Kernel`
pub enum SearchKind {
	/// String similarity, returns all calculated keys
	/// in order from most similar to least.
	All,
	#[default]
	/// [`Self::All`], but only returns the results that are at least 70% similar
	Sim70,
	/// [`Self::All`], but only returns the top 25 results
	Top25,
}

impl SearchKind {
	#[inline]
	/// Returns formatted, human readable versions.
	pub const fn as_str(&self) -> &'static str {
		match self {
			Self::Sim70 => SIM_70,
			Self::Top25 => TOP_25,
			Self::All   => ALL,
		}
	}

	#[inline]
	/// Returns an iterator over all [`Self`] variants.
	pub fn iter() -> std::slice::Iter<'static, Self> {
		[
			Self::All,
			Self::Sim70,
			Self::Top25,
		].iter()
	}

	/// Returns the next sequential [`Self`] variant.
	///
	/// This returns the _first_ if at the _last_.
	pub fn next(&self) -> Self {
		match self {
			Self::All   => Self::Sim70,
			Self::Sim70 => Self::Top25,
			Self::Top25 => Self::All,
		}
	}

	/// Returns the previous sequential [`Self`] variant.
	///
	/// This returns the _last_ if at the _first_.
	pub fn previous(&self) -> Self {
		match self {
			Self::All   => Self::Top25,
			Self::Sim70 => Self::All,
			Self::Top25 => Self::Sim70,
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
