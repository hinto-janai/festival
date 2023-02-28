//! Read Only Lock.
//!
//! This is a simple wrapper around [`Arc<RwLock<T>>`] that only implements [`RwLock::read()`] operations.
//!
//! Usage: Create a normal [`Arc<RwLock<T>>`] in `thread_1`, send a [`RoLock`] to `thread_2`:
//! ```
//! use std::sync::*;
//! use rolock::RoLock;
//!
//! let rw = Arc::new(RwLock::new(0)); // Regular Arc<RwLock<T>>.
//! let ro = RoLock::new(&rw);         // Read Only Lock.
//!
//! *rw.write().unwrap() = 1;          // This can write...
//! assert!(*rw.read().unwrap() == 1); // and read.
//!
//! std::thread::spawn(move|| {
//! 	assert!(*ro.read() == 1);      // This one can only read.
//! });
//! ```
//! - `thread_1` still has full read/write control
//! - `thread_2` can only [`RoLock::read()`]
//!
//! This type guarantees at compile time that you cannot write because the function doesn't even exist:
//! ```compile_fail
//! # use std::sync::*;
//! # use rolock::RoLock;
//! let rw = Arc::new(RwLock::new(0));
//! let ro = RoLock::new(&rw);
//!
//! ro.write();
//! ```
//! Since the inner field of [`RoLock`] (`self.0`) is private, you can't call [`RwLock::write`] directly either:
//! ```compile_fail
//! # use std::sync::*;
//! # use rolock::RoLock;
//! let rw = Arc::new(RwLock::new(0));
//! let ro = RoLock::new(&rw);
//!
//! ro.0.write();
//! ```

use std::sync::*;

/// Read Only Lock.
#[derive(Debug)]
pub struct RoLock<T>(Arc<RwLock<T>>);

impl<T> RoLock<T> {
	/// Get an [`Arc`] to an existing [`RwLock`] but as a [`RoLock`].
	#[inline(always)]
	pub fn new(value: &Arc<RwLock<T>>) -> Self {
		Self::from(value)
	}

	/// Create a whole new [`Arc<RwLock<T>>`], returning it and an additional [`RoLock`].
	#[inline(always)]
	pub fn new_pair(value: T) -> (Arc<RwLock<T>>, Self) {
		let rw = Arc::new(RwLock::new(value));
		let ro = Self::from(&rw);
		(rw, ro)
	}

	/// This calls [`RwLock::read`].
	#[inline(always)]
	pub fn read(&self) -> Result<RwLockReadGuard<'_, T>, PoisonError<RwLockReadGuard<'_, T>>> {
		self.0.read()
	}

	/// This calls [`RwLock::try_read()`].
	#[inline(always)]
	pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<'_, T>> {
		self.0.try_read()
	}

	/// This calls [`RwLock::is_poisoned()`].
	#[inline(always)]
	pub fn is_poisoned(&self) -> bool {
		self.0.is_poisoned()
	}
}

//---------------------------------------------------------------------------------------------------- Common Impls
impl<T> Clone for RoLock<T> {
	#[inline(always)]
	fn clone(&self) -> Self {
		Self(Arc::clone(&self.0))
	}
}

impl<T> From<&Arc<RwLock<T>>> for RoLock<T> {
	#[inline(always)]
	fn from(value: &Arc<RwLock<T>>) -> Self {
		Self(Arc::clone(value))
	}
}

// Allows direct printing of inner value.
//impl<T: std::fmt::Display> std::fmt::Display for RoLock<T> {
//	#[inline(always)]
//	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//		write!(f, "{}", self.read())
//	}
//}
