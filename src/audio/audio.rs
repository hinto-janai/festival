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
	Keychain,
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

	}

	#[inline(always)]
	fn msg(&mut self) {
//		let msg = match self.from_kernel.recv_timeout(std::time::Duration::from_millis(15)) {
//			
//			_ => return,
//		};

		use KernelToAudio::*;
		match msg {
			// Audio playback.
			Play        =>
			Stop        =>
			Next        =>
			Last        =>
			Seek(f64)   =>
			Volume(f64) =>

			// Queue.
			PlayQueueKey(QueueKey) =>

			// Collection.
			DropCollection                 => self.drop_collection(),
			NewCollection(Arc<Collection>) => self.
			NewState(RoLock<State>)        =>
		}
	}

	#[inline(always)]
	fn msg_hang(&mut self) {
	}
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
