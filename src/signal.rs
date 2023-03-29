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
use disk::prelude::*;
use disk::{Empty, empty_file};

//---------------------------------------------------------------------------------------------------- Signals
macro_rules! impl_signal {
	($type:ident, $file_name:literal) => {
		plain_file!($type, Dir::Data, FESTIVAL, "signal", $file_name);
		#[derive(Copy,Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
		/// File representing a signal
		///
		/// Use `touch()` to create the file.
		pub struct $type;

		impl $type {
			pub fn new() -> Self {
				Self
			}
		}
	}
}

impl_signal!(Toggle, "toggle");
impl_signal!(Stop, "stop");
impl_signal!(Play, "play");
impl_signal!(Next, "next");
impl_signal!(Last, "last");
impl_signal!(Shuffle, "shuffle");
impl_signal!(Repeat, "repeat");

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
