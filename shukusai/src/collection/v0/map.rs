//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use std::collections::HashMap;
use crate::collection::v0::{
	Artist,
	Album,
	Song,
};
use crate::collection::{
	ArtistKey,
	AlbumKey,
	SongKey,
};

//---------------------------------------------------------------------------------------------------- Map
#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq,Encode,Decode)]
#[serde(transparent)]
/// A [`HashMap`] that knows all [`Artist`]'s, [`Album`]'s and [`Song`]'s.
///
/// No public functions are implemented on this type directly,
/// use [`Collection`]'s functions instead.
pub(crate) struct Map(pub(crate) HashMap<String, (ArtistKey, AlbumMap)>);

impl Map {
	#[inline(always)]
	pub(crate) fn new() -> Self {
		Self::default()
	}
}

//---------------------------------------------------------------------------------------------------- AlbumMap
#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq,Encode,Decode)]
#[serde(transparent)]
pub(crate) struct AlbumMap(pub(crate) HashMap<String, (AlbumKey, SongMap)>);

//---------------------------------------------------------------------------------------------------- SongMap
#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq,Encode,Decode)]
#[serde(transparent)]
pub(crate) struct SongMap(pub(crate) HashMap<String, SongKey>);
