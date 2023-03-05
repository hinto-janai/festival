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
use crossbeam_channel::{Sender,Receiver};

//---------------------------------------------------------------------------------------------------- Kernel
pub(crate) struct Kernel {
	// GUI Channels.
	to_gui: Sender<KernelToGui>,
	from_gui: Receiver<GuiToKernel>,

	// Search Channels.
	to_search: Sender<KernelToSearch>,
	from_search: Receiver<SearchToKernel>,

	// Audio Channels.
	to_audio: Sender<KernelToAudio>,
	from_audio: Receiver<AudioToKernel>,

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
	pub(crate) fn bios(to_gui: Sender<KernelToGui>, from_gui: Receiver<GuiToKernel>) {
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
	fn boot_loader(collection: Collection, to_gui: Sender<KernelToGui>, from_gui: Receiver<GuiToKernel>) {
		debug!("Kernel [2/12] ... entering boot_loader()");

		// We successfully loaded `Collection`.
		// Create `CCD` channel + thread and make it convert images.
		debug!("Kernel [3/12] ... spawning CCD");
		let (ccd_send, from_ccd) = crossbeam_channel::unbounded::<CcdToKernel>();
		std::thread::spawn(move || Ccd::convert_art(ccd_send, collection));

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
		to_gui:     Sender<KernelToGui>,
		from_gui:   Receiver<GuiToKernel>,
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
		to_gui:     Sender<KernelToGui>,
		from_gui:   Receiver<GuiToKernel>
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
		let (to_search, search_recv) = crossbeam_channel::unbounded::<KernelToSearch>();
		let (to_audio,  audio_recv)  = crossbeam_channel::unbounded::<KernelToAudio>();

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
	fn userspace(mut self) {
		// Array of our channels we can `select` from.
		let mut select = crossbeam_channel::Select::new();
		let gui        = select.recv(&self.from_gui);
		let search     = select.recv(&self.from_search);
		let audio      = select.recv(&self.from_audio);

		// 1) Hang until message is ready.
		// 2) Receive the message and pass to appropriate function.
		// 3) Loop.
		loop {
			match select.ready() {
				i if i == gui    => self.msg_gui(recv!(self.from_gui)),
				i if i == search => self.msg_search(recv!(self.from_search)),
				i if i == audio  => self.msg_audio(recv!(self.from_audio)),
				// TODO: handle this.
				_ => unreachable!(),
			}
		}
	}

	#[inline(always)]
	fn msg_gui(&self, msg: GuiToKernel) {}
	#[inline(always)]
	fn msg_search(&self, msg: SearchToKernel) {}
	#[inline(always)]
	fn msg_audio(&self, msg: AudioToKernel) {}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
