//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};

//----------------------------------------------------------------------------------------------------
//#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[derive(Serialize,Deserialize)]
pub struct Song {
	pub title: String,
	pub path: std::path::PathBuf,
	pub index: u32,
	pub runtime: f32,
	#[serde(with = "serde_bytes")]
	pub img: Vec<u8>,
//	pub runtime_human: HumanTime,
	pub artist: String,
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
