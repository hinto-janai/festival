mod cli;
mod constants;
mod data;
mod text;
mod slice;
mod ui;

fn main() {
	// Set `umask`.
	disk::umask(0o027);

	// Handle CLI arguments.
	cli::Cli::handle_args();

	// Create `Kernel` <-> `GUI` channels.
	let (kernel_to_gui, gui_recv)    = crossbeam_channel::unbounded::<shukusai::kernel::KernelToFrontend>();
	let (gui_to_kernel, kernel_recv) = crossbeam_channel::unbounded::<shukusai::kernel::FrontendToKernel>();

	// Start `egui/eframe`.
	eframe::run_native(
		shukusai::FESTIVAL_NAME_VER,
		data::Gui::options(),
		Box::new(|cc| {
			// Spawn `Kernel`, pass it `egui::Context`.
			shukusai::kernel::Kernel::spawn(
				kernel_to_gui,
				kernel_recv,
				cc.egui_ctx.clone()
			).expect("Kernel::spawn() failed");

			// Start `GUI`.
			Box::new(data::Gui::init(cc, gui_to_kernel, gui_recv))
		})
	).expect("eframe::run_native() failed");
}
