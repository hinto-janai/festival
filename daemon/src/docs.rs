//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use disk::Empty;
use std::path::{Path,PathBuf};
use shukusai::constants::{
	FESTIVAL,FRONTEND_SUB_DIR,
};
use const_format::formatcp;
use once_cell::sync::OnceCell;

//---------------------------------------------------------------------------------------------------- Docs
disk::empty!(Docs, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/docs"), "__docs");
#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct Docs;

const DOCS_ZIP: &[u8] = include_bytes!("../mdbook/docs.zip");
pub static DOCS_PATH: OnceCell<PathBuf> = OnceCell::new();

impl Docs {
	pub fn create() -> Result<PathBuf, anyhow::Error> {
		let mut path = Self::base_path()?;

		std::fs::remove_dir_all(&path)?;

		let mut zip = zip::ZipArchive::new(std::io::Cursor::new(DOCS_ZIP))?;

		// The `ZIP` contains `/docs`, so pop it out.
		path.pop();
		zip.extract(&path)?;
		path.push("docs");

		Ok(path)
	}

	pub fn create_open() -> Result<(), anyhow::Error> {
		match crate::docs::Docs::create() {
			Ok(mut path) => {
				path.push("index.html");
				Ok(open::that_detached(path)?)
			},
			Err(e) => Err(e),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
