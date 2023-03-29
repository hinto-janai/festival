//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use crate::macros::{
	ok_debug,
	send_or_die,
};
use disk::prelude::*;
use disk::{Plain, plain_file};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::constants::FESTIVAL;
use crossbeam_channel::Sender;
use super::msg::WatchToKernel;

//---------------------------------------------------------------------------------------------------- Signals
macro_rules! impl_signal {
	($type:ident, $file_name:literal) => {
		plain_file!($type, Dir::Data, FESTIVAL, "signal", $file_name);
		#[derive(Copy,Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
		struct $type;
	}
}

impl_signal!(Stop, "stop");
impl_signal!(Play, "play");
impl_signal!(Next, "next");
impl_signal!(Last, "last");
impl_signal!(Shuffle, "shuffle");
impl_signal!(Repeat, "repeat");

//---------------------------------------------------------------------------------------------------- Watch
#[derive(Copy,Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub(crate) struct Watch;

impl Watch {
	#[inline(always)]
	// Kernel starts `Audio` with this.
	pub(crate) fn init(to_kernel: Sender<WatchToKernel>) {
		Self::clean();

		Self::main(to_kernel)
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
	fn main(to_kernel: Sender<WatchToKernel>) {
		ok_debug!("Watch");

		loop {
			// Stop/Play.
			//
			// `Stop` will always take priority
			// if both `Stop` and `Play` files exist.
			if let Ok(true) = Stop::exists() {
				send_or_die!(to_kernel, WatchToKernel::Stop)
			} else if let Ok(true) = Play::exists() {
				send_or_die!(to_kernel, WatchToKernel::Play)
			}

			// Next/Last.
			//
			// `Next` takes priority.
			if let Ok(true) = Next::exists() {
				send_or_die!(to_kernel, WatchToKernel::Next)
			} else if let Ok(true) = Last::exists() {
				send_or_die!(to_kernel, WatchToKernel::Last)
			}

			// Shuffle.
			if let Ok(true) = Shuffle::exists() {
				send_or_die!(to_kernel, WatchToKernel::Shuffle)
			}

			// Repeat.
			if let Ok(true) = Repeat::exists() {
				send_or_die!(to_kernel, WatchToKernel::Repeat)
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
