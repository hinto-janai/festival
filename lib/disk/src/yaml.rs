//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;

//---------------------------------------------------------------------------------------------------- Yaml
/// `YAML` file format
pub trait Yaml: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_string!("yml");

	// Required functions for generic-ness.
	#[inline(always)]
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let mut vec = Vec::with_capacity(128);
		serde_yaml::to_writer(&mut vec, self)?;
		Ok(vec)
	}
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_yaml::from_slice(bytes))
	}

	// YAML operations.
	#[inline(always)]
	fn to_string(&self) -> Result<String, anyhow::Error> {
		common::convert_error(serde_yaml::to_string(self))
	}
	#[inline(always)]
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_yaml::from_str(string))
	}
}

/// Quickly implement the [`Yaml`] trait.
#[macro_export]
macro_rules! yaml_file {
	($type:ty, $dir:expr, $project_directory:literal, $sub_directories:literal, $file_name:literal) => {
		const_assert!(const_format!("{}", $project_directory).len() != 0);
		const_assert!(const_format!("{}", $file_name).len() != 0);
 		impl Yaml for $type {
			const OS_DIRECTORY: Dir = $dir;
			const PROJECT_DIRECTORY: &'static str = $project_directory;
			const SUB_DIRECTORIES: &'static str = $sub_directories;
			const FILE_NAME: &'static str = const_format!("{}.{}", $file_name, "yml");
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
