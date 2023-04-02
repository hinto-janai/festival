//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use crate::data::{
	State,
	Settings,
};
use benri::{
	log::*,
	sync::*,
	thread::*,
};
use log::{
	info,
	error,
};
use shukusai::kernel::{
	FrontendToKernel,
	KernelToFrontend,
	KernelState,
};
use disk::Toml;
use crate::data::Gui;
use crossbeam_channel::{
	Sender,
	Receiver,
};
use std::time::Duration;
use rolock::RoLock;

//---------------------------------------------------------------------------------------------------- Gui::exit() - The thread that handles exiting.
impl Gui {
#[inline(always)]
pub(super) fn exit(
	to_kernel: Sender<FrontendToKernel>,
	from_kernel: Receiver<KernelToFrontend>,
	state: State,
	settings: Settings,
	kernel_state: RoLock<KernelState>,
) {
	// Tell `Kernel` to save stuff.
	send!(to_kernel, FrontendToKernel::Exit);

	// Save `State`.
	match state.save() {
		Ok(_)  => ok!("GUI - State save"),
		Err(e) => fail!("GUI - State save: {}", e),
	}

	// Save `Settings`.
	match settings.save() {
		Ok(_)  => ok!("GUI - Settings save"),
		Err(e) => fail!("GUI - Settings save: {}", e),
	}

	// Check if `Kernel` succeeded.
	// Loop through 3 messages just in-case
	// there were others in the channel queue.
	//
	// This waits a max `900ms` before
	// continuing without the response.
	let mut n = 0;
	loop {
		if let Ok(KernelToFrontend::Exit(r)) = from_kernel.recv_timeout(Duration::from_millis(300)) {
			match r {
				Ok(_)  => ok!("GUI - Kernel save"),
				Err(e) => fail!("GUI - Kernel save failed: {}", e),
			}
			break
		} else if n > 3 {
			fail!("GUI - Could not determine Kernel's exit result");
		} else {
			n += 1;
		}
	}

	// Wait max `10` seconds if a
	// `Collection` is being saved.
	let mut n = 0;
	loop {
		if n == 10 {
			// Exit with error.
			error!("GUI - Collection save is taking more than 10 seconds, skipping save...!");
			std::process::exit(1);
		}

		if lock_read!(kernel_state).saving {
			n += 1;
			info!("GUI - Waiting to Collection to be saved...");
			sleep!(1000);
		} else {
			// Exit.
			std::process::exit(0);
		}
	}
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
