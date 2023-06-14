//---------------------------------------------------------------------------------------------------- Use
use once_cell::sync::Lazy;

//----------------------------------------------------------------------------------------------------
/// Get the available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub static THREADS: Lazy<usize> = Lazy::new(|| {
	match std::thread::available_parallelism() {
		Ok(t)  => t.get(),
		Err(_) => {
			1
		}
	}
});

/// Get `25%` of available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub static THREADS_25: Lazy<usize> = Lazy::new(|| {
	match *THREADS {
		0|1|2|3|4 => 1,
		_ => (*THREADS as f64 * 0.25).floor() as usize,
	}
});

/// Get `50%` the available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub static THREADS_50: Lazy<usize> = Lazy::new(|| {
	match *THREADS {
		0|1|2 => 1,
		_ => (*THREADS as f64 * 0.5).floor() as usize,
	}
});

/// Get `75%` of available amount of system threads.
///
/// This is lazily evaluated and returns 1 on errors.
pub static THREADS_75: Lazy<usize> = Lazy::new(|| {
	match *THREADS {
		0|1|2 => 1,
		3 => 2,
		4 => 3,
		_ => (*THREADS as f64 * 0.75).floor() as usize,
	}
});

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
