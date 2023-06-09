//---------------------------------------------------------------------------------------------------- Use
use clap::Parser;
use log::{info,error};
use shukusai::{
	init_logger,
	FESTIVAL_VERSION,COMMIT,COPYRIGHT,
};
use shukusai::signal::{
	Volume,Toggle,Pause,Play,Skip,Back,
	Previous,Next,Stop,Shuffle,Index,
	RepeatSong,RepeatQueue,RepeatOff,
	Seek,SeekForward,SeekBackward,
};
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

	/// Clear queue and stop playback
	#[arg(long)]
	stop: bool,

	/// Shuffle the current queue and reset to the first song
	#[arg(long)]
	shuffle: bool,

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

	/// Seek to the absolute `SEEK` second in the current song
	#[arg(long)]
	seek: Option<u64>,

	/// Seek `SEEK_FORWARD` seconds forwards in the current song
	#[arg(long)]
	seek_forward: Option<u64>,

	/// Seek `SEEK_BACKWARD` seconds backwards in the current song
	#[arg(long)]
	seek_backward: Option<u64>,

	/// Set the current song to the index `INDEX` in the queue.
	///
	/// NOTE: The queue index starts from 1 (first song is `--index 1`).
	///
	/// Providing an index that is out-of-bounds will end the queue (even if repeat is turned on).
	#[arg(long)]
	index: Option<usize>,

	/// Skip `SKIP` amount of songs
	#[arg(long)]
	skip: Option<usize>,

	/// Go backwards in the queue by `BACK` amount of songs
	#[arg(long)]
	back: Option<usize>,

	/// Print JSON metadata about the current `Collection` on disk
	#[arg(long)]
	metadata: bool,

	/// Disable watching the filesystem for signals
	///
	/// The way a newly launched Festival communicates to
	/// an already existing one (e.g, `festival --play`) is
	/// by creating a file in Festival's `signal` directory.
	///
	/// `festival --FLAG` just creates a file in that directory,
	/// which an existing Festival will notice and do the appropriate task.
	///
	/// Using `--disable-watch` will disable that part of the system so that
	/// filesystem signals won't work, e.g, `festival --play` will not work.
	#[arg(long)]
	#[arg(default_value_t = false)]
	disable_watch: bool,

	/// Disable OS media controls
	#[arg(long)]
	#[arg(default_value_t = false)]
	disable_media_controls: bool,

	/// Set filter level for console logs
	#[arg(long, value_name = "OFF|ERROR|INFO|WARN|DEBUG|TRACE")]
	#[arg(default_value_t = log::LevelFilter::Info)]
	log_level: log::LevelFilter,

	/// Print version
	#[arg(short, long)]
	version: bool,
}

//---------------------------------------------------------------------------------------------------- CLI argument handling
impl Cli {
	#[inline(always)]
	pub fn get() -> (bool, bool) {
		Self::parse().handle_args()
	}

	#[inline(always)]
	pub fn handle_args(self) -> (bool, bool) {
		use std::process::exit;

		// Version.
		if self.version {
			println!("Festival GUI {FESTIVAL_VERSION} {COMMIT}\n{COPYRIGHT}");
			exit(0);
		}

		// Metadata.
		if self.metadata {
			match shukusai::collection::metadata() {
				Ok(md) => { println!("{md}"); exit(0); },
				Err(e) => { println!("festival error: {e}"); exit(1); },
			}
		}

		fn handle<T>(result: Result<T, anyhow::Error>) {
			if let Err(e) = result {
				eprintln!("festival error: {e}");
				exit(1);
			} else {
				exit(0);
			}
		}

		// Signals.
		if self.toggle       { handle(Toggle::touch())      }
		if self.pause        { handle(Pause::touch())       }
		if self.play         { handle(Play::touch())        }
		if self.next         { handle(Next::touch())        }
		if self.previous     { handle(Previous::touch())    }
		if self.stop         { handle(Stop::touch())        }
		if self.shuffle      { handle(Shuffle::touch())     }
		if self.repeat_song  { handle(RepeatSong::touch())  }
		if self.repeat_queue { handle(RepeatQueue::touch()) }
		if self.repeat_off   { handle(RepeatOff::touch())   }

		// Content signals.
		use disk::Plain;
		if let Some(volume)           = self.volume        { handle(Volume(shukusai::kernel::Volume::new(volume)).save())
			} else if let Some(seek)  = self.seek          { handle(Seek(seek).save())
			} else if let Some(seek)  = self.seek_forward  { handle(SeekForward(seek).save())
			} else if let Some(seek)  = self.seek_backward { handle(SeekBackward(seek).save())
			} else if let Some(index) = self.index         { handle(Index(index).save())
			} else if let Some(skip)  = self.skip          { handle(Skip(skip).save())
			} else if let Some(back)  = self.back          { handle(Back(back).save())
		}

		// Logger.
		init_logger(self.log_level);

		// Return.
		(self.disable_watch, self.disable_media_controls)
	}
}
