//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use super::Kernel;

//---------------------------------------------------------------------------------------------------- Volume.
/// Wrapper around [`f64`] that is between `0.0..100.0`
///
/// This is the "unit" [`Kernel`] wants audio volume changes in.
///
/// It guarantees the inner [`f64`] is between `0.0..100.0` so that
/// frontends can't just send random floats that make no sense in the
/// context of changing the volume level, like `2342.0123` or [`f64::NAN`].
#[derive(Copy,Clone,Debug,PartialEq,PartialOrd,Serialize,Deserialize)]
pub struct Volume(f64);

impl Volume {
	#[inline]
	/// Create a new [`Volume`] from a [`f64`].
	///
	/// # Errors
	///
	/// The [`f64`] must be between `0.0..100.0` or [`Option::None`] will be returned.
	///
	/// [`f64::NAN`] will also return `None`.
	pub fn new(float: f64) -> Option<Self> {
		if float < 0.0 {
			return None
		} else if float > 100.0 {
			return None
		} else if float.is_nan() {
			return None
		}

		Some(Self(float))
	}

	#[inline(always)]
	/// Creates [`Volume`] without checking if the [`f64`] is between `0.0..100.0`.
	///
	/// # Safety:
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

	#[inline]
	/// Create a new [`Volume`] from a [`u8`].
	///
	/// # Errors
	///
	/// The [`u8`] must be between `0..100` or [`Option::None`] will be returned.
	pub fn from_u8(int: u8) -> Option<Self> {
		if int > 100 {
			return None
		}

		Some(Self(int as f64))
	}

	#[inline(always)]
	/// Creates [`Volume`] without checking if the [`u8`] is between `0..100`.
	///
	/// # Safety:
	///
	/// You must ensure:
	/// 1. The input is between `0..100`
	pub unsafe fn from_u8_unchecked(int: u8) -> Self {
		Self(int as f64)
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
	/// If an overflow occurs, the first [`Volume`] is returned.
    fn add(self, other: Self) -> Self {
		let f = self.0 + other.0;

		if f > 100.0 { return self }

		Self(f)
	}
}

impl std::ops::Sub for Volume {
    type Output = Self;

