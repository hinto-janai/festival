//---------------------------------------------------------------------------------------------------- Use
use shukusai::{
	collection::Collection,
	kernel::{
		FrontendToKernel,KernelToFrontend,
	},
};
use benri::{
	sleep,
	atomic_store,
	ok,send,
};
use log::{info,warn};
use crossbeam::channel::{
	Sender,Receiver,
};
use std::time::Duration;
use std::sync::Arc;

//---------------------------------------------------------------------------------------------------- `shutdown()`
/// Gracefully shutdown `festivald`
pub async fn shutdown(
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
	FROM_KERNEL: &'static Receiver<KernelToFrontend>,
	collection: Arc<Collection>,
) -> ! {
	// If this is the 2nd time, exit forcefully.
	if crate::statics::shutting_down() {
		println!("\n\nfestivald: exiting forcefully...!");
		std::process::exit(1);
	}

	//-------------------------------------------------- Signal to other threads, print message.
	atomic_store!(crate::statics::SHUTTING_DOWN, true);

	println!(
r#"

==========================================================
| Shutdown signal received, starting shutdown routine... |
=========================================================="#);

	//-------------------------------------------------- Wait up to 120 seconds for a potential `Collection` reset.
	for i in 1..=120 {
		if crate::statics::resetting() {
			println!("[....] Waiting for Collection reset to finish [{i}/120]");
			tokio::time::sleep(Duration::from_secs(1)).await;
		} else {
			break;
		}
	}

	//-------------------------------------------------- Check again.
	if crate::statics::resetting() {
		println!("[FAIL] Collection reset wait");
	} else {
		println!("[ OK ] Collection reset wait");
	}

	//-------------------------------------------------- Tell `Kernel` we're exiting.
	send!(TO_KERNEL, FrontendToKernel::Exit);

	//-------------------------------------------------- Cleanup cache.
	if crate::config::config().cache_clean {
		match crate::zip::clean_cache() {
			Ok(_)  => println!("[ OK ] Clean cache"),
			Err(e) => println!("[FAIL] Clean cache ... {e}"),
		}
	} else {
		println!("[SKIP] Clean cache");
	}

	//-------------------------------------------------- Kernel check.
	// Check if `Kernel` succeeded.
	// Loop through messages just in-case
	// there were others in the channel queue.
	//
	// This waits a max `5s` before
	// continuing without the response.
	let mut n = 0;
	loop {
		if let Ok(KernelToFrontend::Exit(r)) = FROM_KERNEL.recv_timeout(Duration::from_secs(1)) {
			match r {
				Ok(_)  => println!("[ OK ] Kernel save"),
				Err(e) => println!("[FAIL] Kernel save ... {e}"),
			}
			break;
		} else if n > 5 {
			println!("[FAIL] Kernel save ... Could not confirm exit result");
		} else {
			n += 1;
		}
	}

	//-------------------------------------------------- Collection check.
	// Wait a little while before forcefully closing connections.
	let mut n = 0;
	const MAX: usize = 15;
	loop {
		let sc = Arc::strong_count(&collection);
		n += 1;

		if n > MAX {
			println!("[FAIL] Forcefully closing connections");
			break;
		}

		if sc > 5 {
			println!("[....] Connections alive ({}), waiting [{n}/{MAX}]", sc - 5);
			tokio::time::sleep(std::time::Duration::from_secs(1)).await;
		} else {
			println!("[ OK ] Connections closed");
			break;
		}
	}

	//-------------------------------------------------- Exit.
	println!("\nfestivald: Total uptime ... {}", readable::Time::from(*shukusai::logger::INIT_INSTANT));
	std::process::exit(0)
}
