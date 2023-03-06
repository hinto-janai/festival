//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure,Error};
use directories::ProjectDirs;
use serde::{Serialize,Deserialize};
use std::path::{Path,PathBuf};

//---------------------------------------------------------------------------------------------------- Constants
pub const DASH: &str = "--------------------------------------------";

//---------------------------------------------------------------------------------------------------- Types of User Dirs
/// The different types of OS directories, provided by [`directories`](https://docs.rs/directories)
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub enum Dir {
	/// |Platform | Value                                                                 | Example                                             |
	/// | ------- | --------------------------------------------------------------------- | --------------------------------------------------- |
	/// | Linux   | `$XDG_CACHE_HOME`/`_project_path_` or `$HOME`/.cache/`_project_path_` | /home/alice/.cache/barapp                           |
	/// | macOS   | `$HOME`/Library/Caches/`_project_path_`                               | /Users/Alice/Library/Caches/com.Foo-Corp.Bar-App    |
	/// | Windows | `{FOLDERID_LocalAppData}`\\`_project_path_`\\cache                    | C:\Users\Alice\AppData\Local\Foo Corp\Bar App\cache |
	Project,
	/// |Platform | Value                                                                 | Example                                             |
	/// | ------- | --------------------------------------------------------------------- | --------------------------------------------------- |
	/// | Linux   | `$XDG_CACHE_HOME`/`_project_path_` or `$HOME`/.cache/`_project_path_` | /home/alice/.cache/barapp                           |
	/// | macOS   | `$HOME`/Library/Caches/`_project_path_`                               | /Users/Alice/Library/Caches/com.Foo-Corp.Bar-App    |
	/// | Windows | `{FOLDERID_LocalAppData}`\\`_project_path_`\\cache                    | C:\Users\Alice\AppData\Local\Foo Corp\Bar App\cache |
	Cache,
	/// |Platform | Value                                                                   | Example                                                        |
	/// | ------- | ----------------------------------------------------------------------- | -------------------------------------------------------------- |
	/// | Linux   | `$XDG_CONFIG_HOME`/`_project_path_` or `$HOME`/.config/`_project_path_` | /home/alice/.config/barapp                                     |
	/// | macOS   | `$HOME`/Library/Application Support/`_project_path_`                    | /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App  |
	/// | Windows | `{FOLDERID_RoamingAppData}`\\`_project_path_`\\config                   | C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config         |
	Config,
	/// This is the default value.
	///
	/// |Platform | Value                                                                      | Example                                                       |
	/// | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------- |
	/// | Linux   | `$XDG_DATA_HOME`/`_project_path_` or `$HOME`/.local/share/`_project_path_` | /home/alice/.local/share/barapp                               |
	/// | macOS   | `$HOME`/Library/Application Support/`_project_path_`                       | /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App |
	/// | Windows | `{FOLDERID_RoamingAppData}`\\`_project_path_`\\data                        | C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\data          |
	#[default]
	Data,
	/// |Platform | Value                                                                      | Example                                                       |
	/// | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------- |
	/// | Linux   | `$XDG_DATA_HOME`/`_project_path_` or `$HOME`/.local/share/`_project_path_` | /home/alice/.local/share/barapp                               |
	/// | macOS   | `$HOME`/Library/Application Support/`_project_path_`                       | /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App |
	/// | Windows | `{FOLDERID_LocalAppData}`\\`_project_path_`\\data                          | C:\Users\Alice\AppData\Local\Foo Corp\Bar App\data            |
	DataLocal,
	/// |Platform | Value                                                                   | Example                                                |
	/// | ------- | ----------------------------------------------------------------------- | ------------------------------------------------------ |
	/// | Linux   | `$XDG_CONFIG_HOME`/`_project_path_` or `$HOME`/.config/`_project_path_` | /home/alice/.config/barapp                             |
	/// | macOS   | `$HOME`/Library/Preferences/`_project_path_`                            | /Users/Alice/Library/Preferences/com.Foo-Corp.Bar-App  |
	/// | Windows | `{FOLDERID_RoamingAppData}`\\`_project_path_`\\config                   | C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config |
	Preference,
}

