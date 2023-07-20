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

//---------------------------------------------------------------------------------------------------- ConfigBuilder
disk::toml!(Config, disk::Dir::Config, FESTIVAL, FRONTEND_SUB_DIR, "festivald");
#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct ConfigBuilder {
	pub ip:              Option<Ipv4Addr>,
	pub port:            Option<u16>,
	pub log_level:       Option<log::LevelFilter>,
	pub log_daemon_only: Option<bool>,
	pub watch:           Option<bool>,
	pub media_controls:  Option<bool>,
	pub password:        Option<String>,
}

impl ConfigBuilder {
	fn build(self) -> Config {
		let ConfigBuilder {
			ip,
			port,
			log_level,
			log_daemon_only,
			watch,
			media_controls,
			password,
		} = self;

		macro_rules! get {
			($option:expr, $field:literal, $default:expr) => {
				match $option {
					Some(v) => v,
					_ => {
						warn!("missing config {}, using default: {}", $field, $default);
						$default
					},
				}
			}
		}

		Config {
			ip:              get!(ip,              "ip",              Ipv4Addr::LOCALHOST),
			port:            get!(port,            "port",            FESTIVALD_PORT),
			log_level:       get!(log_level,       "log-level",       log::LevelFilter::Info),
			log_daemon_only: get!(log_daemon_only, "log-daemon-only", false),
			watch:           get!(watch,           "watch",           true),
			media_controls:  get!(media_controls,  "media_controls",  true),
			password:        get!(password,        "password",        String::from("")),
		}
	}
}

impl Default for ConfigBuilder {
	fn default() -> Self {
		Self {
			ip:              Some(Ipv4Addr::LOCALHOST),
			port:            Some(FESTIVALD_PORT),
			log_level:       Some(log::LevelFilter::Info),
			log_daemon_only: Some(false),
			watch:           Some(true),
			media_controls:  Some(true),
			password:        Some("".to_string()),
		}
	}
}

//---------------------------------------------------------------------------------------------------- Config
#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct Config {
	pub ip:              std::net::Ipv4Addr,
	pub port:            u16,
	pub log_level:       log::LevelFilter,
	pub log_daemon_only: bool,
	pub watch:           bool,
	pub media_controls:  bool,
	pub password:        String,
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use crate::constants::FESTIVALD_CONFIG;

	#[test]
	fn default() {
		let toml: ConfigBuilder = toml_edit::de::from_str(&FESTIVALD_CONFIG).unwrap();
		assert_eq!(toml, ConfigBuilder::default());
	}

	#[test]
	fn build() {
		let t1: ConfigBuilder = toml_edit::de::from_str(&FESTIVALD_CONFIG).unwrap();
		let t2 = t1.clone().build();
		assert_eq!(t1.ip.unwrap(),              t2.ip);
		assert_eq!(t1.port.unwrap(),            t2.port);
		assert_eq!(t1.log_level.unwrap(),       t2.log_level);
		assert_eq!(t1.log_daemon_only.unwrap(), t2.log_daemon_only);
		assert_eq!(t1.watch.unwrap(),           t2.watch);
		assert_eq!(t1.media_controls.unwrap(),  t2.media_controls);
		assert_eq!(t1.password.unwrap(),        t2.password);
	}
}
