//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};

//---------------------------------------------------------------------------------------------------- Kernel
//__DISK__file!(Kernel, Dir::Data, "", "", "");
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
struct Kernel {
}

//impl Kernel {
//#[inline(always)]
//fn new() -> Self {
//	Self {
//		__NEW__
//	}
//}
//}

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
//mod test {
//  #[test]
//  fn __TEST__() {
//  }
//}
