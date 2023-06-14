//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use std::{};
use crate::{
	collection::{
		Collection,
		Key,
		Keychain,
		Queue,
		Playlist,
	},
	audio::Volume,
	kernel::Kernel,
	state::Phase,
};
use readable::Percent;
use once_cell::sync::Lazy;
use std::sync::{
	RwLock,
	RwLockReadGuard,
	RwLockWriteGuard,
	TryLockError,
};
use benri::{lockw,lockr};

//---------------------------------------------------------------------------------------------------- Lazy
/// This is the single, global copy of `ResetState` that `Kernel` uses.
///
/// To obtain a read-only lock, use `RESET_STATE.read()`.
pub static RESET_STATE: ResetStateLock = ResetStateLock(RwLock::new(ResetState::new()));

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
#[derive(Clone,Debug,PartialOrd,PartialEq,Serialize,Deserialize)]
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
	pub specific: String,
}

impl ResetState {
	/// Creates an empty struct.
	pub const fn new() -> Self {
		Self {
			resetting: false,
			percent: Percent::zero(),
			phase: Phase::None,
			specific: String::new(),
		}
	}

	// Sets an initial starting version.
	pub(crate) fn start(&mut self) {
		*self = Self {
			resetting: true,
			percent: Percent::zero(),
			phase: Phase::Start,
			specific: String::new(),
		};
	}

	// Sets the special `Disk` phase.
	pub(crate) fn disk(&mut self) {
		*self = Self {
			resetting: true,
			percent: Percent::zero(),
			phase: Phase::Disk,
			specific: String::new(),
		};
	}

	// Resets, use this after we're done.
	pub(crate) fn done(&mut self) {
		*self = Self {
			resetting: false,
			percent: Percent::const_100(),
			phase: Phase::None,
			specific: String::new(),
		};
	}

	// Set a new increment update, this increments the current values.
	pub(crate) fn new_increment(&mut self, increment: f64, specific: String) {
		let current    = self.percent.inner();
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
			specific: String::new(),
			phase,
			..*self
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
