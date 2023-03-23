mod album;
mod artist;
mod cli;
mod constants;
mod gui;
mod queue;
mod search;
mod settings;
mod tab;

fn main() {
	// Handle CLI arguments.
	cli::Cli::handle_args();

	// Create `Kernel` <-> `GUI` channels.
	let (kernel_to_gui, gui_recv)    = crossbeam_channel::unbounded::<shukusai::KernelToFrontend>();
	let (gui_to_kernel, kernel_recv) = crossbeam_channel::unbounded::<shukusai::FrontendToKernel>();

	// Spawn `Kernel`.
	std::thread::spawn(move || shukusai::Kernel::bios(kernel_to_gui, kernel_recv));

	// Start `GUI`.
	eframe::run_native(
		shukusai::FESTIVAL_NAME_VER,
		gui::Gui::options(),
		Box::new(|cc| Box::new(gui::Gui::init(cc, gui_to_kernel, gui_recv)))
	);
}
