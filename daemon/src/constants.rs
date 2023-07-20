//---------------------------------------------------------------------------------------------------- Use

//---------------------------------------------------------------------------------------------------- Version.
/// `festivald` version
///
/// This is the version of `festivald`, the `daemon`.
pub const FESTIVALD_VERSION: &str = {
	const_assert!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("v", env!("CARGO_PKG_VERSION"))
};

/// `festivald` + version
///
/// Just a string concatenating "festivald" and the current version, e.g: `festivald v0.0.1`
pub const FESTIVALD_NAME_VER: &str = {
	const_assert!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("Festival v", env!("CARGO_PKG_VERSION"))
};

//---------------------------------------------------------------------------------------------------- CONFIG
pub const FESTIVALD_CONFIG: &str = include_str!("../config/festivald.toml");

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
