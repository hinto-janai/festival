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
/// `festival-cli` version
///
/// This is the version of `festival-cli`, the `festivald` client.
pub const FESTIVAL_CLI_VERSION: &str = {
	assertcp!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("v", env!("CARGO_PKG_VERSION"))
};

/// `festival-cli` + version
///
/// Just a string concatenating "festival-cli" and the current version, e.g: `festival-cli v0.0.1`
pub const FESTIVAL_CLI_NAME_VER: &str = {
	assertcp!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("festival-cli v", env!("CARGO_PKG_VERSION"))
};

/// - festival-cli name + version
/// - OS + Arch
/// - Git commit hash
pub const FESTIVAL_CLI_SHUKUSAI_COMMIT: &str = {
	formatcp!(
r#"{FESTIVAL_CLI_NAME_VER}
{OS_ARCH}
{COMMIT}
"#)
};

//---------------------------------------------------------------------------------------------------- Subdir
pub const SUB_DIR: &str = "cli";

//---------------------------------------------------------------------------------------------------- Network
//「祝祭」was released on 2018/04/25.
pub const FESTIVAL_CLI_PORT: u16 = 18425;

//---------------------------------------------------------------------------------------------------- Config
pub const FESTIVAL_CLI_CONFIG: &str = include_str!("../config/festival-cli.toml");

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
