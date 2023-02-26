//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use rmp_serde::{Deserializer, Serializer};

//---------------------------------------------------------------------------------------------------- Rmp
/// [`MessagePack`](https://docs.rs/rmp) (binary) file format
pub trait MessagePack: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_binary!("messagepack");

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(rmp_serde::decode::from_slice(bytes))
	}

	#[inline(always)]
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		common::convert_error(rmp_serde::encode::to_vec(self))
	}
}

/// Quickly implement the [`MessagePack`] trait.
#[macro_export]
macro_rules! messagepack_file {
	($type:ty, $dir:expr, $project_directory:literal, $sub_directories:literal, $file_name:literal) => {
		const_assert!(const_format!("{}", $project_directory).len() != 0);
		const_assert!(const_format!("{}", $file_name).len() != 0);
 		impl MessagePack for $type {
			const OS_DIRECTORY: Dir = $dir;
			const PROJECT_DIRECTORY: &'static str = $project_directory;
			const SUB_DIRECTORIES: &'static str = $sub_directories;
			const FILE_NAME: &'static str = const_format!("{}.{}", $file_name, "messagepack");
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
