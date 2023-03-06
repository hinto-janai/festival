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
pub struct Song {
	// User-facing data.
	pub title: String,
	pub album: AlbumKey,
	pub length_human: HumanTime,  //
	pub track_number: usize,      //
	pub track_artists: String,    //

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
