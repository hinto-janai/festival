//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use lofty::{
	Accessor,
	TaggedFile,
	TaggedFileExt,
};
use std::path::Path;

pub struct AudioMetadata {
	artist: String,
	album: String,
	title: String,
	track: u32,
	track_total: u32,
	disk: u32,
	disk_total: u32,
}

//---------------------------------------------------------------------------------------------------- Metadata functions.
impl super::Ccd {
	#[inline(always)]
	fn audio_path_to_metadata(path: &Path) -> Result<(), anyhow::Error> {
		let tagged_file = lofty::Probe::open(path)?.guess_file_type()?.read()?;

		let tag = {
			if let Some(t) = tagged_file.primary_tag() {
				t
			} else if let Some(t) = tagged_file.first_tag() {
				t
			} else {
				bail!("No tag");
			}
		};

		

		Ok(())
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
