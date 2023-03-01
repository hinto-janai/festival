//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure,Error};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
use std::sync::{Arc,RwLock};
use super::player_state::PlayerState;
use rolock::RoLock;
use crate::macros::{
	ok_debug,
	recv,
	send,
};
use disk::Bincode;
use crate::{
	gui::{KernelToGui, GuiToKernel},
	ccd::{KernelToCcd, CcdToKernel, Ccd},
	search::{KernelToSearch, SearchToKernel, Search},
	audio::{KernelToAudio, AudioToKernel, Audio},
	collection::Collection,
};

//---------------------------------------------------------------------------------------------------- Kernel
pub struct Kernel {
	// GUI Channels.
	to_gui: std::sync::mpsc::Sender<KernelToGui>,
	from_gui: crossbeam_channel::Receiver<GuiToKernel>,

	// CCD Channels.
	to_ccd: std::sync::mpsc::Sender<KernelToCcd>,
	from_ccd: crossbeam_channel::Receiver<CcdToKernel>,

	// Search Channels.
	to_search: std::sync::mpsc::Sender<KernelToSearch>,
	from_search: crossbeam_channel::Receiver<SearchToKernel>,

	// Audio Channels.
	to_audio: std::sync::mpsc::Sender<KernelToAudio>,
	from_audio: crossbeam_channel::Receiver<AudioToKernel>,

	// Audio Playback State.
	player_state: Arc<RwLock<PlayerState>>,

	// Collection.
	collection: Arc<Collection>,
}

// `Kernel` boot process:
//
// `bios()` -> `boot_loader()` -> `kernel()` -> `init()` -> `userspace()`
//
// Ignore the fact that the name of this thing is `Kernel` and it kinda makes sense.
//
// What these phases actually do:
// `bios()`        | Enumerate threads and channels.
// `boot_loader()` | Load vital data into memory, e.g: `Collection`
// `kernel()`      | Transform data, run safety checks on data.
// `init()`        | Signal all threads OK and initialize everything else.
// `userspace()`   | Main loop.

impl Kernel {
	#[inline(always)]
	// `main()` starts `Kernel` with this.
	pub fn bios(
		to_gui: std::sync::mpsc::Sender<KernelToGui>,
		from_gui: crossbeam_channel::Receiver<GuiToKernel>
	) {
		debug!("Kernel [/] ... entering bios()");
		debug!("Kernel [/] ... creating channels");

		// Create `To` channels.
		let (to_ccd,    ccd_recv)    = std::sync::mpsc::channel::<KernelToCcd>();
		let (to_search, search_recv) = std::sync::mpsc::channel::<KernelToSearch>();
		let (to_audio,  audio_recv)  = std::sync::mpsc::channel::<KernelToAudio>();

		// Create `From` channels.
		let (ccd_send,    from_ccd)    = crossbeam_channel::unbounded::<CcdToKernel>();
		let (search_send, from_search) = crossbeam_channel::unbounded::<SearchToKernel>();
		let (audio_send,  from_audio)  = crossbeam_channel::unbounded::<AudioToKernel>();

		// Create temporary "dummy" `Kernel`.
		let kernel = Self {
			// Channels.
			to_gui, from_gui,
			to_ccd, from_ccd,
			to_search, from_search,
			to_audio, from_audio,

			// Create temporary "dummy" `PlayerState`.
			player_state: Arc::new(RwLock::new(PlayerState::dummy())),
			// Create temporary "dummy" `Collection`.
			collection: Arc::new(Collection::dummy()),
		};

		// Spawn everyone :)
		// CCD.
		debug!("Kernel ... spawning CCD");
		let ptr = Arc::clone(&kernel.collection);
		std::thread::spawn(move || Ccd::init(ptr, ccd_send, ccd_recv));

		// Search.
		debug!("Kernel ... spawning Search");
		let ptr = Arc::clone(&kernel.collection);
		std::thread::spawn(move || Search::init(ptr, search_send, search_recv));

		// Audio.
		debug!("Kernel ... spawning Audio");
		let ptr    = Arc::clone(&kernel.collection);
		let rolock = RoLock::new(&kernel.player_state);
		std::thread::spawn(move || Audio::init(ptr, rolock, audio_send, audio_recv));

		// Threads are enumerated.
		// Now we have a few options on what to do.
		// AKA: `boot_loader` phase.
		Self::boot_loader(kernel);
	}

	#[inline(always)]
	fn boot_loader(self) {
		// We need to load the real `Collection` into memory.
		// `CCD` is waiting on the other end for a signal.
		debug!("Kernel ... entering boot_loader()");

		// Load from file... if it exists... or if we can.
		let maybe_collection = Collection::from_file();

		// If success, continue to `kernel` to convert
		// bytes to actual usable `egui` images.
		if let Ok(c) = maybe_collection {
			Self::kernel(self, c);
		// Else, `init`.
		} else if let Err(e) = maybe_collection {
			Self::init(self);
		}
	}

	#[inline(always)]
	fn kernel(self, collection: Collection) { /* TODO: initialize and sanitize collection & misc data */ }

	#[inline(always)]
	fn init(self) { /* TODO: send an OK to all threads and enter userspace */ }
}

//---------------------------------------------------------------------------------------------------- Main Kernel loop (userspace)
impl Kernel {
	#[inline(always)]
	fn userspace(self) {
		/* TODO: loop and forward signals */
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
