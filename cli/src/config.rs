//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Serializer,Deserialize};
use anyhow::anyhow;
use benri::ok;
use log::{error,info,warn,debug,trace};
use disk::Toml;
use shukusai::constants::FESTIVAL;
use crate::constants::{
	SUB_DIR,
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

//---------------------------------------------------------------------------------------------------- Defaults
const DEFAULT_URL: &str = "http://127.0.0.1:18425";
fn default_url() -> String {
	DEFAULT_URL.to_string()
}

const DEFAULT_ID: &str = "festival-cli";
fn default_id() -> json_rpc::Id<'static> {
	json_rpc::Id::Str(Cow::Borrowed(DEFAULT_ID))
}


//---------------------------------------------------------------------------------------------------- ConfigBuilder
/// The `struct` that maps value directly from the disk.
///
/// We can't use this directly, but we can transform it into
/// the `Config` we will be using for the rest of the program.
disk::toml!(ConfigBuilder, disk::Dir::Config, FESTIVAL, SUB_DIR, "festival-cli");
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub struct ConfigBuilder {
	pub festivald:          Option<String>,
	pub timeout:            Option<u64>,
	pub id:                 Option<String>,
	pub authorization:	    Option<String>,
}

impl Default for ConfigBuilder {
	fn default() -> Self {
		Self {
			festivald:          Some(DEFAULT_URL.into()),
			timeout:            Some(0),
			id:                 Some(DEFAULT_ID.into()),
			authorization:      None,
		}
	}
}

impl ConfigBuilder {
	pub fn build(self) -> Config {
		let ConfigBuilder {
			festivald,
			timeout,
			id,
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
//		let festivald = festivald.map(|s| http::uri::Uri::from_str(s.as_str()).unwrap());
		let id = id.map(|s| json_rpc::Id::from(s));

		let timeout = match timeout {
			Some(x) if x == 0 => None,
			Some(x) => Some(std::time::Duration::from_secs(x)),
			_ => None,
		};

		let mut c = Config {
			festivald:          get!(festivald,          "festivald",          default_url()),
			timeout:            sum!(timeout,            "timeout",            None::<std::time::Duration>),
			id:                 get!(id,                 "id",                 default_id()),
			authorization: None,
		};

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

		c
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
					crate::exit!("festival-cli.conf exists but is invalid:\n\n{e}\ntip: use `festival-cli --reset-config` to reset it");
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
			cmd.timeout          => self.timeout,
			cmd.id               => self.id,
			cmd.authorization    => self.authorization
		}
	}
}

//---------------------------------------------------------------------------------------------------- Config
/// The actual `struct` we will use for the whole program.
///
/// The global immutable copy the whole program will refer
/// to is the static `CONFIG` in this module. Or, `config()`.
//disk::toml!(Config, disk::Dir::Config, FESTIVAL, SUB_DIR, "festival-cli");
#[derive(Debug,PartialEq)]
pub struct Config {
	pub festivald:        String,
	pub timeout:          Option<std::time::Duration>,
	pub id:               json_rpc::Id<'static>,
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
