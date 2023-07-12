//---------------------------------------------------------------------------------------------------- Use
use crate::collection::Collection;
use disk::Bincode2;

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

	// SAFETY: memmap is used.
	let collection = unsafe { Collection::from_file_memmap() }?;

	Ok(collection.json(
		Some(path),
		Some(bytes),
		Some(header),
		Some(version),
	))
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use disk::Bincode2;
	use readable::{Runtime, Date};
	use std::path::PathBuf;
	use crate::constants::COLLECTION_VERSION;

	#[test]
	// Tests if `Collection::json` outputs valid `JSON`.
	fn json() {
		let path = PathBuf::from(format!("../assets/shukusai/state/collection{COLLECTION_VERSION}_real.bin"));

		let collection = Collection::from_path(&path).unwrap();

		let json = collection.json(
			Some(path),
			None,
			None,
			None,
		);

		assert_ne!(json.len(), 0);

		let _: serde_json::Value = serde_json::from_str(&json).unwrap();
	}
}
