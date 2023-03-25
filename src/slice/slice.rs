//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use std::collections::VecDeque;
use crate::collection::Collection;
use crate::key::{
	QueueKey,
	PlaylistKey,
	Key,
};

//---------------------------------------------------------------------------------------------------- Queue/Playlist
macro_rules! impl_slice {
	($name:ident, $key:ident) => {
		#[derive(Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
		/// Type-safe wrapper around [`VecDeque`].
		///
		/// Dynamically-sized view into a contiguous [`Key`] sequence.
		///
		/// This reimplements common [`VecDeque`] functions/traits, notably [`std::ops::Index`]. This allows for type-safe indexing.
		///
		/// For example, [`Playlist`] is ONLY allowed to be indexed with a [`PlaylistKey`]:
		/// ```rust,ignore
		/// let my_usize = 0;
		/// let key = PlaylistKey::from(my_usize);
		///
		/// // NOT type-safe, compile error!.
		/// state.playlists[my_usize];
		///
		/// // Type-safe, compiles.
		/// state.playlists[key];
		/// ```
		pub struct $name(pub(super) VecDeque<Key>);

		// Implement `[]` indexing.
		impl std::ops::Index<$key> for $name {
			type Output = Key;

			#[inline(always)]
			/// Index [`Self`] with its appropriate key instead of a [`usize`].
			///
			/// # Panics:
			/// The key must be a valid index.
			fn index(&self, key: $key) -> &Self::Output {
				&self.0[key.inner()]
			}
		}

		impl $name {
			// From a `VecDeque`.
			// This is only used internally.
			#[inline]
			pub(crate) fn from_vec(vec: VecDeque<Key>) -> Self {
				Self(vec)
			}

			// Private mutation functions.
			#[inline(always)]
			pub(crate)fn clear(&mut self) {
				self.0.clear();
			}

			#[inline(always)]
			pub(crate) const fn inner(&self) -> &VecDeque<Key> {
				&self.0
			}

			#[inline(always)]
			pub(crate) fn inner_mut(&mut self) -> &mut VecDeque<Key> {
				&mut self.0
			}

			#[inline(always)]
			pub(crate) fn remove(&mut self, index: usize) -> Option<Key> {
				self.0.remove(index)
			}

			#[inline(always)]
			pub(crate) fn push_back(&mut self, key: Key) {
				self.0.push_back(key)
			}

			#[inline(always)]
			pub(crate) fn push_front(&mut self, key: Key) {
				self.0.push_front(key)
			}

			#[inline(always)]
			pub(crate) fn pop_back(&mut self) -> Option<Key> {
				self.0.pop_back()
			}

			#[inline(always)]
			pub(crate) fn pop_front(&mut self) -> Option<Key> {
				self.0.pop_front()
			}

			// Creation.
			#[inline(always)]
			/// Returns a [`Self`] with `20` capacity reserved upfront.
			pub(crate) fn new() -> Self {
				Self(VecDeque::with_capacity(20))
			}

			#[inline(always)]
			/// Create an empty "dummy" (empty) struct.
			pub(crate) const fn dummy() -> Self {
				Self(VecDeque::new())
			}

			// Common functions `VecDeque` functions.
			#[inline(always)]
			/// Calls [`slice::len`].
			pub fn len(&self) -> usize {
				self.0.len()
			}

			#[inline(always)]
			/// Calls [`slice::get`].
			pub fn get(&self, key: $key) -> Option<&Key> {
				self.0.get(key.inner())
			}

			#[inline(always)]
			/// Calls [`slice::iter`].
			pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, Key> {
				self.0.iter()
			}

			#[inline(always)]
			/// Calls [`slice::iter`] then [`std::iter::Iterator::rev`].
			pub fn iter_rev(&self) -> std::iter::Rev<std::collections::vec_deque::Iter<'_, Key>> {
				self.0.iter().rev()
			}

			#[inline(always)]
			/// Calls [`slice::is_empty`].
			pub fn is_empty(&self) -> bool {
				self.0.is_empty()
			}
		}
	}
}

impl_slice!(Queue, QueueKey);
impl_slice!(Playlist, PlaylistKey);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod test {
//  #[test]
//  fn __TEST__() {
//  }
//}
