//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use crate::constants::{
	TXT_SUB_DIR,
	FESTIVAL,
	HEADER,
	HEADER_STR,
	COLLECTION_VERSION,
	FRONTEND_SUB_DIR,
};
use readable::Unsigned;
use crate::collection::Collection;
use disk::Bincode2;
use const_format::formatcp;
use std::mem::size_of;

//---------------------------------------------------------------------------------------------------- Public function.
/// Function that can access [`Collection`]'s on-disk metadata.
///
/// This function will attempt to parse the [`Collection`] that
/// is currently on disk and extract the header metadata from it.
///
/// The returned [`String`] is in `JSON` form.
pub fn metadata() -> Result<String, anyhow::Error> {
	CollectionMetadata::json()
}

//---------------------------------------------------------------------------------------------------- Constants
// Bincode saves the struct as is, meaning the first fields of our `Collection`
// directly correlate with the first `x` bytes of the actual saved file.
//
// Meaning, we know exactly where fields start/end, and thus how many bytes to read.
//
// All the metadata of the `Collection` is intentionally placed
// at the front of the struct so that we get to do this easily.
const METADATA_SIZE_OF: usize = {
	(size_of::<u8>() * 24) + // `crate::constants::HEADER`
	size_of::<u8>()        + // `crate::constants::COLLECTION_VERSION`
	size_of::<bool>()      + // `empty`
	size_of::<u64>()       + // `timestamp`
	size_of::<Unsigned>()  + // `count_artist`
	size_of::<Unsigned>()  + // `count_album`
	size_of::<Unsigned>()  + // `count_song`
	size_of::<Unsigned>()    // `count_art`
};

//---------------------------------------------------------------------------------------------------- CollectionMetadata
disk::bincode2!(CollectionMetadata, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/{TXT_SUB_DIR}"), "metadata", HEADER, COLLECTION_VERSION);
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Encode,Decode)]
// A struct representing a file that can access [`Collection`]'s metadata.
//
// Running [`Self::get`] will attempt to parse the [`Collection`]
// on disk and extract the header metadata from it in `JSON` form.
struct CollectionMetadata {
	empty: bool,
	timestamp: u64,
	count_artist: Unsigned,
	count_album: Unsigned,
	count_song: Unsigned,
	count_art: Unsigned,
}

impl CollectionMetadata {
	// Get a `JSON` string representing the [`Collection`]'s metadata.
	fn json() -> Result<String, anyhow::Error> {
		let (size, path) = Collection::file_size()?.into_parts();
		let bytes        = Collection::file_bytes(0, METADATA_SIZE_OF)?;
		let s            = Self::from_bytes(&bytes)?;

		Ok(format!(
r#""metadata": {{
    "path": "{}",
    "bytes": {},
    "header": "{}",
    "version": {},
    "empty": {},
    "timestamp": {},
    "artists": {},
    "albums": {},
    "songs": {}
}}"#,
			path.display(),
			size,
			HEADER_STR,
			COLLECTION_VERSION,
			s.empty,
			s.timestamp,
			s.count_artist.inner(),
			s.count_album.inner(),
			s.count_song.inner(),
		))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
