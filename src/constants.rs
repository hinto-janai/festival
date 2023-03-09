//---------------------------------------------------------------------------------------------------- General Strings
pub(crate) const FESTIVAL_VERSION:  &str = concat!("v", env!("CARGO_PKG_VERSION")); // e.g: v1.0.0
pub(crate) const FESTIVAL_NAME_VER: &str = concat!("Festival v", env!("CARGO_PKG_VERSION")); // e.g: Festival v1.0.0
pub(crate) const FESTIVAL:          &str = "Festival";
pub(crate) const COMMIT:            &str = include_str!("../.git/refs/heads/main");
pub(crate) const BUILD:             &str = if cfg!(debug_assertions) { "Debug" } else { "Release" };

pub(crate) const COPYRIGHT: &str =
r#"Festival is licensed under the MIT License.
For more information on the project, see link below:
<https://github.com/hinto-janai/festival>"#;

pub(crate) const DASH: &str = "--------------------------------------------";

// The `24` unique bytes our `.bincode` files will start with.
// It is the UTF-8 encoded string `-----BEGIN FESTIVAL-----` as bytes.
// The next byte after _should_ be our `xxx_VERSION`, then our actual data.
pub(crate) const FESTIVAL_HEADER: [u8; 24] = [
	45, 45, 45, 45, 45,             // `-----`
	66, 69, 71, 73, 78,             // `BEGIN`
	32,                             // ` `
	70, 69, 83, 84, 73, 86, 65, 76, // `FESTIVAL`
	45, 45, 45, 45, 45              // `-----`
];

// Current major version of the `Collection`.
pub(crate) const COLLECTION_VERSION: u8 = 1;

// Current major version of the `State`.
pub(crate) const STATE_VERSION: u8 = 1;

// Log messages.
pub(crate) const OK: &str   = " ... \x1b[1;92mOK\x1b[0m";
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
