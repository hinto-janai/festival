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
	key::CollectionKeychain,
};

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub enum SearchToKernel {
	SearchResult(CollectionKeychain), // Here's the search result.
}

pub enum KernelToSearch {
	Search(String),                 // Start a search on string input.
	DropCollection,                 // Drop your pointer.
	CollectionArc(Arc<Collection>), // Here's a `Collection` pointer.
}

//impl __NAME__ {
//#[inline(always)]
//fn new() -> Self {
//	Self {
//		__NEW__
//	}
//}
//}

//impl std::default::Default for __NAME__ {
//	fn default() -> Self {
//		Self::new()
//	}
//}

//impl std::fmt::Display for __NAME__ {
//	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//		write!(f, "{:?}", self)
//	}
//}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
