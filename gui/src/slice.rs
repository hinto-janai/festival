//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use benri::{
//};
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use compact_str::{
	format_compact,
	CompactString,
};

//---------------------------------------------------------------------------------------------------- Impl
impl<T: AsRef<str> + std::fmt::Display> Head for T {}
impl<T: AsRef<str> + std::fmt::Display> Tail for T {}
impl<T: AsRef<str> + std::fmt::Display> HeadTail for T {}

//---------------------------------------------------------------------------------------------------- Head
/// Get the head part of a `str`.
pub trait Head: AsRef<str> + std::fmt::Display {
	/// Return the first `len` bytes of this [`str`].
	///
	/// This will return the full [`str`] if the `len` is
	/// longer than the actual inner [`str`].
	///
	/// ## Note
	/// Input is split by [`char`]'s, not bytes.
	fn head(&self, len: usize) -> CompactString {
		let s = self.as_ref();

		if let Some((index, _)) = s.char_indices().nth(len) {
			CompactString::new(&s[..index])
		} else {
			CompactString::new(s)
		}
	}

	/// Same as [`head()`] but the [`CompactString`] ends with `...`
	///
	/// This will return the full string without `...` if
	/// the `len` is longer than the actual inner [`str`].
	///
	/// ## Note
	/// Input is split by [`char`]'s, not bytes.
	fn head_dot(&self, len: usize) -> CompactString {
		let s = self.as_ref();

		if let Some((index, _)) = s.char_indices().nth(len) {
			format_compact!("{}...", &s[..index])
		} else {
			CompactString::new(s)
		}
	}
}

//---------------------------------------------------------------------------------------------------- Tail
/// Get the tail part of a `str`.
pub trait Tail: AsRef<str> + std::fmt::Display {
	/// Return the last `len` bytes of this [`str`].
	///
	/// This will return the full [`str`] if the `len` is
	/// longer than the actual inner [`str`].
	///
	/// ## Note
	/// Input is split by [`char`]'s, not bytes.
	fn tail(&self, len: usize) -> CompactString {
		let s = self.as_ref();

		let end = s.chars().count();

		if len >= end {
			return CompactString::new(s);
		}

		if let Some((index, _)) = s.char_indices().nth(end - len) {
			return CompactString::new(&s[index..]);
		}

		CompactString::new(s)
	}

	/// Same as [`tail()`] but returns a [`String`] starting with `...`
	///
	/// This will return the full string without `...` if
	/// the `len` is longer than the actual inner [`str`].
	///
	/// ## Note
	/// Input is split by [`char`]'s, not bytes.
	fn tail_dot(&self, len: usize) -> CompactString {
		let s = self.as_ref();

		let end = s.chars().count();

		if len >= end {
			format_compact!("...{}", s);
		}

		if let Some((index, _)) = s.char_indices().nth(end - len) {
			return format_compact!("...{}", &s[index..]);
		}

		format_compact!("...{}", s)
	}
}

////---------------------------------------------------------------------------------------------------- HeadTail
/// Get the head + tail part of a `str`.
pub trait HeadTail: AsRef<str> + std::fmt::Display {
	/// Return the first `head` bytes and last `tail`
	/// bytes of this string separated with `...`.
	///
	/// ## Note
	/// Input is split by [`char`]'s, not bytes.
	fn head_tail(&self, head: usize, tail: usize) -> CompactString {
		let s = self.as_ref();

		let end = s.chars().count();

		if head >= end || tail >= end || head + tail >= end {
			return CompactString::new(s);
		}

		// Iterator is consumed, must create twice.
		let head = s.char_indices().nth(head);
		let tail = s.char_indices().nth(end - tail);

		if let (Some((head, _)), Some((tail, _))) = (head, tail) {
			return format_compact!("{}...{}", &s[..head], &s[tail..]);
		}

		CompactString::new(s)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn _head() {
		let four_bytes = "desu";
		let six_bytes  = "です";

		assert!(four_bytes.len() == 4);
		assert!(six_bytes.len()  == 6);

		assert!(four_bytes.head(2) == "de");
		assert!(six_bytes.head(1) == "で");

		assert!(!four_bytes.head(2).is_heap_allocated());
		assert!(!six_bytes.head(1).is_heap_allocated());
	}

	#[test]
	fn _head_dot() {
		let four_bytes = "desu";
		let six_bytes  = "です";

		assert!(four_bytes.len() == 4);
		assert!(six_bytes.len()  == 6);

		assert!(four_bytes.head_dot(2) == "de...");
		assert!(six_bytes.head_dot(1) == "で...");

		assert!(!four_bytes.head_dot(2).is_heap_allocated());
		assert!(!six_bytes.head_dot(1).is_heap_allocated());
	}

	#[test]
	fn _tail() {
		let four_bytes = "desu";
		let six_bytes  = "です";

		assert!(four_bytes.len() == 4);
		assert!(six_bytes.len()  == 6);

		assert!(four_bytes.tail(2) == "su");
		assert!(six_bytes.tail(1) == "す");

		assert!(!four_bytes.tail(2).is_heap_allocated());
		assert!(!six_bytes.tail(2).is_heap_allocated());
	}

	#[test]
	fn _tail_dot() {
		let four_bytes = "desu";
		let six_bytes  = "です";

		assert!(four_bytes.len() == 4);
		assert!(six_bytes.len()  == 6);

		assert!(four_bytes.tail_dot(2) == "...su");
		assert!(six_bytes.tail_dot(1) == "...す");

		assert!(!four_bytes.tail_dot(2).is_heap_allocated());
		assert!(!six_bytes.tail_dot(1).is_heap_allocated());
	}

	#[test]
	fn _head_tail() {
		let eight_bytes = "desukedo";
		let twelve_bytes  = "ですけど";

		assert!(eight_bytes.len() == 8);
		assert!(twelve_bytes.len()  == 12);

		assert!(eight_bytes.head_tail(2, 2) == "de...do");
		assert!(twelve_bytes.head_tail(1, 1) == "で...ど");

		assert!(eight_bytes.head_tail(4, 4) == "desukedo");
		assert!(twelve_bytes.head_tail(2, 2) == "ですけど");

		assert!(!eight_bytes.head_tail(2, 2).is_heap_allocated());
		assert!(!twelve_bytes.head_tail(1, 1).is_heap_allocated());
	}
}