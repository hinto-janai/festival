//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use std::{};
use std::sync::{Arc,RwLock};
use crate::collection::{
	Collection,
	Key,
	Keychain,
	Queue,
	Playlist,
};
use crate::constants::{
	FESTIVAL,
	FESTIVAL_HEADER,
	STATE_VERSION,
};
use rolock::RoLock;
use super::Volume;
use crate::kernel::Kernel;
use readable::Percent;
use super::phase::Phase;
use once_cell::sync::Lazy;

//---------------------------------------------------------------------------------------------------- Lazy
// This is an empty, dummy `ResetState`.
pub(crate) static DUMMY_RESET_STATE: Lazy<Arc<RwLock<ResetState>>> = Lazy::new(|| Arc::new(RwLock::new(ResetState::new())));

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
/// You should have received this _once_ from `Kernel`
/// right after you spawn it, until then, use [`ResetState::dummy`]
/// for early initialization without waiting on `Kernel`.
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
	/// Obtain an empty, dummy [`Collection`] wrapped in an [`Arc`].
	///
	/// This is useful when you need to initialize but don't want
	/// to wait on [`Kernel`] to hand you the _real_ `RoLock<KernelState>`.
	///
	/// This is implemented in the exact same way as [`Collection::dummy`].
	///
	/// For more information, read that documentation.
	pub fn dummy() -> RoLock<Self> {
		RoLock::new(&DUMMY_RESET_STATE)
	}

	pub(super) fn new() -> Self {
		Self {
			resetting: false,
			percent: Percent::zero(),
			phase: Phase::None,
			specific: String::new(),
		}
	}

	// Private RwLock version.
	pub(super) fn from_dummy() -> Arc<RwLock<Self>> {
		Arc::clone(&DUMMY_RESET_STATE)
	}

	// Sets an initial starting version.
	pub(super) fn start(&mut self) {
		*self = Self {
			resetting: true,
			percent: Percent::zero(),
			phase: Phase::Start,
			specific: String::new(),
		};
	}

	// Sets the special `Disk` phase.
	pub(super) fn disk(&mut self) {
		*self = Self {
			resetting: true,
			percent: Percent::zero(),
			phase: Phase::Disk,
			specific: String::new(),
		};
	}

	// Resets, use this after we're done.
	pub(super) fn done(&mut self) {
		*self = Self {
			resetting: false,
			percent: Percent::const_100(),
			phase: Phase::None,
			specific: String::new(),
		};
	}

	// Set a new increment update, this increments the current values.
	pub(super) fn new_increment(&mut self, increment: f64, specific: String) {
		let current    = self.percent.inner();
		*self = Self {
			percent: Percent::from(self.percent.inner() + increment),
			specific,
			..*self
		};
	}

	// Set a new phase and percent.
	pub(super) fn new_phase(&mut self, percent: f64, phase: Phase) {
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
