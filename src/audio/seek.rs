//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use crate::kernel::Kernel;
use crate::collection::Song;

//---------------------------------------------------------------------------------------------------- Seek
/// The different we can seek audio.
#[derive(Copy,Clone,Debug,Hash,Eq,Ord,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
pub enum Seek {
	/// Seek forwards a specified amount
	Forward,
	/// Seek backwards a specified amount
	Backward,
	/// Seek to an absolute second timestamp
	Absolute,
}
