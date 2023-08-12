mod cert;
mod cli;
mod config;
mod constants;
mod docs;
mod macros;
mod ptr;
mod seen;
mod statics;
mod shutdown;
mod rest;
mod resp;
mod rpc;
mod router;
mod zip;

fn main() {
	// Handle CLI arguments.
	let (dry_run, disable_watch, disable_media_controls, log, config_cmd) = {
		if std::env::args_os().len() == 1 {
			(false, false, false, None, None)
		} else {
			crate::cli::Cli::get()
		}
	};

	// Init logger.
	shukusai::logger::init_logger(log.unwrap_or_else(|| log::LevelFilter::Error));

	// Set `umask` (`rwxr-x---`)
	disk::umask(0o027);

	// We want to read config in dry-run as well but don't want to
	// start `Kernel` since we're exiting anyway, this de-dups the code.
	fn set_config(config_cmd: Option<crate::config::ConfigBuilder>) -> &'static crate::config::Config {
		// Start config construction.
		let mut config_builder: crate::config::ConfigBuilder = crate::config::ConfigBuilder::file_or();

		// Merge disk config with command-line config.
		if let Some(mut config_cmd) = config_cmd {
			config_builder.merge(&mut config_cmd);
		}

		// INVARIANT: Initialize `CONFIG`. This must be set, and once only.
		config_builder.build_and_set()
	}

	// Exit early if `dry_run`.
	if dry_run {
		println!("{}", serde_json::to_string_pretty(set_config(config_cmd)).unwrap());
		std::process::exit(0);
	}

	// Setup `Kernel` <-> `Frontend` channels.
	let (to_kernel, from_kernel) = match shukusai::kernel::Kernel::spawn(!disable_watch, !disable_media_controls) {
		Ok((t, f)) => (t, f),
		Err(e)     => panic!("Kernel::spawn() failed: {e}"),
	};

	// These last forever.
	let TO_KERNEL:   &'static crossbeam::channel::Sender<shukusai::kernel::FrontendToKernel>   = Box::leak(Box::new(to_kernel));
	let FROM_KERNEL: &'static crossbeam::channel::Receiver<shukusai::kernel::KernelToFrontend> = Box::leak(Box::new(from_kernel));
	let CONFIG = set_config(config_cmd);

	// Create documentation.
	if CONFIG.docs {
		match crate::docs::Docs::create() {
			Ok(path)  => {
				// SAFETY: we only set this `OnceCell` here.
				crate::docs::DOCS_PATH.set(path).unwrap();
				benri::ok!("festivald ... Docs");
			}
			Err(e) => log::warn!("festivald ... Could not create docs: {e}"),
		}
	} else {
		log::info!("festivald ... Skipping docs");
	}

	// Tell `Kernel` to cache directories.
	benri::send!(TO_KERNEL, shukusai::kernel::FrontendToKernel::CachePath(CONFIG.collection_paths.clone()));

	// Cleanup cache.
	match crate::zip::clean_cache() {
		Ok(_)  => benri::ok!("festivald ... Cache clean"),
		Err(e) => log::warn!("festivald ... Could not clean cache: {e}"),
	}

	// Start HTTP router.
	crate::router::init(CONFIG, TO_KERNEL, FROM_KERNEL);
}
