//---------------------------------------------------------------------------------------------------- Use
use clap::Parser;
use log::{info,error};
use shukusai::*;
use shukusai::signal::*;
use disk::Empty;

//---------------------------------------------------------------------------------------------------- CLI Parser (clap)
#[derive(Parser, Debug)]
#[command(override_usage = "festival [OPTIONS]")]
pub struct Cli {
	/// Toggle playback (play/pause)
	#[arg(long)]
	toggle: bool,

	/// Stop playback
	#[arg(long)]
	stop: bool,

	/// Start playback
	#[arg(long)]
	play: bool,

	/// Skip to next track
	#[arg(long)]
	next: bool,

	/// Play previous track
	#[arg(long)]
	last: bool,

	/// Play track shuffle
	#[arg(long)]
	shuffle: bool,

	/// Toggle track repeating
	#[arg(long)]
	repeat: bool,

	/// Set filter level for console logs
	#[arg(long, value_name = "TRACE|DEBUG|WARN|INFO|ERROR")]
	log_level: Option<log::LevelFilter>,

	/// Print version
	#[arg(short, long)]
	version: bool,
}

//---------------------------------------------------------------------------------------------------- CLI argument handling
impl Cli {
	#[inline(always)]
	pub fn handle_args() {
		use std::process::exit;

		let cli = Self::parse();

		// Version.
		if cli.version {
			println!("Festival {} {}\n{}", FESTIVAL_VERSION, COMMIT, COPYRIGHT);
			exit(88);
		}

		// Logger.
		match cli.log_level {
			Some(log_level) => init_logger(log_level),
			None            => init_logger(log::LevelFilter::Info),
		}

		// Signals.
		if cli.toggle  { if let Err(e) = Toggle::touch()  { error!("Failed: {}", e); exit(1); } else { exit(0); } }
		if cli.stop    { if let Err(e) = Stop::touch()    { error!("Failed: {}", e); exit(1); } else { exit(0); } }
		if cli.play    { if let Err(e) = Play::touch()    { error!("Failed: {}", e); exit(1); } else { exit(0); } }
		if cli.next    { if let Err(e) = Next::touch()    { error!("Failed: {}", e); exit(1); } else { exit(0); } }
		if cli.last    { if let Err(e) = Last::touch()    { error!("Failed: {}", e); exit(1); } else { exit(0); } }
		if cli.shuffle { if let Err(e) = Shuffle::touch() { error!("Failed: {}", e); exit(1); } else { exit(0); } }
		if cli.repeat  { if let Err(e) = Repeat::touch()  { error!("Failed: {}", e); exit(1); } else { exit(0); } }
	}
}
