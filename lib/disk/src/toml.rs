//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Toml
/// `TOML` file format
pub trait Toml: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_string!("toml");

	// Required functions for generic-ness.
	#[inline(always)]
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		Ok(Self::to_string(self)?.into_bytes())
	}
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(toml_edit::de::from_slice(bytes))
	}

	// TOML operations.
	#[inline(always)]
	/// This uses [`toml_edit::ser::to_string_pretty`];
	fn to_string(&self) -> Result<String, anyhow::Error> {
		common::convert_error(toml_edit::ser::to_string_pretty(self))
	}
	#[inline(always)]
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		common::convert_error(toml_edit::de::from_str(string))
	}
}

/// Quickly implement the [`Toml`] trait.
#[macro_export]
macro_rules! toml_file {
	($type:ty, $dir:expr, $project_directory:literal, $sub_directories:literal, $file_name:literal) => {
		const_assert!(const_format!("{}", $project_directory).len() != 0);
		const_assert!(const_format!("{}", $file_name).len() != 0);
 		impl Toml for $type {
			const OS_DIRECTORY: Dir = $dir;
			const PROJECT_DIRECTORY: &'static str = $project_directory;
			const SUB_DIRECTORIES: &'static str = $sub_directories;
			const FILE_NAME: &'static str = const_format!("{}.{}", $file_name, "toml");
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
