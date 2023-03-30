//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use std::sync::Arc;
use crate::collection::{
	Collection,
};

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum CcdToKernel {
	NewCollection(Arc<Collection>), // Here's the new (or modified) `Collection`.
	Failed(anyhow::Error),          // Creating new or converting `Collection` has failed.
	Update(String),                 // This is the current `Path/Artist/Album/Song` I'm working on and the `%` of work done.
}

pub(crate) enum KernelToCcd {
	// You can rest now.
	//
	// (But before you do, save `Collection`
	// to disk and deconstruct the old one)
	Die,

	// Since the rest of `CCD` stuff are one-shot operations, there's no
	// need for `Kernel` to have a channel since it can just start `CCD`
	// with a function specific to whatever job it needs to do:
	//
	// `Kernel` will need to send multiple messages only in one case: When creating a new `Collection`:
	//
	// 1. `Kernel` ---- "Create a new Collection, here's the old pointer" ---> `CCD`
	// 3. `Kernel` <---            "Here's the new Collection"            ---- `CCD`
	// 4. `Kernel` ---- "Okay, I've dropped my pointer, you can die now." ---> `CCD`
	//
	// `CCD` needs to know for sure `Kernel` has dropped the old `Collection`
	// before it drops its own since that determines who actually destructs it.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
