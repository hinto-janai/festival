//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Toml
/// Plain text file format
///
/// This is a plain text file with no extension.
/// Typically used for small and simple data types like integers, strings, and enums.
pub trait Plain: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_string!("plain");

	// Required functions for generic-ness.
	#[inline(always)]
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		Ok(Self::to_string(self)?.into_bytes())
	}
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		let string = std::str::from_utf8(bytes)?;
		common::convert_error(serde_plain::from_str(string))
	}

	// Plain text operations.
	#[inline(always)]
	/// This uses [`toml_edit::ser::to_string_pretty`];
	fn to_string(&self) -> Result<String, anyhow::Error> {
		common::convert_error(serde_plain::to_string(self))
	}
	#[inline(always)]
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_plain::from_str(string))
	}

	#[inline]
	/// Reads a range of bytes of the associated file of [`Self`].
	fn file_bytes(range: core::ops::Range<u16>) -> Result<Vec<u8>, anyhow::Error> {
		use std::io::Read;
		use std::io::{Seek,SeekFrom};

		let (start, end) = (range.start, range.end);

		let mut len;
		let mut seek;
		if end < start {
			len = start - end;
			seek = SeekFrom::End(end.into());
		} else {
			len = end - start;
			seek = SeekFrom::Start(start.into());
		}

		let mut byte = vec![0; len.into()];

		let mut file = std::fs::File::open(Self::absolute_path()?)?;

		file.seek(seek)?;
		file.read_exact(&mut byte)?;

		Ok(byte)
	}
}

/// Quickly implement the [`Plain`] trait.
#[macro_export]
macro_rules! plain_file {
	($type:ty, $dir:expr, $project_directory:expr, $sub_directories:expr, $file_name:expr) => {
		const_assert!(const_format!("{}", $project_directory).len() != 0);
		const_assert!(const_format!("{}", $file_name).len() != 0);
 		impl Plain for $type {
			const OS_DIRECTORY: Dir = $dir;
			const PROJECT_DIRECTORY: &'static str = $project_directory;
			const SUB_DIRECTORIES: &'static str = $sub_directories;
			const FILE_NAME: &'static str = $file_name;
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