//---------------------------------------------------------------------------------------------------- Common Functions.
#[inline(always)]
// Create the `ProjectDirs` struct from a project name.
pub(crate) fn base(project_name: &str) -> Result<ProjectDirs, Error> {
	match ProjectDirs::from("", "", project_name) {
		Some(p) => Ok(p),
		None    => Err(anyhow!("User directories could not be found")),
	}
}

// Get the absolute OS + Project PATH.
pub(crate) fn get_projectdir(dir: &Dir, project_name: &str) -> Result<PathBuf, Error> {
	let project_dir = base(project_name)?;

	use Dir::*;
	Ok(match &dir {
		Project    => project_dir.project_path(),
		Cache      => project_dir.cache_dir(),
		Config     => project_dir.config_dir(),
		Data       => project_dir.data_dir(),
		DataLocal  => project_dir.data_local_dir(),
		Preference => project_dir.preference_dir(),
	}.to_path_buf())
}

#[inline(always)]
// Some errors don't work with `anyhow` since they don't implement `std::error::Error`
// but they usually do implement `Display`, so use that and rewrap the `Result`.
pub(crate) fn convert_error<T, E: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static>(result: Result<T, E>) -> Result<T, Error> {
	match result {
		Ok(t)  => Ok(t),
		Err(e) => Err(anyhow!(e)),
	}
}

#[inline(always)]
// Assert PATH isn't empty.
pub(crate) fn handle_empty_path(path: &str, err: &str) -> Result<(), Error> {
	if path.is_empty() { bail!("Aborting: Empty PATH for {}", err) }

	Ok(())
}

#[inline(always)]
// Assert PATH is safe (absolute).
pub(crate) fn assert_safe_path(path: &Path) -> Result<(), Error> {
	if !path.is_absolute() { bail!("Aborting: dangerous PATH detected") }

	Ok(())
}

