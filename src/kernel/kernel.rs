//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
use std::sync::{Arc,RwLock};
use super::player_state::PlayerState;

//---------------------------------------------------------------------------------------------------- Kernel
//__DISK__file!(Kernel, Dir::Data, "", "", "");
struct Kernel {
	player_state: Arc<RwLock<PlayerState>>,
}

impl Kernel {
	#[inline(always)]
	fn new() -> Self {
		Self {
			player_state: Arc::new(RwLock::new(PlayerState::new())),
		}
	}
}

//impl std::default::Default for Kernel {
//	fn default() -> Self {
//		Self::new()
//	}
//}

//impl std::fmt::Display for Kernel {
//	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//		write!(f, "{:?}", self)
//	}
//}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
