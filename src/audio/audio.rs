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
	CollectionKeychain,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use crate::kernel::State;
use std::sync::Arc;
use rolock::RoLock;
use super::msg::{
	AudioToKernel,
	KernelToAudio,
};
use crossbeam_channel::{Sender,Receiver};

//---------------------------------------------------------------------------------------------------- Audio
pub(crate) struct Audio {
	collection:  Arc<Collection>,         // Pointer to `Collection`
	state:       RoLock<State>,           // Read-Only lock to the `State`
	to_kernel:   Sender<AudioToKernel>,   // Channel TO `Kernel`
	from_kernel: Receiver<KernelToAudio>, // Channel FROM `Kernel`
}

impl Audio {
	#[inline(always)]
	// Kernel starts `Audio` with this.
	pub(crate) fn init(
		collection: Arc<Collection>,
		state: RoLock<State>,
		to_kernel: Sender<AudioToKernel>,
		from_kernel: Receiver<KernelToAudio>,
	) {
		// Init data.
		let audio = Self {
			collection,
			state,
			to_kernel,
			from_kernel,
		};

		// Start `main()`.
		Self::main(audio);
	}
}

//---------------------------------------------------------------------------------------------------- Main Audio loop.
impl Audio {
	#[inline(always)]
	fn main(mut self) {
		ok_debug!("Audio");

		// Block, wait for signal.
		let msg = recv!(self.from_kernel);

		// Match message and do action.
		use KernelToAudio::*;
		match msg {
			_ => self.msg_new(),
		}
	}

	#[inline(always)]
	fn msg_new(&mut self) { /* TODO: create new collection */ }
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
