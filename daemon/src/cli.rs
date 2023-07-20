//---------------------------------------------------------------------------------------------------- Use
use clap::{Args,Parser,Subcommand};
//use crate::text::FESTIVAL_SHUKUSAI_COMMIT;
use shukusai::signal::{
	Volume,Toggle,Pause,Play,Skip,Back,
	Previous,Next,Stop,Shuffle,Index,
	RepeatSong,RepeatQueue,RepeatOff,
	Clear,Seek,SeekForward,SeekBackward,
};
use crate::constants::{
	FESTIVALD_SHUKUSAI_COMMIT,
};
use shukusai::constants::COPYRIGHT;
use disk::Empty;
use std::num::NonZeroUsize;
use disk::{Bincode2, Json, Plain, Toml};
use const_format::formatcp;
use std::process::exit;

//---------------------------------------------------------------------------------------------------- CLI Parser (clap)
#[cfg(windows)]
const BIN: &str = "festivald.exe";
#[cfg(unix)]
const BIN: &str = "festivald";

const USAGE: &str = formatcp!(
r#"{BIN} [OPTIONS] [COMMAND + OPTIONS] [ARGS...]

    Example 1 | Print the PATH used by Festival    | {BIN} --path
    Example 2 | Send play signal to local Festival | {BIN} signal --play
    Example 3 | Set log level and send a signal    | {BIN} --log-level debug signal --index 1"#);

#[derive(Parser)]
// Clap puts a really ugly non-wrapping list
// of possible args if this isn't set.
#[command(override_usage = USAGE)]
pub struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,

	#[arg(long, verbatim_doc_comment)]
	/// The IPv4 address `festivald` will bind to [default: 127.0.0.1]
	ip: Option<std::net::Ipv4Addr>,

	#[arg(long, verbatim_doc_comment)]
	/// The port `festivald` will bind to [default: 18425]
	port: Option<u16>,

	#[arg(long, verbatim_doc_comment)]
	/// Threads used for JSON-RPC requests [default: all]
	///
	/// `0` will spawn as many system threads you have.
	threads_rpc: Option<usize>,

	#[arg(long, verbatim_doc_comment)]
	/// Threads used for REST requests [default: all]
	///
	/// `0` will spawn as many system threads you have.
	threads_rest: Option<usize>,

	#[arg(long, verbatim_doc_comment)]
	/// Password required for all requests [default: disabled]
	///
	/// Only process `festivald` connections to
	/// `festivald` that have this password.
	/// Empty password disables this feature.
	/// By default, this is sent in clear-text via HTTP.
	/// Do not expect this to be anything more
	/// than a small security measure.
	password: Option<String>,

	#[arg(long, verbatim_doc_comment)]
	/// Print the PATHs used by Festival
	///
	/// All data saved by Festival is saved in these directories.
	/// For more information, see:
	/// https://github.com/hinto-janai/festival/tree/main/daemon#Disk
	path: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Print JSON metadata about the current `Collection` on disk
	///
	/// WARNING:
	/// This output is not meant to be relied on (yet).
	///
	/// It it mostly for quick displaying and debugging
	/// purposes and may be changed at any time.
	///
	/// This flag will attempt to parse the `Collection` that
	/// is currently on disk and extract the metadata from it.
	///
	/// This also means the entire `Collection` will be read
	/// and deserialized from disk, which may be very expensive
	/// if you have a large `Collection`.
	metadata: bool,

	#[arg(long, verbatim_doc_comment, default_value_t = false)]
	/// Disable watching the filesystem for signals
	///
	/// The way a newly launched Festival communicates to
	/// an already existing one (e.g, `festivald --play`) is
	/// by creating a file in Festival's `signal` directory.
	///
	/// `festivald --FLAG` just creates a file in that directory,
	/// which an existing Festival will notice and do the appropriate task.
	///
	/// Using `--disable-watch` will disable that part of the system so that
	/// filesystem signals won't work, e.g, `festivald --play` will not work.
	disable_watch: bool,

	#[arg(long, verbatim_doc_comment, default_value_t = false)]
	/// Disable OS media controls
	///
	/// Festival plugs into the native OS's media controls so that signals
	/// like `play/pause/stop` and/or keyboard controls can be processed.
	///
	/// `--disable-media-controls` disables this.
	disable_media_controls: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Disable the REST API
	///
	/// This is responsible for the `/rest` API that
	/// serves image, audio, and other heavy resource data.
	///
	/// Setting this to `false` will disable this part
	/// of the system, and will only leave the JSON-RPC
	/// API available.
	disable_rest: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Delete all Festival files that are currently on disk
	///
	/// This deletes all `daemon` Festival folder, which contains:
	/// - The `Collection`
	/// - `daemon` configuration (`festivald.toml`)
	/// - Audio state (currently playing song, queue, etc)
	/// - Cached images for the OS media controls
	///
	/// The PATH deleted will be printed on success.
	delete: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Only print logs for `festivald`
	///
	/// Logs that aren't directly from `festivald`, e.g
	/// anything related to the internals, `shukusai`,
	/// will not be printed if this is enabled.
	log_daemon_only: bool,

	#[arg(long, value_name = "OFF|ERROR|INFO|WARN|DEBUG|TRACE")]
	#[arg(default_value_t = log::LevelFilter::Info)]
	/// Set filter level for console logs
	log_level: log::LevelFilter,

	#[arg(short, long)]
	/// Print version
	version: bool,
}

