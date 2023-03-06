//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
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
		Self::remove_all();

		Self::main(to_kernel)
	}

	#[inline(always)]
	// Remove all files, log errors (file probably doesn't exist).
	fn remove_all() {
		if let Err(e) = Stop::remove()    { info!("Watch | Stop: {}", e); }
		if let Err(e) = Play::remove()    { info!("Watch | Play: {}", e); }
		if let Err(e) = Next::remove()    { info!("Watch | Next: {}", e); }
		if let Err(e) = Last::remove()    { info!("Watch | Last: {}", e); }
		if let Err(e) = Shuffle::remove() { info!("Watch | Shuffle: {}", e); }
		if let Err(e) = Repeat::remove()  { info!("Watch | Repeat: {}", e); }
	}

	#[inline(always)]
	fn main(to_kernel: Sender<WatchToKernel>) {
		// Stop/Play.
		//
		// `Stop` will always take priority
		// if both `Stop` and `Play` files exist.
		if let Ok(true) = Stop::exists() {
			// TODO: send signal
		} else if let Ok(true) = Play::exists() {
		}

		// Next/Last.
		//
		// `Next` takes priority.
		if let Ok(true) = Next::exists() {
		} else if let Ok(true) = Last::exists() {
		}

		// Shuffle.
		if let Ok(true) = Shuffle::exists() {
		}

		// Repeat.
		if let Ok(true) = Repeat::exists() {
		}

		// Clean folder.
		Self::remove_all();
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
