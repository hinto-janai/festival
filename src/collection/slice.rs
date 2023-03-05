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
	CollectionKey,
};

//---------------------------------------------------------------------------------------------------- Queue/Playlist
// Both `Queue` and `Playlist` are practically the same thing:
//   - A `Slice` of the `Collection`
//
// They contain a bunch of `CollectionKey`s that point
// to "segments" of the `Collection` (it's a slice).
//
// They both are saved to disk via `State` which saves as `state.bincode`.
#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub(crate) struct CollectionSlice(VecDeque<CollectionKey>);

impl CollectionSlice {
	#[inline(always)]
	pub(crate) fn new() -> Self {
		Self(VecDeque::with_capacity(20))
	}

	#[inline(always)]
	// Create an empty "dummy" struct.
	pub(crate) fn dummy() -> Self {
		Self(VecDeque::new())
	}

	// Allows using `VecDeque` methods.
	#[inline(always)]
	pub(crate) fn inner(&self) -> &VecDeque<CollectionKey> {
		&self.0
	}
	#[inline(always)]
	pub(crate) fn inner_mut(&mut self) -> &mut VecDeque<CollectionKey> {
		&mut self.0
	}

	// Bypasses `Self` and directly indexes the inner `VecDeque`.
	#[inline(always)]
	pub(crate) fn index(&self, index: usize) -> &CollectionKey {
		&self.0[index]
	}

	// Common functions `VecDeque` functions.
	#[inline(always)]
	pub(crate) fn len(&self) -> usize {
		self.0.len()
	}
	#[inline(always)]
	pub(crate) fn iter(&self) -> std::collections::vec_deque::Iter<'_, CollectionKey> {
		self.0.iter()
	}
	#[inline(always)]
	pub(crate) fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
	#[inline(always)]
	pub(crate) fn clear(&mut self) {
		self.0.clear();
	}
	#[inline(always)]
	pub(crate) fn remove(&mut self, index: usize) -> Option<CollectionKey> {
		self.0.remove(index)
	}
	#[inline(always)]
	pub(crate) fn push_back(&mut self, key: CollectionKey) {
		self.0.push_back(key)
	}
	#[inline(always)]
	pub(crate) fn push_front(&mut self, key: CollectionKey) {
		self.0.push_front(key)
	}
	#[inline(always)]
	pub(crate) fn pop_back(&mut self) -> Option<CollectionKey> {
		self.0.pop_back()
	}
	#[inline(always)]
	pub(crate) fn pop_front(&mut self) -> Option<CollectionKey> {
		self.0.pop_front()
	}

}

impl std::default::Default for CollectionSlice {
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
