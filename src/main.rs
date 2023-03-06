mod audio;
mod ccd;
mod cli;
mod collection;
mod constants;
mod gui;
mod kernel;
mod logger;
mod macros;
mod search;
mod watch;

fn main() {
	// Handle CLI arguments.
	cli::Cli::handle_args();

	// Create `Kernel` <-> `GUI` channels.
	let (kernel_to_gui, gui_recv)    = crossbeam_channel::unbounded::<crate::gui::KernelToGui>();
	let (gui_to_kernel, kernel_recv) = crossbeam_channel::unbounded::<crate::gui::GuiToKernel>();

	// Spawn `Kernel`.
	std::thread::spawn(move || kernel::Kernel::bios(kernel_to_gui, kernel_recv));

	// Start `GUI`.
	eframe::run_native(
		constants::FESTIVAL_NAME_VER,
		gui::Gui::options(),
		Box::new(|cc| Box::new(gui::Gui::init(cc, gui_to_kernel, gui_recv)))
	);
}
