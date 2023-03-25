//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use super::{
	tab::Tab,
};
use std::path::PathBuf;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use super::constants::{
	STATE_VERSION,
	ALBUM_ART_DEFAULT_SIZE,
};
use shukusai::{
	FESTIVAL,
	FESTIVAL_HEADER,
};

//---------------------------------------------------------------------------------------------------- State
/// `GUI`'s State.
///
/// Holds user-mutable `GUI` state.
bincode_file!(State, Dir::Data, FESTIVAL, "gui", "state", FESTIVAL_HEADER, STATE_VERSION);
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
