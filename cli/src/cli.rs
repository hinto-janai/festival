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
use rpc::Rpc;

//---------------------------------------------------------------------------------------------------- CLI Parser (clap)
#[cfg(windows)]
pub const BIN: &str = "festival-cli.exe";
#[cfg(unix)]
pub const BIN: &str = "festival-cli";

const USAGE: &str = formatcp!(
r#"{BIN} [--OPTIONS] <METHOD> [--PARAM <VALUE>]

Arguments passed to `festival-cli` will always take
priority over configuration options read from disk."#);

#[derive(Parser)]
// Clap puts a really ugly non-wrapping list
// of possible args if this isn't set.
#[command(override_usage = USAGE)]
pub struct Cli {
	#[command(subcommand)]
	rpc: Option<Rpc>,

	#[arg(short, long, verbatim_doc_comment, value_name = "URL")]
	/// URL of the `festivald` to connect to
	///
	/// The protocol, IPv4 address, and port of the
	/// `festivald` that `festival-cli` will connect
	/// to by default.
	///
	/// Protocol must be:
	///   - http
	///   - https
	///
	/// IP address must be IPv4.
	///
	/// Default is: `http://127.0.0.1:18425`
	festivald: Option<String>,

	#[arg(short, long, verbatim_doc_comment, value_name = "SECONDS")]
	/// Set a timeout for a non-responding `festivald`
	///
	/// If `festivald` does not respond with _at least_
	/// a basic HTTP header within this time (seconds),
	/// `festival-cli` will disconnect.
	///
	/// 0 means never disconnect.
	timeout: Option<f64>,

	#[arg(short, long, verbatim_doc_comment, value_name = "ID")]
	/// The `JSON-RPC 2.0` ID to send to `festivald`.
	///
	/// See below for more info:
	/// <https://jsonrpc.org/specification>
	id: Option<String>,

	#[arg(short, long, verbatim_doc_comment, value_name = "USER:PASS or FILE")]
	/// Authorization sent to `festivald`
	///
	/// This matches the `authorization` config
	/// in `festivald`, see here for more info:
	/// <https://docs.festival.pm/daemon/authorization/authorization.html>
	///
	/// A `festivald` with `HTTPS` or `.onion` must be
	/// used or `festival-cli` will refuse to start.
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

	#[arg(short, long, verbatim_doc_comment)]
	/// Route connections through a proxy
	///
	/// Requests/responses will be routed via this proxy.
	///
	/// Supported proxies are: `HTTP`, `SOCKS4`, and `SOCKS5`.
	///
	/// The input must be:
	///   - Proxy protocol (`http://`, `socks4://`, `socks5://`)
	///   - Proxy IP
	///   - Proxy port
	///
	/// For example: `festival-cli --proxy socks5://127.0.0.1:9050`
	proxy: Option<String>,

	#[arg(short, long, verbatim_doc_comment)]
	/// Print debug information about the config/request/response
	debug: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Allow `--authorization` even without TLS
	///
	/// This will let you set the authorization
	/// setting even if TLS is disabled.
	///
	/// This means your `user:pass` will be sent in clear-text HTTP,
	/// unless you are wrapping HTTP in something else, like SSH
	/// port forwarding, or Tor.
	confirm_no_tls_auth: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Print debug information, but don't actually connect to `festivald`
	///
	/// This implies `--debug`.
	dry_run: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Open `festival-cli` documentation locally in browser
	docs: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Delete all `festival-cli` files that are on disk
	///
	/// This deletes all `cli` Festival folders.
	/// The PATHs deleted will be printed on success.
	delete: bool,

	#[arg(long, verbatim_doc_comment)]
	/// Print the PATHs used by `festival-cli`
	///
	/// All data saved by `festival-cli` is saved in these directories.
	/// For more information, see: <https://docs.festival.pm/cli/disk.html>
	path: bool,

	#[arg(short, long, verbatim_doc_comment)]
	/// Reset the current `festival-cli.toml` config file to the default
	///
	/// Exits with `0` if everything went ok, otherwise shows error.
	reset_config: bool,

	#[arg(short, long, verbatim_doc_comment)]
	/// Print all the JSON-RPC methods available
	methods: bool,

	#[arg(short, long)]
	/// Print version
	version: bool,
}

//---------------------------------------------------------------------------------------------------- Regular CLI argument handling
impl Cli {
	pub fn get() -> (Option<ConfigBuilder>, Option<Rpc>, bool, bool) {
		Self::parse().handle_args()
	}

	fn handle_args(mut self) -> (Option<ConfigBuilder>, Option<Rpc>, bool, bool) {
		// Version.
		if self.version {
			eprintln!("{FESTIVAL_CLI_SHUKUSAI_COMMIT}\n{COPYRIGHT}");
			exit(0);
		}

		// Path.
		if self.path {
			// Config.
			let p = crate::config::ConfigBuilder::sub_dir_parent_path().unwrap();
			eprintln!("{}", p.display());

			// macOS is in the same folder.
			#[cfg(not(target_os = "macos"))]
			{
				// `.local/share`
				let p = crate::docs::Docs::sub_dir_parent_path().unwrap();
				eprintln!("{}", p.display());
			}

			exit(0);
		}

		// Methods
		if self.methods {
			rpc::Method::println_all();
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
			#[cfg(not(target_os = "macos"))]
			let paths = [
				// Config.
				crate::config::ConfigBuilder::sub_dir_parent_path().unwrap(),
				// `.local/share`
				crate::docs::Docs::sub_dir_parent_path().unwrap(),
			];

			// macOS is in the same folder.
			#[cfg(target_os = "macos")]
			let paths = [crate::config::ConfigBuilder::sub_dir_parent_path().unwrap()];

			let mut code = 0;

			for p in paths {
				if !p.exists() {
					eprintln!("festival-cli: PATH does not exist ... {}", p.display());
					continue;
				}

				match std::fs::remove_dir_all(&p) {
					Ok(_)  => eprintln!("{}", p.display()),
					Err(e) => { eprintln!("festival-cli error: {} - {e}", p.display()); code = 1; },
				}
			}

			exit(code);
		}

		let config = self.handle_config();

		// Return.
		(config, self.rpc, self.debug || self.dry_run, self.dry_run)
	}

	pub fn handle_config(&mut self) -> Option<ConfigBuilder> {
		let mut cb = ConfigBuilder::default();
		let mut diff = false;

		let mut confirm_no_tls_auth = self.confirm_no_tls_auth.then_some(self.confirm_no_tls_auth);

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
			self.festivald      => cb.festivald,
			self.timeout        => cb.timeout,
			self.id             => cb.id,
			confirm_no_tls_auth => cb.confirm_no_tls_auth,
			self.authorization  => cb.authorization,
			self.proxy          => cb.proxy
		}

		if diff {
			Some(cb)
		} else {
			None
		}
	}
}
