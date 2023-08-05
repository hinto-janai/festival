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
	FESTIVAL_CLI_SHUKUSAI_COMMIT,
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
pub const BIN: &str = "festival-cli.exe";
#[cfg(unix)]
pub const BIN: &str = "festival-cli";

const USAGE: &str = formatcp!(
r#"{BIN} [OPTIONS] [COMMAND + OPTIONS] [ARGS...]

Arguments passed to `festival-cli` will always take
priority over configuration options read from disk."#);

#[derive(Parser)]
// Clap puts a really ugly non-wrapping list
// of possible args if this isn't set.
#[command(override_usage = USAGE)]
pub struct Cli {
	#[command(subcommand)]
	rpc: Option<Rpc>,

	#[arg(long, verbatim_doc_comment, value_name = "URI")]
	/// `festivald` URI
	///
	/// The protocol, IPv4 address, and port of the
	/// `festivald` that `festival-cli` will connect
	/// to by default.
	///
	/// Default is: `http://127.0.0.1:18425`
	festivald: Option<String>,

	#[arg(long, verbatim_doc_comment)]
	/// If using HTTPS, do not validate `festivald`'s TLS certificate
	///
	/// This is similar to `curl --insecure` or `wget --no-check-certificate`.
	///
	/// Useful for connecting to your own `festivald`,
	/// which may be using a self-signed certificate.
	ignore_cert: bool,

	#[arg(long, verbatim_doc_comment, value_name = "SECONDS")]
	/// Disconnect from a non-responding `festivald`
	///
	/// If `festivald` does not respond with _at least_
	/// a basic HTTP header within this time (seconds),
	/// `festival-cli` will disconnect.
	///
	/// 0 means never disconnect.
	timeout: Option<u64>,

	#[arg(long, verbatim_doc_comment, value_name = "ID")]
	/// The `JSON-RPC 2.0` ID to send to `festivald`.
	/// See below for more info:
	/// <https://jsonrpc.org/specification>
	id: Option<String>,

	#[arg(long, verbatim_doc_comment, value_name = "PATH")]
	/// Upon a `collection_new` JSON-RPC method call, if the
	/// `--paths` parameter is empty or not passed,
	/// these PATHs will be sent instead.
	///
	/// If this is also empty, the default OS `Music`
	/// directory will be used
	///
	/// Windows-style PATHs will only work if the target
	/// `festivald` is running on Windows (`C:\\Users\\User\\Music`)
	///
	/// Note these are PATHs on the _target_ `festivald`,
	/// NOT on the filesystem `festival-cli` is running on.
	///
	/// To set multiple PATHs, use this flag per PATH.
	///
	/// Example: `festivald --collection-path /my/path/1 --collection-path /my/path/2`
	collection_path: Vec<PathBuf>,


	#[arg(long, verbatim_doc_comment, value_name = "USER:PASS or FILE")]
	/// Authorization sent to `festivald`
	///
	/// This matches the `authorization` config
	/// in `festivald`, see here for more info:
	/// <https://docs.festival.pm/daemon/authorization/authorization.html>
	///
	/// A `festivald` with HTTPS must be used or `festival-cli`
	/// will refuse to start.
	///
	/// An empty string disables this feature.
	///
	/// Alternatively, you can input an absolute PATH to a file
	/// `festival-cli` can access, containing the string, e.g:
	/// ```
	/// authorization = "/path/to/user_and_pass.txt"
	/// ```
	///
	/// In this case, `festival-cli` will read the file and attempt
	/// to parse it with the same syntax, i.e, the file should contain:
	/// ```
	/// my_user:my_pass
	/// ```
	authorization: Option<String>,

	#[arg(long, verbatim_doc_comment)]
	/// Open `festival-cli` documentation locally in browser
	docs: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Print the PATHs used by `festival-cli`
	///
	/// All data saved by `festival-cli` is saved in these directories.
	/// For more information, see: <https://docs.festival.pm/cli/disk.html>
	path: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Reset the current `festival-cli.toml` config file to the default
	///
	/// Exits with `0` if everything went ok, otherwise shows error.
	reset_config: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Delete all `festival-cli` files that are on disk
	///
	/// This deletes all `cli` Festival folders.
	/// The PATHs deleted will be printed on success.
	delete: bool,

	#[arg(long, value_name = "OFF|ERROR|INFO|WARN|DEBUG|TRACE")]
	/// Set filter level for console logs
	log_level: Option<log::LevelFilter>,

	#[arg(short, long)]
	/// Print version
	version: bool,
}

//---------------------------------------------------------------------------------------------------- Subcommands
#[derive(Subcommand,Debug,Clone)]
#[command(rename_all = "snake_case")]
pub enum Rpc {
	CollectionNew(rpc::param::CollectionNew),
	CollectionBrief(rpc::param::CollectionBrief),
	CollectionFull(rpc::param::CollectionFull),
	CollectionRelation(rpc::param::CollectionRelation),
	CollectionRelationFull(rpc::param::CollectionRelationFull),
	CollectionPerf(rpc::param::CollectionPerf),
	CollectionResourceSize(rpc::param::CollectionResourceSize),

	StateIp(rpc::param::StateIp),
	StateConfig(rpc::param::StateConfig),
	StateDaemon(rpc::param::StateDaemon),
	StateAudio(rpc::param::StateAudio),
	StateReset(rpc::param::StateReset),

	KeyArtist(rpc::param::KeyArtist),
	KeyAlbum(rpc::param::KeyAlbum),
	KeySong(rpc::param::KeySong),

	MapArtist(rpc::param::MapArtistOwned),
	MapAlbum(rpc::param::MapAlbumOwned),
	MapSong(rpc::param::MapSongOwned),

	CurrentArtist(rpc::param::CurrentArtist),
	CurrentAlbum(rpc::param::CurrentAlbum),
	CurrentSong(rpc::param::CurrentSong),

	RandArtist(rpc::param::RandArtist),
	RandAlbum(rpc::param::RandAlbum),
	RandSong(rpc::param::RandSong),

	Search(rpc::param::SearchOwned),
	SearchArtist(rpc::param::SearchArtistOwned),
	SearchAlbum(rpc::param::SearchAlbumOwned),
	SearchSong(rpc::param::SearchSongOwned),

	Toggle(rpc::param::Toggle),
	Play(rpc::param::Play),
	Pause(rpc::param::Pause),
	Next(rpc::param::Next),
	Stop(rpc::param::Stop),
	Shuffle(rpc::param::Shuffle),
	RepeatOff(rpc::param::RepeatOff),
	RepeatSong(rpc::param::RepeatSong),
	RepeatQueue(rpc::param::RepeatQueue),
	Previous(rpc::param::Previous),
	Volume(rpc::param::Volume),
	Clear(rpc::param::Clear),
	Seek(rpc::param::Seek),
	Skip(rpc::param::Skip),
	Back(rpc::param::Back),

	AddQueueKeyArtist(rpc::param::AddQueueKeyArtist),
	AddQueueKeyAlbum(rpc::param::AddQueueKeyAlbum),
	AddQueueKeySong(rpc::param::AddQueueKeySong),
	AddQueueMapArtist(rpc::param::AddQueueMapArtistOwned),
	AddQueueMapAlbum(rpc::param::AddQueueMapAlbumOwned),
	AddQueueMapSong(rpc::param::AddQueueMapSongOwned),
	AddQueueRandArtist(rpc::param::AddQueueRandArtist),
	AddQueueRandAlbum(rpc::param::AddQueueRandAlbum),
	AddQueueRandSong(rpc::param::AddQueueRandSong),
	SetQueueIndex(rpc::param::SetQueueIndex),
	RemoveQueueRange(rpc::param::RemoveQueueRange),
}

//---------------------------------------------------------------------------------------------------- Subcommand - Data
#[derive(Args)]
#[command(arg_required_else_help(true))]
#[command(override_usage = formatcp!("{BIN} data OPTION [ARG]"))]
pub struct Data {
}

//---------------------------------------------------------------------------------------------------- CLI argument handling
impl Cli {
	pub fn get() -> (Option<log::LevelFilter>, Option<ConfigBuilder>) {
		Self::parse().handle_args()
	}

	fn handle_args(mut self) -> (Option<log::LevelFilter>, Option<ConfigBuilder>) {
		// Version.
		if self.version {
			println!("{FESTIVAL_CLI_SHUKUSAI_COMMIT}\n{COPYRIGHT}");
			exit(0);
		}

		// Path.
		if self.path {
			// Config.
			let p = crate::config::ConfigBuilder::sub_dir_parent_path().unwrap();
			println!("{}", p.display());

			// `.local/share`
			let p = shukusai::collection::Collection::sub_dir_parent_path().unwrap();
			println!("{}", p.display());

			exit(0);
		}

		// `reset_config`
		if self.reset_config {
			let p = crate::config::ConfigBuilder::absolute_path().unwrap();
			crate::config::ConfigBuilder::mkdir().unwrap();
			std::fs::write(&p, crate::constants::FESTIVAL_CLI_CONFIG).unwrap();
			exit(0);
		}

		// Docs.
		if self.docs {
			// Create documentation.
			if let Err(e) = crate::docs::Docs::create_open() {
				crate::exit!("Could not create docs: {e}");
			}

			exit(0);
		}

		// Delete.
		if self.delete {
			let paths = [
				// Config.
				crate::config::ConfigBuilder::sub_dir_parent_path().unwrap(),
				// `.local/share`
				shukusai::collection::Collection::sub_dir_parent_path().unwrap(),
			];

			let mut code = 0;

			for p in paths {
				if !p.exists() {
					eprintln!("festival-cli: PATH does not exist ... {}", p.display());
					continue;
				}

				match std::fs::remove_dir_all(&p) {
					Ok(_)  => println!("{}", p.display()),
					Err(e) => { eprintln!("festival-cli error: {} - {e}", p.display()); code = 1; },
				}
			}

			exit(code);
		}

		// RPC
		if let Some(rpc) = &self.rpc {
			println!("rpc: {:?}", self.rpc);
		}

		let config = self.handle_config();

		// Return.
		(self.log_level, config)
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

		let mut ignore_cert = if_true_some(self.ignore_cert);

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
			self.festivald     => cb.festivald,
			ignore_cert        => cb.ignore_cert,
			self.timeout       => cb.timeout,
			self.id            => cb.id,
			collection_paths   => cb.collection_paths,
			self.authorization => cb.authorization,
			log_level          => cb.log_level
		}

		if diff {
			Some(cb)
		} else {
			None
		}
	}
}
