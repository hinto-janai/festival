//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
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
	Collection,
	key::CollectionKeychain,
	key::ArtistKey,
	key::AlbumKey,
	key::SongKey,
};
use std::sync::Arc;
use super::msg::{
	CcdToKernel,
	KernelToCcd,
};

//---------------------------------------------------------------------------------------------------- CCD
pub struct Ccd {
	old_collection:  Arc<Collection>,                       // Pointer to the OLD `Collection`
	to_kernel:   crossbeam_channel::Sender<CcdToKernel>,   // Channel TO `Kernel`
	from_kernel: std::sync::mpsc::Receiver<KernelToCcd>,    // Channel FROM `Kernel`
}

impl Ccd {
	// Kernel starts `CCD` with this.
	pub fn init(
		old_collection: Arc<Collection>,
		to_kernel: crossbeam_channel::Sender<CcdToKernel>,
		from_kernel: std::sync::mpsc::Receiver<KernelToCcd>,
	) {
		// Init data.
		let ccd = Self {
			old_collection,
			to_kernel,
			from_kernel,
		};

		// Start `main()`.
		Self::main(ccd);
	}
}

//---------------------------------------------------------------------------------------------------- Main CCD loop.
impl Ccd {
	#[inline(always)]
	fn main(mut self) {
		ok_debug!("CCD");

		// Block, wait for signal.
		let msg = recv!(self.from_kernel);

		// Match message and do action.
		use KernelToCcd::*;
		match msg {
			NewCollection(old_ptr) => self.msg_new(),
			ConvertImg(collection) => self.msg_convert(),
			Die                    => (),
		}

		// Drop old `Collection` and die.
		debug!("CCD: Dropping Collection...");
		let now = std::time::Instant::now();
		drop(self.old_collection);
		debug!("CCD: Took {} seconds.", now.elapsed().as_secs_f32());
		debug!("CCD: Goodbye world.");
	}

	#[inline(always)]
	fn msg_new(&mut self) { /* TODO: create new collection */ }
	fn msg_convert(&mut self) { /* TODO: convert existing collection */ }
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
