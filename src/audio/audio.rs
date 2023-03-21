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

//---------------------------------------------------------------------------------------------------- Constants.
// How long should `Audio` wait for a message from `Kernel` before timing out?
//
// This is done in the same `loop` as the audio demuxing/decoding,
// so a timeout that is too _long_ will cause audible skips within
// the played audio, while a timeout that is too _short_ will wake
// the sleeping CPU core more often.
const AUDIO_MESSAGE_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(15);

//---------------------------------------------------------------------------------------------------- Main Audio loop.
impl Audio {
	#[inline(always)]
	fn main(mut self) {
		ok_debug!("Audio");

		loop {
			// Listen for message.
			self = self.msg();

			// TODO:
			// Audio loop.
		}
	}

	#[inline(always)]
	fn msg(mut self) -> Self {
		let msg = match self.from_kernel.recv_timeout(AUDIO_MESSAGE_TIMEOUT) {
			Ok(msg) => msg,
			_ => return self,
		};

		use KernelToAudio::*;
		match msg {
			// Audio playback.
			Play        => todo!(),
			Stop        => todo!(),
			Next        => todo!(),
			Last        => todo!(),
			Seek(f64)   => todo!(),
			Volume(f64) => todo!(),

			// Queue.
			PlayQueueKey(QueueKey) => todo!(),

			// Collection.
			DropCollection     => self = self.msg_drop(),
			NewCollection(arc) => self.collection = arc,
			NewState(rolock)   => self.state = rolock,
		}

		self
	}

	#[inline(always)]
	fn msg_drop(mut self) -> Self {
		// Drop pointer.
		drop(self.collection);

		// Hang until we get the new one.
		debug!("Audio: Dropped Collection, waiting...");

		// Ignore messages until it's a pointer.
		loop {
			if let KernelToAudio::NewCollection(arc) = recv!(self.from_kernel) {
				ok_debug!("Audio: New Collection");
				self.collection = arc;
				return self
			}

			error!("Audio: Incorrect message received");
		}
	}
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
