//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use benri::{
	debug_panic,
	log::*,
	sync::*,
};
use crate::collection::{
	Collection,
	Keychain,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use std::sync::{
	Arc,RwLock,
};
use crate::audio::{
	AudioToKernel,
	KernelToAudio,
	AudioState,
};
use crossbeam::channel::{Sender,Receiver};

//---------------------------------------------------------------------------------------------------- Audio
pub(crate) struct Audio {
	collection:  Arc<Collection>,         // Pointer to `Collection`
	state:       Arc<RwLock<AudioState>>, // RwLock to the global `AudioState`
	to_kernel:   Sender<AudioToKernel>,   // Channel TO `Kernel`
	from_kernel: Receiver<KernelToAudio>, // Channel FROM `Kernel`
}

impl Audio {
	#[inline(always)]
	// Kernel starts `Audio` with this.
	pub(crate) fn init(
		collection:  Arc<Collection>,
		state:       AudioState,
		to_kernel:   Sender<AudioToKernel>,
		from_kernel: Receiver<KernelToAudio>,
	) {
		// Re-write global `AudioState`.
		let global_state = AudioState::get_priv();
		*lockw!(global_state) = state;

		// Init data.
		let audio = Self {
			collection,
			state: global_state,
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
			// TODO: Implement.
			Toggle      => trace!("Audio - Toggle"),
			Play        => trace!("Audio - Play"),
			Stop        => trace!("Audio - Stop"),
			Next        => trace!("Audio - Next"),
			Last        => trace!("Audio - Last"),
			Shuffle     => trace!("Audio - Shuffle"),
			Repeat      => trace!("Audio - Repeat"),
			Volume(v)   => trace!("Audio - Volume"),
			Seek(f)     => trace!("Audio - Seek"),

			// Queue.
			// TODO: Implement.
			PlayQueueKey(queue_key) => trace!("Audio - PlayQueueKey"),

			// Collection.
			DropCollection     => self = self.msg_drop(),
			NewCollection(arc) => self.collection = arc,
		}

		self
	}

	#[inline(always)]
	fn msg_drop(mut self) -> Self {
		// Drop pointer.
		drop(self.collection);

		// Hang until we get the new one.
		debug!("Audio - Dropped Collection, waiting...");

		// Ignore messages until it's a pointer.
		loop {
			if let KernelToAudio::NewCollection(arc) = recv!(self.from_kernel) {
				ok_debug!("Audio - New Collection");
				self.collection = arc;
				return self
			}

			debug_panic!("Audio - Incorrect message received");
			error!("Audio - Incorrect message received");
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
