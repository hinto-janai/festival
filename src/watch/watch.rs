//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
use serde::{Serialize,Deserialize};
use benri::{
	log::*,
	sync::*,
};
use disk::Empty;
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
		if let Err(e) = Toggle::rm()  { error!("Watch - Toggle: {}", e); }
		if let Err(e) = Stop::rm()    { error!("Watch - Stop: {}", e); }
		if let Err(e) = Play::rm()    { error!("Watch - Play: {}", e); }
		if let Err(e) = Next::rm()    { error!("Watch - Next: {}", e); }
		if let Err(e) = Last::rm()    { error!("Watch - Last: {}", e); }
		if let Err(e) = Shuffle::rm() { error!("Watch - Shuffle: {}", e); }
		if let Err(e) = Repeat::rm()  { error!("Watch - Repeat: {}", e); }
	}

	#[inline(always)]
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
			if Toggle::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Toggle);
			}

			// Stop/Play.
			//
			// `Stop` will always take priority
			// if both `Stop` and `Play` files exist.
			if Stop::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Stop);
			} else if Play::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Play);
			}

			// Next/Last.
			//
			// `Next` takes priority.
			if Next::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Next);
			} else if Last::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Last);
			}

			// Shuffle.
			if Shuffle::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Shuffle);
			}

			// Repeat.
			if Repeat::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Repeat);
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