//---------------------------------------------------------------------------------------------------- impl_common
// Implements common methods for all traits.
// The actual traits themselves need to implement:
//   - to_bytes()
//   - from_bytes()
//
macro_rules! impl_common {
	($file_ext:literal) => {
		/// Which OS directory it will be saved in.
		const OS_DIRECTORY: common::Dir;
		/// What the main project directory will be.
		const PROJECT_DIRECTORY: &'static str;
		/// Optional sub directories in between the project directory and file.
		const SUB_DIRECTORIES: &'static str;
		/// What the filename will be.
		const FILE_NAME: &'static str;

		#[inline(always)]
		/// Read the file directly as bytes.
		fn read_to_bytes() -> Result<Vec<u8>, anyhow::Error> {
			Ok(std::fs::read(Self::absolute_path()?)?)
		}

		/// Read the file directly as bytes, and attempt `gzip` decompression.
		///
		/// This assumes the file is suffixed with `.gz`, for example:
		/// ```
		/// config.json    // What `.read_to_bytes()` will look for
		/// config.json.gz // What `.read_to_bytes_gzip()` will look for
		/// ```
		fn read_to_bytes_gzip() -> Result<Vec<u8>, anyhow::Error> {
			use std::io::prelude::*;
			use flate2::read::GzDecoder;

			// Buffer to store decompressed bytes.
			let mut buf = Vec::new();

			// Decode compressed file bytes into buffer.
			GzDecoder::new(
				&std::fs::read(Self::absolute_path_gzip()?)?[..]
			).read_to_end(&mut buf)?;

			Ok(buf)
		}

		#[inline(always)]
		/// Check if the file exists.
		///
		/// `true`  == The file exists.
		/// `false` == The file does not exist.
		/// `anyhow::Error` == There was an error, existance is unknown.
		fn exists() -> Result<bool, anyhow::Error> {
			let path = Self::absolute_path()?;

			Ok(path.exists())
		}

		#[inline(always)]
		/// Same as `Self::exists()` but checks if the `gzip` file exists.
		///
		/// - `Self::exists()` checks for `file.toml`.
		/// - `Self::exists_gzip()` checks for`file.toml.gz`.
		fn exists_gzip() -> Result<bool, anyhow::Error> {
			let path = format!("{}.gz", Self::absolute_path()?.display());

			Ok(PathBuf::from(path).exists())
		}

		#[inline(always)]
		/// Read the file directly as bytes and turn into a Rust structure.
		fn from_file() -> Result<Self, anyhow::Error> {
			Ok(Self::from_bytes(&Self::read_to_bytes()?)?)
		}

		#[inline(always)]
		/// Read the file directly as bytes, decompress with `gzip` and turn into a Rust structure.
		fn from_file_gzip() -> Result<Self, anyhow::Error> {
			Ok(Self::from_bytes(&Self::read_to_bytes_gzip()?)?)
		}

		#[inline]
		/// Create the directories leading up-to the file.
		///
		/// This is not necessary when using any variant of
		/// `Self::write()` as the directories are created implicitly.
		fn create_dir() -> Result<(), anyhow::Error> {
			Ok(std::fs::create_dir_all(Self::base_path()?)?)
		}

		/// Try writing a Rust structure as a file.
		///
		/// Calling this will automatically create the directories leading up to the file.
		fn write(&self) -> Result<(), anyhow::Error> {
			// Create PATH.
			let mut path = Self::base_path()?;
			std::fs::create_dir_all(&path)?;
			path.push(Self::FILE_NAME);

			// Write.
			std::fs::write(path, self.into_writable_fmt()?)?;
			Ok(())
		}

		/// Try writing a Rust structure as a compressed file using `gzip`.
		///
		/// This will suffix the file with `.gz`, for example:
		/// ```
		/// config.json    // Normal file name with `.write()`
		/// config.json.gz // File name when using `.write_gzip()`
		/// ```
		///
		/// Calling this will automatically create the directories leading up to the file.
		fn write_gzip(&self) -> Result<(), anyhow::Error> {
			use std::io::prelude::*;
			use flate2::Compression;
			use flate2::write::GzEncoder;

			// Create PATH.
			let mut path = Self::base_path()?;
			std::fs::create_dir_all(&path)?;
			path.push(format!("{}.gz", Self::FILE_NAME));

			// Compress bytes and write.
			let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
			encoder.write_all(&self.to_bytes()?[..])?;
			std::fs::write(path, encoder.finish()?)?;

			Ok(())
		}

		/// **Note: This may not truely be atomic on Windows.**
		///
		/// Try writing a Rust structure to a TEMPORARY file first, then renaming it to the associated file.
		///
		/// This lowers the chance for data corruption on interrupt.
		///
		/// The temporary file is removed if the rename fails.
		///
		/// The temporary file name is: `file_name` + `extension` + `.tmp`, for example:
		/// ```
		/// config.toml     // <- Real file
		/// config.toml.tmp // <- Temporary version
		/// ```
		/// Already existing `.tmp` files will be overwritten.
		///
		/// Calling this will automatically create the directories leading up to the file.
		fn write_atomic(&self) -> Result<(), anyhow::Error> {
			// Create PATH.
			let mut path = Self::base_path()?;
			std::fs::create_dir_all(&path)?;

			// TMP and normal PATH.
			let mut tmp = path.clone();
			tmp.push(format!("{}.tmp", Self::FILE_NAME));
			path.push(Self::FILE_NAME);

			// Write to TMP.
			if let Err(e) = std::fs::write(&tmp, self.into_writable_fmt()?) {
				std::fs::remove_file(&tmp)?;
				bail!(e);
			}

			// Rename TMP to normal.
			if let Err(e) = std::fs::rename(&tmp, &path) {
				std::fs::remove_file(&tmp)?;
				bail!(e);
			}

			Ok(())
		}

		/// Combines [`Self::write_gzip()`] and [`Self::write_atomic()`].
		fn write_atomic_gzip(&self) -> Result<(), anyhow::Error> {
			use std::io::prelude::*;
			use flate2::Compression;
			use flate2::write::GzEncoder;

			// Create PATH.
			let mut path = Self::base_path()?;
			std::fs::create_dir_all(&path)?;

			// Create TMP and normal.
			let mut tmp = path.clone();
			tmp.push(format!("{}.gz.tmp", Self::FILE_NAME));
			path.push(format!("{}.gz", Self::FILE_NAME));

			// Compress bytes.
			let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
			encoder.write_all(&self.to_bytes()?[..])?;

			// Write to TMP.
			if let Err(e) = std::fs::write(&tmp, &encoder.finish()?) {
				std::fs::remove_file(&tmp)?;
				bail!(e);
			}

			// Rename TMP to normal.
			if let Err(e) = std::fs::rename(&tmp, &path) {
				std::fs::remove_file(&tmp)?;
				bail!(e);
			}

			Ok(())
		}

		/// Try deleting the current file associated with the Rust structure.
		///
		/// This will return success if the file doesn't exist or if deleted.
		///
		/// It will return failure if the file existed but could not be deleted or if any other error occurs.
		fn remove() -> Result<(), anyhow::Error> {
			let mut path = Self::base_path()?;
			path.push(Self::FILE_NAME);

			if !path.exists() { return Ok(()) }

			Ok(std::fs::remove_file(path)?)
		}

		/// **Note: This may not truely be atomic on Windows.**
		///
		/// Rename the associated file before attempting to delete it.
		///
		/// This lowers the chance for data corruption on interrupt.
		///
		/// The temporary file name is: `file_name` + `extension` + `.tmp`, for example:
		/// ```
		/// config.toml     // <- Real file
		/// config.toml.tmp // <- Temporary version
		/// ```
		/// Already existing `.tmp` files will be overwritten.
		fn remove_atomic(&self) -> Result<(), anyhow::Error> {
			let mut path = Self::base_path()?;

			let mut tmp = path.clone();
			tmp.push(format!("{}.tmp", Self::FILE_NAME));
			path.push(Self::FILE_NAME);

			if !path.exists() { return Ok(()) }

			std::fs::rename(&path, &tmp)?;
			std::fs::remove_file(&tmp)?;

			Ok(())
		}

		/// Same as [`Self::remove_atomic()`] but looks for the `.gz` extension.
		fn remove_atomic_gzip(&self) -> Result<(), anyhow::Error> {
			let mut path = Self::base_path()?;

			let mut tmp = path.clone();
			tmp.push(format!("{}.tmp", Self::FILE_NAME));
			path.push(format!("{}.gz", Self::FILE_NAME));

			if !path.exists() { return Ok(()) }

			std::fs::rename(&path, &tmp)?;
			std::fs::remove_file(&tmp)?;

			Ok(())
		}

		/// Try deleting any leftover `.tmp` files from [`Self::write_atomic()`] or [`Self::write_atomic_gzip()`]
		///
		/// This will return success if the files don't exist or if deleted.
		///
		/// It will return failure if files existed but could not be deleted or if any other error occurs.
		fn remove_tmp() -> Result<(), anyhow::Error> {
			let mut tmp = Self::base_path()?;
			let mut gzip = tmp.clone();

			tmp.push(format!("{}.tmp", Self::FILE_NAME));
			gzip.push(format!("{}.gz.tmp", Self::FILE_NAME));

			if !tmp.exists() && !gzip.exists() { return Ok(()) }

			std::fs::remove_file(tmp)?;
			std::fs::remove_file(gzip)?;
			Ok(())
		}

		#[inline(always)]
		/// The main project directory.
		///
		/// You can also access this directly on your type:
		/// ```
		/// assert!(Data::project_directory() == Data::PROJECT_DIRECTORY);
		/// ```
		fn project_directory() -> &'static str {
			Self::PROJECT_DIRECTORY
		}

		#[inline(always)]
		/// The directories after the main project directory, before the file. (the first directory specified in the SUB_DIRECTORIES constant).
		///
		/// You can also access this directly on your type:
		/// ```
		/// assert!(Data::sub_dirs() == Data::SUB_DIRECTORIES);
		/// ```
		fn sub_directories() -> &'static str {
			Self::SUB_DIRECTORIES
		}

		#[inline(always)]
		/// The filename + extension associated with this struct.
		///
		/// You can also access this directly on your type:
		/// ```
		/// assert!(Data::file_name() == Data::FILE_NAME);
		/// ```
		fn file_name() -> &'static str {
			Self::FILE_NAME
		}

		/// The base path associated with this struct (PATH leading up to the file).
		fn base_path() -> Result<PathBuf, anyhow::Error> {
			// PATH safety checks.
			common::handle_empty_path(Self::PROJECT_DIRECTORY, "Project directory")?;
			common::handle_empty_path(Self::FILE_NAME, "File name")?;

			// Get a `ProjectDir` from our project name.
			let mut base = common::get_projectdir(&Self::OS_DIRECTORY, &Self::PROJECT_DIRECTORY)?.to_path_buf();

			// Append sub directories (if any).
			if Self::SUB_DIRECTORIES.len() != 0 {
				#[cfg(target_os = "windows")]
				Self::SUB_DIRECTORIES.split_terminator(&['/', '\\'][..]).for_each(|dir| base.push(dir));
				#[cfg(target_family = "unix")]
				Self::SUB_DIRECTORIES.split_terminator('/').for_each(|dir| base.push(dir));
			}

			Ok(base)
		}

		#[inline(always)]
		/// The absolute PATH of the file associated with this struct.
		fn absolute_path() -> Result<PathBuf, anyhow::Error> {
			let mut base = Self::base_path()?;
			base.push(Self::FILE_NAME);

			common::assert_safe_path(&base)?;

			Ok(base)
		}

		#[inline(always)]
		/// The absolute PATH of the file associated with this struct WITH the `.gz` extension.
		fn absolute_path_gzip() -> Result<PathBuf, anyhow::Error> {
			let mut base = Self::base_path()?;
			base.push(format!("{}.gz", Self::FILE_NAME));

			common::assert_safe_path(&base)?;

			Ok(base)
		}
	}
}
pub(crate) use impl_common;

