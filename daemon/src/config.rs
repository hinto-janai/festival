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
use std::collections::BTreeSet;
use std::path::PathBuf;
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
pub static AUTH: OnceCell<rpc::hash::Hash> = OnceCell::new();

//---------------------------------------------------------------------------------------------------- Constants
const LOG_DEFAULT: log::LevelFilter = log::LevelFilter::Error;

//---------------------------------------------------------------------------------------------------- ConfigBuilder
/// The `struct` that maps value directly from the disk.
///
/// We can't use this directly, but we can transform it into
/// the `Config` we will be using for the rest of the program.
disk::toml!(ConfigBuilder, disk::Dir::Config, FESTIVAL, FRONTEND_SUB_DIR, "festivald");
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub struct ConfigBuilder {
	pub ip:                  Option<Ipv4Addr>,
	pub port:                Option<u16>,
	pub max_connections:     Option<u64>,
	pub exclusive_ips:       Option<BTreeSet<Ipv4Addr>>,
	pub sleep_on_fail:       Option<u64>,
	pub collection_paths:    Option<Vec<PathBuf>>,
	pub tls:                 Option<bool>,
	pub certificate:         Option<PathBuf>,
	pub key:                 Option<PathBuf>,
	pub rest:                Option<bool>,
	pub docs:                Option<bool>,
	pub direct_download:     Option<bool>,
	pub filename_separator:  Option<String>,
	pub log_level:           Option<log::LevelFilter>,
	pub watch:               Option<bool>,
	pub cache_clean:         Option<bool>,
	pub cache_time:          Option<u64>,
	pub media_controls:      Option<bool>,
	pub authorization:	     Option<String>,
	pub confirm_no_tls_auth: Option<bool>,
	pub no_auth_rpc:         Option<BTreeSet<rpc::Method>>,
	pub no_auth_rest:        Option<BTreeSet<rpc::resource::Resource>>,
	pub no_auth_docs:        Option<bool>,
}

impl Default for ConfigBuilder {
	fn default() -> Self {
		Self {
			ip:                  Some(Ipv4Addr::LOCALHOST),
			port:                Some(FESTIVALD_PORT),
			max_connections:     None,
			exclusive_ips:       None,
			sleep_on_fail:       Some(3000),
			collection_paths:    Some(vec![]),
			tls:                 Some(false),
			certificate:         None,
			key:                 None,
			rest:                Some(true),
			docs:                Some(true),
			direct_download:     Some(false),
			filename_separator:  Some(" - ".to_string()),
			log_level:           Some(LOG_DEFAULT),
			watch:               Some(true),
			cache_clean:         Some(true),
			cache_time:          Some(3600),
			media_controls:      Some(true),
			authorization:       None,
			confirm_no_tls_auth: Some(false),
			no_auth_rpc:         None,
			no_auth_rest:        None,
			no_auth_docs:        Some(false),
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
			collection_paths,
			tls,
			certificate,
			key,
			rest,
			docs,
			direct_download,
			filename_separator,
			log_level,
			watch,
			cache_clean,
			cache_time,
			media_controls,
			authorization,
			confirm_no_tls_auth,
			no_auth_rpc,
			no_auth_rest,
			no_auth_docs,
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

		let mut c = Config {
			ip:                  get!(ip,                  "ip",                  Ipv4Addr::LOCALHOST),
			port:                get!(port,                "port",                FESTIVALD_PORT),
			max_connections:     sum!(max_connections,     "max_connections",     None::<u64>),
			exclusive_ips:       sum!(exclusive_ips,       "exclusive_ips",       None::<BTreeSet<Ipv4Addr>>),
			sleep_on_fail:       sum!(sleep_on_fail,       "sleep_on_fail",       Some(3000)),
			collection_paths:    get!(collection_paths,    "collection_paths",    if let Some(p) = dirs::audio_dir() { vec![p] } else { Vec::<PathBuf>::with_capacity(0) }),
			tls:                 get!(tls,                 "tls",                 false),
			certificate:         sum!(certificate,         "certificate",         None::<PathBuf>),
			key:                 sum!(key,                 "key",                 None::<PathBuf>),
			rest:                get!(rest,                "rest",                true),
			docs:                get!(docs,                "docs",                true),
			direct_download:     get!(direct_download,     "direct_download",     false),
			filename_separator:  get!(filename_separator,  "filename_separator",  " - ".to_string()),
			log_level:           get!(log_level,           "log_level",           LOG_DEFAULT),
			watch:               get!(watch,               "watch",               true),
			cache_clean:         get!(cache_clean,         "cache_clean",         true),
			cache_time:          get!(cache_time,          "cache_time",          3600),
			media_controls:      get!(media_controls,      "media_controls",      true),
			confirm_no_tls_auth: get!(confirm_no_tls_auth, "confirm_no_tls_auth", false),
			no_auth_rpc:         sum!(no_auth_rpc,         "no_auth_rpc",         None::<BTreeSet<rpc::Method>>),
			no_auth_rest:        sum!(no_auth_rest,        "no_auth_rest",        None::<BTreeSet<rpc::resource::Resource>>),
			no_auth_docs:        get!(no_auth_docs,        "no_auth_docs",        false),
		};

		if c.max_connections == Some(0) {
			c.max_connections = None;
		}

		if c.sleep_on_fail == Some(0) {
			c.sleep_on_fail = None;
		}

		if c.filename_separator.contains("/") {
			crate::exit!("[filename_separator] cannot contain '/', it is the PATH separator for ZIPs");
		}

		// FIXME TODO: testing.
//		c.tls = true;
//		c.certificate = Some(PathBuf::from("../../assets/tls/cert.pem"));
//		c.key = Some(PathBuf::from("../../assets/tls/key.pem"));
//		let authorization = Some("user:pass".to_string());
//		c.no_auth_rpc = Some([rpc::Method::Toggle].into());
//		c.no_auth_rest = Some([rpc::resource::Resource::Song].into());

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
			} else if !c.confirm_no_tls_auth && c.ip != Ipv4Addr::LOCALHOST && (!c.tls || c.certificate.is_none() || c.key.is_none()) {
				crate::exit!("[authorization] field was provided but TLS is not enabled, exiting for safety");
			} else {
				if c.ip == Ipv4Addr::LOCALHOST {
					info!("[authorization] is enabled, TLS is not, but we're binding on [localhost], allowing");
				}

				// Base64 encode before hashing.
				// This means we don't parse + decode every HTTP input,
				// instead, we just hash it assuming it is in the correct
				// `Basic <BASE64_ENCODED_USER_PASS>` format, then we
				// can just directly compare with this.
				let s = rpc::base64::encode_with_authorization_basic_header(s);

				// SAFETY: unwrap is okay, we only set `AUTH` here.
				AUTH.set(rpc::hash::Hash::new(s)).unwrap();
			}
		} else {
			warn!("missing config [authorization], skipping");
		}

