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
// Handle to a randomized `PathBuf` where
// temporary zip files can be written.
//
// File is deleted on `drop()`, async.
disk::empty!(TmpZip, disk::Dir::Cache, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/zip"), "tmp_");
#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct TmpZip(PathBuf);

impl TmpZip {
	pub fn new() -> Self {
		let rng = Alphanumeric
			.sample_string(&mut rand::thread_rng(), 28);

		// SAFETY: if we can't access the user path
		// something is really wrong, and we can't
		// do much without the FS anyway, so panic is ok.
		let mut path = Self::absolute_path().unwrap();
		path.push(rng);
		Self(path)
	}

	pub fn path(&self) -> &Path {
		&self.0
	}
}

impl Drop for TmpZip {
	fn drop(&mut self) {
		let path = std::mem::take(&mut self.0);
		tokio::task::spawn(async move {
			tokio::fs::remove_file(path).await
		});
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
