//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use strum::{
	IntoEnumIterator,
};
use strum_macros::{
	EnumIter,
	EnumString,
	IntoStaticStr,
};

//---------------------------------------------------------------------------------------------------- CollectionSort
// All the ways to sort the [Collection].
// String sorting is done lexicographically as per the stdlib [Ord] implementation:
// https://doc.rust-lang.org/std/primitive.str.html#impl-Ord
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,IntoStaticStr,EnumString,EnumIter,Serialize,Deserialize)]
#[strum(serialize_all = "PascalCase")]
pub enum CollectionSort {
	#[default]
	ArtistRelease, // Artist sorted lexicographically, Albums of artist sorted oldest to latest
	ArtistTitle,   // Artist sorted lexicographically, Albums of artist sorted lexicographically
	Release,       // Albums sorted oldest to latest
	Title,         // Albums sorted lexicographically
}

impl CollectionSort {
	#[inline(always)]
	pub fn as_str(&self) -> &'static str {
		self.into()
	}
}

//----------------------------------------------------------------------------------------------------
//#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
// Sort impl TODO:
// - [Artist, Release]
// - [Album]
// - [Release]
//
// This sorts for [Artist, Album].
//fn sort_artist_album(collection: Collection) {
//	collection.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
//	for artist in collection.iter_mut() {
//	collection.into_par_iter().for_each(|mut artist| {
//		artist.albums.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
//	}
//}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
