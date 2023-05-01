//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};

//----------------------------------------------------------------------------------------------------
lazy_static::lazy_static! {
	static ref AVAILABLE_THREADS: usize = {
		match std::thread::available_parallelism() {
			Ok(t)  => t.get(),
			Err(_) => {
				1
			}
		}
	};

	static ref AVAILABLE_THREADS_25: usize = {
		match *AVAILABLE_THREADS {
			0|1|2|3|4 => 1,
			_ => (*AVAILABLE_THREADS as f64 * 0.25).floor() as usize,
		}
	};

	static ref AVAILABLE_THREADS_50: usize = {
		match *AVAILABLE_THREADS {
			0|1|2 => 1,
			_ => (*AVAILABLE_THREADS as f64 * 0.5).floor() as usize,
		}
	};

	static ref AVAILABLE_THREADS_75: usize = {
		match *AVAILABLE_THREADS {
			0|1|2 => 1,
			3 => 2,
			4 => 3,
			_ => (*AVAILABLE_THREADS as f64 * 0.75).floor() as usize,
		}
	};
}

/// Get the available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub fn threads_available() -> usize {
	*AVAILABLE_THREADS
}

/// Get `25%` of available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub fn threads_available_25() -> usize {
	*AVAILABLE_THREADS_25
}

/// Get `50%` the available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub fn threads_available_50() -> usize {
	*AVAILABLE_THREADS_50
}

/// Get `75%` of available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub fn threads_available_75() -> usize {
	*AVAILABLE_THREADS_75
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
