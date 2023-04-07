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
/// Wrapper around [`u8`] that is between `0..100`
///
/// This is the "unit" [`Kernel`] wants audio volume changes in.
///
/// It guarantees the inner [`u8`] is between `0..100` so that
/// frontends can't just send random numbers that make no sense in the
/// context of changing the volume level, like `253`.
#[derive(Copy,Clone,Debug,PartialEq,PartialOrd,Serialize,Deserialize)]
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
		write!(f, "{}", self)
	}
}

impl std::ops::Add for Volume {
    type Output = Self;

	/// Add a [`Volume`] to a [`Volume`].
	///
	/// If an overflow occurs, [`Volume::new_100`] is returned.
    fn add(self, other: Self) -> Self {
		let f = self.0 + other.0;

		if f > 100 { return Self::new_100() }

		Self(f)
	}
}

impl std::ops::Sub for Volume {
    type Output = Self;

	/// Subtract a [`Volume`] to a [`Volume`].
	///
	/// If the result is negative, [`Volume::new_0`] is returned.
    fn sub(self, other: Self) -> Self {
		let f = self.0 - other.0;

		if f < 0 { return Self(0) }

		Self(f)
	}
}

//---------------------------------------------------------------------------------------------------- Volume new.
impl Volume {
	/// Returns `Self(0)`.
	pub const fn new_0() -> Self { Self(0) }
	/// Returns `Self(1)`.
	pub const fn new_1() -> Self { Self(1) }
	/// Returns `Self(2)`.
	pub const fn new_2() -> Self { Self(2) }
	/// Returns `Self(3)`.
	pub const fn new_3() -> Self { Self(3) }
	/// Returns `Self(4)`.
	pub const fn new_4() -> Self { Self(4) }
	/// Returns `Self(5)`.
	pub const fn new_5() -> Self { Self(5) }
	/// Returns `Self(6)`.
	pub const fn new_6() -> Self { Self(6) }
	/// Returns `Self(7)`.
	pub const fn new_7() -> Self { Self(7) }
	/// Returns `Self(8)`.
	pub const fn new_8() -> Self { Self(8) }
	/// Returns `Self(9)`.
	pub const fn new_9() -> Self { Self(9) }
	/// Returns `Self(10)`.
	pub const fn new_10() -> Self { Self(10) }
	/// Returns `Self(11)`.
	pub const fn new_11() -> Self { Self(11) }
	/// Returns `Self(12)`.
	pub const fn new_12() -> Self { Self(12) }
	/// Returns `Self(13)`.
	pub const fn new_13() -> Self { Self(13) }
	/// Returns `Self(14)`.
	pub const fn new_14() -> Self { Self(14) }
	/// Returns `Self(15)`.
	pub const fn new_15() -> Self { Self(15) }
	/// Returns `Self(16)`.
	pub const fn new_16() -> Self { Self(16) }
	/// Returns `Self(17)`.
	pub const fn new_17() -> Self { Self(17) }
	/// Returns `Self(18)`.
	pub const fn new_18() -> Self { Self(18) }
	/// Returns `Self(19)`.
	pub const fn new_19() -> Self { Self(19) }
	/// Returns `Self(20)`.
	pub const fn new_20() -> Self { Self(20) }
	/// Returns `Self(21)`.
	pub const fn new_21() -> Self { Self(21) }
	/// Returns `Self(22)`.
	pub const fn new_22() -> Self { Self(22) }
	/// Returns `Self(23)`.
	pub const fn new_23() -> Self { Self(23) }
	/// Returns `Self(24)`.
	pub const fn new_24() -> Self { Self(24) }
	/// Returns `Self(25)`.
	pub const fn new_25() -> Self { Self(25) }
	/// Returns `Self(26)`.
	pub const fn new_26() -> Self { Self(26) }
	/// Returns `Self(27)`.
	pub const fn new_27() -> Self { Self(27) }
	/// Returns `Self(28)`.
	pub const fn new_28() -> Self { Self(28) }
	/// Returns `Self(29)`.
	pub const fn new_29() -> Self { Self(29) }
	/// Returns `Self(30)`.
	pub const fn new_30() -> Self { Self(30) }
	/// Returns `Self(31)`.
	pub const fn new_31() -> Self { Self(31) }
	/// Returns `Self(32)`.
	pub const fn new_32() -> Self { Self(32) }
	/// Returns `Self(33)`.
	pub const fn new_33() -> Self { Self(33) }
	/// Returns `Self(34)`.
	pub const fn new_34() -> Self { Self(34) }
	/// Returns `Self(35)`.
	pub const fn new_35() -> Self { Self(35) }
	/// Returns `Self(36)`.
	pub const fn new_36() -> Self { Self(36) }
	/// Returns `Self(37)`.
	pub const fn new_37() -> Self { Self(37) }
	/// Returns `Self(38)`.
	pub const fn new_38() -> Self { Self(38) }
	/// Returns `Self(39)`.
	pub const fn new_39() -> Self { Self(39) }
	/// Returns `Self(40)`.
	pub const fn new_40() -> Self { Self(40) }
	/// Returns `Self(41)`.
	pub const fn new_41() -> Self { Self(41) }
	/// Returns `Self(42)`.
	pub const fn new_42() -> Self { Self(42) }
	/// Returns `Self(43)`.
	pub const fn new_43() -> Self { Self(43) }
	/// Returns `Self(44)`.
	pub const fn new_44() -> Self { Self(44) }
	/// Returns `Self(45)`.
	pub const fn new_45() -> Self { Self(45) }
	/// Returns `Self(46)`.
	pub const fn new_46() -> Self { Self(46) }
	/// Returns `Self(47)`.
	pub const fn new_47() -> Self { Self(47) }
	/// Returns `Self(48)`.
	pub const fn new_48() -> Self { Self(48) }
	/// Returns `Self(49)`.
	pub const fn new_49() -> Self { Self(49) }
	/// Returns `Self(50)`.
	pub const fn new_50() -> Self { Self(50) }
	/// Returns `Self(51)`.
	pub const fn new_51() -> Self { Self(51) }
	/// Returns `Self(52)`.
	pub const fn new_52() -> Self { Self(52) }
	/// Returns `Self(53)`.
	pub const fn new_53() -> Self { Self(53) }
	/// Returns `Self(54)`.
	pub const fn new_54() -> Self { Self(54) }
	/// Returns `Self(55)`.
	pub const fn new_55() -> Self { Self(55) }
	/// Returns `Self(56)`.
	pub const fn new_56() -> Self { Self(56) }
	/// Returns `Self(57)`.
	pub const fn new_57() -> Self { Self(57) }
	/// Returns `Self(58)`.
	pub const fn new_58() -> Self { Self(58) }
	/// Returns `Self(59)`.
	pub const fn new_59() -> Self { Self(59) }
	/// Returns `Self(60)`.
	pub const fn new_60() -> Self { Self(60) }
	/// Returns `Self(61)`.
	pub const fn new_61() -> Self { Self(61) }
	/// Returns `Self(62)`.
	pub const fn new_62() -> Self { Self(62) }
	/// Returns `Self(63)`.
	pub const fn new_63() -> Self { Self(63) }
	/// Returns `Self(64)`.
	pub const fn new_64() -> Self { Self(64) }
	/// Returns `Self(65)`.
	pub const fn new_65() -> Self { Self(65) }
	/// Returns `Self(66)`.
	pub const fn new_66() -> Self { Self(66) }
	/// Returns `Self(67)`.
	pub const fn new_67() -> Self { Self(67) }
	/// Returns `Self(68)`.
	pub const fn new_68() -> Self { Self(68) }
	/// Returns `Self(69)`.
	pub const fn new_69() -> Self { Self(69) }
	/// Returns `Self(70)`.
	pub const fn new_70() -> Self { Self(70) }
	/// Returns `Self(71)`.
	pub const fn new_71() -> Self { Self(71) }
	/// Returns `Self(72)`.
	pub const fn new_72() -> Self { Self(72) }
	/// Returns `Self(73)`.
	pub const fn new_73() -> Self { Self(73) }
	/// Returns `Self(74)`.
	pub const fn new_74() -> Self { Self(74) }
	/// Returns `Self(75)`.
	pub const fn new_75() -> Self { Self(75) }
	/// Returns `Self(76)`.
	pub const fn new_76() -> Self { Self(76) }
	/// Returns `Self(77)`.
	pub const fn new_77() -> Self { Self(77) }
	/// Returns `Self(78)`.
	pub const fn new_78() -> Self { Self(78) }
	/// Returns `Self(79)`.
	pub const fn new_79() -> Self { Self(79) }
	/// Returns `Self(80)`.
	pub const fn new_80() -> Self { Self(80) }
	/// Returns `Self(81)`.
	pub const fn new_81() -> Self { Self(81) }
	/// Returns `Self(82)`.
	pub const fn new_82() -> Self { Self(82) }
	/// Returns `Self(83)`.
	pub const fn new_83() -> Self { Self(83) }
	/// Returns `Self(84)`.
	pub const fn new_84() -> Self { Self(84) }
	/// Returns `Self(85)`.
	pub const fn new_85() -> Self { Self(85) }
	/// Returns `Self(86)`.
	pub const fn new_86() -> Self { Self(86) }
	/// Returns `Self(87)`.
	pub const fn new_87() -> Self { Self(87) }
	/// Returns `Self(88)`.
	pub const fn new_88() -> Self { Self(88) }
	/// Returns `Self(89)`.
	pub const fn new_89() -> Self { Self(89) }
	/// Returns `Self(90)`.
	pub const fn new_90() -> Self { Self(90) }
	/// Returns `Self(91)`.
	pub const fn new_91() -> Self { Self(91) }
	/// Returns `Self(92)`.
	pub const fn new_92() -> Self { Self(92) }
	/// Returns `Self(93)`.
	pub const fn new_93() -> Self { Self(93) }
	/// Returns `Self(94)`.
	pub const fn new_94() -> Self { Self(94) }
	/// Returns `Self(95)`.
	pub const fn new_95() -> Self { Self(95) }
	/// Returns `Self(96)`.
	pub const fn new_96() -> Self { Self(96) }
	/// Returns `Self(97)`.
	pub const fn new_97() -> Self { Self(97) }
	/// Returns `Self(98)`.
	pub const fn new_98() -> Self { Self(98) }
	/// Returns `Self(99)`.
	pub const fn new_99() -> Self { Self(99) }
	/// Returns `Self(100)`.
	pub const fn new_100() -> Self { Self(100) }
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

		// Make sure overflowed result is `51`.
		assert!(v5 + v4 == v5);

		// Make sure underflowed result is `0`.
		assert!(v3 - v4 == v1);
	}
}
