//---------------------------------------------------------------------------------------------------- Use
use crate::const_assert;

//---------------------------------------------------------------------------------------------------- General Strings
/// `shukusai` version
///
/// This is the version of `Festival`'s internals, `shukusai`.
///
/// It uses `CARGO_PKG_VERSION`, or `version` found in `Cargo.toml`.
pub const SHUKUSAI_VERSION: &str = {
	const_assert!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("v", env!("CARGO_PKG_VERSION"))
};

/// `shukusai` + version
///
/// Just a string concatenating "shukusai" and the current version, e.g: `shukusai v0.0.1`
pub const SHUKUSAI_NAME_VER: &str = {
	const_assert!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("shukusai v", env!("CARGO_PKG_VERSION"))
};

#[cfg(not(target_os = "macos"))]
/// Festival's icon:
/// - `512x512`
/// - `RGBA`
/// - `PNG`
pub const FESTIVAL_ICON: &[u8] = include_bytes!("../../assets/images/icon/512.png");
#[cfg(not(target_os = "macos"))]
/// The height and width of [`FESTIVAL_ICON`].
pub const FESTIVAL_ICON_SIZE: u32 = 512;

#[cfg(target_os = "macos")]
/// Festival's icon:
/// - `1024x1024`
/// - `RGBA`
/// - `PNG`
pub const FESTIVAL_ICON: &[u8] = include_bytes!("../../assets/images/icon/icon@2x.png");
#[cfg(target_os = "macos")]
/// The height and width of [`FESTIVAL_ICON`].
pub const FESTIVAL_ICON_SIZE: u32 = 1024;

/// Festival's `dbus` connection name.
pub const FESTIVAL_DBUS: &str = "pm.festival.Festival";

/// "Festival", the main project folder.
pub const FESTIVAL: &str = "Festival";
/// The main sub-directory within the `festival`
/// directory for each `Frontend`'s files.
#[cfg(feature = "gui")]
pub const FRONTEND_SUB_DIR: &str = "gui";
#[cfg(feature = "daemon")]
pub const FRONTEND_SUB_DIR: &str = "daemon";
#[cfg(feature = "cli")]
pub const FRONTEND_SUB_DIR: &str = "cli";
#[cfg(feature = "web")]
pub const FRONTEND_SUB_DIR: &str = "web";

/// The sub-directory where state is saved.
///
/// This include the [`Collection`] and [`AudioState`].
pub const STATE_SUB_DIR: &str = "state";

/// The sub-directory for image cache in the OS's cache directory.
pub const IMAGE_CACHE_SUB_DIR: &str = "image";

/// The sub-directory for misc text files.
pub const TXT_SUB_DIR: &str = "txt";

/// The sub-directory that is watched for [`crate::signal`]'s.
pub const SIGNAL_SUB_DIR: &str = "signal";

/// Current `git` commit of `festival`
pub const COMMIT: &str = {
	const COMMIT: &str = include_str!("../../.git/refs/heads/main");
	const_assert!(COMMIT.len() != 0, "Commit file is 0 length");
	COMMIT
};

/// Build profile (debug/release)
///
/// This is `Debug` is `debug_assertions` is detected, else it is `Release`.
pub const BUILD: &str = if cfg!(debug_assertions) { "Debug" } else { "Release" };

/// Festival's copyright notice.
///
/// Most (if not all) of Festival's dependencies are `MIT` or`Apache-2.0`,
/// with the major exception being `Symphonia`, which is `MPL-2.0`.
///
/// Under these guidelines: `https://www.mozilla.org/en-US/MPL/2.0/FAQ/`
/// (questions 9-10), we must make modified MPL-2.0 code available,
/// and inform users how they can obtain the source.
pub const COPYRIGHT: &str =
r#"Festival is licensed under the MIT License.
For more information on the project, see below:
<https://github.com/hinto-janai/festival>

Symphonia, the audio decoding/demuxing/metadata
library used by Festival is licensed under MPL-2.0.
For more details and source code, see below:
<https://github.com/pdeljanov/Symphonia>"#;

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
		assert_eq!(SHUKUSAI_VERSION.len(), 6);
	}

	#[test]
	fn git_commit_eq_or_gt_40_chars() {
		assert!(COMMIT.len() >= 40);
	}

	#[test]
	fn icon() {
		let icon = winit::window::Icon::from_rgba(FESTIVAL_ICON.to_vec(), FESTIVAL_ICON_SIZE, FESTIVAL_ICON_SIZE).unwrap();
	}
}
