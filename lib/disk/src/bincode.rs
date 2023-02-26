//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
use bincode::config::*;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Bincode
lazy_static::lazy_static! {
	pub static ref ENCODING_OPTIONS: WithOtherIntEncoding<DefaultOptions, VarintEncoding> =
		bincode::DefaultOptions::new().with_varint_encoding();
}


/// [`Bincode`](https://docs.rs/bincode) (binary) file format
pub trait Bincode: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_binary!("bincode");

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		let len = bytes.len();

		// Ensure our `[u8; 25]` HEADER + VERSION bytes are there.
		if len < 25 {
			bail!("Invalid Bincode header data, total byte length less than 25: {}", len);
		}

		common::convert_error(ENCODING_OPTIONS.deserialize(&bytes[25..]))
	}

	#[inline(always)]
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let mut vec = common::convert_error(ENCODING_OPTIONS.serialize(self))?;

		let mut bytes = self.header_version_bytes().to_vec();
		bytes.append(&mut vec);

		Ok(bytes)
	}

	// Bincode specific.
	/// A custom 24-byte length identifying header for your Bincode file.
	///
	/// This is combined with [`Self::VERSION`] to prefix your file with 25 bytes.
	///
	/// **Note: [`Self::write_gzip()`] applies compression AFTER, meaning the entire file must be decompressed to get these headers.**
	const HEADER: [u8; 24];
	/// What the version byte will be (0-255).
	const VERSION: u8;

	#[inline]
	/// Return the 25 bytes header bytes.
	///
	/// First 24 bytes are the [`Self::HEADER`] bytes.
	///
	/// Last byte is [`Self::VERSION`].
	fn header_version_bytes(&self) -> [u8; 25] {
		[
			Self::HEADER[0],
			Self::HEADER[1],
			Self::HEADER[2],
			Self::HEADER[3],
			Self::HEADER[4],
			Self::HEADER[5],
			Self::HEADER[6],
			Self::HEADER[7],
			Self::HEADER[8],
			Self::HEADER[9],
			Self::HEADER[10],
			Self::HEADER[11],
			Self::HEADER[12],
			Self::HEADER[13],
			Self::HEADER[14],
			Self::HEADER[15],
			Self::HEADER[16],
			Self::HEADER[17],
			Self::HEADER[18],
			Self::HEADER[19],
			Self::HEADER[20],
			Self::HEADER[21],
			Self::HEADER[22],
			Self::HEADER[23],
			Self::VERSION
		]
	}

	#[inline(always)]
	/// Read the associated file and attempt to convert the first 24 bytes to a [`String`].
	///
	/// This is useful if your [`Self::HEADER`] should be bytes representing a UTF-8 string.
	fn file_header_to_string(&self) -> Result<String, anyhow::Error> {
		let bytes = Self::file_bytes(0..24)?;

		Ok(String::from_utf8(bytes.to_vec())?)
	}

	#[inline]
	/// Reads the first 24 bytes of the associated file and matches it against [`Self::HEADER`].
	///
	/// If the bytes match, the next byte _should_ be our [`Self::VERSION`] and is returned.
	///
	/// **Note: This only works on a non-compressed version.**
	fn file_version() -> Result<u8, anyhow::Error> {
		use std::io::Read;

		let mut bytes = [0; 25];

		let mut file = std::fs::File::open(Self::absolute_path()?)?;

		file.read_exact(&mut bytes)?;

		if bytes[0..24] == Self::HEADER {
			Ok(bytes[24])
		} else {
			bail!("Bincode header failed to match.\nExpected: {:?}\nFound: {:?}", Self::HEADER, &bytes[0..24]);
		}
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

/// Quickly implement the [`Bincode`] trait.
#[macro_export]
macro_rules! bincode_file {
	($type:ty, $dir:expr, $project_directory:tt, $sub_directories:literal, $file_name:literal, $header:tt, $version:tt) => {
		const_assert!(const_format!("{}", $project_directory).len() != 0);
		const_assert!(const_format!("{}", $file_name).len() != 0);
 		impl Bincode for $type {
			const OS_DIRECTORY: Dir = $dir;
			const PROJECT_DIRECTORY: &'static str = $project_directory;
			const SUB_DIRECTORIES: &'static str = $sub_directories;
			const FILE_NAME: &'static str = const_format!("{}.{}", $file_name, "bincode");
			const HEADER: [u8; 24] = $header;
			const VERSION: u8 = $version;
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
