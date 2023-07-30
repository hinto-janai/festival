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
use once_cell::sync::OnceCell;

//---------------------------------------------------------------------------------------------------- Docs
disk::empty!(Docs, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/docs"), "__docs");
#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct Docs;

const DOCS_ZIP: &[u8] = include_bytes!("../docs/docs.zip");
pub static DOCS_PATH: OnceCell<PathBuf> = OnceCell::new();

impl Docs {
	pub fn create() -> Result<PathBuf, anyhow::Error> {
		let mut path = Self::base_path()?;

		let _ = std::fs::remove_dir(&path);

		let mut zip = zip::ZipArchive::new(std::io::Cursor::new(DOCS_ZIP))?;

		// The `ZIP` contains `/docs`, so pop it out.
		path.pop();
		zip.extract(&path)?;
		path.push("docs");

		Ok(path)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
