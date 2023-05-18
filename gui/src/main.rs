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
	// Set `umask`.
	disk::umask(0o027);

	// Handle CLI arguments.
	crate::cli::Cli::handle_args();

	// Create `Kernel` <-> `GUI` channels.
	let (kernel_to_gui, gui_recv)    = crossbeam::channel::unbounded::<shukusai::kernel::KernelToFrontend>();
	let (gui_to_kernel, kernel_recv) = crossbeam::channel::unbounded::<shukusai::kernel::FrontendToKernel>();

	// Start `egui/eframe`.
	if let Err(e) = eframe::run_native(
		shukusai::FESTIVAL_NAME_VER,
		crate::data::Gui::options(),
		Box::new(|cc| {
			// Spawn `Kernel`, pass it `egui::Context`.
			if let Err(e) = shukusai::kernel::Kernel::spawn(
				kernel_to_gui,
				kernel_recv,
				cc.egui_ctx.clone()
			) {
				panic!("Kernel::spawn() failed: {e}");
			}

			// Start `GUI`.
			Box::new(crate::data::Gui::init(cc, gui_to_kernel, gui_recv))
		})
	) {
		panic!("eframe::run_native() failed: {e}");
	}
}
