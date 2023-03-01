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

fn main() {
	// Handle CLI arguments.
	crate::cli::Cli::handle_args();

	// Create `GUI` <-> `Kernel` channels.
	let (kernel_to_gui, gui_recv)    = std::sync::mpsc::channel::<crate::gui::KernelToGui>();
	let (gui_to_kernel, kernel_recv) = crossbeam_channel::unbounded::<crate::gui::GuiToKernel>();

	// Spawn `Kernel`.
	std::thread::spawn(move || crate::kernel::Kernel::bios(kernel_to_gui, kernel_recv));

	// Start `GUI`.
	eframe::run_native(
		crate::constants::FESTIVAL_NAME_VER,
		gui::Gui::options(),
		Box::new(|cc| Box::new(gui::Gui::init(cc, gui_to_kernel, gui_recv)))
	);
}
