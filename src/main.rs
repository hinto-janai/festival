mod cli;
mod state;
mod gui;
mod collection;
mod constants;
mod logger;
mod macros;
mod kernel;
mod ccd;

fn main() {
	// Handle CLI arguments.
	crate::cli::Cli::handle_args();

	// Run eframe+egui app.
	eframe::run_native(
		crate::constants::FESTIVAL_NAME_VER,
		gui::app::App::options(),
		Box::new(|cc| Box::new(gui::app::App::new(cc)))
	);
}
