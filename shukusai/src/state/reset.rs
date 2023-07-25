//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use crate::{
	state::Phase,
};
use readable::Percent;
use std::sync::{
	Arc,
	RwLock,
	RwLockReadGuard,
	RwLockWriteGuard,
	TryLockError,
};
use benri::{lockw,lockr};
use once_cell::sync::Lazy;

//---------------------------------------------------------------------------------------------------- Lazy
/// This is the single, global copy of `ResetState` that `Kernel` uses.
///
/// To obtain a read-only lock, use `RESET_STATE.read()`.
// HACK:
// This didn't have a `Lazy` before, but `specific` was
// changed from `String` to `Arc<str>`, and `Arc` doesn't
// have a `const` constructor so `Lazy` must be used.
//
// Not ideal.
pub static RESET_STATE: Lazy<ResetStateLock> = Lazy::new(|| ResetStateLock(RwLock::new(ResetState::new())));

//---------------------------------------------------------------------------------------------------- ResetStateLock
/// There is only a single, global copy of `ResetState` that `Kernel` uses: [`RESET_STATE`].
///
/// To obtain a read-only lock, use `RESET_STATE.read()`.
pub struct ResetStateLock(RwLock<ResetState>);

impl ResetStateLock {
	#[inline(always)]
	/// Obtain a read-only lock to the global [`ResetState`].
	pub fn read(&'static self) -> RwLockReadGuard<'static, ResetState> {
		lockr!(self.0)
	}

	#[inline(always)]
	/// Call the non-blocking `.try_read()` on the global [`ResetState`].
	pub fn try_read(&'static self) -> Result<RwLockReadGuard<'static, ResetState>, TryLockError<RwLockReadGuard<'static, ResetState>>> {
		self.0.try_read()
	}

	#[inline(always)]
	// Private write.
	pub(crate) fn write(&'static self) -> RwLockWriteGuard<'static, ResetState> {
		lockw!(self.0)
	}

	#[inline(always)]
	// Private write.
	pub(crate) fn try_write(&'static self) -> Result<RwLockWriteGuard<'static, ResetState>, TryLockError<RwLockWriteGuard<'static, ResetState>>> {
		self.0.try_write()
	}
}

//---------------------------------------------------------------------------------------------------- ResetState
/// Reset State.
///
/// Some in-progress status updates for when the [`Collection`] is reset.
///
/// This holds a:
/// - [`bool`] representing: Are we currently resetting the [`Collection`]?
/// - [`Percent`] representing the total work done out of `100%`
/// - [`Phase`] representing what phase we're on
/// - [`String`] representing what work we're currently doing
///
/// This values in this struct will be updated during the process.
///
/// There is only a single, global copy of this struct that `Kernel` uses.
///
/// To obtain a read-only lock, use `RESET_STATE.read()`.
#[derive(Clone,Debug,PartialOrd,PartialEq)]
pub struct ResetState {
	/// [`bool`] representing: Are we currently resetting the [`Collection`]?
	pub resetting: bool,

	/// [`Percent`] representing the total work done out of `100%`
	pub percent: Percent,

	/// Represents what [`Phase`] we're on
	pub phase: Phase,

	/// [`String`] representing the specific work we're currently doing
	///
	/// Example: Current `Artist/Album/Song`.
	pub specific: Arc<str>,
}

impl ResetState {
	/// Creates an empty struct.
	pub fn new() -> Self {
		Self {
			resetting: false,
			percent: Percent::zero(),
			phase: Phase::None,
			specific: "".into(),
		}
	}

	// Sets an initial starting version.
	pub(crate) fn start(&mut self) {
		*self = Self {
			resetting: true,
			percent: Percent::zero(),
			phase: Phase::Start,
			specific: "".into(),
		};
	}

	// Sets an initial waiting version.
	pub(crate) fn wait(&mut self) {
		*self = Self {
			resetting: true,
			percent: Percent::zero(),
			phase: Phase::Wait,
			specific: "".into(),
		};
	}

	// Sets the special `Disk` phase.
	pub(crate) fn disk(&mut self) {
		*self = Self {
			resetting: true,
			percent: Percent::zero(),
			phase: Phase::Disk,
			specific: "".into(),
		};
	}

	// Resets, use this after we're done.
	pub(crate) fn done(&mut self) {
		*self = Self {
			resetting: false,
			percent: Percent::const_100(),
			phase: Phase::None,
			specific: "".into(),
		};
	}

	// Set a new increment update, this increments the current values.
	pub(crate) fn new_increment(&mut self, increment: f64, specific: Arc<str>) {
		*self = Self {
			percent: Percent::from(self.percent.inner() + increment),
			specific,
			..*self
		};
	}

	// Set a new phase and percent.
	pub(crate) fn new_phase(&mut self, percent: f64, phase: Phase) {
		*self = Self {
			percent: Percent::from(percent),
			specific: "".into(),
			phase,
			..*self
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	// Tests `new_increment()` and asserts the following behavior:
	//
	// 1. Percent is incremented
	// 2. String is updated
	// 3. Nothing else is changed
	fn new_increment() {
		let mut r = ResetState::new();
		let old_resetting = r.resetting;
		let old_phase     = r.phase;

		assert_eq!(r.percent, 0.0);
		r.new_increment(10.0, "New string".into());

		// 1
		assert_eq!(r.percent, 10.0);
		// 2
		assert_eq!(r.specific, "New string".into());
		// 3
		assert_eq!(r.resetting, old_resetting);
		assert_eq!(r.phase, old_phase);
	}

	#[test]
	// Tests `new_phase()` and asserts the following behavior:
	//
	// 1. Percent is incremented
	// 2. String is reset
	// 3. Phase is updated
	// 4. Nothing else is changed
	fn new_phase() {
		const PHASE: Phase = Phase::Disk;

		let mut r = ResetState::new();
		let old_resetting = r.resetting;

		assert_eq!(r.percent, 0.0);
		r.new_phase(10.0, PHASE);

		// 1
		assert_eq!(r.percent, 10.0);
		// 2
		assert_eq!(r.specific, "".into());
		// 4
		assert_eq!(r.phase, PHASE);
		// 3
		assert_eq!(r.resetting, old_resetting);
	}
}
