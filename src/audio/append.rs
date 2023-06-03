//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use crate::kernel::Kernel;

//---------------------------------------------------------------------------------------------------- Volume.
/// The different ways you can append songs to the audio queue.
///
/// The [`Default`] is `Append::Front`.
#[derive(Copy,Clone,Default,Debug,Hash,Eq,Ord,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
pub enum Append {
	#[default]
	/// Add a single or multiple songs to the front.
	///
	/// Queue:
	/// - Before: `a, b, c`
	/// - Input: `1, 2, 3`
	/// - After: `1, 2, 3, a, b, c`
	Front,

	/// Add a single or multiple songs to the back.
	///
	/// - Before: `a, b, c`
	/// - Input: `1, 2, 3`
	/// - After: `a, b, c, 1, 2, 3`
	Back,

	/// Add a single or multiple songs starting at an index.
	///
	/// - Before: `a, b, c`
	/// - Input: `1, 2, 3` with index `1`
	/// - After: `a, 1, 2, 3, b, c`
	Index(usize),
}
