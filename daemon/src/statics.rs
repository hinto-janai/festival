//---------------------------------------------------------------------------------------------------- Use
use std::sync::atomic::{
	Ordering,
	AtomicUsize,
	AtomicBool,
};
use const_format::assertcp;
use benri::{
	atomic_load,
	atomic_add,
	atomic_sub,
};

//---------------------------------------------------------------------------------------------------- Reset
static CONNECTIONS: AtomicUsize = AtomicUsize::new(0);

#[inline(always)]
/// Get current connections count.
pub fn connections() -> usize {
    atomic_load!(CONNECTIONS)
}

/// On each connection, this should be constructed, it increments an atomic counter.
/// Upon drop, the internal counter is decremented.
pub struct ConnectionToken(());

impl Clone for ConnectionToken {
	fn clone(&self) -> Self {
		Self::new()
	}
}

impl Default for ConnectionToken {
	fn default() -> Self {
		Self::new()
	}
}

impl ConnectionToken {
	pub fn new() -> Self {
	    atomic_add!(CONNECTIONS, 1);
		Self(())
	}
}

impl Drop for ConnectionToken {
	fn drop(&mut self) {
		atomic_sub!(CONNECTIONS, 1);
	}
}

//---------------------------------------------------------------------------------------------------- Reset
/// Local version of `shukusai::statics::RESETTING`
pub(crate) static RESETTING: AtomicBool = AtomicBool::new(false);
#[inline(always)]
/// This [`bool`] represents if we are currently in
/// the process of resetting the [`Collection`].
pub fn resetting() -> bool {
    atomic_load!(RESETTING)
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn token() {
		assert_eq!(std::mem::size_of::<ConnectionToken>(), 0);
		assert_eq!(connections(), 0);

		{
			let default = ConnectionToken::default();
			let marker  = ConnectionToken::new();
			let clone   = marker.clone();
			assert_eq!(connections(), 3);
		} // drop

		assert_eq!(connections(), 0);
	}
}
