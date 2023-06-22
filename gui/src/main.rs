#![allow(
	clippy::len_zero,
	clippy::type_complexity,
	clippy::module_inception,

	// Should be cleaned up after v1.0.0.
	dead_code,
	unused_variables,
	unused_imports,
)]

// Hide console in Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod constants;
mod data;
mod func;
mod text;
mod ui;

fn main() {
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

	// Start `egui/eframe`.
	if let Err(e) = eframe::run_native(
		shukusai::constants::FESTIVAL,
		crate::data::Gui::options(),
		Box::new(move |cc| {
			// Set `Festival`'s `GUI_CONTEXT`.
			shukusai::frontend::egui::GUI_CONTEXT
				.set(cc.egui_ctx.clone())
				.expect("GUI_CONTEXT.set() failed");

			let (to_kernel, from_kernel) = match shukusai::kernel::Kernel::spawn(!disable_watch, !disable_media_controls) {
				Ok((to, from)) => (to, from),
				Err(e)         => panic!("Kernel::spawn() failed: {e}"),
			};

			// Start `GUI`.
			Box::new(crate::data::Gui::init(cc, to_kernel, from_kernel))
		})
	) {
		panic!("eframe::run_native() failed: {e}");
	}
}
