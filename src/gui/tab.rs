//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use strum::{
	IntoEnumIterator,
};
use strum_macros::{
	EnumIter,
	EnumString,
	IntoStaticStr,
};

//----------------------------------------------------------------------------------------------------
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,IntoStaticStr,EnumString,EnumIter,Serialize,Deserialize)]
#[strum(serialize_all = "PascalCase")]
pub enum Tab {
	#[default]
	Album,
	Artist,
	Queue,
	Rank,
	Stats,
	Search,
	Settings,
}

impl Tab {
	#[inline(always)]
	pub fn as_str(&self) -> &'static str {
		self.into()
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
