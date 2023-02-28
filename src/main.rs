mod cli;
mod gui;
mod collection;
mod constants;
mod logger;
mod macros;
mod kernel;
mod ccd;
mod search;
mod audio;

fn main() {
	// Handle CLI arguments.
	crate::cli::Cli::handle_args();

	// Run eframe+egui GUI.
	eframe::run_native(
		crate::constants::FESTIVAL_NAME_VER,
		gui::Gui::options(),
		Box::new(|cc| Box::new(gui::Gui::new(cc)))
	);
}
