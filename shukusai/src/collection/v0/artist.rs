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
pub(crate) struct Artist {
	/// The [`Artist`]'s name.
	pub(crate) name: String,
	/// Total runtime.
	pub(crate) runtime: Runtime,
	/// Keys to the associated [`Album`]\(s\).
	pub(crate) albums: Vec<AlbumKey>,
	/// Keys to every [`Song`] by this [`Artist`].
	///
	/// The order is [`Album`] release order, then [`Song`] track order.
	pub(crate) songs: Box<[SongKey]>,
}
