//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use std::marker::PhantomData;
use readable::Runtime;
use crate::collection::{
	AlbumKey,
	SongKey,
};

//----------------------------------------------------------------------------------------------------
#[derive(Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Encode,Decode)]
/// Struct holding [`Artist`] metadata, with pointers to [`Album`]\(s\)
///
/// This struct holds all the metadata about a particular [`Artist`].
///
/// It contains an [`Vec`] of [`AlbumKey`]\(s\) that are the indices of the associated [`Album`]\(s\), in the [`Collection`].
pub struct Artist {
	/// The [`Artist`]'s name.
	pub name: String,
	/// Total runtime.
	pub runtime: Runtime,
	/// Keys to the associated [`Album`]\(s\).
	pub albums: Vec<AlbumKey>,
	/// Keys to every [`Song`] by this [`Artist`].
	///
	/// The order is [`Album`] release order, then [`Song`] track order.
	pub songs: Box<[SongKey]>,

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
