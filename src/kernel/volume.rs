//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use super::Kernel;

//---------------------------------------------------------------------------------------------------- Volume.
/// Wrapper around [`u8`] that is between `0..100`
///
/// This is the "unit" [`Kernel`] wants audio volume changes in.
///
/// It guarantees the inner [`u8`] is between `0..100` so that
/// frontends can't just send random numbers that make no sense in the
/// context of changing the volume level, like `253`.
#[derive(Copy,Clone,Debug,Hash,Eq,Ord,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
pub struct Volume(u8);

impl Volume {
	#[inline]
	/// Create a new [`Volume`] from a [`u8`].
	///
	/// # Errors
	///
	/// The [`u8`] must be less than `100` or [`Self::new_100`] will be returned.
	pub const fn new(volume: u8) -> Self {
		if volume > 100 {
			return Self::new_100()
		}

		Self(volume)
	}

	#[inline(always)]
	/// Creates [`Volume`] without checking if the [`u8`] is between `0..100`.
	///
	/// # Safety
	///
	/// You must ensure:
	/// 1. The input is between `0..100`
	pub const unsafe fn new_unchecked(volume: u8) -> Self {
		Self(volume)
	}

	#[inline(always)]
	/// Returns the inner `u8`.
	pub const fn inner(&self) -> u8 {
		self.0
	}
}

impl Default for Volume {
	#[inline]
	/// Calls [`Volume::new_50`].
	fn default() -> Self {
		Self::new_50()
	}
}

impl std::fmt::Display for Volume {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl std::ops::Add for Volume {
    type Output = Self;

	/// Add a [`Volume`] to a [`Volume`].
	///
	/// If an overflow occurs, [`Volume::new_100`] is returned.
    fn add(self, other: Self) -> Self {
		if u16::from(self.0) + u16::from(other.0) > 100 {
			return Self::new_100();
		}

		Self(self.0 + other.0)
	}
}

impl std::ops::Sub for Volume {
    type Output = Self;

	/// Subtract a [`Volume`] to a [`Volume`].
	///
	/// If the result is negative, [`Volume::new_0`] is returned.
    fn sub(self, other: Self) -> Self {
		if self.0 < other.0 {
			return Self(0);
		}

		Self(self.0 - other.0)
	}
}

//---------------------------------------------------------------------------------------------------- Volume new.
macro_rules! impl_new {
    ( $num:tt ) => {
		paste::item! {
			#[doc = "Returns [`Volume`] with a value of `" $num "`"]
			pub const fn [<new_ $num>]() -> Self {
				Self($num)
			}
		}
	}
}

// God bless dtolnay.
impl Volume {
	seq_macro::seq!(N in 0..=100 {
		impl_new!(N);
	});
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn math() {
		let v1 = Volume::new_0();
		let v2 = Volume::new_0();
		let v3 = Volume::new_50();
		let v4 = Volume::new_100();
		let v5 = Volume::new_51();

		// Make sure result is `0`.
		assert!(v1 - v2 == v1);

		// Make sure result is `50`.
		assert!(v4 - v3 == v3);

		// Make sure result is `100`.
		assert!(v3 + v3 == v4);

		// Make sure overflowed result is `100`.
		println!("{}", v3 + v5);
		assert!(v3 + v5 == v4);

		// Make sure underflowed result is `0`.
		assert!(v3 - v4 == v1);
	}
}