//---------------------------------------------------------------------------------------------------- Subcommands
#[derive(Subcommand)]
pub enum Commands {
	#[command(verbatim_doc_comment)]
	/// Send a signal to a `festivald` running on the same machine
	///
	/// This will not start a new `festivald`, but send a
	/// signal to an already running one. This only works
	/// if there's a `festivald` already running on the
	/// same machine.
	///
	/// The flag `--disable-media-controls` disables this feature.
	Signal(Signal),
}

const USAGE_SIGNAL: &str = formatcp!("{BIN} signal OPTION [ARG]");

#[derive(Args)]
#[command(override_usage = USAGE_SIGNAL)]
pub struct Signal {
	#[arg(long)]
	/// Start playback
	play: bool,

	#[arg(long)]
	/// Pause playback
	pause: bool,

	#[arg(long)]
	/// Toggle playback (play/pause)
	toggle: bool,

	#[arg(long)]
	/// Skip to next track
	next: bool,

	#[arg(long)]
	/// Play previous track
	previous: bool,

	#[arg(long)]
	/// Clear queue and stop playback
	stop: bool,

	#[arg(long)]
	/// Clear queue but don't stop playback
	clear: bool,

	#[arg(long)]
	/// Shuffle the current queue and reset to the first song
	shuffle: bool,

	#[arg(long)]
	/// Turn on single `Song` track repeat
	repeat_song: bool,

	#[arg(long)]
	/// Turn on queue repeat
	repeat_queue: bool,

	#[arg(long)]
	/// Turn off repeating
	repeat_off: bool,

	#[arg(long)]
	#[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
	/// Set the volume to `VOLUME` (0-100)
	volume: Option<u8>,

	#[arg(long)]
	/// Seek to the absolute `SEEK` second in the current song
	seek: Option<u64>,

	#[arg(long)]
	/// Seek `SEEK_FORWARD` seconds forwards in the current song
	seek_forward: Option<u64>,

	#[arg(long)]
	/// Seek `SEEK_BACKWARD` seconds backwards in the current song
	seek_backward: Option<u64>,

	#[arg(long, verbatim_doc_comment)]
	/// Set the current song to the index `INDEX` in the queue.
	///
	/// NOTE:
	/// The queue index starts from 1 (first song is `--index 1`).
	///
	/// Providing an index that is out-of-bounds
	/// will end the queue (even if repeat is turned on).
	index: Option<NonZeroUsize>,

