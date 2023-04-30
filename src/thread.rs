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

	static ref __50: usize = (*AVAILABLE_THREADS as f64 * 0.5).floor() as usize;
	static ref HALF_AVAILABLE_THREADS: usize = {
		match *AVAILABLE_THREADS {
			// Special cases (low thread-count).
			1|2 => 1,

			// Around 50%.
			_ => *__50,
		}
	};

	static ref __75: usize = (*AVAILABLE_THREADS as f64 * 0.75).floor() as usize;
	static ref MOST_AVAILABLE_THREADS: usize = {
		match *AVAILABLE_THREADS {
			// Special cases (low thread-count).
			1 => 1,
			2 => 1,
			3 => 2,
			4 => 3,

			// Around 75%.
			_ => *__75,
		}
	};
}

/// Get the available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub fn threads_available() -> usize {
	*AVAILABLE_THREADS
}

/// Get half the available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub fn threads_half_available() -> usize {
	*HALF_AVAILABLE_THREADS
}

/// Get `75%` of available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub fn threads_most_available() -> usize {
	*MOST_AVAILABLE_THREADS
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
