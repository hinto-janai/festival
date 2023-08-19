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

	// Set `umask` (`rwxr-x---`)
	disk::umask(0o027);

	// Init config.
	let CONFIG: &'static crate::config::Config = {
		// Start config construction.
		// INVARIANT: Logger gets set here.
		let mut config_builder: crate::config::ConfigBuilder = crate::config::ConfigBuilder::file_or_and_init_logger(log);

		// Merge disk config with command-line config.
		if let Some(mut config_cmd) = config_cmd {
			config_builder.merge(&mut config_cmd);
		}

		// INVARIANT: Initialize `CONFIG`. This must be set, and once only.
		config_builder.build_and_set()
	};

	// Exit early if `dry_run`.
	if dry_run {
		println!("{}", serde_json::to_string_pretty(CONFIG).unwrap());
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

	// Tell `Kernel` to cache directories.
	benri::send!(TO_KERNEL, shukusai::kernel::FrontendToKernel::CachePath(CONFIG.collection_paths.clone()));
	// Tell `Kernel` to restore audio state.
	if CONFIG.restore_audio_state {
		benri::send!(TO_KERNEL, shukusai::kernel::FrontendToKernel::RestoreAudioState);
	}

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

	// Cleanup cache.
	if CONFIG.cache_clean {
		match crate::zip::clean_cache() {
			Ok(_)  => benri::ok!("festivald ... Cache clean"),
			Err(e) => log::warn!("festivald ... Could not clean cache: {e}"),
		}
	} else {
		log::info!("festivald ... Skipping cache clean");
	}

	// Start HTTP router.
	crate::router::init(CONFIG, TO_KERNEL, FROM_KERNEL);
}
