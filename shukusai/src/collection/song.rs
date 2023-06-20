//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use std::path::PathBuf;
use readable::Runtime;
use std::marker::PhantomData;
use crate::collection::{
	Collection,
	Artist,
	Album,
	AlbumKey,
	SongKey,
};

//----------------------------------------------------------------------------------------------------
#[derive(Clone,Debug,Default,Hash,PartialEq,PartialOrd,Encode,Decode)]
/// Struct holding [`Song`] metadata, with a pointer to the [`Album`] it belongs to
///
/// This struct holds all the metadata about a particular [`Song`].
///
/// It contains a [`SongKey`] that is the index of the owning [`Album`], in the [`Collection`].
pub struct Song {
	// User-facing data.
	/// Title of the [`Song`].
	pub title: String,
	/// Key to the [`Album`].
	pub album: AlbumKey,
	/// Total runtime of this [`Song`].
	pub runtime: Runtime,
	/// Sample rate of this [`Song`].
	pub sample_rate: u32,
	/// The track number of this [`Song`].
	pub track: Option<u32>,
	/// The disc number of this [`Song`].
	pub disc: Option<u32>,
	/// The [`PathBuf`] this [`Song`] is located at.
	pub path: PathBuf,

	// Reserved fields that should SOMEDAY be implemented.
	/// Additional [`Artist`]'s that are on this [`Song`].
	pub(crate) _track_artists: PhantomData<Option<String>>,
	/// The [`Song`]'s lyrics.
	pub(crate) _lyrics: PhantomData<Option<String>>,

	// Unknown reserved fields and their `size_of()`.
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
