use crossbeam_channel::{Sender,Receiver};
use std::sync::{Mutex,RwLock};
use rolock::RoLock;

#[macro_export]
/// Lock a [`Mutex`] or [`mass_panic!()`]
macro_rules! lock {
	($lock:expr) => {
		match $lock.lock() {
			Ok(lock) => lock,
			Err(e)   => mass_panic!(e),
		}
	}
}
pub use lock;

/// Read a [`RwLock`]/[`RoLock`] or [`mass_panic!`]
#[macro_export]
macro_rules! lock_read {
	($lock:expr) => {
		match $lock.read() {
			Ok(lock) => lock,
			Err(e)   => mass_panic!(e),
		}
	}
}
pub use lock_read;

#[macro_export]
/// Write to a [`RwLock`] or [`mass_panic!`]
macro_rules! lock_write {
	($lock:expr) => {
		match $lock.write() {
			Ok(lock) => lock,
			Err(e)   => mass_panic!(e),
		}
	}
}
pub use lock_write;

#[macro_export]
/// Sleep the current thread for `x` milliseconds
macro_rules! sleep {
    ($millis:expr) => {
		std::thread::sleep(std::time::Duration::from_millis($millis))
    }
}
pub use sleep;

#[macro_export]
/// Flip a [`bool`] in place (bitwise XOR assign)
macro_rules! flip {
	($b:expr) => {
		$b ^= true
	}
}
pub use flip;

#[macro_export]
/// Flip a [`std::sync::atomic::AtomicBool`] in place with `Ordering::SeqCst`
macro_rules! atomic_flip {
	($b:expr) => {
		$b.fetch_xor(true, std::sync::atomic::Ordering::SeqCst)
	}
}
pub use atomic_flip;

#[macro_export]
/// [`load()`] from [`std::sync::atomic`] with `Ordering::SeqCst`
macro_rules! atomic_load {
	($b:expr) => {
		$b.load(std::sync::atomic::Ordering::SeqCst)
	}
}
pub use atomic_load;

#[macro_export]
/// [`store()`] from [`std::sync::atomic::AtomicBool`] with `Ordering::SeqCst`
macro_rules! atomic_store {
	($atomic:expr, $b:expr) => {
		$atomic.store($b, std::sync::atomic::Ordering::SeqCst)
	}
}
pub use atomic_store;

#[macro_export]
/// Forward input to [`log::info`], appended with green `... OK`
macro_rules! ok {
	($($tts:tt)*) => {
		log::info!("{} ... \x1b[1;92mOK\x1b[0m", $($tts)*)
	}
}
pub use ok;

#[macro_export]
/// Forward input to [`log::debug`], appended with green `... OK`
macro_rules! ok_debug {
	($($tts:tt)*) => {
		log::debug!("{} ... \x1b[1;92mOK\x1b[0m", $($tts)*)
	}
}
pub use ok_debug;

#[macro_export]
/// Forward input to [`log::trace`], appended with green `... OK`
macro_rules! ok_trace {
	($($tts:tt)*) => {
		log::trace!("{} ... \x1b[1;92mOK\x1b[0m", $($tts)*)
	}
}
pub use ok_trace;

#[macro_export]
/// Forward input to [`log::info`], appended with white `... SKIP`
macro_rules! skip {
	($($tts:tt)*) => {
		log::info!("{} ... \x1b[1;97mSKIP\x1b[0m", $($tts)*)
	}
}
pub use skip;

#[macro_export]
/// Forward input to [`log::warn`], appended with white `... SKIP`
macro_rules! skip_warn {
	($($tts:tt)*) => {
		log::warn!("{} ... \x1b[1;97mSKIP\x1b[0m", $($tts)*)
	}
}
pub use skip_warn;

#[macro_export]
/// Forward input to [`log::debug`], appended with white `... SKIP`
macro_rules! skip_debug {
	($($tts:tt)*) => {
		log::debug!("{} ... \x1b[1;97mSKIP\x1b[0m", $($tts)*)
	}
}
pub use skip_debug;

#[macro_export]
/// Forward input to [`log::trace`], appended with white `... SKIP`
macro_rules! skip_trace {
	($($tts:tt)*) => {
		log::trace!("{} ... \x1b[1;97mSKIP\x1b[0m", $($tts)*)
	}
}
pub use skip_trace;

