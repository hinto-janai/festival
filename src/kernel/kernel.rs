//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure,Error};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
use std::sync::{Arc,RwLock};
use super::state::State;
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

	// Search Channels.
	to_search: std::sync::mpsc::Sender<KernelToSearch>,
	from_search: crossbeam_channel::Receiver<SearchToKernel>,

	// Audio Channels.
	to_audio: std::sync::mpsc::Sender<KernelToAudio>,
	from_audio: crossbeam_channel::Receiver<AudioToKernel>,

	// Data.
	collection: Arc<Collection>,
	state: Arc<RwLock<State>>,
}

// `Kernel` boot process:
//
// `bios()` ---> `boot_loader()` ---> `kernel()` ---> `init()` ---> `userspace()`
//          |                                          |
//          |--- (bios error occured, skip to init) ---|
//
//
//
// Ignore the fact that the name of this thing is `Kernel` and it kinda makes sense.
//
// What these phases actually do:
// `bios()`        | Attempt to read `collection.bincode`. Skip to `init()` with default data on failure.
// `boot_loader()` | Wait on `CCD` to transform `Collection`, load other data.
// `kernel()`      | Run safety checks on data.
// `init()`        | Spawn all threads and initialize everything else.
// `userspace()`   | Main loop.

impl Kernel {
	//-------------------------------------------------- bios()
	#[inline(always)]
	// `main()` starts `Kernel` with this.
	pub fn bios(
		to_gui:   std::sync::mpsc::Sender<KernelToGui>,
		from_gui: crossbeam_channel::Receiver<GuiToKernel>
	) {
		debug!("Kernel [1/12] ... entering bios()");

		// Attempt to load `Collection` from file.
		match Collection::from_file() {
			// If success, continue to `boot_loader` to convert
			// bytes to actual usable `egui` images.
			Ok(collection) => Self::boot_loader(collection, to_gui, from_gui),

			// Else, straight to `init` with default flag set.
			Err(e) => Self::init(None, None, to_gui, from_gui),
		}
	}

	//-------------------------------------------------- boot_loader()
	#[inline(always)]
	fn boot_loader(
		collection: Collection,
		to_gui:     std::sync::mpsc::Sender<KernelToGui>,
		from_gui:   crossbeam_channel::Receiver<GuiToKernel>
	) {
		debug!("Kernel [2/12] ... entering boot_loader()");

		// We successfully loaded `Collection`.
		// Create `CCD` channel + thread and make it convert images.
		debug!("Kernel [3/12] ... spawning CCD");
		let (ccd_send, from_ccd) = std::sync::mpsc::channel::<CcdToKernel>();
		std::thread::spawn(move || Ccd::convert_img(ccd_send));

		// Before hanging on `CCD`, read `State` file.
		// Note: This is a `Result`.
		debug!("Kernel [4/12] ... reading State");
		let state = State::from_file();

		// Wait for `Collection` to be returned by `CCD`.
		debug!("Kernel [5/12] ... waiting on CCD");
		let collection = loop {
			use CcdToKernel::*;
			match recv!(from_ccd) {
				NewCollection(collection) => break collection,
				Failed(string)            => (), // TODO: Forward to `GUI`.
				Update(string)            => (), // TODO: Forward to `GUI`.
			}
		};

		// Continue to `kernel` to verify data.
		Self::kernel(collection, state, to_gui, from_gui);
	}

	//-------------------------------------------------- kernel()
	#[inline(always)]
	fn kernel(
		collection: Collection,
		state:      Result<State, anyhow::Error>,
		to_gui:     std::sync::mpsc::Sender<KernelToGui>,
		from_gui:   crossbeam_channel::Receiver<GuiToKernel>,
	) {
		/* TODO: initialize and sanitize collection & misc data */
		debug!("Kernel [6/12] ... entering kernel()");
		let state = state.unwrap();

		Self::init(Some(collection), Some(state), to_gui, from_gui);
	}

	//-------------------------------------------------- init()
	#[inline(always)]
	fn init(
		collection: Option<Collection>,
		state:      Option<State>,
		to_gui:     std::sync::mpsc::Sender<KernelToGui>,
		from_gui:   crossbeam_channel::Receiver<GuiToKernel>,
	) {
		debug!("Kernel [7/12] ... entering init()");

		// Handle potentially missing `Collection`.
		let collection = match collection {
			Some(c) => { debug!("Kernel [8/12] ... Collection found"); Arc::new(c) },
			None    => { debug!("Kernel [8/12] ... Collection NOT found, returning default"); Arc::new(Collection::new()) },
		};

		// Handle potentially missing `State`.
		let state = match state {
			Some(s) => { debug!("Kernel [9/12] ... State found"); Arc::new(RwLock::new(s)) },
			None    => { debug!("Kernel [9/12] ... State NOT found, returning default"); Arc::new(RwLock::new(State::new())) },
		};

		// Create `To` channels.
		let (to_search, search_recv) = std::sync::mpsc::channel::<KernelToSearch>();
		let (to_audio,  audio_recv)  = std::sync::mpsc::channel::<KernelToAudio>();

		// Create `From` channels.
		let (search_send, from_search) = crossbeam_channel::unbounded::<SearchToKernel>();
		let (audio_send,  from_audio)  = crossbeam_channel::unbounded::<AudioToKernel>();

		// Create `Kernel`.
		let kernel = Self {
			// Channels.
			to_gui, from_gui,
			to_search, from_search,
			to_audio, from_audio,

			// Data.
			collection, state,
		};

		// Spawn `Search`.
		debug!("Kernel [10/12] ... spawning Search");
		let collection = Arc::clone(&kernel.collection);
		std::thread::spawn(move || Search::init(collection, search_send, search_recv));

		// Spawn `Audio`.
		debug!("Kernel [11/12] ... spawning Audio");
		let collection = Arc::clone(&kernel.collection);
		let state      = RoLock::new(&kernel.state);
		std::thread::spawn(move || Audio::init(collection, state, audio_send, audio_recv));

		// We're done, enter main `userspace` loop.
		ok_debug!("Kernel [12/12] ... BOOT PROCESS");
		debug!("Kernel: entering userspace()");
		Self::userspace(kernel);
	}

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
