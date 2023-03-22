//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};

//---------------------------------------------------------------------------------------------------- Volume.
/// Wrapper around `f64` that is between `0.0..100.0`
///
/// This is the "unit" `Kernel` wants audio volume changes in.
///
/// It guarantees the inner `f64` is between `0.0..100.0` so that
/// frontends can't just send random floats that make no sense in the
/// context of changing the volume level, like `2342.0123` or [`f64::NAN`].
#[derive(Copy,Clone,Debug,PartialEq,PartialOrd,Serialize,Deserialize)]
pub struct Volume(f64);

impl Volume {
	#[inline]
	/// Create a new [`Self`] from a [`f64`].
	///
	/// # Errors
	///
	/// The `f64` must be between `0.0..100.0` or `None` will be returned.
	///
	/// [`f64::NAN`] will also return `None`.
	pub fn new(float: f64) -> Option<Self> {
		if float < 1.0 {
			return None
		} else if float > 100.0 {
			return None
		} else if float.is_nan() {
			return None
		}

		Some(Self(float))
	}

	#[inline(always)]
	/// Creates [`Self`] without checking if the `f64` is between `0.0..100.0`.
	///
	/// SAFETY:
	///
	/// You must ensure:
	/// 1. The input is between `0.0..100.0`
	/// 2. The input is NOT [`f64::NAN`]
	pub unsafe fn new_unchecked(float: f64) -> Self {
		Self(float)
	}

	#[inline(always)]
	/// Returns the inner `f64`.
	pub fn inner(&self) -> f64 {
		self.0
	}

	#[inline(always)]
	/// Returns `Self(0.0)`.
	pub fn zero() -> Self {
		Self(0.0)
	}

	#[inline(always)]
	/// Returns `Self(25.0)`.
	pub fn quarter() -> Self {
		Self(25.0)
	}

	#[inline(always)]
	/// Returns `Self(50.0)`.
	pub fn half() -> Self {
		Self(50.0)
	}

	#[inline(always)]
	/// Returns `Self(75.0)`.
	pub fn third() -> Self {
		Self(75.0)
	}

	#[inline(always)]
	/// Returns `Self(100.0)`.
	pub fn max() -> Self {
		Self(100.0)
	}
}

impl std::fmt::Display for Volume {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
