/*
 * `*Ptr` types.
 *
 * As long as this comment is here, this is not
 * actually in use. If the day comes where the
 * `*Ptr` types are actually used, this comment
 * will be removed and this file will be added
 * to the `mod.rs` file.
 */

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
		pub(crate) struct $ptr(pub(crate) *const $name);
		unsafe impl Send for $ptr {}
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
			fn from(object: &$name) -> Self {
				// INVARIANT:
				// This only works because:
				// 1. The objects should be `Box`'ed (unmoving address)
				// 2. The outer container (`Collection`) is immutable
				//
				// These invariants must be upheld to ensure
				// dereferencing these pointers don't cause problems.
				Self(std::ptr::addr_of!(*object))
			}
		}

		//-------------------------------------------------- Encode
		impl Encode for $ptr {
			#[inline(always)]
			fn encode<E: Encoder>(&self, encoder: &mut E) -> std::result::Result<(), EncodeError> {
				// INVARIANT:
				// We cannot save memory addresses to disk, so
				// `PhantomData` is saved (ZST) under the assumption
				// that the pointers will be properly initialized
				// after/during the construction process.
				Encode::encode(&std::marker::PhantomData::<Self>, encoder)
			}
		}

		//-------------------------------------------------- Decode
		impl Decode for $ptr {
			#[inline(always)]
			fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
				// INVARIANT:
				// These `nullptr`'s must be properly initialized
				// during the outer `Collection` decoding process.
				Ok(Self(std::ptr::null()))
			}
		}

		//-------------------------------------------------- `nullptr` conversion
		impl $ptr {
			#[inline(always)]
			// Take the `bincode::Decode`'ed `Box` (which are full
			// of `nullptr`'s) and replace them by chasing the index
			// and getting the address of the actual object.
			pub(crate) fn decode(
				plural: &$plural,
				mut boxed: Box<[($ptr, $key)]>,
			) -> Box<[($ptr, $key)]> {
				for (ptr, key) in boxed.iter_mut() {
					*ptr = Self::from(&plural[*key]);
				}

				boxed
			}
		}
	}}
}

impl_ptr!("Artist", Artist, Artists, ArtistPtr, ArtistKey);
impl_ptr!("Album", Album, Albums, AlbumPtr, AlbumKey);
impl_ptr!("Song", Song, Songs, SongPtr, SongKey);
