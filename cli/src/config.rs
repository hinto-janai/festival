//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Serializer,Deserialize};
use anyhow::anyhow;
use benri::ok;
use log::{error,info,warn,debug,trace};
use disk::Toml;
use shukusai::constants::{
	FESTIVAL,FRONTEND_SUB_DIR,
};
use crate::constants::{
	FESTIVAL_CLI_NAME_VER,
	FESTIVAL_CLI_PORT,
	FESTIVAL_CLI_CONFIG,
};
use std::{
	net::{
		Ipv4Addr,
		SocketAddrV4,
	},
	collections::HashSet,
	path::PathBuf,
	borrow::Cow,
	str::FromStr,
};
use once_cell::sync::OnceCell;
use shukusai::constants::DASH;

//---------------------------------------------------------------------------------------------------- Statics
static CONFIG: OnceCell<Config> = OnceCell::new();
#[inline(always)]
/// Acquire our runtime configuration.
pub fn config() -> &'static Config {
	// SAFETY: this should always get
	// initialized in the `.builder()` below.
	unsafe { CONFIG.get_unchecked() }
}

//---------------------------------------------------------------------------------------------------- Defaults
const DEFAULT_URL: &str = "http://127.0.0.1:18425";
fn default_url() -> http::uri::Uri {
	// SAFETY: unwrap ok, static str.
	http::uri::Uri::from_str(DEFAULT_URL).unwrap()
}


const DEFAULT_ID: &str = FESTIVAL_CLI_NAME_VER;
fn default_id() -> json_rpc::Id<'static> {
	json_rpc::Id::Str(Cow::Borrowed(FESTIVAL_CLI_NAME_VER))
}


//---------------------------------------------------------------------------------------------------- ConfigBuilder
/// The `struct` that maps value directly from the disk.
///
/// We can't use this directly, but we can transform it into
/// the `Config` we will be using for the rest of the program.
disk::toml!(ConfigBuilder, disk::Dir::Config, FESTIVAL, FRONTEND_SUB_DIR, "festival-cli");
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub struct ConfigBuilder {
	pub festivald:          Option<String>,
	pub ignore_cert:        Option<bool>,
	pub timeout:            Option<u64>,
	pub id:                 Option<String>,
	pub collection_paths:   Option<Vec<PathBuf>>,
	pub log_level:          Option<log::LevelFilter>,
	pub authorization:	    Option<String>,
}

impl Default for ConfigBuilder {
	fn default() -> Self {
		Self {
			festivald:          Some(DEFAULT_URL.into()),
			ignore_cert:        Some(false),
			timeout:            Some(0),
			id:                 Some(DEFAULT_ID.into()),
			collection_paths:   Some(vec![]),
			log_level:          Some(log::LevelFilter::Error),
			authorization:      None,
		}
	}
}

impl ConfigBuilder {
	// INVARIANT: must be called once and only once.
	// Sets `CONFIG`, and returns a ref.
	pub fn build_and_set(self) -> &'static Config {
		let ConfigBuilder {
			festivald,
			ignore_cert,
			timeout,
			id,
			collection_paths,
			log_level,
			authorization,
		} = self;

		macro_rules! get {
			($option:expr, $field:literal, $default:expr) => {
				match $option {
					Some(v) => v,
					_ => {
						warn!("missing config [{}], using default [{:?}]", $field, $default);
						$default
					},
				}
			}
		}

		macro_rules! sum {
			($option:expr, $field:literal, $default:expr) => {
				match $option {
					Some(v) => Some(v),
					_ => {
						warn!("missing config [{}], using default: [{:?}]", $field, $default);
						$default
					},
				}
			}
		}

		// TODO
		let festivald = festivald.map(|s| http::uri::Uri::from_str(s.as_str()).unwrap());
		let id = id.map(|s| json_rpc::Id::from(s));

		let mut c = Config {
			festivald:          get!(festivald,          "festivald",          default_url()),
			ignore_cert:        get!(ignore_cert,        "ignore_cert",        false),
			timeout:            sum!(timeout,            "timeout",            None::<u64>),
			id:                 get!(id,                 "id",                 default_id()),
			collection_paths:   get!(collection_paths,   "collection_paths",   if let Some(p) = dirs::audio_dir() { vec![p] } else { Vec::<PathBuf>::with_capacity(0) }),
			log_level:          get!(log_level,          "log_level",          log::LevelFilter::Error),
			authorization: None,
		};

