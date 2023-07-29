//---------------------------------------------------------------------------------------------------- Use
use const_format::assertcp;
use benri::{
	atomic_load,
	atomic_add,
	atomic_sub,
};
use shukusai::collection::Collection;
use std::sync::{
	Arc,
	atomic::{
		Ordering,
		AtomicUsize,
		AtomicBool,
		AtomicPtr,
		AtomicU64,
	},
};

//---------------------------------------------------------------------------------------------------- Total Stats
// Used in `state_daemon` RPC call.
pub static TOTAL_REQUESTS:    AtomicU64 = AtomicU64::new(0);
pub static TOTAL_CONNECTIONS: AtomicU64 = AtomicU64::new(0);

//---------------------------------------------------------------------------------------------------- Connections
static CONNECTIONS: AtomicU64 = AtomicU64::new(0);

#[inline(always)]
/// Get connection count.
pub fn connections() -> u64 {
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

//---------------------------------------------------------------------------------------------------- Kernel Busy
// The channel between `Frontend` <-> `Kernel` is MPMC, although
// `Kernel` acts as if there is only "1" `Frontend`.
//
// This is a problem in `festivald` where there could
// be _multiple_ tasks sending signals to `Kernel` and
// expecting a routed response back.
//
// The Problem:
//   - `Frontend` (`task A`) sends a search request to `Kernel`, hangs on `recv!()`
//   - `Frontend` (`task B`) sends a search request to `Kernel`, hangs on `recv!()`
//   - `Kernel` forwards `task A` request to `Search`
//   - `Kernel` forwards `task B` request to `Search`
//   - `Search` finishes `task B`, sends to `Kernel`
//   - `Kernel` does the only thing it knows how to do:
//    forward the response to the one and only `Frontend` channel
//   - `Search` finishes `task A`, sends to `Kernel`
//   - Oops! Since _every_ task is holding onto a `channel`, and
//     there's no "routing" in place (no address, no way to identify
//     _which_ task sent the request, it's just a two-way queue)
//     we can't make sure `task B` will `recv!()` the correct message
//   - Now `task B` got the wrong message, so now `task A` will also
//     `recv!()` `task B`'s message, everything is messed up
//
// The (terrible) Solution:
//   - Only 1 message at a time
//
// Now it's serial, so the task that sent the message will acquire
// a lock before `recv!()`'ing, and only drops it until it receives a response
// and any new tasks will `try_lock()` before proceeding to send+recv.
//
// This is pretty bad but maybe not the worst considering:
//   - No `shukusai` internals have to be touched
//   - The only `FrontendToKernel` messages that have a response is
//     `Search` keychains, `Collection` resets (which must be serial anyway),
//     the rest are response-less commands (play, toggle, set volume, etc)
pub(crate) static KERNEL_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

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
