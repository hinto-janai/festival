//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::Toml;
use shukusai::constants::{
	FESTIVAL,FRONTEND_SUB_DIR,
};

//---------------------------------------------------------------------------------------------------- __NAME__
disk::toml!(Config, disk::Dir::Config, FESTIVAL, FRONTEND_SUB_DIR, "festivald");
#[derive(Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct Config {}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
