//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::Empty;
use std::path::{Path,PathBuf};
use shukusai::constants::{
	FESTIVAL,FRONTEND_SUB_DIR,
};
use const_format::formatcp;
use rand::{
	Rng,
	distributions::{DistString,Alphanumeric},
};

//---------------------------------------------------------------------------------------------------- TmpZip
pub fn clean_cache() -> Result<(), anyhow::Error> {
	let dir = ArtistZip::sub_dir_parent_path()?;

	if !dir.exists() {
		return Ok(());
	}

	match std::fs::remove_dir(&dir) {
		Ok(_)  => Ok(()),
		Err(e) => Err(e.into()),
	}
}

macro_rules! impl_zip {
	($type:ident, $sub_dir:literal) => {
		disk::empty!($type, disk::Dir::Cache, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/zip/{}", $sub_dir), "tmp");
		#[derive(Debug)]
		pub struct $type {
			pub real: PathBuf,
			pub tmp: PathBuf,
		}

		impl $type {
			pub fn new(input: &str) -> Result<Self, anyhow::Error> {
				let mut real = Self::mkdir()?;

				let mut tmp = Self::absolute_path()?;
				std::fs::create_dir_all(&tmp);
				tmp.push(Alphanumeric.sample_string(&mut rand::thread_rng(), 16));

				real.push(input);

				Ok(Self { real, tmp })
			}

			pub fn exists(&self) -> bool {
				self.real.exists()
			}

			pub fn tmp_to_real(&self) -> std::io::Result<()> {
				std::fs::rename(&self.tmp, &self.real)
			}
		}

		impl Drop for $type {
			fn drop(&mut self) {
				let real = std::mem::take(&mut self.real);
				let tmp  = std::mem::take(&mut self.tmp);

				// Removes the temporary ZIPs.
				tokio::task::spawn(async move {
					if tmp.exists() {
						match tokio::fs::remove_file(&tmp).await {
							Ok(_)  => debug!("Task - Removed tmp: {}", tmp.display()),
							Err(e) => warn!("Task - Failed to remove tmp: {e} ... {}", tmp.display()),
						}
					}
				});

				// Removes the created cached ZIPs `x` seconds _after_ creation.
				tokio::task::spawn(async move {
					tokio::time::sleep(std::time::Duration::from_secs(crate::config::config().cache_time)).await;

					if real.exists() {
						match tokio::fs::remove_file(&real).await {
							Ok(_)  => debug!("Task - Removed cache: {}", real.display()),
							Err(e) => warn!("Task - Failed to remove cache: {e} ... {}", real.display()),
						}
					}
				});
			}
		}
	}
}

impl_zip!(CollectionZip, "collection");
impl_zip!(ArtistZip,     "artist");
impl_zip!(AlbumZip,      "album");
impl_zip!(ArtZip,        "art");

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
