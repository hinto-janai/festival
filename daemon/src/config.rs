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
	FESTIVALD_PORT,
	FESTIVALD_CONFIG,
};
use strum::{
	AsRefStr,
	Display,
	EnumCount,
	EnumIter,
	EnumString,
	EnumVariantNames,
	IntoStaticStr,
};
use std::net::{
	Ipv4Addr,
	SocketAddrV4,
};
use std::collections::HashSet;
use std::path::PathBuf;
use crate::hash::Hash;
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

// SAFETY: This does not get initialized if there's no `authorization` config.
// This is okay because we will only ever use `.get()`.
pub static AUTH: OnceCell<Hash> = OnceCell::new();

//---------------------------------------------------------------------------------------------------- ConfigBuilder
/// The `struct` that maps value directly from the disk.
///
/// We can't use this directly, but we can transform it into
/// the `Config` we will be using for the rest of the program.
disk::toml!(ConfigBuilder, disk::Dir::Config, FESTIVAL, FRONTEND_SUB_DIR, "festivald");
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub struct ConfigBuilder {
	pub ip:                 Option<Ipv4Addr>,
	pub port:               Option<u16>,
	pub max_connections:    Option<u64>,
	pub exclusive_ips:      Option<HashSet<Ipv4Addr>>,
	pub sleep_on_fail:      Option<u64>,
	pub tls:                Option<bool>,
	pub certificate:        Option<PathBuf>,
	pub key:                Option<PathBuf>,
	pub rest:               Option<bool>,
	pub direct_download:    Option<bool>,
	pub filename_separator: Option<String>,
	pub log_level:          Option<log::LevelFilter>,
	pub watch:              Option<bool>,
	pub media_controls:     Option<bool>,

	// Statics.
	pub authorization:	 Option<String>,
}

impl Default for ConfigBuilder {
	fn default() -> Self {
		Self {
			ip:                 Some(Ipv4Addr::LOCALHOST),
			port:               Some(FESTIVALD_PORT),
			max_connections:    None,
			exclusive_ips:      None,
			sleep_on_fail:      Some(3000),
			tls:                Some(false),
			certificate:        None,
			key:                None,
			rest:               Some(true),
			direct_download:    Some(false),
			filename_separator: Some(" - ".to_string()),
			log_level:          Some(log::LevelFilter::Info),
			watch:              Some(true),
			media_controls:     Some(true),
			authorization:      None,
		}
	}
}

impl ConfigBuilder {
	// INVARIANT: must be called once and only once.
	// Sets `CONFIG`, and returns a ref.
	pub fn build_and_set(self) -> &'static Config {
		let ConfigBuilder {
			ip,
			port,
			max_connections,
			exclusive_ips,
			sleep_on_fail,
			tls,
			certificate,
			key,
			rest,
			direct_download,
			filename_separator,
			log_level,
			watch,
			media_controls,
			authorization,
		} = self;

