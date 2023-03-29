//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use crate::macros::{
	fail,
	ok_debug,
	send_or_die,
	ok_trace,
};
use disk::prelude::*;
use disk::{Plain, plain_file};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::constants::FESTIVAL;
use crossbeam_channel::{
	Sender,
};
use super::msg::WatchToKernel;
use notify::{
	Watcher,
	RecommendedWatcher,
	RecursiveMode,
	Config,
	Event,
};
use crate::kernel::Kernel;
use crate::signal::*;
use disk;

//---------------------------------------------------------------------------------------------------- Watch
#[derive(Debug)]
pub(crate) struct Watch {
	// Channel to `Kernel`.
	to_kernel: Sender<WatchToKernel>,
	// Channel from `notify`.
	from_notify: std::sync::mpsc::Receiver<Result<Event, notify::Error>>,
}

impl Watch {
	#[inline(always)]
	// Kernel starts `Audio` with this.
	pub(crate) fn init(to_kernel: Sender<WatchToKernel>) {
		Self::clean();

		// Get PATH.
		let path = match Play::base_path() {
			Ok(p) => {
				debug!("Watch - Watching PATH: {}", p.display());
				p
			},
			Err(e) => {
				fail!("Watch - {}", e);
				error!("Watch - Failed to get PATH. Signals will be ignored!");
				panic!("{e}");
			},
		};

		// Set up watcher.
		let (tx, from_notify) = std::sync::mpsc::channel();
		let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
			Ok(w) => w,
			Err(e) => {
				fail!("Watch - {}", e);
				error!("Watch - Failed to create watcher. Signals will be ignored!");
				panic!("{e}");
			},
		};

		// Add PATH to watcher.
		if let Err(e) = watcher.watch(&path, RecursiveMode::NonRecursive) {
			fail!("Watch - {}", e);
			error!("Watch - Failed to watch. Signals will be ignored!");
			panic!("{e}");
		}

		// Create self.
		let watch = Self {
			to_kernel,
			from_notify,
		};

		Self::main(watch)
	}

	#[inline(always)]
	// Remove all files (if they exist), log errors.
	// Make sure the directory exists.
	fn clean() {
		// Create base directory.
		if let Err(e) = Stop::mkdir() { error!("Watch - Could not create signal folder"); }

		// Clean files.
		if let Err(e) = Stop::remove()    { info!("Watch - Stop: {}", e); }
		if let Err(e) = Play::remove()    { info!("Watch - Play: {}", e); }
		if let Err(e) = Next::remove()    { info!("Watch - Next: {}", e); }
		if let Err(e) = Last::remove()    { info!("Watch - Last: {}", e); }
		if let Err(e) = Shuffle::remove() { info!("Watch - Shuffle: {}", e); }
		if let Err(e) = Repeat::remove()  { info!("Watch - Repeat: {}", e); }
	}

	#[inline(always)]
	// Since `Watch` dying isn't _that_ bad, instead of calling
	// `mass_panic!()` on failures and killing everything, we'll
	// just call `send_or_die!()` which only panics `Watch` itself.
	fn main(self) {
		ok_debug!("Watch");

		loop {
			// Wait for a change in the filesystem.
			// We only care if it was a file creation.
			loop {
				if let Ok(Ok(event)) = self.from_notify.recv() {                 // If we got a msg...
					if let notify::event::EventKind::Create(kind) = event.kind { // and it was a `Create`...
						if let notify::event::CreateKind::File = kind {          // and it was a `File`...
							break                                                // break, and check files.
						}
					}
				}
			}

			// Toggle.
			if let Ok(true) = Toggle::exists() {
				send_or_die!(self.to_kernel, WatchToKernel::Toggle)
			}

			// Stop/Play.
			//
			// `Stop` will always take priority
			// if both `Stop` and `Play` files exist.
			if let Ok(true) = Stop::exists() {
				send_or_die!(self.to_kernel, WatchToKernel::Stop)
			} else if let Ok(true) = Play::exists() {
				send_or_die!(self.to_kernel, WatchToKernel::Play)
			}

			// Next/Last.
			//
			// `Next` takes priority.
			if let Ok(true) = Next::exists() {
				send_or_die!(self.to_kernel, WatchToKernel::Next)
			} else if let Ok(true) = Last::exists() {
				send_or_die!(self.to_kernel, WatchToKernel::Last)
			}

			// Shuffle.
			if let Ok(true) = Shuffle::exists() {
				send_or_die!(self.to_kernel, WatchToKernel::Shuffle)
			}

			// Repeat.
			if let Ok(true) = Repeat::exists() {
				send_or_die!(self.to_kernel, WatchToKernel::Repeat)
			}

			// Clean folder.
			Self::clean();
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
