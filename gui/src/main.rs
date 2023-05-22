#![allow(
	clippy::len_zero,
	clippy::type_complexity,
	clippy::module_inception,

	// Should be cleaned up after v1.0.0.
	dead_code,
	unused_variables,
	unused_imports,
)]

mod cli;
mod constants;
mod data;
mod func;
mod text;
mod slice;
mod ui;

fn main() {
	// Set `umask` (`rwxr-x---`)
	disk::umask(0o027);

	// Handle CLI arguments.
	crate::cli::Cli::handle_args();

	// Start `egui/eframe`.
	if let Err(e) = eframe::run_native(
		shukusai::FESTIVAL_NAME_VER,
		crate::data::Gui::options(),
		Box::new(|cc| {
			// Spawn `Kernel`, pass it `egui::Context`.
			let ctx = cc.egui_ctx.clone();

			let (to_kernel, from_kernel) = match shukusai::kernel::Kernel::spawn(ctx) {
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
