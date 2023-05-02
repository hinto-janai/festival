//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use crate::collection::{
	Collection,
	Artist,
	Album,
	Song,
};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};

//---------------------------------------------------------------------------------------------------- Macros to implement common traits.
macro_rules! impl_common {
	($type:ty) => {
		impl $type {
			#[inline(always)]
			/// Returns `Self(0)`.
			pub const fn new() -> Self {
				Self(0)
			}
			#[inline(always)]
			/// Returns `Self(0)`.
			pub const fn zero() -> Self {
				Self(0)
			}
			#[inline(always)]
			/// Returns the inner `usize`.
			pub const fn inner(&self) -> usize {
				self.0
			}
		}
		impl From<u8> for $type {
			#[inline(always)]
			fn from(index: u8) -> Self {
				Self(index as usize)
			}
		}
		impl From<u16> for $type {
			#[inline(always)]
			fn from(index: u16) -> Self {
				Self(index as usize)
			}
		}
		impl From<u32> for $type {
			#[inline(always)]
			fn from(index: u32) -> Self {
				Self(index as usize)
			}
		}
		#[cfg(target_pointer_width = "64")]
		impl From<u64> for $type {
			#[inline(always)]
			fn from(index: u64) -> Self {
				Self(index as usize)
			}
		}
		impl From<usize> for $type {
			#[inline(always)]
			fn from(index: usize) -> Self {
				Self(index)
			}
		}
		impl std::default::Default for $type {
			#[inline(always)]
			fn default() -> Self {
				Self::new()
			}
		}
		impl std::fmt::Display for $type {
			#[inline(always)]
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.0)
			}
		}
		impl PartialEq<$type> for usize {
			fn eq(&self, other: &$type) -> bool {
				*self == other.0
			}
		}
		impl PartialEq<usize> for $type {
			fn eq(&self, other: &usize) -> bool {
				self.0 == *other
			}
		}
	}
}

//---------------------------------------------------------------------------------------------------- Key
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
/// [`Key`] into the [`Collection`]
///
/// This represents an _absolute_ index into:
/// - a particular [`Song`] in
/// - a particular [`Album`] by
/// - a particular [`Artist`]
pub struct Key(ArtistKey, AlbumKey, SongKey);

impl Key {
	#[inline(always)]
	// Private function.
	pub(crate) const fn from_keys(
		artist: ArtistKey,
		album: AlbumKey,
		song: SongKey,
	) -> Self {
		Self(artist, album, song)
	}

	#[inline(always)]
	/// Returns [`Key`] with all inner keys set to `0`.
	pub const fn new() -> Self {
		Self(ArtistKey::new(), AlbumKey::new(), SongKey::new())
	}

	#[inline(always)]
	/// Returns the inner [`ArtistKey`]
	pub const fn artist(&self) -> ArtistKey {
		self.0
	}

	#[inline(always)]
	/// Returns the inner [`AlbumKey`]
	pub const fn album(&self) -> AlbumKey {
		self.1
	}

	#[inline(always)]
	/// Returns the inner [`SongKey`]
	pub const fn song(&self) -> SongKey {
		self.2
	}

	#[inline(always)]
	/// Returns the inner keys.
	pub const fn inner(&self) -> (ArtistKey, AlbumKey, SongKey) {
		(self.0, self.1, self.2)
	}

	#[inline(always)]
	/// Returns the inner usize's of the inner keys.
	pub const fn inner_usize(&self) -> (usize, usize, usize) {
		(self.0.inner(), self.1.inner(), self.2.inner())
	}
}

// Converts any tuple of 3 integers that can losslessly `.into()` a `u64`.
impl<A, B, C> From<(A, B, C)> for Key
where
	A: Into<usize>,
	B: Into<usize>,
	C: Into<usize>,
{
	#[inline]
	fn from(tuple: (A, B, C)) -> Self {
		Self(ArtistKey(tuple.0.into()), AlbumKey(tuple.1.into()), SongKey(tuple.2.into()))
	}
}

//---------------------------------------------------------------------------------------------------- Keychain
#[derive(Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
/// A separated collection of keys
///
/// These keys aren't linked like in [`Key`].
///
/// Each inner [`Vec`] in [`Keychain`] hold separate key types.
pub struct Keychain {
	/// [`Vec`] of [`ArtistKey`]'s.
	pub artists: Box<[ArtistKey]>,
	/// [`Vec`] of [`AlbumKey`]'s.
	pub albums: Box<[AlbumKey]>,
	/// [`Vec`] of [`SongKey`]'s.
	pub songs: Box<[SongKey]>,
}

impl Keychain {
	#[inline(always)]
	/// Returns [`Keychain`] with empty [`Box`]'s
	pub fn new() -> Self {
		Self { ..Default::default() }
	}

	#[inline(always)]
	/// Consumes [`Keychain`], returning the inner [`Box`]'s.
	pub fn into_vecs(self) -> (Box<[ArtistKey]>, Box<[AlbumKey]>, Box<[SongKey]>) {
		(self.artists, self.albums, self.songs)
	}

	#[inline(always)]
	/// Creates a [`Keychain`] from [`Box`]'s.
	pub fn from_boxes(
		artists: Box<[ArtistKey]>,
		albums: Box<[AlbumKey]>,
		songs: Box<[SongKey]>,
	) -> Self {
		Self { artists, albums, songs }
	}

	#[inline(always)]
	/// Creates a [`Keychain`] from [`Vec`]'s.
	pub fn from_vecs(
		artists: Vec<ArtistKey>,
		albums: Vec<AlbumKey>,
		songs: Vec<SongKey>,
	) -> Self {
		Self {
			artists: artists.into_boxed_slice(),
			albums: albums.into_boxed_slice(),
			songs: songs.into_boxed_slice(),
		}
	}

	#[inline(always)]
	/// Returns `true` if all inner [`Vec`]'s are empty.
	pub fn is_empty(&self) -> bool {
		self.artists.is_empty() && self.albums.is_empty() && self.songs.is_empty()
	}
}

//---------------------------------------------------------------------------------------------------- ArtistKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[serde(transparent)]
/// A key representing the index of an [`Artist`] in the [`Collection`]
///
/// The inner type is just a `usize`.
pub struct ArtistKey(usize);
impl_common!(ArtistKey);

//---------------------------------------------------------------------------------------------------- AlbumKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[serde(transparent)]
/// A key representing the index of an [`Album`] in the [`Collection`]
///
/// The inner type is just a `usize`.
pub struct AlbumKey(usize);
impl_common!(AlbumKey);

//---------------------------------------------------------------------------------------------------- SongKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[serde(transparent)]
/// A key representing the index of a [`Song`] in the [`Collection`]
///
/// The inner type is just a `usize`.
pub struct SongKey(usize);
impl_common!(SongKey);

//---------------------------------------------------------------------------------------------------- QueueKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[serde(transparent)]
/// A key representing an index in the `Queue`
///
/// This is used to index `Queue`, e.g:
///
/// 1. user clicks 'remove song #4 from queue'
/// 2. gui sends QueueKey(3) to helper
/// 3. kernel deletes queue\[3\]
///
/// This is just for type safety.
///
/// The inner type is just a [`usize`].
pub struct QueueKey(usize);
impl_common!(QueueKey);

//---------------------------------------------------------------------------------------------------- PlaylistKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[serde(transparent)]
/// A key representing an index in a `Playlist`
///
/// This is the same as [`QueueKey`] but for a `Playlist`.
///
/// The inner type is just a [`usize`].
pub struct PlaylistKey(usize);
impl_common!(PlaylistKey);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