		info!("{DASH} Configuration");
		for line in format!("{c:#?}").lines() {
			info!("{line}");
		}
		info!("Authorization: {}", AUTH.get().is_some());
		info!("{DASH} Configuration");

		// SAFETY: unwrap is okay, we only set `CONFIG` here.
		CONFIG.set(c).unwrap();
		config()
	}

	// Read from disk, or create a default.
	pub fn file_or_and_init_logger(log_cmd: Option<log::LevelFilter>) -> Self {
		use disk::Toml;

		match Self::from_file() {
			Ok(c)  => {
				// Set logger, favor command-line.
				let log = match (log_cmd, c.log_level) {
					(Some(l), _) => l,
					(_, Some(l)) => l,
					_ => LOG_DEFAULT,
				};

				// Init logger.
				shukusai::logger::init_logger(log);

				ok!("festivald.conf ... from disk");

				c
			},
			Err(e) => {
				// SAFETY: if we can't get the config, panic is ok.
				let p = Config::absolute_path().unwrap();

				if p.exists() {
					crate::exit!("festivald.conf exists but is invalid:\n\n{e}\ntip: use `festivald data --reset-config` to reset it");
				} else {
					Config::mkdir().unwrap();
					std::fs::write(&p, FESTIVALD_CONFIG).unwrap();
				}

				// Set logger, favor command-line.
				shukusai::logger::init_logger(log_cmd.unwrap_or_else(|| LOG_DEFAULT));

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
			cmd.ip                  => self.ip,
			cmd.port                => self.port,
			cmd.max_connections     => self.max_connections,
			cmd.exclusive_ips       => self.exclusive_ips,
			cmd.sleep_on_fail       => self.sleep_on_fail,
			cmd.collection_paths    => self.collection_paths,
			cmd.tls                 => self.tls,
			cmd.certificate         => self.certificate,
			cmd.key                 => self.key,
			cmd.rest                => self.rest,
			cmd.docs                => self.docs,
			cmd.direct_download     => self.direct_download,
			cmd.filename_separator  => self.filename_separator,
			cmd.log_level           => self.log_level,
			cmd.watch               => self.watch,
			cmd.cache_clean         => self.cache_clean,
			cmd.cache_time          => self.cache_time,
			cmd.media_controls      => self.media_controls,
			cmd.authorization       => self.authorization,
			cmd.confirm_no_tls_auth => self.confirm_no_tls_auth
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
	pub ip:                  std::net::Ipv4Addr,
	pub port:                u16,
	pub max_connections:     Option<u64>,
	pub exclusive_ips:       Option<BTreeSet<Ipv4Addr>>,
	pub sleep_on_fail:       Option<u64>,
	pub collection_paths:    Vec<PathBuf>,
	pub tls:                 bool,
	pub certificate:         Option<PathBuf>,
	pub key:                 Option<PathBuf>,
	pub rest:                bool,
	pub docs:                bool,
	pub direct_download:     bool,
	pub filename_separator:  String,
	pub log_level:           log::LevelFilter,
	pub watch:               bool,
	pub cache_clean:         bool,
	pub cache_time:          u64,
	pub media_controls:      bool,
	pub confirm_no_tls_auth: bool,
	pub no_auth_rpc:         Option<BTreeSet<rpc::Method>>,
	pub no_auth_rest:        Option<BTreeSet<rpc::resource::Resource>>,
	pub no_auth_docs:        bool,
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
