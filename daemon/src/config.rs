//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Serializer,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::Toml;
use shukusai::constants::{
	FESTIVAL,FRONTEND_SUB_DIR,
};
use crate::constants::{
	FESTIVALD_PORT,
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

//---------------------------------------------------------------------------------------------------- ConfigBuilder
disk::toml!(Config, disk::Dir::Config, FESTIVAL, FRONTEND_SUB_DIR, "festivald");
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub struct ConfigBuilder {
	pub ip:              Option<Ipv4Addr>,
	pub port:            Option<u16>,
	pub max_connections: Option<usize>,
	pub exclusive_ips:   Option<HashSet<Ipv4Addr>>,
	pub tls:             Option<bool>,
	pub certificate:     Option<PathBuf>,
	pub key:             Option<PathBuf>,
	pub rest:            Option<bool>,
	pub direct_download: Option<bool>,
	pub log_level:       Option<log::LevelFilter>,
	pub log_daemon_only: Option<bool>,
	pub watch:           Option<bool>,
	pub media_controls:  Option<bool>,
}

const DEFAULT_CONFIG_BUILDER: ConfigBuilder = ConfigBuilder {
	ip:                Some(Ipv4Addr::LOCALHOST),
	port:              Some(FESTIVALD_PORT),
	max_connections:   None,
	exclusive_ips:     None,
	tls:               Some(false),
	certificate:       None,
	key:               None,
	rest:              Some(true),
	direct_download:   Some(true),
	log_level:         Some(log::LevelFilter::Info),
	log_daemon_only:   Some(false),
	watch:             Some(true),
	media_controls:    Some(true),
};

impl ConfigBuilder {
	pub const fn new() -> Self {
		DEFAULT_CONFIG_BUILDER
	}

	pub fn build(self) -> Config {
		let ConfigBuilder {
			ip,
			port,
			max_connections,
			exclusive_ips,
			tls,
			certificate,
			key,
			rest,
			direct_download,
			log_level,
			log_daemon_only,
			watch,
			media_controls,
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
			ip:                get!(ip,                "ip",                Ipv4Addr::LOCALHOST),
			port:              get!(port,              "port",              FESTIVALD_PORT),
			max_connections:   sum!(max_connections,   "max_connections",   None::<usize>),
			exclusive_ips:     sum!(exclusive_ips,     "exclusive_ips",     None::<HashSet<Ipv4Addr>>),
			tls:               get!(tls,               "tls",               false),
			certificate:       sum!(certificate,       "certificate",       None::<PathBuf>),
			key:               sum!(key,               "key",               None::<PathBuf>),
			rest:              get!(rest,              "rest",              true),
			direct_download:   get!(direct_download,   "direct_download",   false),
			log_level:         get!(log_level,         "log-level",         log::LevelFilter::Info),
			log_daemon_only:   get!(log_daemon_only,   "log-daemon-only",   false),
			watch:             get!(watch,             "watch",             true),
			media_controls:    get!(media_controls,    "media_controls",    true),
		};

		if c.max_connections == Some(0) {
			c.max_connections = None;
		}

		if let Some(ref hs) = c.exclusive_ips {
			if hs.is_empty() {
				c.exclusive_ips = None;
			} else if hs.contains(&Ipv4Addr::UNSPECIFIED) {
				c.exclusive_ips = None;
			}
		}

		if let Some(ref cert) = c.certificate {
			if !cert.exists() {
				c.certificate = None;
			}
		}

		if let Some(ref key) = c.key {
			if !key.exists() {
				c.key = None;
			}
		}

		c
	}
}

impl Default for ConfigBuilder {
	fn default() -> Self {
		DEFAULT_CONFIG_BUILDER
	}
}

//---------------------------------------------------------------------------------------------------- Config
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub struct Config {
	pub ip:                std::net::Ipv4Addr,
	pub port:              u16,
	pub max_connections:   Option<usize>,
	pub exclusive_ips:     Option<HashSet<Ipv4Addr>>,
	pub tls:               bool,
	pub certificate:       Option<PathBuf>,
	pub key:               Option<PathBuf>,
	pub rest:              bool,
	pub direct_download:   bool,
	pub log_level:         log::LevelFilter,
	pub log_daemon_only:   bool,
	pub watch:             bool,
	pub media_controls:    bool,
}

impl Default for Config {
	fn default() -> Self {
		ConfigBuilder::default().build()
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use crate::constants::FESTIVALD_CONFIG;

	#[test]
	fn default() {
		let t1: ConfigBuilder = toml_edit::de::from_str(&FESTIVALD_CONFIG).unwrap();
		let t1 = t1.build();
		let t2 = Config::default();

		assert_eq!(t1, t2);
	}
}