		macro_rules! get {
			($option:expr, $field:literal, $default:expr) => {
				match $option {
					Some(v) => v,
					_ => {
						warn!("missing config [{}], using default [{}]", $field, $default);
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

		let mut c = Config {
			ip:                 get!(ip,                 "ip",                 Ipv4Addr::LOCALHOST),
			port:               get!(port,               "port",               FESTIVALD_PORT),
			max_connections:    sum!(max_connections,    "max_connections",    Some(4)),
			exclusive_ips:      sum!(exclusive_ips,      "exclusive_ips",      None::<HashSet<Ipv4Addr>>),
			sleep_on_fail:      sum!(sleep_on_fail,      "sleep_on_fail",      Some(3000)),
			tls:                get!(tls,                "tls",                false),
			certificate:        sum!(certificate,        "certificate",        None::<PathBuf>),
			key:                sum!(key,                "key",                None::<PathBuf>),
			rest:               get!(rest,               "rest",               true),
			direct_download:    get!(direct_download,    "direct_download",    false),
			filename_separator: get!(filename_separator, "filename_separator", " - ".to_string()),
			log_level:          get!(log_level,          "log_level",          log::LevelFilter::Info),
			watch:              get!(watch,              "watch",              true),
			media_controls:     get!(media_controls,     "media_controls",     true),
		};

		if c.max_connections == Some(0) {
			c.max_connections = None;
		}

		// FIXME TODO: testing.
//		c.tls = true;
//		c.certificate = Some(PathBuf::from("/tmp/cert.pem"));
//		c.key = Some(PathBuf::from("/tmp/key.pem"));
//		let authorization = Some("my_username:my_password".to_string());

		if let Some(ref hs) = c.exclusive_ips {
			if hs.is_empty() ||  hs.contains(&Ipv4Addr::UNSPECIFIED) {
				c.exclusive_ips = None;
			}
		}

		if let Some(ref cert) = c.certificate {
			if cert.as_os_str().is_empty() {
				warn!("TLS certificate is empty PATH, ignoring");
				c.certificate = None;
			} else if !cert.exists() {
				crate::exit!("TLS certificate [{}] does not exist", cert.display());
			}
		}

		if let Some(ref key) = c.key {
			if key.as_os_str().is_empty() {
				warn!("TLS key is empty PATH, ignoring");
				c.key = None;
			} else if !key.exists() {
				crate::exit!("TLS key [{}] does not exist", key.display());
			}
		}

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
			} else if !c.tls || c.certificate.is_none() || c.key.is_none() {
				crate::exit!("[authorization] field was provided but TLS is not enabled, exiting for safety");
			} else {
				// Base64 encode before hashing.
				// This means we don't parse + decode every HTTP input,
				// instead, we just hash it assuming it is in the correct
				// `Basic <BASE64_ENCODED_USER_PASS>` format, then we
				// can just directly compare with this.
				let s = crate::base64::encode_with_authorization_basic_header(s);

				// SAFETY: unwrap is okay, we only set `AUTH` here.
				AUTH.set(Hash::new(s)).unwrap();
			}
		} else {
			warn!("missing config [authorization], skipping");
		}

		debug!("{DASH} Configuration");
		for line in format!("{c:#?}").lines() {
			debug!("{line}");
		}
		debug!("Authorization: {}", AUTH.get().is_some());
		debug!("{DASH} Configuration");

		// SAFETY: unwrap is okay, we only set `CONFIG` here.
		CONFIG.set(c).unwrap();
		config()
	}

	// Read from disk, or create a default.
	pub fn file_or() -> Self {
		use disk::Toml;
		match Self::from_file() {
			Ok(c)  => { ok!("festivald.conf ... from disk"); c },
			Err(e) => {
				warn!("festivald.conf ... failed from disk: {e}, returning default");

				if let Ok(p) = Config::absolute_path() {
					let _ = Config::mkdir();
					let _ = std::fs::write(&p, FESTIVALD_CONFIG);
				}

				Self::default()
			},
		}
	}
}

//---------------------------------------------------------------------------------------------------- Config
/// The actual `struct` we will use for the whole program.
///
/// The global immutable copy the whole program will refer
/// to is the static `CONFIG` in this module. Or, `config()`.
disk::toml!(Config, disk::Dir::Config, FESTIVAL, FRONTEND_SUB_DIR, "festivald");
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub struct Config {
	pub ip:                 std::net::Ipv4Addr,
	pub port:               u16,
	pub max_connections:    Option<u64>,
	pub exclusive_ips:      Option<HashSet<Ipv4Addr>>,
	pub sleep_on_fail:      Option<u64>,
	pub tls:                bool,
	pub certificate:        Option<PathBuf>,
	pub key:                Option<PathBuf>,
	pub rest:               bool,
	pub direct_download:    bool,
	pub filename_separator: String,
	pub log_level:          log::LevelFilter,
	pub watch:              bool,
	pub media_controls:     bool,
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use crate::constants::FESTIVALD_CONFIG;

	#[test]
	fn default() {
		let t1: ConfigBuilder = toml_edit::de::from_str(&FESTIVALD_CONFIG).unwrap();
		let t1 = t1.build_and_set();
		let t2 = config();

		assert_eq!(t1, t2);
	}
}
