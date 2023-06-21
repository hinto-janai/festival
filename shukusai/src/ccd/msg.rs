//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use std::sync::Arc;
use crate::{
	collection::Collection,
	state::Phase,
};

//---------------------------------------------------------------------------------------------------- Kernel Messages.
// Since  `CCD` stuff are one-shot operations, there's no
// need for `Kernel` to have a channel since it can just start `CCD`
// with a function specific to whatever job it needs to do:
pub(crate) enum CcdToKernel {
	NewCollection(Arc<Collection>), // Here's the new (or modified) `Collection`.
	Failed(anyhow::Error),          // Creating new or converting `Collection` has failed.
	UpdatePhase((f64, Phase)),      // I'm starting a new phase. Set your `%` to this, and phase to this.
	UpdateIncrement((f64, String)), // Increment your `%` by this much, and update the working string to this.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
