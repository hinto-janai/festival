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
pub enum CcdToKernel {
	PathUpdate(String),        // This is the current `Path` I'm working on.
	ArtistUpdate(String),      // This is the current `Artist` I'm working on.
	AlbumUpdate(String),       // This is the current `Album` I'm working on.
	SongUpdate(String),        // This is the current `Song` I'm working on.
	PercentUpdate(String),     // This is the current `%` of work I've done so far (out of 100).
	NewCollection(Collection), // Here's the new `Collection`.
	Failed,                    // Creating new or converting `Collection` has failed.
}

pub enum KernelToCcd {
	NewCollection(Arc<Collection>), // Start work on a new `Collection`, here's the _old_ `Collection` pointer.
	ConvertImg(Collection),         // Convert an existing `Collection` image bytes into usable `egui` images.
	Die,                            // You can rest now.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