	/// Subtract a [`Volume`] to a [`Volume`].
	///
	/// If the result is negative, [`Volume`] with a value of `0.0` is returned.
    fn sub(self, other: Self) -> Self {
		let f = self.0 - other.0;

		if f < 0.0 { return Self(0.0) }

		Self(f)
	}
}

//---------------------------------------------------------------------------------------------------- Volume new.
impl Volume {
	#[inline(always)]
	/// Returns `Self(0.0)`.
	pub fn new_0() -> Self { Self(0.0) }
	#[inline(always)]
	/// Returns `Self(1.0)`.
	pub fn new_1() -> Self { Self(1.0) }
	#[inline(always)]
	/// Returns `Self(2.0)`.
	pub fn new_2() -> Self { Self(2.0) }
	#[inline(always)]
	/// Returns `Self(3.0)`.
	pub fn new_3() -> Self { Self(3.0) }
	#[inline(always)]
	/// Returns `Self(4.0)`.
	pub fn new_4() -> Self { Self(4.0) }
	#[inline(always)]
	/// Returns `Self(5.0)`.
	pub fn new_5() -> Self { Self(5.0) }
	#[inline(always)]
	/// Returns `Self(6.0)`.
	pub fn new_6() -> Self { Self(6.0) }
	#[inline(always)]
	/// Returns `Self(7.0)`.
	pub fn new_7() -> Self { Self(7.0) }
	#[inline(always)]
	/// Returns `Self(8.0)`.
	pub fn new_8() -> Self { Self(8.0) }
	#[inline(always)]
	/// Returns `Self(9.0)`.
	pub fn new_9() -> Self { Self(9.0) }
	#[inline(always)]
	/// Returns `Self(10.0)`.
	pub fn new_10() -> Self { Self(10.0) }
	#[inline(always)]
	/// Returns `Self(11.0)`.
	pub fn new_11() -> Self { Self(11.0) }
	#[inline(always)]
	/// Returns `Self(12.0)`.
	pub fn new_12() -> Self { Self(12.0) }
	#[inline(always)]
	/// Returns `Self(13.0)`.
	pub fn new_13() -> Self { Self(13.0) }
	#[inline(always)]
	/// Returns `Self(14.0)`.
	pub fn new_14() -> Self { Self(14.0) }
	#[inline(always)]
	/// Returns `Self(15.0)`.
	pub fn new_15() -> Self { Self(15.0) }
	#[inline(always)]
	/// Returns `Self(16.0)`.
	pub fn new_16() -> Self { Self(16.0) }
	#[inline(always)]
	/// Returns `Self(17.0)`.
	pub fn new_17() -> Self { Self(17.0) }
	#[inline(always)]
	/// Returns `Self(18.0)`.
	pub fn new_18() -> Self { Self(18.0) }
	#[inline(always)]
	/// Returns `Self(19.0)`.
	pub fn new_19() -> Self { Self(19.0) }
	#[inline(always)]
	/// Returns `Self(20.0)`.
	pub fn new_20() -> Self { Self(20.0) }
	#[inline(always)]
	/// Returns `Self(21.0)`.
	pub fn new_21() -> Self { Self(21.0) }
	#[inline(always)]
	/// Returns `Self(22.0)`.
	pub fn new_22() -> Self { Self(22.0) }
	#[inline(always)]
	/// Returns `Self(23.0)`.
	pub fn new_23() -> Self { Self(23.0) }
	#[inline(always)]
	/// Returns `Self(24.0)`.
	pub fn new_24() -> Self { Self(24.0) }
	#[inline(always)]
	/// Returns `Self(25.0)`.
	pub fn new_25() -> Self { Self(25.0) }
	#[inline(always)]
	/// Returns `Self(26.0)`.
	pub fn new_26() -> Self { Self(26.0) }
	#[inline(always)]
	/// Returns `Self(27.0)`.
	pub fn new_27() -> Self { Self(27.0) }
	#[inline(always)]
	/// Returns `Self(28.0)`.
	pub fn new_28() -> Self { Self(28.0) }
	#[inline(always)]
	/// Returns `Self(29.0)`.
	pub fn new_29() -> Self { Self(29.0) }
	#[inline(always)]
	/// Returns `Self(30.0)`.
	pub fn new_30() -> Self { Self(30.0) }
	#[inline(always)]
	/// Returns `Self(31.0)`.
	pub fn new_31() -> Self { Self(31.0) }
	#[inline(always)]
	/// Returns `Self(32.0)`.
	pub fn new_32() -> Self { Self(32.0) }
	#[inline(always)]
	/// Returns `Self(33.0)`.
	pub fn new_33() -> Self { Self(33.0) }
	#[inline(always)]
	/// Returns `Self(34.0)`.
	pub fn new_34() -> Self { Self(34.0) }
	#[inline(always)]
	/// Returns `Self(35.0)`.
	pub fn new_35() -> Self { Self(35.0) }
	#[inline(always)]
	/// Returns `Self(36.0)`.
	pub fn new_36() -> Self { Self(36.0) }
	#[inline(always)]
	/// Returns `Self(37.0)`.
	pub fn new_37() -> Self { Self(37.0) }
	#[inline(always)]
	/// Returns `Self(38.0)`.
	pub fn new_38() -> Self { Self(38.0) }
	#[inline(always)]
	/// Returns `Self(39.0)`.
	pub fn new_39() -> Self { Self(39.0) }
	#[inline(always)]
	/// Returns `Self(40.0)`.
	pub fn new_40() -> Self { Self(40.0) }
	#[inline(always)]
	/// Returns `Self(41.0)`.
	pub fn new_41() -> Self { Self(41.0) }
	#[inline(always)]
	/// Returns `Self(42.0)`.
	pub fn new_42() -> Self { Self(42.0) }
	#[inline(always)]
	/// Returns `Self(43.0)`.
	pub fn new_43() -> Self { Self(43.0) }
	#[inline(always)]
	/// Returns `Self(44.0)`.
	pub fn new_44() -> Self { Self(44.0) }
	#[inline(always)]
	/// Returns `Self(45.0)`.
	pub fn new_45() -> Self { Self(45.0) }
	#[inline(always)]
	/// Returns `Self(46.0)`.
	pub fn new_46() -> Self { Self(46.0) }
	#[inline(always)]
	/// Returns `Self(47.0)`.
	pub fn new_47() -> Self { Self(47.0) }
	#[inline(always)]
	/// Returns `Self(48.0)`.
	pub fn new_48() -> Self { Self(48.0) }
	#[inline(always)]
	/// Returns `Self(49.0)`.
	pub fn new_49() -> Self { Self(49.0) }
	#[inline(always)]
	/// Returns `Self(50.0)`.
	pub fn new_50() -> Self { Self(50.0) }
	#[inline(always)]
	/// Returns `Self(51.0)`.
	pub fn new_51() -> Self { Self(51.0) }
	#[inline(always)]
	/// Returns `Self(52.0)`.
	pub fn new_52() -> Self { Self(52.0) }
	#[inline(always)]
	/// Returns `Self(53.0)`.
	pub fn new_53() -> Self { Self(53.0) }
	#[inline(always)]
	/// Returns `Self(54.0)`.
	pub fn new_54() -> Self { Self(54.0) }
	#[inline(always)]
	/// Returns `Self(55.0)`.
	pub fn new_55() -> Self { Self(55.0) }
	#[inline(always)]
	/// Returns `Self(56.0)`.
	pub fn new_56() -> Self { Self(56.0) }
	#[inline(always)]
	/// Returns `Self(57.0)`.
	pub fn new_57() -> Self { Self(57.0) }
	#[inline(always)]
	/// Returns `Self(58.0)`.
	pub fn new_58() -> Self { Self(58.0) }
	#[inline(always)]
	/// Returns `Self(59.0)`.
	pub fn new_59() -> Self { Self(59.0) }
	#[inline(always)]
	/// Returns `Self(60.0)`.
	pub fn new_60() -> Self { Self(60.0) }
	#[inline(always)]
	/// Returns `Self(61.0)`.
	pub fn new_61() -> Self { Self(61.0) }
	#[inline(always)]
	/// Returns `Self(62.0)`.
	pub fn new_62() -> Self { Self(62.0) }
	#[inline(always)]
	/// Returns `Self(63.0)`.
	pub fn new_63() -> Self { Self(63.0) }
	#[inline(always)]
	/// Returns `Self(64.0)`.
	pub fn new_64() -> Self { Self(64.0) }
	#[inline(always)]
	/// Returns `Self(65.0)`.
	pub fn new_65() -> Self { Self(65.0) }
	#[inline(always)]
	/// Returns `Self(66.0)`.
	pub fn new_66() -> Self { Self(66.0) }
	#[inline(always)]
	/// Returns `Self(67.0)`.
	pub fn new_67() -> Self { Self(67.0) }
	#[inline(always)]
	/// Returns `Self(68.0)`.
	pub fn new_68() -> Self { Self(68.0) }
	#[inline(always)]
	/// Returns `Self(69.0)`.
	pub fn new_69() -> Self { Self(69.0) }
	#[inline(always)]
	/// Returns `Self(70.0)`.
	pub fn new_70() -> Self { Self(70.0) }
	#[inline(always)]
	/// Returns `Self(71.0)`.
	pub fn new_71() -> Self { Self(71.0) }
	#[inline(always)]
	/// Returns `Self(72.0)`.
	pub fn new_72() -> Self { Self(72.0) }
	#[inline(always)]
	/// Returns `Self(73.0)`.
	pub fn new_73() -> Self { Self(73.0) }
	#[inline(always)]
	/// Returns `Self(74.0)`.
	pub fn new_74() -> Self { Self(74.0) }
	#[inline(always)]
	/// Returns `Self(75.0)`.
	pub fn new_75() -> Self { Self(75.0) }
	#[inline(always)]
	/// Returns `Self(76.0)`.
	pub fn new_76() -> Self { Self(76.0) }
	#[inline(always)]
	/// Returns `Self(77.0)`.
	pub fn new_77() -> Self { Self(77.0) }
	#[inline(always)]
	/// Returns `Self(78.0)`.
	pub fn new_78() -> Self { Self(78.0) }
	#[inline(always)]
	/// Returns `Self(79.0)`.
	pub fn new_79() -> Self { Self(79.0) }
	#[inline(always)]
	/// Returns `Self(80.0)`.
	pub fn new_80() -> Self { Self(80.0) }
	#[inline(always)]
	/// Returns `Self(81.0)`.
	pub fn new_81() -> Self { Self(81.0) }
	#[inline(always)]
	/// Returns `Self(82.0)`.
	pub fn new_82() -> Self { Self(82.0) }
	#[inline(always)]
	/// Returns `Self(83.0)`.
	pub fn new_83() -> Self { Self(83.0) }
	#[inline(always)]
	/// Returns `Self(84.0)`.
	pub fn new_84() -> Self { Self(84.0) }
	#[inline(always)]
	/// Returns `Self(85.0)`.
	pub fn new_85() -> Self { Self(85.0) }
	#[inline(always)]
	/// Returns `Self(86.0)`.
	pub fn new_86() -> Self { Self(86.0) }
	#[inline(always)]
	/// Returns `Self(87.0)`.
	pub fn new_87() -> Self { Self(87.0) }
	#[inline(always)]
	/// Returns `Self(88.0)`.
	pub fn new_88() -> Self { Self(88.0) }
	#[inline(always)]
	/// Returns `Self(89.0)`.
	pub fn new_89() -> Self { Self(89.0) }
	#[inline(always)]
	/// Returns `Self(90.0)`.
	pub fn new_90() -> Self { Self(90.0) }
	#[inline(always)]
	/// Returns `Self(91.0)`.
	pub fn new_91() -> Self { Self(91.0) }
	#[inline(always)]
	/// Returns `Self(92.0)`.
	pub fn new_92() -> Self { Self(92.0) }
	#[inline(always)]
	/// Returns `Self(93.0)`.
	pub fn new_93() -> Self { Self(93.0) }
	#[inline(always)]
	/// Returns `Self(94.0)`.
	pub fn new_94() -> Self { Self(94.0) }
	#[inline(always)]
	/// Returns `Self(95.0)`.
	pub fn new_95() -> Self { Self(95.0) }
	#[inline(always)]
	/// Returns `Self(96.0)`.
	pub fn new_96() -> Self { Self(96.0) }
	#[inline(always)]
	/// Returns `Self(97.0)`.
	pub fn new_97() -> Self { Self(97.0) }
	#[inline(always)]
	/// Returns `Self(98.0)`.
	pub fn new_98() -> Self { Self(98.0) }
	#[inline(always)]
	/// Returns `Self(99.0)`.
	pub fn new_99() -> Self { Self(99.0) }
	#[inline(always)]
	/// Returns `Self(100.0)`.
	pub fn new_100() -> Self { Self(100.0) }
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

		// Make sure result is `0.0`.
		assert!(v1 - v2 == v1);

		// Make sure result is `50.0`.
		assert!(v4 - v3 == v3);

		// Make sure result is `100.0`.
		assert!(v3 + v3 == v4);

		// Make sure overflowed result is `51.0`.
		assert!(v5 + v4 == v5);

		// Make sure underflowed result is `0.0`.
		assert!(v3 - v4 == v1);
	}
}