	#[arg(long, verbatim_doc_comment)]
	/// Skip `SKIP` amount of songs
	///
	/// If the last song in the queue is skipped over,
	/// and queue repeat is turned on, this will reset
	/// the current song to the 1st in the queue.
	skip: Option<usize>,

	#[arg(long, verbatim_doc_comment)]
	/// Go backwards in the queue by `BACK` amount of songs
	///
	/// If `BACK` is greater than the amount of songs we can
	/// skip backwards, this will reset the current song to
	/// the 1st in the queue.
	back: Option<usize>,
}

//---------------------------------------------------------------------------------------------------- CLI argument handling
impl Cli {
	#[inline(always)]
	pub fn get() -> (bool, bool, log::LevelFilter) {
		Self::parse().handle_args()
	}

	#[inline(always)]
	pub fn handle_args(self) -> (bool, bool, log::LevelFilter) {
		// Version.
		if self.version {
			println!("{FESTIVALD_SHUKUSAI_COMMIT}\n{COPYRIGHT}");
			exit(0);
		}

		// Metadata.
		if self.metadata {
			match shukusai::collection::metadata() {
				Ok(md) => { println!("{md}"); exit(0); },
				Err(e) => { eprintln!("festival error: {e}"); exit(1); },
			}
		}

		// Path.
		if self.path {
			// SAFETY:
			// If we can't get a PATH, `panic!()`'ing is fine.
			let p = crate::config::Config::sub_dir_parent_path().unwrap();
			println!("{}", p.display());

			let p = shukusai::collection::Collection::sub_dir_parent_path().unwrap();
			println!("{}", p.display());

			exit(0);
		}

//		// Delete.
//		if self.delete {
//			// SAFETY:
//			// If we can't get a PATH, `panic!()`'ing is fine.
//			let p = crate::data::State::sub_dir_parent_path().unwrap();
//			match crate::data::State::rm_sub() {
//				Ok(md) => { println!("{}", md.path().display()); exit(0); },
//				Err(e) => { eprintln!("festival error: {} - {e}", p.display()); exit(1); },
//			}
//		}
//
		// Return.
		(self.disable_watch, self.disable_media_controls, self.log_level)
	}

	#[inline(always)]
	pub fn handle_signal(s: Signal) -> ! {
		fn handle<T>(result: Result<T, anyhow::Error>) {
			if let Err(e) = result {
				eprintln!("{BIN} error: {e}");
				exit(1);
			} else {
				exit(0);
			}
		}

		// Signals.
		if s.toggle       { handle(Toggle::touch())      }
		if s.pause        { handle(Pause::touch())       }
		if s.play         { handle(Play::touch())        }
		if s.next         { handle(Next::touch())        }
		if s.previous     { handle(Previous::touch())    }
		if s.stop         { handle(Stop::touch())        }
		if s.clear        { handle(Clear(true).save())   }
		if s.shuffle      { handle(Shuffle::touch())     }
		if s.repeat_song  { handle(RepeatSong::touch())  }
		if s.repeat_queue { handle(RepeatQueue::touch()) }
		if s.repeat_off   { handle(RepeatOff::touch())   }

		// Content signals.
		if let Some(volume) = s.volume        { handle(Volume(shukusai::audio::Volume::new(volume)).save()) }
		if let Some(seek)   = s.seek          { handle(Seek(seek).save())          }
		if let Some(seek)   = s.seek_forward  { handle(SeekForward(seek).save())   }
		if let Some(seek)   = s.seek_backward { handle(SeekBackward(seek).save())  }
		if let Some(index)  = s.index         { handle(Index(index.into()).save()) }
		if let Some(skip)   = s.skip          { handle(Skip(skip).save())          }
		if let Some(back)   = s.back          { handle(Back(back).save())          }

		exit(0);
	}
}