#[macro_export]
/// Forward input to [`log::error!`], appended with red `... FAIL`
macro_rules! fail {
	($($tts:tt)*) => {
		log::error!("{} ... \x1b[1;91mFAIL\x1b[0m", format_args!($($tts)*))
	}
}
pub use fail;

#[macro_export]
/// [`log::error`] a message and terminate all threads
macro_rules! mass_panic {
	($($tts:tt)*) => {{
		// Log.
		log::error!("");
		log::error!("");
		log::error!("");
		log::error!("----- THREAD PANIC -----");
		log::error!("{}", $($tts)*);
		log::error!("{}", $($tts)*);
		log::error!("{}", $($tts)*);
		log::error!("{}", $($tts)*);
		log::error!("{}", $($tts)*);
		log::error!("{}", $($tts)*);
		log::error!("----- THREAD PANIC -----");
		log::error!("");
		log::error!("");
		log::error!("");

		// Exit all threads.
		std::process::exit(111)
	}}
}
pub use mass_panic;

#[macro_export]
/// `match` a [`Result`], [`mass_panic!`] on [`Result::Err`]
macro_rules! unwrap_or_mass {
	($var:tt) => {
		match $var {
			Ok(o)  => o,
			Err(e) => mass_panic!(e),
		}
	}
}
pub use unwrap_or_mass;

#[macro_export]
/// [`send`] a channel message, [`mass_panic!`] on failure
macro_rules! send {
	($channel:expr, $($msg:tt)*) => {
		if let Err(e) = $channel.send($($msg)*) {
			mass_panic!(e);
		}
	}
}
pub use send;

#[macro_export]
/// [`recv`] a channel message, [`mass_panic!`] on failure
macro_rules! recv {
	($channel:expr) => {
		match $channel.recv() {
			Ok(msg) => msg,
			Err(e)  => mass_panic!(e),
		}
	}
}
pub use recv;

#[macro_export]
/// [`send`] a channel message, [`panic!`] current thread on failure
macro_rules! send_or_die {
	($channel:expr, $($msg:tt)*) => {
		if let Err(e) = $channel.send($($msg)*) {
			log::error!("THREAD PANIC - FAILED TO SEND: {}", e);
			panic!("{}", e);
		}
	}
}
pub use send_or_die;

#[macro_export]
/// [`recv`] a channel message, [`panic!`] current thread on failure
macro_rules! recv_or_die {
	($channel:expr) => {
		match $channel.recv() {
			Ok(msg) => msg,
			Err(e)  => {
				log::error!("THREAD PANIC - FAILED TO RECEIVE: {}", e);
				panic!("{}", e);
			},
		}
	}
}
pub use recv_or_die;

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use std::sync::atomic::{AtomicBool,AtomicU64,Ordering};

	#[test]
	fn flip() {
		let mut b = true;
		flip!(b);
		assert!(b == false);
		flip!(b);
		assert!(b == true);
		flip!(b);
		assert!(b == false);
	}

	#[test]
	fn atomic_flip() {
		let mut b = AtomicBool::new(true);
		atomic_flip!(b);
		assert!(b.load(Ordering::SeqCst) == false);
		atomic_flip!(b);
		assert!(b.load(Ordering::SeqCst) == true);
		atomic_flip!(b);
		assert!(b.load(Ordering::SeqCst) == false);
	}

	#[test]
	fn atomic() {
		let mut u = AtomicU64::new(0);
		atomic_store!(u, 123);
		assert!(atomic_load!(u) == 123);
		atomic_store!(u, 0);
		assert!(atomic_load!(u) == 0);
	}

	#[test]
	fn send_recv() {
		let (tx, rx) = crossbeam_channel::unbounded::<u8>();
		send_or_die!(tx, 0);
		let msg = recv_or_die!(rx);
		assert!(msg == 0);
	}

	#[test]
	#[should_panic]
	fn send_recv_panic() {
		use log::error;

		let (tx, rx) = crossbeam_channel::unbounded::<u8>();
		send_or_die!(tx, 0);
		recv_or_die!(rx);

		drop(rx);
		send_or_die!(tx, 0);
	}

	#[test]
	fn lock() {
		use std::sync::{Arc,Mutex};
		let lock = Arc::new(Mutex::new(0));
		let lock = lock!(lock);
		assert!(*lock == 0);
	}

	#[test]
	fn lock_write_read() {
		use std::sync::{Arc,RwLock};
		let lock = Arc::new(RwLock::new(0));
		*lock_write!(lock) = 1;
		assert!(*lock_read!(lock) == 1);
	}
}