		if c.timeout == Some(0) {
			c.timeout = None;
		}

		// FIXME TODO: testing.
//		let authorization = Some("user:pass".to_string());

		// AUTHORIZATION
		if let Some(s) = authorization {
			// Check if it's a PATH or a String.
			let path = PathBuf::from(&s);
			let s = if path.is_absolute() && path.exists() {
				match std::fs::read_to_string(path) {
					Ok(s) => {
						match s.lines().next() {
							Some(s) => s.to_string(),
							None    => crate::exit!("[authorization] PATH file is empty"),
						}
					},
					Err(e) => crate::exit!("[authorization] PATH read error: {e}"),
				}
			} else {
				s
			};

			// Skip empty `username:password`.
			if s.is_empty() {
				warn!("config [authorization] is empty, skipping");
			// Look for `:` split.
			} else if s.split_once(":").is_none() {
				crate::exit!("[authorization] field is not in `USERNAME:PASSWORD` format");
			// Reject if TLS is not enabled.
			// TODO: replace with https check
//			} else if !c.tls || c.certificate.is_none() || c.key.is_none() {
//				crate::exit!("[authorization] field was provided but TLS is not enabled, exiting for safety");
			} else {
				// Set auth.
				c.authorization = Some(crate::auth::Auth::new(s));
			}
		} else {
			warn!("missing config [authorization], skipping");
		}

		info!("{DASH} Configuration");
		for line in format!("{c:#?}").lines() {
			info!("{line}");
		}
		info!("Authorization: {}", c.authorization.is_some());
		info!("{DASH} Configuration");

		// SAFETY: unwrap is okay, we only set `CONFIG` here.
		CONFIG.set(c).unwrap();
		config()
	}

	// Read from disk, or create a default.
	pub fn file_or() -> Self {
		use disk::Toml;

		match Self::from_file() {
			Ok(c)  => { ok!("festival-cli.conf ... from disk"); c },
			Err(e) => {
				// SAFETY: if we can't get the config, panic is ok.
				let p = ConfigBuilder::absolute_path().unwrap();

				if p.exists() {
					crate::exit!("festival-cli.conf exists but is invalid:\n\n{e}\ntip: use `festival-cli data --reset-config` to reset it");
				} else {
					ConfigBuilder::mkdir().unwrap();
					std::fs::write(&p, FESTIVAL_CLI_CONFIG).unwrap();
				}

				Self::default()
			},
		}
	}

	// Used to merge the command-line version with the disk.
	pub fn merge(&mut self, cmd: &mut Self) {
		macro_rules! if_some_swap {
			($($command:expr => $config:expr),*) => {
				$(
					if $command.is_some() {
						std::mem::swap(&mut $command, &mut $config);
					}
				)*
			}
		}

		if_some_swap! {
			cmd.festivald        => self.festivald,
			cmd.ignore_cert      => self.ignore_cert,
			cmd.timeout          => self.timeout,
			cmd.id               => self.id,
			cmd.collection_paths => self.collection_paths,
			cmd.authorization    => self.authorization,
			cmd.log_level        => self.log_level
		}
	}
}

//---------------------------------------------------------------------------------------------------- Config
/// The actual `struct` we will use for the whole program.
///
/// The global immutable copy the whole program will refer
/// to is the static `CONFIG` in this module. Or, `config()`.
//disk::toml!(Config, disk::Dir::Config, FESTIVAL, FRONTEND_SUB_DIR, "festival-cli");
#[derive(Debug,PartialEq)]
pub struct Config {
	pub festivald:        http::uri::Uri,
	pub ignore_cert:      bool,
	pub timeout:          Option<u64>,
	pub id:               json_rpc::Id<'static>,
	pub collection_paths: Vec<PathBuf>,
	pub log_level:        log::LevelFilter,
	pub authorization:	  Option<crate::auth::Auth>,
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use crate::constants::FESTIVAL_CLI_CONFIG;

	#[test]
	fn default() {
		let t1: ConfigBuilder = toml_edit::de::from_str(&FESTIVAL_CLI_CONFIG).unwrap();
		let t1 = t1.build_and_set();
		let t2 = config();

		assert_eq!(t1, t2);
	}
}
