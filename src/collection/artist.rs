//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use super::{
	Album,
	AlbumKey,
	Collection,
};

//----------------------------------------------------------------------------------------------------
//#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[derive(Debug,Serialize,Deserialize)]
/// Struct holding [`Artist`] metadata, with pointers to [`Album`]\(s\).
///
/// This struct holds all the metadata about a particular [`Artist`].
///
/// It contains an [`Vec`] of [`AlbumKey`]\(s\) that are the indicies of the associated [`Album`]\(s\), in the [`Collection`].
pub struct Artist {
	/// The [`Artist`]'s name.
	pub name: String,
	/// Keys to the associated [`Album`]\(s\).
	pub albums: Vec<AlbumKey>,
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
