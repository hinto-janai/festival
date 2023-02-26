//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Rmp
/// [`Bson`](https://docs.rs/bson) (binary) file format
pub trait Bson: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_binary!("bson");

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(bson::from_slice(bytes))
	}

	#[inline(always)]
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		common::convert_error(bson::to_vec(self))
	}
}

/// Quickly implement the [`Bson`] trait.
#[macro_export]
macro_rules! bson_file {
	($type:ty, $dir:expr, $project_directory:literal, $sub_directories:literal, $file_name:literal) => {
		const_assert!(const_format!("{}", $project_directory).len() != 0);
		const_assert!(const_format!("{}", $file_name).len() != 0);
 		impl Bson for $type {
			const OS_DIRECTORY: Dir = $dir;
			const PROJECT_DIRECTORY: &'static str = $project_directory;
			const SUB_DIRECTORIES: &'static str = $sub_directories;
			const FILE_NAME: &'static str = const_format!("{}.{}", $file_name, "bson");
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
