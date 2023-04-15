//---------------------------------------------------------------------------------------------------- Use
use crate::collection::Collection;
use crate::kernel::KernelState;

//---------------------------------------------------------------------------------------------------- General Strings
/// Festival Version
///
/// This is the version of the `Festival`'s internals.
///
/// It uses `CARGO_PKG_VERSION`, or `version` found in `Cargo.toml`.
pub const FESTIVAL_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

/// Festival + Version
///
/// Just a string concatenating "Festival" and the current version, e.g: `Festival v1.0.0`
pub const FESTIVAL_NAME_VER: &str = concat!("Festival v", env!("CARGO_PKG_VERSION"));

/// "Festival", as a `&'static str`
pub const FESTIVAL: &str = "Festival";

/// Current `git` commit of `festival`
pub const COMMIT: &str = include_str!("commit");

/// Build profile (debug/release)
///
/// This is `Debug` is `debug_assertions` is detected, else it is `Release`.
pub const BUILD: &str = if cfg!(debug_assertions) { "Debug" } else { "Release" };

/// Copyright notice
///
/// Festival's copyright.
pub const COPYRIGHT: &str =
r#"Festival is licensed under the MIT License.
For more information on the project, see link below:
<https://github.com/hinto-janai/festival>"#;

/// Logging separator
///
/// This is used in logging to visually separate some things.
pub const DASH: &str = "--------------------------------------------";

/// Unique `Bincode` header
///
/// The `24` unique bytes our `Bincode` files will start with.
///
/// It is the UTF-8 encoded string `-----BEGIN FESTIVAL-----` as bytes.
///
/// The next byte _should_ be our `VERSION`, then our actual data.
pub const FESTIVAL_HEADER: [u8; 24] = [
	45, 45, 45, 45, 45,             // -----
	66, 69, 71, 73, 78,             // BEGIN
	32,                             //
	70, 69, 83, 84, 73, 86, 65, 76, // FESTIVAL
	45, 45, 45, 45, 45              // -----
];

/// Current major version of the [`Collection`]
pub const COLLECTION_VERSION: u8 = 1;

/// Current major version of the [`KernelState`]
pub const STATE_VERSION: u8 = 1;

// Log messages.
pub(crate) const OK:   &str = " ... \x1b[1;92mOK\x1b[0m";
pub(crate) const SKIP: &str = " ... \x1b[1;97mSKIP\x1b[0m";
pub(crate) const FAIL: &str = " ... \x1b[1;91mFAIL\x1b[0m";

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use crate::constants::*;

	#[test]
	fn version_is_semver() {
		assert_eq!(FESTIVAL_VERSION.len(), 6);
	}

	#[test]
	fn git_commit_eq_or_gt_40_chars() {
		assert!(COMMIT.len() >= 40);
	}
}
