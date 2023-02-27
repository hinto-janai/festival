//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
use std::num::NonZeroUsize;

//---------------------------------------------------------------------------------------------------- Constants.
const ONE_THREAD: usize = 1;

//---------------------------------------------------------------------------------------------------- Thread Functions.
pub fn available_threads() -> usize {
	match std::thread::available_parallelism() {
		Ok(t)  => t.get(),
		Err(_) => {
			warn!("Available thread function failed, defaulting to 1!");
			ONE_THREAD
		}
	}
}

pub fn get_half_threads() -> usize {
	let threads = available_threads();

	if threads == 1 {
		return threads
	}

	// Converting to float might be more accurate but threads
	// are almost always cleanly divisible by 2 so it's fine.
	threads / 2
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn _get_half_threads() {
		let threads = available_threads();

		assert!(threads / 2  == get_half_threads());
	}
}
