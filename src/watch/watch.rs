//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
use serde::{Serialize,Deserialize};
use benri::{
	log::*,
	sync::*,
};
use disk::{Empty, Plain};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::constants::FESTIVAL;
use crossbeam::channel::{
	Sender,Receiver,
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
	from_notify: Receiver<Result<Event, notify::Error>>,
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
		let (tx, from_notify) = crossbeam::channel::unbounded();
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
		if let Err(e) = Toggle::rm()        { error!("Watch - Toggle: {}", e); }
		if let Err(e) = Pause::rm()         { error!("Watch - Pause: {}", e); }
		if let Err(e) = Play::rm()          { error!("Watch - Play: {}", e); }
		if let Err(e) = Next::rm()          { error!("Watch - Next: {}", e); }
		if let Err(e) = Previous::rm()      { error!("Watch - Previous: {}", e); }
		if let Err(e) = Stop::rm()          { error!("Watch - Stop: {}", e); }
		if let Err(e) = Shuffle::rm()       { error!("Watch - Shuffle: {}", e); }
		if let Err(e) = RepeatSong::rm()    { error!("Watch - RepeatSong: {}", e); }
		if let Err(e) = RepeatQueue::rm()   { error!("Watch - RepeatQueue: {}", e); }
		if let Err(e) = RepeatOff::rm()     { error!("Watch - RepeatOff: {}", e); }

		// Content files.
		if let Err(e) = Volume::rm() { error!("Watch - Volume: {}", e); }
		if let Err(e) = Seek::rm()   { error!("Watch - Seek: {}", e); }
		if let Err(e) = Skip::rm()   { error!("Watch - Skip: {}", e); }
		if let Err(e) = Back::rm()   { error!("Watch - Back: {}", e); }
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

			// Stop/Pause/Play.
			//
			// Priority is `Stop` > `Pause` > `Play`.
			if Stop::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Stop);
			} else if Pause::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Pause);
			} else if Play::exists().is_ok() {
				send!(self.to_kernel, WatchToKernel::Play);
			}

			// Next/Prev.
			//
			// These two will cancel each-other
			// out if they both exist.
			let next = Next::exists().is_ok();
			let prev = Previous::exists().is_ok();
			if next && prev {
				debug!("Watch - Next & Previous existed, doing nothing");
			} else if next {
				send!(self.to_kernel, WatchToKernel::Next);
			} else if prev {
				send!(self.to_kernel, WatchToKernel::Previous);
			}

			// Shuffle/Repeat.
			if Shuffle::exists().is_ok()     { send!(self.to_kernel, WatchToKernel::Shuffle); }
			if RepeatSong::exists().is_ok()  { send!(self.to_kernel, WatchToKernel::RepeatSong); }
			if RepeatQueue::exists().is_ok() { send!(self.to_kernel, WatchToKernel::RepeatQueue); }
			if RepeatOff::exists().is_ok()   { send!(self.to_kernel, WatchToKernel::RepeatOff); }

			// Content signals.
			if let Ok(v) = Volume::from_file() { send!(self.to_kernel, WatchToKernel::Volume(v.0)); }
			if let Ok(s) = Skip::from_file()   { send!(self.to_kernel, WatchToKernel::Skip(s.0)); }
			if let Ok(s) = Seek::from_file()   { send!(self.to_kernel, WatchToKernel::Seek(s.0)); }
			if let Ok(s) = Back::from_file()   { send!(self.to_kernel, WatchToKernel::Back(s.0)); }

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
