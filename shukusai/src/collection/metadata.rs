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
/// This output is not meant to be relied on (yet).
///
/// It it mostly for quick displaying and debugging
/// purposes and may be changed at any time.
///
/// The returned [`String`] is in `JSON` form.
///
/// This function will attempt to parse the [`Collection`] that
/// is currently on disk and extract the metadata from it.
pub fn metadata() -> Result<String, anyhow::Error> {
	let (bytes, path) = Collection::file_size()?.to_parts();

	let header = Collection::file_header_to_string()?;

	let version = Collection::file_version()?;

	let collection = unsafe { Collection::from_file_memmap() }?;

	Ok(collection.json(
		Some(path),
		Some(bytes),
		Some(header),
		Some(version),
	))
}
