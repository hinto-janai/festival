//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use super::{
	tab::Tab,
	constants::ALBUM_ART_DEFAULT_SIZE,
};
use std::path::PathBuf;
//use disk::prelude::*;
//use disk::{Toml,toml_file};

//---------------------------------------------------------------------------------------------------- State
/// `GUI`'s State.
///
/// Holds user-mutable `GUI` state.
//toml_file!(Settings, Dir::Config, "Festival", "", "settings");
#[derive(Clone,Debug,Default,PartialEq,PartialOrd,Serialize,Deserialize)]
pub(super) struct State {
	/// Which [`Tab`] are currently on?
	pub(super) tab: Tab,
}

impl State {
	pub(super) fn new() -> Self {
		Self {
			..Default::default()
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod test {
//  #[test]
//  fn _() {
//  }
//}
