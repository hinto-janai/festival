//---------------------------------------------------------------------------------------------------- Use
use const_format::{formatcp,assertcp};
use shukusai::constants::{
	COMMIT,
	SHUKUSAI_NAME_VER,
	COLLECTION_VERSION,
	AUDIO_VERSION,
	OS_ARCH,
};

//---------------------------------------------------------------------------------------------------- Version.
/// `festivald` version
///
/// This is the version of `festivald`, the `daemon`.
pub const FESTIVALD_VERSION: &str = {
	assertcp!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("v", env!("CARGO_PKG_VERSION"))
};

/// `festivald` + version
///
/// Just a string concatenating "festivald" and the current version, e.g: `festivald v0.0.1`
pub const FESTIVALD_NAME_VER: &str = {
	assertcp!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("festivald v", env!("CARGO_PKG_VERSION"))
};

/// - festivald name + version
/// - shukusai name + version
/// - OS + Arch
/// - Git commit hash
/// - Binary struct versions
pub const FESTIVALD_SHUKUSAI_COMMIT: &str = {
	formatcp!(
r#"{FESTIVALD_NAME_VER}
{SHUKUSAI_NAME_VER}
{OS_ARCH}
Collection v{COLLECTION_VERSION}
Audio v{AUDIO_VERSION}
{COMMIT}
"#)
};

//---------------------------------------------------------------------------------------------------- Network
//「祝祭」was released on 2018/04/25.
pub const FESTIVALD_PORT: u16 = 18425;

//---------------------------------------------------------------------------------------------------- Config
pub const FESTIVALD_CONFIG: &str = include_str!("../config/festivald.toml");

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
