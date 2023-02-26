//---------------------------------------------------------------------------------------------------- Use
use clap::Parser;
use log::info;
use crate::constants::{
	FESTIVAL_VERSION,
	COMMIT,
	COPYRIGHT,
};

//---------------------------------------------------------------------------------------------------- CLI Parser (clap)
#[derive(Parser)]
#[command(override_usage = "festival [OPTIONS]")]
pub struct Cli {
	/// Toggle playback (play/pause)
    #[arg(short, long)]
	toggle: bool,
	/// Stop playback
    #[arg(short, long)]
	stop: bool,
	/// Skip to next track
    #[arg(short, long)]
	next: bool,
	/// Play previous track
    #[arg(short, long)]
	previous: bool,
	/// Set filter level for console logs
    #[arg(short, long, value_name = "TRACE|DEBUG|WARN|INFO|ERROR")]
    log_level: Option<log::LevelFilter>,
	/// Print version
    #[arg(short, long)]
	version: bool,
}

//---------------------------------------------------------------------------------------------------- CLI argument handling
impl Cli {
	#[inline(always)]
	pub fn handle_args() {
		let cli = Self::parse();

		// Version
		if cli.version {
			println!("Festival {} {}\n{}", FESTIVAL_VERSION, COMMIT, COPYRIGHT);
			std::process::exit(88);
		}

		// Logger
		match cli.log_level {
			Some(log_level) => crate::logger::init_logger(log_level),
			None            => crate::logger::init_logger(log::LevelFilter::Info),
		}

	}
}
