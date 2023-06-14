//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use crate::{
	state::ResetState,
	collection::Collection,
};

//---------------------------------------------------------------------------------------------------- Tab Constants
// This is the text actually displayed in the `GUI`.
const NONE:     &str = "...";
const DISK:     &str = "Reading From Disk";
const START:    &str = "Starting...";
const WALKDIR:  &str = "Walking Directories";
const PARSE:    &str = "Parsing Metadata";
const FIX:      &str = "Fixing Metadata";
const SORT:     &str = "Sorting";
const SEARCH:   &str = "Creating Search Engine";
const PREPARE:  &str = "Preparing Collection";
const ART:      &str = "Preparing Album Art";
const CLONE:    &str = "Preparing Collection For Disk";
const CONVERT:  &str = "Converting Album Art";
const FINALIZE: &str = "Finalizing Collection";

//---------------------------------------------------------------------------------------------------- Phase
#[derive(Copy,Clone,Debug,Hash,Serialize,Deserialize,PartialEq,Eq,PartialOrd,Ord)]
/// The different phases of creating a new [`Collection`]
///
/// [`ResetState::phase`] will hold a [`Phase`] representing
/// exactly what step we're on when creating a new [`Collection`].
///
/// These enum variants align with the steps sequentially, aka,
/// [`Phase::Start`] is the 1st step and [`Phase::Finalize`] is the last.
///
/// ## Exceptions
/// [`Phase::None`] represents that we _aren't_ currently resetting the [`Collection`].
/// This is set before we ever reset a [`Collection`] and after we're done resetting one.
///
/// [`Phase::Disk`] represents we're not _resetting_, but in a startup process.
/// This is set before `Kernel` reads the [`Collection`] from disk.
///
/// Use [`Phase::as_str()`] to get a more `Frontend` friendly message related to the [`Phase`]:
/// ```rust
/// # use shukusai::kernel::Phase;
/// assert!(Phase::None.as_str()     == "...");
/// assert!(Phase::Disk.as_str()     == "Reading From Disk");
///
/// assert!(Phase::Start.as_str()    == "Starting...");
/// assert!(Phase::WalkDir.as_str()  == "Walking Directories");
/// assert!(Phase::Parse.as_str()    == "Parsing Metadata");
/// assert!(Phase::Fix.as_str()      == "Fixing Metadata");
/// assert!(Phase::Sort.as_str()     == "Sorting");
/// assert!(Phase::Search.as_str()   == "Creating Search Engine");
/// assert!(Phase::Prepare.as_str()  == "Preparing Collection");
/// assert!(Phase::Art.as_str()      == "Preparing Album Art");
/// assert!(Phase::Clone.as_str()    == "Preparing Collection For Disk");
/// assert!(Phase::Convert.as_str()  == "Converting Album Art");
/// assert!(Phase::Finalize.as_str() == "Finalizing Collection");
/// ```
pub enum Phase {
	// Exceptions.
	/// Phase 0
	None,
	/// Phase 0.5
	Disk,

	// Reset.
	/// Phase 1 (start)
	Start,
	/// Phase 2
	WalkDir,
	/// Phase 3
	Parse,
	/// Phase 4
	Fix,
	/// Phase 5
	Sort,
	/// Phase 6
	Search,
	/// Phase 7
	Prepare,
	/// Phase 8
	Art,
	/// Phase 9
	Clone,
	/// Phase 10
	Convert,
	/// Phase 11 (final)
	Finalize,
}

impl Phase {
	/// Human-readable version, no [`String`] allocation.
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::None     => NONE,
			Self::Disk     => DISK,

			Self::Start    => START,
			Self::WalkDir  => WALKDIR,
			Self::Parse    => PARSE,
			Self::Fix      => FIX,
			Self::Sort     => SORT,
			Self::Search   => SEARCH,
			Self::Prepare  => PREPARE,
			Self::Art      => ART,
			Self::Clone    => CLONE,
			Self::Convert  => CONVERT,
			Self::Finalize => FINALIZE,
		}
	}
//
//	#[inline]
//	/// Returns an iterator over all [`Phase`] variants in sequential order.
//	///
//	/// # Note
//	/// This excludes [`Phase::None`].
//	pub fn iter() -> std::slice::Iter<'static, Self> {
//		[
//			Self::Start,
//			Self::WalkDir,
//			Self::Parse,
//			Self::Fix,
//			Self::Sort,
//			Self::Search,
//			Self::Prepare,
//			Self::Resize,
//			Self::Finalize,
//		].iter()
//	}
}

impl AsRef<str> for Phase {
	fn as_ref(&self) -> &'static str {
		self.as_str()
	}
}

impl std::fmt::Display for Phase {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
