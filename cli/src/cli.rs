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
use std::net::Ipv4Addr;
use std::path::PathBuf;
use crate::config::ConfigBuilder;

//---------------------------------------------------------------------------------------------------- CLI Parser (clap)
#[cfg(windows)]
const BIN: &str = "festivald.exe";
#[cfg(unix)]
const BIN: &str = "festivald";

const USAGE: &str = formatcp!(
r#"{BIN} [OPTIONS] [COMMAND + OPTIONS] [ARGS...]

Arguments passed to `festivald` will always take
priority over configuration options read from disk."#);

#[derive(Parser)]
// Clap puts a really ugly non-wrapping list
// of possible args if this isn't set.
#[command(override_usage = USAGE)]
pub struct Cli {
	#[command(subcommand)]
	command: Option<Command>,

	#[arg(long, verbatim_doc_comment)]
	/// The IPv4 address `festivald` will bind to [default: 127.0.0.1]
	ip: Option<std::net::Ipv4Addr>,

	#[arg(long, verbatim_doc_comment)]
	/// The port `festivald` will bind to [default: 18425]
	port: Option<u16>,

	#[arg(long, verbatim_doc_comment, value_name = "NUMBER")]
	/// Max amount of connections [default: unlimited]
	///
	/// The max amount of connections `festivald`
	/// will serve at any given moment.
	/// `0` means unlimited.
	///
	/// Note that 1 client doesn't necessarily mean
	/// 1 connection. A single web browser client for
	/// example can make many multiple connections
	/// to `festivald`.
	max_connections: Option<u64>,

	#[arg(long, verbatim_doc_comment, value_name = "IP")]
	/// Only accept connections from these IPs
	///
	/// `festivald` will only serve connections coming
	/// from these IPs. If there's no value given or
	/// any of the values is "0.0.0.0", `festivald`
	/// will serve all IP ranges.
	///
	/// To allow multiple IPs, use this flag per IP.
	///
	/// Example: `festivald --exclusive-ip 127.0.0.1 --exclusive-ip 192.168.2.1`
	exclusive_ip: Option<Vec<Ipv4Addr>>,

	#[arg(long, verbatim_doc_comment, requires = "certificate", requires = "key")]
	/// Enable HTTPS
	///
	/// You must also provide a PEM-formatted X509 certificate
	/// and key in the below options for this to work.
	///
	/// Example: `festivald --tls --certificate /path/to/cert.pem --key /path/to/key.pem`
	tls: bool,

	#[arg(long, verbatim_doc_comment, value_name = "FILE", requires = "key", requires = "tls")]
	/// The PEM-formatted X509 certificate file used for TLS
	certificate: Option<PathBuf>,

	#[arg(long, verbatim_doc_comment, value_name = "FILE", requires = "certificate", requires = "tls")]
	/// The PEM-formatted key file used for TLS
	key: Option<PathBuf>,

	#[arg(long, verbatim_doc_comment, value_name = "USER:PASS or FILE", requires = "certificate", requires = "key", requires = "tls")]
	/// Enforce a `username` and `password` for connections to `festivald`
	///
	/// Only process connections to `festivald` that have a
	/// "authorization" HTTP header with this username and password.
	///
	/// If either the `--no-auth-rpc` or `--no-auth-rest` options are
	/// used, then every RPC call/REST endpoint _NOT_ in those lists
	/// will require this authorization.
	///
	/// TLS must be enabled for this feature to work
	/// or `festivald` will refuse to start.
	///
	/// This value must be:
	///   1. The "username"
	///   2. Followed by a single colon ":"
	///   3. Then the "password", e.g:
	/// ```
	/// festivald --authorization my_user:my_pass
	/// ```
	/// An empty string disables this feature.
	///
	/// Alternatively, you can input an absolute PATH to a file
	/// `festivald` can access, containing the string, e.g:
	/// ```
	/// festivald --authorization "/path/to/user_and_pass.txt"
	/// ```
	/// In this case, `festivald` will read the file and attempt
	/// to parse it with the same syntax, i.e, the file should contain:
	/// ```
	/// my_user:my_pass
	/// ```
	authorization: Option<String>,

	#[arg(long, verbatim_doc_comment, value_name = "METHOD", requires = "authorization")]
	/// Allow specified JSON-RPC calls without authorization
	///
	/// If a JSON-RPC method is listed in this array,
	/// `festivald` will allow any client to use it,
	/// regardless of authorization.
	///
	/// This allows you to have `authorization` enabled
	/// across the board, but allow specific JSON-RPC
	/// calls for public usage.
	///
	/// For example, if only `toggle` is listed, then
	/// clients WITHOUT authorization will only be
	/// allowed to use the `toggle` method, for every
	/// other method, they must authenticate.
	///
	/// The method names listed here must match the
	/// exact names when using them, or shown in the
	/// documentation, see here:
	///
	/// <https://docs.festival.pm/daemon/json-rpc/json-rpc.html>
	///
	/// OR WITH
	///
	/// ```
	/// festivald data --docs
	/// ```
	///
	/// To allow multiple methods, use this flag per method.
	///
	/// Example: `festivald --no-auth-rpc toggle --no-auth-rpc volume`
	no_auth_rpc: Option<Vec<rpc::Method>>,

	#[arg(long, verbatim_doc_comment, value_name = "RESOURCE", requires = "authorization")]
	/// Allow specified REST resources without authorization
	///
	/// REST resources, from most expensive to least:
	///   - `collection`
	///   - `artist`
	///   - `album`
	///   - `song`
	///   - `art`
	///
	/// If a REST resource is listed in this array,
	/// `festivald` will allow any client to use it,
	/// regardless of authorization.
	///
	/// For example, if only `art` is listed, then
	/// clients WITHOUT authorization will only be
	/// allowed to use the `art` related endpoints
	/// (/rand/art, /current/art, etc). For every
	/// other endpoint (/rand/song, /collection, etc),
	/// they must authenticate.
	///
	/// To allow multiple methods, use this flag per method.
	///
	/// Example: `festivald --no-auth-rest art --no-auth-rest song`
	no_auth_rest: Option<Vec<rpc::resource::Resource>>,

	#[arg(long, verbatim_doc_comment, requires = "authorization")]
	/// Allow documentation to be served without authorization
	no_auth_docs: bool,

	#[arg(long, verbatim_doc_comment, value_name = "MILLI")]
	/// Sleep before responding to (potentially malicious) failed connections
	///
	/// Upon a failed, potentially malicious request, instead of
	/// immediately responding, `festivald` will randomly sleep
	/// up to this many milliseconds before responding to the connection.
	///
	/// This includes:
	///   - Authentication failure
	///   - IPs not in the `exclusive_ips` list
	///   - IPv6 connections
	///
	/// If 0, `festivald` will immediately respond. This may
	/// not be wanted to due potential DoS and timing attacks.
	///
	/// If you're hosting locally (127.0.0.1), you can set this
	/// to 0 (unless you don't trust your local network?).
	sleep_on_fail: Option<u64>,

	#[arg(long, verbatim_doc_comment, value_name = "PATH")]
	/// Default PATHs to use for the `Collection`
	///
	/// Upon a `collection_new` JSON-RPC method call, if the
	/// `paths` parameter is empty, these PATHs will be scanned
	/// instead.
	///
	/// If this is not set, the default OS `Music` directory will be scanned.
	///
	/// If `festivald` is running on Windows, you can use
	/// Windows-style PATHs: `C:\\Users\\User\\Music`.
	///
	/// To set multiple PATHs, use this flag per PATH.
	///
	/// Example: `festivald --collection-path /my/path/1 --collection-path /my/path/2`
	collection_path: Vec<PathBuf>,

	#[arg(long, verbatim_doc_comment)]
	/// Enable direct downloads via the REST API for browsers
	///
	/// By default, accessing the REST API via a browser
	/// will open the resource in the browser (audio player,
	/// image viewer, etc)
	///
	/// Using this flag will make browsers download
	/// the file directly, without opening it.
	direct_download: bool,

	#[arg(long, verbatim_doc_comment, value_name = "SEPARATOR")]
	/// When files are downloaded via the REST API, and the
	/// file is a nested object referencing multiple things
	/// (e.g, an _album_ owned by an _artist_), we must include
	/// that information, but what string should separate them?
	///
	/// The default separator is " - ", e.g:
	/// ```
	/// Artist Name - Album Title.zip
	/// ```
	/// it can be changed to any string, like "/":
	/// ```
	/// Artist Name/Album Title.zip
	/// ```
	/// or left empty "" for no separator at all.
	filename_separator: Option<String>,

	#[arg(long, verbatim_doc_comment, value_name = "SECONDS")]
	/// Set the `REST` API cache time limit
	///
	/// When serving `ZIP` files via the REST API, `festivald`
	/// will first write them to disk, then serve those files
	/// instead of directly storing everything in memory,
	/// as to not get OOM-killed on more than a few requests.
	///
	/// This option sets the time limit on how many seconds
	/// `festivald` will hold onto this cache for.
	///
	/// Once the time limit is up, `festivald` will remove the
	/// file. This cache is also reset on startup and shutdown.
	///
	/// Default is 3600 seconds (1 hour).
	cache_time: Option<u64>,

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
	/// `--disable-rest` will disable this part of the system,
	/// and will only leave the JSON-RPC API available.
	disable_rest: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Enable/disable serving documentation
	///
	/// By default, `festivald` serves a markdown book
	/// of it's own documentation, accessible at the
	/// root `/` endpoint, e.g:
	/// ```
	/// http://localhost:18425/
	/// ```
	///
	/// `--disable-docs` will disable that.
	disable_docs: bool,

	#[arg(long, value_name = "OFF|ERROR|INFO|WARN|DEBUG|TRACE")]
	/// Set filter level for console logs
	log_level: Option<log::LevelFilter>,

	#[arg(short, long)]
	/// Print version
	version: bool,
}

//---------------------------------------------------------------------------------------------------- Subcommands
#[derive(Subcommand)]
pub enum Command {
	#[command(verbatim_doc_comment)]
	/// Various utility flags related to `festivald` data
	Data(Data),

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


//---------------------------------------------------------------------------------------------------- Subcommand - Data
#[derive(Args)]
#[command(arg_required_else_help(true))]
#[command(override_usage = formatcp!("{BIN} data OPTION [ARG]"))]
pub struct Data {
	#[arg(long, verbatim_doc_comment)]
	/// Open documentation locally in browser
	///
	/// This opens `festivald'`s documentation in a web
	/// browser, and does not start `festivald` itself.
	docs: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Print the PATHs used by Festival
	///
	/// All data saved by Festival is saved in these directories.
	/// For more information, see: <https://docs.festival.pm/daemon/disk.html>
	path: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Reset the current `festivald.toml` config file to the default
	///
	/// Exits with `0` if everything went ok, otherwise shows error.
	reset_config: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Print JSON metadata about the current `Collection` on disk
	///
	/// This output is the exact same as the `state_collection_full` JSON-RPC call.
	state_collection_full: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Delete all `festivald` files that are on disk
	///
	/// This deletes all `daemon` Festival folders.
	/// The PATHs deleted will be printed on success.
	delete: bool,
}

//---------------------------------------------------------------------------------------------------- Subcommand - Signal
#[derive(Args)]
#[command(arg_required_else_help(true))]
#[command(override_usage = formatcp!("{BIN} signal OPTION [ARG]"))]
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
	#[arg(value_parser = clap::value_parser!(u8).range(0..=100), value_name = "VOLUME")]
	/// Set the volume to `VOLUME` (0-100)
	volume: Option<u8>,

	#[arg(long, value_name = "SECOND")]
	/// Seek to the absolute `SECOND` second in the current song
	seek: Option<u64>,

	#[arg(long, value_name = "SECOND")]
	/// Seek `SECOND` seconds forwards in the current song
	seek_forward: Option<u64>,

	#[arg(long, value_name = "SECOND")]
	/// Seek `SECOND` seconds backwards in the current song
	seek_backward: Option<u64>,

	#[arg(long, verbatim_doc_comment, value_name = "NUMBER")]
	/// Set the current song to the index `NUMBER` in the queue.
	///
	/// NOTE:
	/// The queue index starts from 1 (first song is `--index 1`).
	///
	/// Providing an index that is out-of-bounds
	/// will end the queue (even if repeat is turned on).
	index: Option<NonZeroUsize>,

	#[arg(long, verbatim_doc_comment, value_name = "NUMBER")]
	/// Skip `NUMBER` amount of songs
	///
	/// If the last song in the queue is skipped over,
	/// and queue repeat is turned on, this will reset
	/// the current song to the 1st in the queue.
	skip: Option<usize>,

	#[arg(long, verbatim_doc_comment, value_name = "NUMBER")]
	/// Go backwards in the queue by `NUMBER` amount of songs
	///
	/// If `NUMBER` is greater than the amount of songs we can
	/// skip backwards, this will reset the current song to
	/// the 1st in the queue.
	back: Option<usize>,
}

//---------------------------------------------------------------------------------------------------- CLI argument handling
impl Cli {
	pub fn get() -> (bool, bool, Option<log::LevelFilter>, Option<ConfigBuilder>) {
		Self::parse().handle_args()
	}

	fn handle_args(mut self) -> (bool, bool, Option<log::LevelFilter>, Option<ConfigBuilder>) {
		// Version.
		if self.version {
			println!("{FESTIVALD_SHUKUSAI_COMMIT}\n{COPYRIGHT}");
			exit(0);
		}

		self.handle_command();
		let config = self.handle_config();

		// Return.
		(self.disable_watch, self.disable_media_controls, self.log_level, config)
	}

	fn handle_command(&self) {
		if let Some(c) = &self.command {
			match c {
				Command::Data(x)   => self.handle_data(x),
				Command::Signal(x) => self.handle_signal(x),
			}
			exit(0);
		} else {
			return;
		}
	}

	fn handle_data(&self, u: &Data) {
		// `state_collection_full`
		if u.state_collection_full {
			match shukusai::collection::rpc::state_collection_full() {
				Ok(md) => { println!("{md}"); exit(0); },
				Err(e) => { eprintln!("festivald error: {e}"); exit(1); },
			}
		}

		// Path.
		if u.path {
			// Cache.
			let p = crate::zip::CollectionZip::sub_dir_parent_path().unwrap();
			println!("{}", p.display());

			// Config.
			let p = crate::config::Config::sub_dir_parent_path().unwrap();
			println!("{}", p.display());

			// `.local/share`
			let p = shukusai::collection::Collection::sub_dir_parent_path().unwrap();
			println!("{}", p.display());

			exit(0);
		}

		// `reset_config`
		if u.reset_config {
			let p = crate::config::Config::absolute_path().unwrap();
			crate::config::Config::mkdir().unwrap();
			std::fs::write(&p, crate::constants::FESTIVALD_CONFIG).unwrap();
			exit(0);
		}

		// Docs.
		if u.docs {
			// Create documentation.
			match crate::docs::Docs::create() {
				Ok(mut path) => {
					path.push("index.html");

					match open::that_detached(path) {
						Ok(_)  => exit(0),
						Err(e) => { eprintln!("festivald: Could not open docs: {e}"); exit(1); },
					}
				},
				Err(e) => { eprintln!("festivald: Could not create docs: {e}"); exit(1); },
			}
			exit(0);
		}

		// Delete.
		if u.delete {
			let paths = [
				// Cache.
				crate::zip::CollectionZip::sub_dir_parent_path().unwrap(),
				// Config.
				crate::config::Config::sub_dir_parent_path().unwrap(),
				// `.local/share`
				shukusai::collection::Collection::sub_dir_parent_path().unwrap(),
			];

			let mut code = 0;

			for p in paths {
				if !p.exists() {
					println!("festivald: PATH does not exist ... {}", p.display());
					continue;
				}

				match std::fs::remove_dir_all(&p) {
					Ok(_)  => println!("{}", p.display()),
					Err(e) => { eprintln!("festivald error: {} - {e}", p.display()); code = 1; },
				}
			}

			exit(code);
		}

	}

	pub fn handle_signal(&self, s: &Signal) -> ! {
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

	pub fn handle_config(&mut self) -> Option<ConfigBuilder> {
		let mut cb = ConfigBuilder::default();
		let mut diff = false;

		fn if_true_some(b: bool) -> Option<bool> {
			if b {
				Some(!b)
			} else {
				None
			}
		}

		let mut tls             = if_true_some(self.tls);
		let mut direct_download = if_true_some(self.direct_download);
		let mut no_auth_docs    = if_true_some(self.no_auth_docs);

		fn if_true_negate_some(b: bool) -> Option<bool> {
			if b {
				Some(!b)
			} else {
				None
			}
		}

		// `disable_*` negation.
		let mut docs           = if_true_negate_some(self.disable_docs);
		let mut media_controls = if_true_negate_some(self.disable_media_controls);
		let mut rest           = if_true_negate_some(self.disable_rest);
		let mut watch          = if_true_negate_some(self.disable_watch);

		// Special-case conversions.
		macro_rules! vec_to_some_hashset {
			($vec:expr) => {
				if let Some(vec) = $vec.take() {
					let mut hashset = std::collections::HashSet::with_capacity(vec.len());
					for entry in vec {
						hashset.insert(entry);
					}
					Some(hashset)
				} else {
					None
				}
			}
		}

		let mut exclusive_ips = vec_to_some_hashset!(self.exclusive_ip);
		let mut no_auth_rpc   = vec_to_some_hashset!(self.no_auth_rpc);
		let mut no_auth_rest  = vec_to_some_hashset!(self.no_auth_rest);

		let mut collection_paths = if self.collection_path.is_empty() {
			None
		} else {
			Some(std::mem::take(&mut self.collection_path))
		};

		let mut log_level = self.log_level.clone();

		macro_rules! if_some {
			($($command:expr => $config:expr),*) => {
				$(
					if $command.is_some() {
						std::mem::swap(&mut $command, &mut $config);
						diff = true;
					} else {
						$config = None;
					}
				)*
			}
		}

		if_some! {
			self.ip                 => cb.ip,
			self.port               => cb.port,
			self.max_connections    => cb.max_connections,
			exclusive_ips           => cb.exclusive_ips,
			self.sleep_on_fail      => cb.sleep_on_fail,
			collection_paths        => cb.collection_paths,
			tls                     => cb.tls,
			self.certificate        => cb.certificate,
			self.key                => cb.key,
			rest                    => cb.rest,
			docs                    => cb.docs,
			direct_download         => cb.direct_download,
			self.filename_separator => cb.filename_separator,
			log_level               => cb.log_level,
			watch                   => cb.watch,
			self.cache_time         => cb.cache_time,
			media_controls          => cb.media_controls,
			self.authorization      => cb.authorization,
			no_auth_rpc             => cb.no_auth_rpc,
			no_auth_rest            => cb.no_auth_rest,
			no_auth_docs            => cb.no_auth_docs
		}

		if diff {
			Some(cb)
		} else {
			None
		}
	}
}
