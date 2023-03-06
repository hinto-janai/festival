//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum WatchToKernel {
	// Signals.
	Play,
	Stop,
	Next,
	Last,
	Shuffle,
	Repeat,
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
