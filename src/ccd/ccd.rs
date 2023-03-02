//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use crate::macros::{
	ok_debug,
	recv,
	send,
};
use crate::collection::{
	Collection,
	CollectionKeychain,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use std::sync::Arc;
use super::msg::{
	CcdToKernel,
	KernelToCcd,
};




// TODO:
// CCD is a oneshot thing. Kernel sends 1 command and that's it.
// There's no need for doing these generic message channels since Kernel
// knows the exact context. Remove `from_kernel` and `old_collection` and
// just pass the needed data directly from `Kernel` in a function instead.





//---------------------------------------------------------------------------------------------------- CCD
pub struct Ccd;

//---------------------------------------------------------------------------------------------------- CCD `ConvertImg` function.
impl Ccd {
	#[inline(always)]
	pub fn convert_img(to_kernel: std::sync::mpsc::Sender<CcdToKernel>) {
		ok_debug!("CCD - Purpose in life: ConvertImg");

		/* TODO: convert img bytes */
	}

//---------------------------------------------------------------------------------------------------- CCD `NewCollection` function.
	pub fn new_collection(
		to_kernel: std::sync::mpsc::Sender<CcdToKernel>,
		from_kernel: std::sync::mpsc::Receiver<KernelToCcd>,
	) {
		/* TODO: create new collection */
		ok_debug!("CCD - Purpose in life: NewCollection");
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
