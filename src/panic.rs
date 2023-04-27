//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::kernel::Kernel;
use crate::FESTIVAL;
use serde::{Serialize,Deserialize};

//----------------------------------------------------------------------------------------------------
disk::plain!(Panic, disk::Dir::Data, FESTIVAL, "", "panic");
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
/// File representing a `panic!()` log.
///
/// This gets written in the `festival` folder as `panic.txt`.
pub struct Panic(pub(crate) String);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
