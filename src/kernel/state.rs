//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::collection::{
	CollectionKey,
	CollectionKeychain,
	CollectionSlice,
};
use crate::constants::{
	FESTIVAL,
	FESTIVAL_HEADER,
	STATE_VERSION,
};

//---------------------------------------------------------------------------------------------------- State
bincode_file!(State, Dir::Data, FESTIVAL, "", "state", FESTIVAL_HEADER, STATE_VERSION);
#[derive(Clone,Debug,Default,PartialEq,Serialize,Deserialize)]
pub(crate) struct State {
	// Audio.
	pub(crate) current_key: CollectionKey,
	pub(crate) current_elapsed: f64,

	// Search.
	pub(crate) search_result: CollectionKeychain,

	// Queue/Playlist.
	pub(crate) queue: CollectionSlice,
	pub(crate) playlists: CollectionSlice,
}

impl State {
	#[inline(always)]
	// Create empty struct.
	pub(crate) fn new() -> Self {
		Self {
			current_key: CollectionKey::new(),
			current_elapsed: 0.0,

			search_result: CollectionKeychain::new(),

			queue: CollectionSlice::new(),
			playlists: CollectionSlice::new(),
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
