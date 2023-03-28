//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use std::sync::Arc;
use crate::collection::Collection;
use crate::key::Keychain;

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum SearchToKernel {
	// Here's the search (similarity) result.
	SearchSim(Keychain),
}

pub(crate) enum KernelToSearch {
	SearchSim(String),              // Start a (similarity) search on string input.
	DropCollection,                 // Drop your pointer.
	NewCollection(Arc<Collection>), // Here's a new `Collection` pointer.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
