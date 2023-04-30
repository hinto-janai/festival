//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use super::{
	Album,
	Collection,
};
use crate::key::AlbumKey;

//----------------------------------------------------------------------------------------------------
#[derive(Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
/// Struct holding [`Artist`] metadata, with pointers to [`Album`]\(s\)
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
