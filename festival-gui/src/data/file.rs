//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use benri::{
//};
//use disk::prelude::*;
//use disk::{};
//use std::{};
use std::sync::{
	Arc,
	Mutex,
	atomic::AtomicBool,
};
use std::path::PathBuf;
use benri::{
	atomic_store,
	lock,
};
use log::info;

//---------------------------------------------------------------------------------------------------- __NAME__
pub fn spawn_rfd_thread(rfd_open: Arc<AtomicBool>, rfd_new: Arc<Mutex<Option<PathBuf>>>) {
	std::thread::spawn(move || {
		atomic_store!(rfd_open, true);

		match rfd::FileDialog::new().set_title("Add folder to the Collection").pick_folder() {
			Some(path) => {
				info!("RFD - Selected PATH: {}", path.display());
				lock!(rfd_new).replace(path);
			},
			None => info!("RFD - No PATH selected"),
		}

		atomic_store!(rfd_open, false);
	});
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
