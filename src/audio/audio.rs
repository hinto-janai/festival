//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use benri::{
	sleep,
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
	AUDIO_STATE,
	AudioToKernel,
	KernelToAudio,
	AudioState,
};
use crossbeam::channel::{Sender,Receiver};
use rodio::{Sink,OutputStream};

//---------------------------------------------------------------------------------------------------- Audio Init
pub(crate) struct Audio {
	sink:        Sink,                    // Audio sink, holder and controller of all `Source`'s
	collection:  Arc<Collection>,         // Pointer to `Collection`
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
		trace!("Audio - State:\n{state:#?}");

		const RETRY_SECONDS: u64 = 5;

		// Loop until we can connect to an audio device.
		let sink = {
			let (stream, stream_handle) = loop {
				 match OutputStream::try_default() {
					Ok((s, sh)) => { debug!("Audio [1/2] - Output device"); break (s, sh); },
					Err(e) => {
						warn!("Audio - Output device error: {e} ... retrying in {RETRY_SECONDS} seconds");
					},
				}
				sleep!(RETRY_SECONDS);
			};

			loop {
				match Sink::try_new(&stream_handle) {
					Ok(s)  => { debug!("Audio [2/2] - Sink"); break s; },
					Err(e) => warn!("Audio - Sink error: {e} ... retrying in {RETRY_SECONDS} seconds"),
				}
				sleep!(RETRY_SECONDS);
			}
		};

		// Re-write global `AudioState`.
		*AUDIO_STATE.write() = state;

		// Init data.
		let audio = Self {
			sink,
			collection,
			to_kernel,
			from_kernel,
		};

		// Start `main()`.
		ok_debug!("Audio");
		Self::main(audio);
	}
}

//---------------------------------------------------------------------------------------------------- Main Audio loop.
impl Audio {
	#[inline(always)]
	fn main(mut self) {
		loop {
			// Listen for message.
			let msg = recv!(self.from_kernel);

			use KernelToAudio::*;
			match msg {
				// TODO: Implement.
				// Audio playback.
				Toggle      => trace!("Audio - Toggle"),
				Play        => trace!("Audio - Play"),
				Stop        => trace!("Audio - Stop"),
				Next        => trace!("Audio - Next"),
				Last        => trace!("Audio - Last"),

				// Audio settings.
				Shuffle     => trace!("Audio - Shuffle"),
				Repeat      => trace!("Audio - Repeat"),
				Volume(v)   => trace!("Audio - Volume"),
				Seek(f)     => trace!("Audio - Seek"),

				// Queue.
				AddQueueSongFront(s_key)    => (),
				AddQueueSongBack(s_key)     => (),
				AddQueueAlbumFront(al_key)  => (),
				AddQueueAlbumBack(al_key)   => (),
				AddQueueArtistFront(ar_key) => (),
				AddQueueArtistBack(ar_key)  => (),

				// Queue Index.
				PlayQueueIndex(q_key)   => (),
				RemoveQueueIndex(q_key) => (),

				// Collection.
				DropCollection     => self.msg_drop(),
				NewCollection(arc) => self.collection = arc,
			}
		}
	}

	#[inline(always)]
	fn msg_drop(&mut self) {
		// Drop pointer.
		self.collection = Collection::dummy();

		// Hang until we get the new one.
		debug!("Audio - Dropped Collection, waiting...");

		// Ignore messages until it's a pointer.
		loop {
			if let KernelToAudio::NewCollection(arc) = recv!(self.from_kernel) {
				ok_debug!("Audio - New Collection");
				self.collection = arc;
				return;
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
