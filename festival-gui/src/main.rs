mod cli;
mod constants;
mod data;
mod text;
mod ui;

fn main() {
	// Handle CLI arguments.
	cli::Cli::handle_args();

	// Create `Kernel` <-> `GUI` channels.
	let (kernel_to_gui, gui_recv)    = crossbeam_channel::unbounded::<shukusai::kernel::KernelToFrontend>();
	let (gui_to_kernel, kernel_recv) = crossbeam_channel::unbounded::<shukusai::kernel::FrontendToKernel>();

	// Spawn `Kernel`.
	std::thread::spawn(move || shukusai::kernel::Kernel::bios(kernel_to_gui, kernel_recv));

	// Start `GUI`.
	eframe::run_native(
		shukusai::FESTIVAL_NAME_VER,
		data::Gui::options(),
		Box::new(|cc| Box::new(data::Gui::init(cc, gui_to_kernel, gui_recv)))
	).expect("eframe::run_native() failed");
}
