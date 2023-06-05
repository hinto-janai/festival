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
	/// Start playback
	#[arg(long)]
	play: bool,

	/// Pause playback
	#[arg(long)]
	pause: bool,

	/// Toggle playback (play/pause)
	#[arg(long)]
	toggle: bool,

	/// Skip to next track
	#[arg(long)]
	next: bool,

	/// Play previous track
	#[arg(long)]
	previous: bool,

	/// Turn on track shuffle
	#[arg(long)]
	shuffle_on: bool,

	/// Turn off track shuffle
	#[arg(long)]
	shuffle_off: bool,

	/// Toggle track shuffle
	#[arg(long)]
	shuffle_toggle: bool,

	/// Turn on single `Song` track repeat
	#[arg(long)]
	repeat_song: bool,

	/// Turn on queue repeat
	#[arg(long)]
	repeat_queue: bool,

	/// Turn off repeating
	#[arg(long)]
	repeat_off: bool,

	/// Set the volume to `VOLUME` (0..=100)
	#[arg(long)]
	#[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
	volume: Option<u8>,

	/// Seek to the `SEEK` second in the current song
	#[arg(long)]
	seek: Option<usize>,

	/// Skip `SKIP` amount of songs
	#[arg(long)]
	skip: Option<usize>,

	/// Skip `SKIP` amount of songs, backwards
	#[arg(long)]
	back: Option<usize>,

	/// Print JSON metadata about the current `Collection` on disk
	#[arg(long)]
	metadata: bool,

	/// Set filter level for console logs
	#[arg(long, value_name = "OFF|ERROR|INFO|WARN|DEBUG|TRACE")]
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
			exit(0);
		}

		// Metadata.
		if cli.metadata {
			match shukusai::collection::metadata() {
				Ok(md) => { println!("{md}"); exit(0); },
				Err(e) => { println!("ERROR: {e}"); exit(1); },
			}
		}

		// Signals.
		if cli.toggle         { if let Err(e) = Toggle::touch()        { error!("Failed: {e}"); exit(1); } else { exit(0); } }
		if cli.pause          { if let Err(e) = Pause::touch()         { error!("Failed: {e}"); exit(1); } else { exit(0); } }
		if cli.play           { if let Err(e) = Play::touch()          { error!("Failed: {e}"); exit(1); } else { exit(0); } }
		if cli.next           { if let Err(e) = Next::touch()          { error!("Failed: {e}"); exit(1); } else { exit(0); } }
		if cli.previous       { if let Err(e) = Previous::touch()      { error!("Failed: {e}"); exit(1); } else { exit(0); } }
		if cli.shuffle_on     { if let Err(e) = ShuffleOn::touch()     { error!("Failed: {e}"); exit(1); } else { exit(0); } }
		if cli.shuffle_off    { if let Err(e) = ShuffleOff::touch()    { error!("Failed: {e}"); exit(1); } else { exit(0); } }
		if cli.shuffle_toggle { if let Err(e) = ShuffleToggle::touch() { error!("Failed: {e}"); exit(1); } else { exit(0); } }
		if cli.repeat_song    { if let Err(e) = RepeatSong::touch()    { error!("Failed: {e}"); exit(1); } else { exit(0); } }
		if cli.repeat_queue   { if let Err(e) = RepeatQueue::touch()   { error!("Failed: {e}"); exit(1); } else { exit(0); } }
		if cli.repeat_off     { if let Err(e) = RepeatOff::touch()     { error!("Failed: {e}"); exit(1); } else { exit(0); } }

		// Content signals.
		use disk::Plain;
		if let Some(volume) = cli.volume {
			let volume = shukusai::kernel::Volume::new(volume);
			let signal = shukusai::signal::Volume(volume);
			if let Err(e) = signal.save() { error!("Failed: {e}"); exit(1); } else { exit(0); }
		} else if let Some(seek) = cli.seek {
			let signal = shukusai::signal::Seek(seek);
			if let Err(e) = signal.save() { error!("Failed: {e}"); exit(1); } else { exit(0); }
		} else if let Some(skip) = cli.skip {
			let signal = shukusai::signal::Skip(skip);
			if let Err(e) = signal.save() { error!("Failed: {e}"); exit(1); } else { exit(0); }
		} else if let Some(back) = cli.back {
			let signal = shukusai::signal::Back(back);
			if let Err(e) = signal.save() { error!("Failed: {e}"); exit(1); } else { exit(0); }
		}

		// Logger.
		match cli.log_level {
			Some(log_level) => init_logger(log_level),
			None            => init_logger(log::LevelFilter::Info),
		}
	}
}
