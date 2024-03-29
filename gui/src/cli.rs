//---------------------------------------------------------------------------------------------------- Use
use crate::text::FESTIVAL_SHUKUSAI_COMMIT;
use clap::Parser;
use disk::Empty;
use disk::{Bincode2, Json, Plain};
use shukusai::constants::COPYRIGHT;
use shukusai::signal::{
    Back, Clear, Index, Next, Pause, Play, Previous, RepeatOff, RepeatQueue, RepeatSong, Seek,
    SeekBackward, SeekForward, Shuffle, Skip, Stop, Toggle, Volume,
};
use std::num::NonZeroUsize;

//---------------------------------------------------------------------------------------------------- CLI Parser (clap)
#[cfg(windows)]
const USAGE: &str = "Festival.exe [OPTIONS]";
#[cfg(unix)]
const USAGE: &str = "festival [OPTIONS]";

#[derive(Parser, Debug)]
// Clap puts a really ugly non-wrapping list
// of possible args if this isn't set.
#[command(override_usage = USAGE)]
pub struct Cli {
    //--------------------------------------------------------------- Signals
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

    //--------------------------------------------------------------- Data related, will return early
    #[arg(long, verbatim_doc_comment)]
    /// Open documentation locally in browser
    ///
    /// This opens Festivals documentation in a web
    /// browser, and does not start Festival itself.
    docs: bool,

    #[arg(long, verbatim_doc_comment)]
    /// Print the PATH used by Festival
    ///
    /// All data saved by Festival is saved here.
    /// For more information, see:
    /// <https://docs.festival.pm/gui#Disk>
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
    /// an already existing one (e.g, `festival --play`) is
    /// by creating a file in Festival's `signal` directory.
    ///
    /// `festival --FLAG` just creates a file in that directory,
    /// which an existing Festival will notice and do the appropriate task.
    ///
    /// Using `--disable-watch` will disable that part of the system so that
    /// filesystem signals won't work, e.g, `festival --play` will not work.
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
    /// Delete all Festival files that are currently on disk
    ///
    /// This deletes the entire `GUI` Festival folder, which contains:
    /// - The `Collection`
    /// - The `Playlists`
    /// - `GUI` settings (sort methods, color, etc)
    /// - `GUI` state (currently selected album/artist, etc)
    /// - Audio state (currently playing song, queue, etc)
    /// - Cached images for the OS media controls
    /// - Documentation files
    ///
    /// The PATH deleted will be printed on success.
    delete: bool,

    #[arg(long, value_name = "OFF|ERROR|INFO|WARN|DEBUG|TRACE")]
    #[arg(default_value_t = log::LevelFilter::Info)]
    /// Set filter level for console logs
    log_level: log::LevelFilter,

    #[arg(short, long)]
    /// Print version
    version: bool,
}

//---------------------------------------------------------------------------------------------------- CLI argument handling
impl Cli {
    #[inline(always)]
    pub fn get() -> (bool, bool, log::LevelFilter) {
        Self::parse().handle_args()
    }

    #[inline(always)]
    pub fn handle_args(self) -> (bool, bool, log::LevelFilter) {
        use std::process::exit;

        // Version.
        if self.version {
            println!("{FESTIVAL_SHUKUSAI_COMMIT}\n{COPYRIGHT}");
            exit(0);
        }

        // Metadata.
        if self.metadata {
            match shukusai::collection::metadata() {
                Ok(md) => {
                    println!("{md}");
                    exit(0);
                }
                Err(e) => {
                    eprintln!("festival error: {e}");
                    exit(1);
                }
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
        if self.toggle {
            handle(Toggle::touch())
        }
        if self.pause {
            handle(Pause::touch())
        }
        if self.play {
            handle(Play::touch())
        }
        if self.next {
            handle(Next::touch())
        }
        if self.previous {
            handle(Previous::touch())
        }
        if self.stop {
            handle(Stop::touch())
        }
        if self.clear {
            handle(Clear(true).save())
        }
        if self.shuffle {
            handle(Shuffle::touch())
        }
        if self.repeat_song {
            handle(RepeatSong::touch())
        }
        if self.repeat_queue {
            handle(RepeatQueue::touch())
        }
        if self.repeat_off {
            handle(RepeatOff::touch())
        }

        // Content signals.
        if let Some(volume) = self.volume {
            handle(Volume(shukusai::audio::Volume::new(volume)).save())
        }
        if let Some(seek) = self.seek {
            handle(Seek(seek).save())
        }
        if let Some(seek) = self.seek_forward {
            handle(SeekForward(seek).save())
        }
        if let Some(seek) = self.seek_backward {
            handle(SeekBackward(seek).save())
        }
        if let Some(index) = self.index {
            handle(Index(index.into()).save())
        }
        if let Some(skip) = self.skip {
            handle(Skip(skip).save())
        }
        if let Some(back) = self.back {
            handle(Back(back).save())
        }

        // Docs
        if self.docs {
            // Create documentation.
            if let Err(e) = crate::docs::Docs::create_open() {
                eprintln!("festival error: Could not create docs: {e}");
                exit(1);
            }

            exit(0);
        }

        // Path.
        if self.path {
            // SAFETY:
            // If we can't get a PATH, `panic!()`'ing is fine.
            let p = crate::data::State::sub_dir_parent_path().unwrap();
            println!("{}", p.display());
            exit(0);
        }

        // Delete.
        if self.delete {
            // SAFETY:
            // If we can't get a PATH, `panic!()`'ing is fine.
            let p = crate::data::State::sub_dir_parent_path().unwrap();
            match crate::data::State::rm_sub() {
                Ok(md) => {
                    println!("{}", md.path().display());
                    exit(0);
                }
                Err(e) => {
                    eprintln!("festival error: {} - {e}", p.display());
                    exit(1);
                }
            }
        }

        // Return.
        (
            self.disable_watch,
            self.disable_media_controls,
            self.log_level,
        )
    }
}
