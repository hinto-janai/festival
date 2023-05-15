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
use disk::{Bincode2,Toml,Json};
use crate::data::Gui;
use crossbeam::channel::{
	Sender,
	Receiver,
};
use std::time::Duration;
use rolock::RoLock;
use std::sync::{
	Arc,
	atomic::AtomicU8,
};
use crate::constants::EXIT_COUNTDOWN;

//---------------------------------------------------------------------------------------------------- Gui::exit() - The thread that handles exiting.
impl Gui {
#[inline(always)]
pub(super) fn exit(
	to_kernel: Sender<FrontendToKernel>,
	from_kernel: Receiver<KernelToFrontend>,
	state: State,
	settings: Settings,
	kernel_state: RoLock<KernelState>,
	exit_countdown: Arc<AtomicU8>,
) {
	// Tell `Kernel` to save stuff.
	send!(to_kernel, FrontendToKernel::Exit);

	// Save `State`.
	match state.save() {
		Ok(o)  => ok!("GUI - State save: {o}"),
		Err(e) => fail!("GUI - State save: {e}"),
	}

	// Save `Settings`.
	match settings.save() {
		Ok(o)  => ok!("GUI - Settings save: {o}"),
		Err(e) => fail!("GUI - Settings save: {e}"),
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

	// Wait until `Collection` is saved,
	// or until we've elapsed total time.
	atomic_store!(exit_countdown, EXIT_COUNTDOWN);
	loop {
		let e = atomic_load!(exit_countdown);

		if e == 0 {
			// Exit with error.
			error!("GUI - Collection save is taking more than {e} seconds, skipping save...!");
			std::process::exit(1);
		}

		if lockr!(kernel_state).saving {
			atomic_sub!(exit_countdown, 1);
			info!("GUI - Waiting to Collection to be saved, force exit in [{e}] seconds");
			sleep!(1);
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
