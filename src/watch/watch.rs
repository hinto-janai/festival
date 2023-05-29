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
use crossbeam::channel::{
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
	// Kernel starts `Audio` with this.
	pub(crate) fn init(to_kernel: Sender<WatchToKernel>) {
		Self::clean();

		// Get PATH.
		let path = match Play::base_path() {
			Ok(p) => {
				trace!("Watch - Watching PATH: {}", p.display());
				p
			},
			Err(e) => {
				error!("Watch - Failed to get PATH. Signals will be ignored: {e}");
				return;
			},
		};

		// Set up watcher.
		let (tx, from_notify) = std::sync::mpsc::channel();
		let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
			Ok(w) => w,
			Err(e) => {
				error!("Watch - Failed to create watcher. Signals will be ignored: {e}");
				return;
			},
		};

		// Add PATH to watcher.
		if let Err(e) = watcher.watch(&path, RecursiveMode::NonRecursive) {
			error!("Watch - Failed to watch. Signals will be ignored: {e}");
			return;
		}

		// Create self.
		let watch = Self {
			to_kernel,
			from_notify,
		};

		ok_debug!("Watch");
		Self::main(watch)
	}

	#[inline(always)]
	// Remove all files (if they exist), log errors.
	// Make sure the directory exists.
	fn clean() {
		// Create base directory.
		if let Err(e) = Pause::mkdir() { error!("Watch - Could not create signal folder"); }

		// Clean files.
		if let Err(e) = Toggle::rm()   { error!("Watch - Toggle: {}", e); }
		if let Err(e) = Pause::rm()    { error!("Watch - Pause: {}", e); }
		if let Err(e) = Play::rm()     { error!("Watch - Play: {}", e); }
		if let Err(e) = Next::rm()     { error!("Watch - Next: {}", e); }
		if let Err(e) = Previous::rm() { error!("Watch - Previous: {}", e); }
		if let Err(e) = Shuffle::rm()  { error!("Watch - Shuffle: {}", e); }
		if let Err(e) = Repeat::rm()   { error!("Watch - Repeat: {}", e); }
	}

	fn main(self) {
		use notify::event::{EventKind,CreateKind};

		loop {
			// Wait for a change in the filesystem.
			// We only care if it was a file creation.
			loop {
				if let Ok(Ok(event)) = self.from_notify.recv() {              // If we got a msg...
					if let EventKind::Create(CreateKind::File) = event.kind { // and it was a `Create` and it was a `File`...
						break                                                 // break, and check files.
					}
				}
			}

			// Toggle.
			if Toggle::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Toggle);
			}

			// Pause/Play.
			//
			// `Pause` will always take priority
			// if both `Pause` and `Play` files exist.
			if Pause::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Pause);
			} else if Play::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Play);
			}

			// Next/Prev.
			//
			// `Next` takes priority.
			if Next::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Next);
			} else if Previous::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Previous);
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
