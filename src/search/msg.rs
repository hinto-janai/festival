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
	Keychain,
};

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum SearchToKernel {
	SearchResult(Keychain), // Here's the search result.
}

pub(crate) enum KernelToSearch {
	Search(String),                 // Start a search on string input.
	DropCollection,                 // Drop your pointer.
	CollectionArc(Arc<Collection>), // Here's a new `Collection` pointer.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
