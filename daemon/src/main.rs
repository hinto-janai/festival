mod cert;
mod cli;
mod config;
mod constants;
mod hash;
mod statics;
mod rest;
mod rpc;
mod router;
mod tls;

fn main() -> std::process::ExitCode {
	// Handle CLI arguments.
	let (disable_watch, disable_media_controls, log) = {
		if std::env::args_os().len() == 1 {
			(false, false, log::LevelFilter::Info)
		} else {
			crate::cli::Cli::get()
		}
	};

	// Init logger.
	shukusai::logger::init_logger(log);

	// Set `umask` (`rwxr-x---`)
	disk::umask(0o027);

	// Setup `Kernel` <-> `Frontend` channels.
//	let (to_kernel, from_kernel) = match shukusai::kernel::Kernel::spawn(!disable_watch, !disable_media_controls) {
//		Ok((t, f)) => (t, f),
//		Err(e)     => panic!("Kernel::spawn() failed: {e}"),
//	};

	// Start HTTP router.
//	match crate::router::init(to_kernel, from_kernel, Default::default()) {
	match crate::router::init(Default::default()) {
		Ok(_)  => std::process::ExitCode::SUCCESS,
		Err(e) => { eprintln!("festivald error: {e}"); std::process::ExitCode::FAILURE },
	}
}
