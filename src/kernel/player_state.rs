//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};

//---------------------------------------------------------------------------------------------------- PlayerState
//__DISK__file!(PlayerState, Dir::Data, "", "", "");
//#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct PlayerState {
	pub playlists: bool,
	pub queue: bool,
}

impl PlayerState {
	#[inline(always)]
	pub fn dummy() -> Self {
		Self {
			playlists: false,
			queue: false,
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
