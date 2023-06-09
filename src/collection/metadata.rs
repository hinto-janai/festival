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
const METADATA_SIZE_OF: usize = {
	25 + // Header
	1  + // bool
	8  + // u64
	(48 * 4) // Unsigned
};

//---------------------------------------------------------------------------------------------------- CollectionMetadata
disk::bincode2!(CollectionMetadata, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/{TXT_SUB_DIR}"), "metadata", HEADER, COLLECTION_VERSION);
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Encode,Decode)]
// A struct representing a file that can access [`Collection`]'s metadata.
//
// Running [`Self::get`] will attempt to parse the [`Collection`]
// on disk and extract the header metadata from it in `JSON` form.
struct CollectionMetadata {
	empty: bool,            // 1
	timestamp: u64,         // 8
	count_artist: Unsigned, // 48
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
