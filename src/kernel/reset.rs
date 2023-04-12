//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
//use std::{};
use std::sync::{Arc,RwLock};
use crate::key::{
	Key,
	Keychain,
};
use crate::slice::{
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
use crate::collection::Collection;
use readable::Percent;

//---------------------------------------------------------------------------------------------------- Lazy
lazy_static::lazy_static! {
	// This is an empty, dummy `ResetState`.
	pub(crate) static ref DUMMY_RESET_STATE: Arc<RwLock<ResetState>> = Arc::new(RwLock::new(ResetState::new()));
}

//---------------------------------------------------------------------------------------------------- ResetState
/// Reset State.
///
/// Some in-progress status updates for when the [`Collection`] is reset.
///
/// This holds a:
/// - [`bool`] representing: Are we currently resetting the [`Collection`]?
/// - [`Percent`] representing the total work done out of `100%`
/// - [`String`] representing what phase we're on
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

	/// [`String`] representing what phase we're on
	///
	/// Example: `Walking Directories`, `Parsing Metadata`, etc.

	pub phase: String,

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
			phase: "...".to_string(),
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
			phase: "Starting...".to_string(),
			specific: String::new(),
		};
	}

	// Resets, use this after we're done.
	pub(super) fn done(&mut self) {
		*self = Self {
			resetting: false,
			percent: Percent::const_100(),
			phase: "Done".to_string(),
			specific: String::new(),
		};
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
