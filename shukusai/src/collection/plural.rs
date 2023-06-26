//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use crate::collection::{
	Artist,Album,Song,
	ArtistKey,AlbumKey,SongKey,
};

//---------------------------------------------------------------------------------------------------- Plural newtypes around `Vec<T>`.
macro_rules! impl_plural {
	($name:ident, $plural:ident, $key:ident) => { paste::paste! {
		#[derive(Clone,Debug,PartialEq,PartialOrd,Encode,Decode)]
		/// Type-safe wrapper around a [`Box`]'ed [`slice`].
		///
		#[doc = "This struct's inner value is just `Box<[" $name "]>`"]
		///
		/// This reimplements common [`slice`] functions/traits, notably [`std::ops::Index`]. This allows for type-safe indexing.
		///
		/// For example, [`Albums`] is ONLY allowed to be indexed with an [`AlbumKey`]:
		/// ```rust,ignore
		/// let my_usize = 0;
		/// let key = AlbumKey::from(my_usize);
		///
		/// // NOT type-safe, compile error!.
		/// collection.albums[my_usize];
		///
		/// // Type-safe, compiles.
		/// collection.albums[key];
		///```
		#[doc = "[`Collection`] itself can also be directly index with [`" $key "`]."]
		//-------------------------------------------------- Define plural `struct`.
		pub struct $plural(pub(crate) Box<[$name]>);

		//-------------------------------------------------- Implement `[]` indexing.
		impl std::ops::Index<$key> for $plural {
			type Output = $name;

			#[inline(always)]
			#[doc = "Index [`" $plural "`] with [`" $key "`]."]
			///
			/// # Panics:
			/// The key must be a valid index.
			fn index(&self, key: $key) -> &Self::Output {
				&self.0[key.inner()]
			}
		}
		impl std::ops::Index<&$key> for $plural {
			type Output = $name;

			#[inline(always)]
			#[doc = "Index [`" $plural "`] with [`" $key "`]."]
			///
			/// # Panics:
			/// The key must be a valid index.
			fn index(&self, key: &$key) -> &Self::Output {
				&self.0[key.inner()]
			}
		}

		impl $plural {
			//-------------------------------------------------- `pub(crate)` functions
			#[inline(always)]
			pub(crate) fn new() -> Self {
				Self(Box::new([]))
			}

			#[inline(always)]
			/// Calls [`slice::iter_mut`].
			pub(crate) fn iter_mut(&mut self) -> std::slice::IterMut<'_, $name> {
				self.0.iter_mut()
			}

			#[inline(always)]
			/// Create self from a [`Vec`].
			pub(crate) fn from_vec(vec: Vec<$name>) -> Self {
				Self(vec.into_boxed_slice())
			}

			//-------------------------------------------------- Common `Vec` and related functions.
			#[inline(always)]
			/// Calls [`slice::iter`].
			pub fn iter(&self) -> std::slice::Iter<'_, $name> {
				self.0.iter()
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
	}}
}

impl_plural!(Artist, Artists, ArtistKey);
impl_plural!(Album, Albums, AlbumKey);
impl_plural!(Song, Songs, SongKey);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
