//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use super::AlbumKey;
use std::path::PathBuf;
use human::HumanTime;

//----------------------------------------------------------------------------------------------------
//#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[derive(Serialize,Deserialize)]
pub(crate) struct Song {
	// User-facing data.
	pub(crate) title: String,
	pub(crate) album: AlbumKey,
	pub(crate) length_human: HumanTime,  //
	pub(crate) track_number: usize,      //
	pub(crate) track_artists: String,    //

	// "Raw" data.
	pub(crate) length: f32,
	pub(crate) path: PathBuf, //
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