//---------------------------------------------------------------------------------------------------- impl_string
// Implements common methods on a [String] based trait.
// This automatically implements [impl_common!()].
macro_rules! impl_string {
	($file_ext:literal) => {
		common::impl_common!($file_ext);

		#[inline(always)]
		/// Turn [`Self`] into a [`String`], maintaining formatting if possible.
		fn into_writable_fmt(&self) -> Result<String, anyhow::Error> {
			self.to_string()
		}

		#[inline(always)]
		/// Read the file directly as a [`String`].
		fn read_to_string() -> Result<String, anyhow::Error> {
			Ok(std::fs::read_to_string(Self::absolute_path()?)?)
		}

		#[inline(always)]
		#[cfg(feature = "log")]
		/// Print the file's contents to console surrounded by dashes with the [`log`] crate.
		fn info_dash(string: &str) {
			log::info!("{}", common::DASH);
			string.lines().for_each(|i| log::info!("{}", i));
			log::info!("{}", common::DASH);
		}
	};
}
pub(crate) use impl_string;

//---------------------------------------------------------------------------------------------------- impl_binary
// Implements common methods on a binary based trait.
// This automatically implements `impl_common!()`.
macro_rules! impl_binary {
	($file_ext:literal) => {
		common::impl_common!($file_ext);

		#[inline(always)]
		/// Turn [`Self`] into bytes that can be written to disk.
		fn into_writable_fmt(&self) -> Result<Vec<u8>, anyhow::Error> {
			self.to_bytes()
		}
	};
}
pub(crate) use impl_binary;

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod test {
//  #[test]
//  fn _() {
//  }
//}
