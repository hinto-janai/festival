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
use shukusai::{
	kernel::{
		FrontendToKernel,
		KernelToFrontend,
	},
};
use disk::{Bincode2,Toml,Json};
use crate::data::Gui;
use crossbeam::channel::{
	Sender,
	Receiver,
};
use std::time::Duration;
use std::sync::{
	Arc,
	atomic::{AtomicU8,AtomicBool},
};
use crate::data::{
	EXIT_COUNTDOWN,
	SHOULD_EXIT,
};

//---------------------------------------------------------------------------------------------------- Gui::exit() - The thread that handles exiting.
impl Gui {
#[inline(always)]
pub(super) fn exit(
	to_kernel: Sender<FrontendToKernel>,
	from_kernel: Receiver<KernelToFrontend>,
	state: State,
	settings: Settings,
) {
	// Tell `Kernel` to save stuff.
	send!(to_kernel, FrontendToKernel::Exit);

	// Save `State`.
	match state.save() {
		Ok(md) => ok!("GUI - State save: {md}"),
		Err(e) => fail!("GUI - State save: {e}"),
	}

	// Save `Settings`.
	match settings.save() {
		Ok(md) => ok!("GUI - Settings save: {md}"),
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
	loop {
		let e = atomic_load!(EXIT_COUNTDOWN);

		if e == 0 {
			// Exit with error.
			error!("GUI - Collection save is taking more than {e} seconds, skipping save...!");
			break;
		}

		if shukusai::state::saving() {
			atomic_sub!(EXIT_COUNTDOWN, 1);
			info!("GUI - Waiting for Collection to be saved, force exit in [{e}] seconds");
			sleep!(1);
		} else {
			break;
		}
	}

	// FIXME:
	// This used to be `std::process::exit()` but
	// it caused some weird segfaults on certain machines
	// if the main `GUI` thread was not the one calling it.
	//
	// So, use this signal so that `main()` can exit instead.
	atomic_store!(SHOULD_EXIT, true);
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
