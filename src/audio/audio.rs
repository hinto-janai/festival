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
use crate::kernel::PlayerState;
use std::sync::Arc;
use rolock::RoLock;
use super::msg::{
	AudioToKernel,
	KernelToAudio,
};

//---------------------------------------------------------------------------------------------------- Audio
pub struct Audio {
	collection:   Arc<Collection>,                             // Pointer to `Collection`
	player_state: RoLock<PlayerState>,                         // Read-Only lock to the `PlayerState`
	to_kernel:    crossbeam_channel::Sender<AudioToKernel>,   // Channel TO `Kernel`
	from_kernel:  std::sync::mpsc::Receiver<KernelToAudio>,    // Channel FROM `Kernel`
}

impl Audio {
	// Kernel starts `Audio` with this.
	pub fn init(
		collection: Arc<Collection>,
		player_state: RoLock<PlayerState>,
		to_kernel: crossbeam_channel::Sender<AudioToKernel>,
		from_kernel: std::sync::mpsc::Receiver<KernelToAudio>,
	) {
		// Init data.
		let audio = Self {
			collection,
			player_state,
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
