//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use std::marker::PhantomData;
use crate::collection::{
	Album,
	Collection,
	AlbumKey,
};

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

	// Reserved fields and their `size_of()`.
	pub(crate) _reserved1: PhantomData<Box<[usize]>>, // 16
	pub(crate) _reserved2: PhantomData<Box<[usize]>>, // 16
	pub(crate) _reserved3: PhantomData<Box<[usize]>>, // 16
	pub(crate) _reserved4: PhantomData<Box<[usize]>>, // 16
	pub(crate) _reserved5: PhantomData<String>,       // 24
	pub(crate) _reserved6: PhantomData<usize>,        // 8
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
