//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use super::{
	{Artist,Album,Song},
	{ArtistKey,AlbumKey,SongKey},
	{QueueKey,PlaylistKey},
};

//---------------------------------------------------------------------------------------------------- Plural newtypes around `Vec<T>`.
macro_rules! impl_plural {
	($name:ident, $plural:ident, $key:ident) => {
		#[derive(Debug,Serialize,Deserialize)]
		/// Type-safe wrapper around [`Vec`].
		///
		/// This struct's inner value is just [`Vec<T>`], where `T` is the non-plural version of this `struct`'s name.
		///
		/// E.g: `AlbumKeys` is just a `Vec<AlbumKey>`.
		///
		/// This reimplements common [`Vec`] functions/traits, notably [`std::ops::Index`]. This allows for type-safe indexing.
		///
		/// For example, [`AlbumKeys`] is ONLY allowed to be indexed with a [`AlbumKey`]:
		/// ```rust
		/// let my_usize = 0;
		/// let key = AlbumKey::from(my_usize);
		///
		/// // NOT type-safe, compile error!.
		/// collection.albums[my_usize];
		///
		/// // Type-safe, compiles.
		/// collection.albums[key];
		/// ```
		// Define plural `struct`.
		pub struct $plural(pub(super) Vec<$name>);

		// Implement `[]` indexing.
		impl std::ops::Index<$key> for $plural {
			type Output = $name;

			#[inline(always)]
			/// Index [`Self`] with its appropriate key instead of a [`usize`].
			///
			/// # Panics:
			/// The key must be a valid index.
			fn index(&self, key: $key) -> &Self::Output {
				&self.0[key.inner()]
			}
		}

		// Implement common `Vec` and related functions.
		impl $plural {
			#[inline(always)]
			/// Calls [`slice::iter`].
			pub fn iter(&self) -> std::slice::Iter<'_, $name> {
				self.0.iter()
			}

			#[inline(always)]
			/// Calls [`slice::iter`] then [`std::iter::Iterator::rev`].
			pub fn iter_rev(&self) -> std::iter::Rev<std::slice::Iter<'_, $name>> {
				self.0.iter().rev()
			}

			#[inline(always)]
			/// Calls [`slice::get`].
			pub fn get(&self, key: $key) -> Option<&$name> {
				self.0.get(key.inner())
			}

			#[inline(always)]
			/// Calls [`slice::first`].
			pub fn first(&self) -> Option<&$name> {
				self.0.first()
			}

			#[inline(always)]
			/// Calls [`slice::last`].
			pub fn last(&self) -> Option<&$name> {
				self.0.last()
			}

			#[inline(always)]
			/// Calls [`slice::len`].
			pub fn len(&self) -> usize {
				self.0.len()
			}

			#[inline(always)]
			/// Calls [`slice::is_empty`].
			pub fn is_empty(&self) -> bool {
				self.0.is_empty()
			}
		}

		// From a `Vec`.
		// This is only used internally.
		impl From<Vec<$name>> for $plural {
			#[inline]
			fn from(vec: Vec<$name>) -> Self {
				Self(vec)
			}
		}
	}
}

impl_plural!(Artist, Artists, ArtistKey);
impl_plural!(Album, Albums, AlbumKey);
impl_plural!(Song, Songs, SongKey);
impl_plural!(SongKey, SongKeys, SongKey);
impl_plural!(QueueKey, QueueKeys, QueueKey);
impl_plural!(PlaylistKey, PlaylistKeys, PlaylistKey);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
