//---------------------------------------------------------------------------------------------------- Use
use crate::collection::Collection;
use disk::Bincode2;

//----------------------------------------------------------------------------------------------------
/// Read `Collection` from disk, output same output as the `state_collection_full` JSON-RPC call.
pub fn state_collection_full() -> Result<String, anyhow::Error> {
	// SAFETY: memmap is used.
	let collection = unsafe { Collection::from_file_memmap() }?;

	Ok(serde_json::to_string_pretty(&collection)?)
}
