//---------------------------------------------------------------------------------------------------- Use
use crate::collection::{
	Artists,Albums,Songs,
	Collection,Artist,Album,Song,
	ArtistKey,AlbumKey,SongKey,
};
use std::pin::Pin;
use std::sync::Arc;
use bincode::{
	de::{Decode,Decoder},
	enc::{Encode,Encoder},
	error::{DecodeError,EncodeError},
};
use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- CollectionPtr
#[derive(Clone,Debug,PartialEq)]
///
pub struct CollectionPtr(Pin<Arc<Collection>>);

impl CollectionPtr {
	pub(crate) fn new(collection: Collection) -> Self {
		Self(Pin::new(Arc::new(collection)))
	}

	pub(crate) fn into_inner(self) -> Arc<Collection> {
		Pin::into_inner(self.0)
	}
}

impl std::ops::Deref for CollectionPtr {
	type Target = Collection;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl AsRef<Collection> for CollectionPtr {
	fn as_ref(&self) -> &Collection {
		&self.0
	}
}

//---------------------------------------------------------------------------------------------------- *Ptr
macro_rules! impl_ptr {
	($name_lit:literal, $name:ident, $plural:ident, $ptr:ident, $key:ident) => { paste::paste! {
		#[derive(Copy,Clone,Debug,PartialEq,PartialOrd)]
		/// Raw pointer to [`" $name "`]
		///
		#[doc = "This struct's inner value is just `*const " $name "`"]
		///
		/// ## Safety
		/// These raw pointers are initialized alongside the `Collection`, thus
		/// the pointers that exist _within_ one will always point to valid objects.
		///
		/// As long as you are accessing these pointers via a `Collection`:
		/// ```rust,ignore
		/// for (ptr, key) in collection.sort_album_release {
		///     // Safe!
		///     let artist: &Artist = &*ptr;
		/// }
		/// ```
		/// it will be safe.
		///
		/// **Do not ever ever ever `Copy/Clone` these pointers and
		/// deref them outside the context of a parent `Collection`:**
		/// ```rust,ignore
		/// let (ptr, key) = collection.sort_album_release[0];
		/// drop(collection);
		///
		/// /* some time passes */
		///
		/// // VERY UNSAFE
		/// let artist: &Artist = &*ptr;
		/// ```
		/// If "pointer" like objects need to be saved for later use,
		/// `Copy` the `*Key` types and use those instead.
		//-------------------------------------------------- Define ptr `struct`.
		pub struct $ptr(pub(crate) *const $name);
		// SAFETY: only safe in the context of a outer `Collection`.
		unsafe impl Send for $ptr {}
		// SAFETY: only safe in the context of a outer `Collection`.
		unsafe impl Sync for $ptr {}

		//-------------------------------------------------- Implement Deref
		impl std::ops::Deref for $ptr {
			type Target = $name;

			#[inline(always)]
			fn deref(&self) -> &Self::Target {
				#[cfg(debug_assertions)]
				if self.0.is_null() {
					panic!("nullptr for {} @ {:?}", $name_lit, self.0);
				}

				// SAFETY:
				// This should only be dereferenced
				// in the context of a `Collection`.
				unsafe { &*self.0 }
			}
		}

		//-------------------------------------------------- From
		impl From<&$name> for $ptr {
			#[inline]
			fn from(object: &$name) -> Self {
				// INVARIANT:
				// This only works because:
				// 1. The objects should be `Box`'ed (unmoving address)
				// 2. The outer container (`Collection`) is immutable and pinned
				//
				// These invariants must be upheld to ensure
				// dereferencing these pointers don't cause problems.
				Self(std::ptr::addr_of!(*object))
			}
		}

		//-------------------------------------------------- Encode
		impl Encode for $ptr {
			fn encode<E: Encoder>(&self, encoder: &mut E) -> std::result::Result<(), EncodeError> {
				// INVARIANT:
				// We cannot save memory addresses to disk, so
				// nothing is saved under the assumption
				// that the pointers will be properly initialized
				// after/during the construction process.
				Ok(())
			}
		}

		//-------------------------------------------------- Decode
		impl Decode for $ptr {
			fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
				// INVARIANT:
				// These `nullptr`'s must be properly initialized
				// during the outer `Collection` decoding process.
				Ok(Self(std::ptr::null()))
			}
		}

		//-------------------------------------------------- `nullptr` conversion
		impl $ptr {
			// 1. Index into plural (Artists, Albums, Songs)
			// 2. Chase the index
			// 3. Acquire and set the pointer
			pub(crate) fn fix(plural: &$plural, slice: &mut [($key, Self)]) {
				slice
					.iter_mut()
					.for_each(|tuple| tuple.1 = Self::from(&plural[tuple.0]));
			}

			pub(crate) fn from_key(plural: &$plural, slice: Box<[$key]>) -> Box<[($key, Self)]> {
				slice
					.into_iter()
					.map(|key| (*key, Self::from(&plural[key])))
					.collect()
			}

			// Return a self with an inner null pointer.
			pub(crate) fn null() -> Self {
				Self(std::ptr::null())
			}
		}
	}}
}

impl_ptr!("Artist", Artist, Artists, ArtistPtr, ArtistKey);
impl_ptr!("Album", Album, Albums, AlbumPtr, AlbumKey);
impl_ptr!("Song", Song, Songs, SongPtr, SongKey);
