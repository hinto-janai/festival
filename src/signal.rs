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

//---------------------------------------------------------------------------------------------------- Signals
macro_rules! impl_signal {
	($type:ident, $file_name:literal) => {
		use disk::*;
		disk::empty!($type, disk::Dir::Data, FESTIVAL, "signal", $file_name);
		#[derive(Copy,Clone,Debug,PartialEq,Eq)]
		/// File representing a signal
		///
		/// Use [`Self::touch()`] to create the file.
		pub struct $type;
	}
}

impl_signal!(Toggle,        "toggle");
impl_signal!(Pause,         "pause");
impl_signal!(Play,          "play");
impl_signal!(Next,          "next");
impl_signal!(Previous,      "previous");
impl_signal!(ShuffleOn,     "shuffle_on");
impl_signal!(ShuffleOff,    "shuffle_off");
impl_signal!(ShuffleToggle, "shuffle_toggle");
impl_signal!(RepeatSong,    "repeat_song");
impl_signal!(RepeatQueue,   "repeat_queue");
impl_signal!(RepeatOff,     "repeat_off");

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
