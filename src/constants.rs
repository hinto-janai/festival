//---------------------------------------------------------------------------------------------------- Use
use crate::collection::Collection;
use crate::audio::AudioState;
use crate::const_assert;

//---------------------------------------------------------------------------------------------------- General Strings
/// Festival Version
///
/// This is the version of the `Festival`'s internals.
///
/// It uses `CARGO_PKG_VERSION`, or `version` found in `Cargo.toml`.
pub const FESTIVAL_VERSION: &str = {
	const_assert!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("v", env!("CARGO_PKG_VERSION"))
};

/// Festival + Version
///
/// Just a string concatenating "Festival" and the current version, e.g: `Festival v1.0.0`
pub const FESTIVAL_NAME_VER: &str = {
	const_assert!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("Festival v", env!("CARGO_PKG_VERSION"))
};

/// Festival's `dbus` connection name.
pub const FESTIVAL_DBUS: &str = "pm.festival.Festival";

/// "Festival", the main project folder.
pub const FESTIVAL: &str = "Festival";
/// "shukusai", the internals sub-directory.
pub const SHUKUSAI: &str = "shukusai";
/// "txt", the internals sub-directory for misc text files.
pub const TXT: &str = "txt";

/// Current `git` commit of `festival`
pub const COMMIT: &str = {
	const_assert!(include_str!("commit").len() != 0, "Commit file is 0 length");
	include_str!("commit")
};

/// Build profile (debug/release)
///
/// This is `Debug` is `debug_assertions` is detected, else it is `Release`.
pub const BUILD: &str = if cfg!(debug_assertions) { "Debug" } else { "Release" };

/// Copyright notice
///
/// Festival's copyright.
pub const COPYRIGHT: &str =
r#"Festival is licensed under the MIT License.
For more information on the project, see below:
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
pub const HEADER: [u8; 24] = [
	45, 45, 45, 45, 45,             // -----
	66, 69, 71, 73, 78,             // BEGIN
	32,                             //
	70, 69, 83, 84, 73, 86, 65, 76, // FESTIVAL
	45, 45, 45, 45, 45              // -----
];

/// [`HEADER`] as a `&'static str`.
pub const HEADER_STR: &str = match std::str::from_utf8(&HEADER) {
	Ok(s)  => s,
	Err(_) => panic!(),
};

/// Current major version of the [`Collection`]
pub const COLLECTION_VERSION: u8 = 0;

/// Current major version of the [`AudioState`]
pub const AUDIO_VERSION: u8 = 0;

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
