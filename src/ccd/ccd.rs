//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use crate::macros::{
	ok_debug,
	recv,
	send,
};
use crate::collection::{
	Album,
	Collection,
	CollectionKeychain,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use super::msg::{
	CcdToKernel,
	KernelToCcd,
};
use crate::collection::Art;
use crossbeam_channel::{Sender,Receiver};
use std::path::{Path,PathBuf};

// TODO:
// - Document code
// - Send `Kernel` messages
// - Log


//---------------------------------------------------------------------------------------------------- CCD
pub(crate) struct Ccd;

impl Ccd {
	#[inline(always)]
	//-------------------------------------------------------------------------------- CCD `convert_art()`
	// Public facing "front-end" function for image conversion.
	// Dynamically selects internal functions for single/multi-thread.
	pub(crate) fn convert_art(to_kernel: Sender<CcdToKernel>, collection: Collection) {
		ok_debug!("CCD - Purpose in life: convert_art()");

		// If no albums, return.
		if collection.albums.len() == 0 {
			send!(to_kernel, CcdToKernel::NewCollection(collection));
		// Else, convert art, send to `Kernel`.
		} else {
			send!(to_kernel, CcdToKernel::NewCollection(Self::priv_convert_art(&to_kernel, collection)));
		}
	}

	#[inline(always)]
	//-------------------------------------------------------------------------------- CCD `new_collection()`
	// Public facing "front-end" function for making a new `Collection`.
	//
	// These operations are split up into different private
	// functions mostly for testing flexability.
	pub(crate) fn new_collection<P>(
		to_kernel: Sender<CcdToKernel>,
		from_kernel: Receiver<KernelToCcd>,
		paths: &[&P],
	) where
		P: AsRef<Path>
	{
		ok_debug!("CCD - Purpose in life: new_collection()");
		// TODO: new_collection() steps:
		// 1. WalkDir given path(s).
		// 2. Filter for audio files.
		// 3. For each file, get metadata, add to `Collection`.
		// 4.
		//     a) If image metadata exists, append
		//     b) If not, search parent dir for `jpeg/png`
		//     c) Given multiple images, pick the highest quality image
		//     d) Given no image, append `None`
		//     e) Sort `Vec` keys, append `Collection` metadata
		//
		// 5. Save to disk.
		// 6. Transform in-memory `Collection` with `priv_convert_art()`
		// 7. Send to `Kernel`
		// 8. Wait for `Die` signal.
		// 9. Die, destruct the old `Collection`.

		// TODO: Handle potential errors:
		// 1. No albums
		// 2. Path error
		// 3. Permission error
		// 4. Disk error

		// TODO: Send updates to `Kernel` throughout and `log!()`.
	}
}
