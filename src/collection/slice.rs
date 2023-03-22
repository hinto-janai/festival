//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use std::collections::VecDeque;
use crate::collection::key::{
	QueueKey,
	PlaylistKey,
	Key,
};

//---------------------------------------------------------------------------------------------------- Queue/Playlist
#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
/// Dynamically-sized view into a contiguous [`Key`] sequence.
///
/// Both `Queue` and `Playlist` are practically the same thing:
///   - A `Slice` of the `Collection`
///
/// They contain a bunch of `Key`s that point
/// to "segments" of the `Collection` (it's a slice).
///
/// They both are saved to disk via `State` which saves as `state.bin`.
pub struct Slice(VecDeque<Key>);

impl Slice {
	#[inline(always)]
	/// Returns a [`Slice`] with `20` capacity reserved upfront.
	pub fn new() -> Self {
		Self(VecDeque::with_capacity(20))
	}

	#[inline(always)]
	/// Create an empty "dummy" (empty) struct.
	pub const fn dummy() -> Self {
		Self(VecDeque::new())
	}

	/// Allows using `VecDeque` methods.
	#[inline(always)]
	pub const fn inner(&self) -> &VecDeque<Key> {
		&self.0
	}
	#[inline(always)]
	pub fn inner_mut(&mut self) -> &mut VecDeque<Key> {
		&mut self.0
	}

	#[inline(always)]
	pub fn queue(&self, key: QueueKey) -> Key {
		self.0[key.inner()]
	}

	#[inline(always)]
	pub fn playlist(&self, key: PlaylistKey) -> Key {
		self.0[key.inner()]
	}

	// Common functions `VecDeque` functions.
	#[inline(always)]
	pub fn len(&self) -> usize {
		self.0.len()
	}
	#[inline(always)]
	pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, Key> {
		self.0.iter()
	}
	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
	#[inline(always)]
	pub fn clear(&mut self) {
		self.0.clear();
	}
	#[inline(always)]
	pub fn remove(&mut self, index: usize) -> Option<Key> {
		self.0.remove(index)
	}
	#[inline(always)]
	pub fn push_back(&mut self, key: Key) {
		self.0.push_back(key)
	}
	#[inline(always)]
	pub fn push_front(&mut self, key: Key) {
		self.0.push_front(key)
	}
	#[inline(always)]
	pub fn pop_back(&mut self) -> Option<Key> {
		self.0.pop_back()
	}
	#[inline(always)]
	pub fn pop_front(&mut self) -> Option<Key> {
		self.0.pop_front()
	}
}

impl std::default::Default for Slice {
	#[inline(always)]
	fn default() -> Self {
		Self::new()
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod test {
//  #[test]
//  fn __TEST__() {
//  }
//}
