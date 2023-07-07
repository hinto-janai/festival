//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};

//---------------------------------------------------------------------------------------------------- Macros to implement common traits.
macro_rules! impl_common {
	($type:ty) => {
		impl $type {
			#[inline(always)]
			/// Returns `Self(0)`.
			pub(crate) const fn new() -> Self {
				Self(0)
			}
			#[inline(always)]
			/// Returns `Self(0)`.
			pub(crate) const fn zero() -> Self {
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
		impl From<&$type> for $type {
			#[inline(always)]
			fn from(key: &$type) -> Self {
				*key
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
		impl PartialEq<$type> for &$type {
			fn eq(&self, other: &$type) -> bool {
				self.0 == other.0
			}
		}
		impl PartialEq<&$type> for $type {
			fn eq(&self, other: &&$type) -> bool {
				self.0 == other.0
			}
		}
	}
}

//---------------------------------------------------------------------------------------------------- ArtistKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[repr(transparent)]
#[serde(transparent)]
/// A key representing the index of an [`Artist`] in the [`Collection`]
///
/// The inner type is just a `usize`.
pub struct ArtistKey(usize);
impl_common!(ArtistKey);

//---------------------------------------------------------------------------------------------------- AlbumKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[repr(transparent)]
#[serde(transparent)]
/// A key representing the index of an [`Album`] in the [`Collection`]
///
/// The inner type is just a `usize`.
pub struct AlbumKey(usize);
impl_common!(AlbumKey);

//---------------------------------------------------------------------------------------------------- SongKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[repr(transparent)]
#[serde(transparent)]
/// A key representing the index of a [`Song`] in the [`Collection`]
///
/// The inner type is just a `usize`.
pub struct SongKey(usize);
impl_common!(SongKey);

////---------------------------------------------------------------------------------------------------- QueueKey
//#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
//#[repr(transparent)]
//#[serde(transparent)]
///// A key representing an index in the `Queue`
/////
///// This is used to index `Queue`, e.g:
/////
///// 1. user clicks 'remove song #4 from queue'
///// 2. gui sends QueueKey(3) to helper
///// 3. kernel deletes queue\[3\]
/////
///// This is just for type safety.
/////
///// The inner type is just a [`usize`].
//pub struct QueueKey(usize);
//impl_common!(QueueKey);
//
////---------------------------------------------------------------------------------------------------- PlaylistKey
//#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
//#[repr(transparent)]
//#[serde(transparent)]
///// A key representing an index in a `Playlist`
/////
///// This is the same as [`QueueKey`] but for a `Playlist`.
/////
///// The inner type is just a [`usize`].
//pub struct PlaylistKey(usize);
//impl_common!(PlaylistKey);

//---------------------------------------------------------------------------------------------------- Key
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
// INVARIANT:
// User's should never be able to construct this.
//
// This is an _opaque_ index into the `Collection`.
/// [`Key`] into the [`Collection`]
///
/// This represents an _absolute_ index into:
/// - a particular [`Song`] in
/// - a particular [`Album`] by
/// - a particular [`Artist`]
pub struct Key {
	artist: ArtistKey,
	album: AlbumKey,
	song: SongKey
}

impl Key {
	#[inline(always)]
	// Create a new [`Key`] from existing keys.
	pub(crate) const fn from_keys(
		artist: ArtistKey,
		album: AlbumKey,
		song: SongKey,
	) -> Self {
		Self { artist, album, song }
	}

	#[inline(always)]
	// Create a [`Key`] from raw [`usize`]'s.
	pub(crate) const fn from_raw(
		artist: usize,
		album: usize,
		song: usize,
	) -> Self {
		Self {
			artist: ArtistKey(artist),
			album: AlbumKey(album),
			song: SongKey(song),
		}
	}

	#[inline(always)]
	/// Returns [`Key`] with all inner keys set to `0`.
	pub const fn zero() -> Self {
		Self { artist: ArtistKey::zero(), album: AlbumKey::zero(), song: SongKey::zero() }
	}

	#[inline(always)]
	/// Returns the inner [`ArtistKey`]
	pub const fn artist(&self) -> ArtistKey {
		self.artist
	}

	#[inline(always)]
	/// Returns the inner [`AlbumKey`]
	pub const fn album(&self) -> AlbumKey {
		self.album
	}

	#[inline(always)]
	/// Returns the inner [`SongKey`]
	pub const fn song(&self) -> SongKey {
		self.song
	}

	#[inline(always)]
	/// Returns the inner keys.
	pub const fn inner(&self) -> (ArtistKey, AlbumKey, SongKey) {
		(self.artist, self.album, self.song)
	}

	#[inline(always)]
	/// Returns the inner keys, consuming [`Self`]
	pub const fn into_inner(self) -> (ArtistKey, AlbumKey, SongKey) {
		(self.artist, self.album, self.song)
	}

	#[inline(always)]
	/// Returns the inner usize's of the inner keys.
	pub const fn usize(&self) -> (usize, usize, usize) {
		(self.artist.0, self.album.0, self.song.0)
	}

	#[inline(always)]
	/// Returns the inner usize's of the inner keys, consuming [`Self`].
	pub const fn into_usize(&self) -> (usize, usize, usize) {
		(self.artist.0, self.album.0, self.song.0)
	}
}

//---------------------------------------------------------------------------------------------------- Keychain
#[derive(Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
/// A separated collection of keys
///
/// These keys aren't linked like in [`Key`].
///
/// Each inner [`Box`] in [`Keychain`] hold separate key types.
pub struct Keychain {
	/// [`Box`] of [`ArtistKey`]'s.
	pub artists: Box<[ArtistKey]>,
	/// [`Box`] of [`AlbumKey`]'s.
	pub albums: Box<[AlbumKey]>,
	/// [`Box`] of [`SongKey`]'s.
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
	pub fn into_boxes(self) -> (Box<[ArtistKey]>, Box<[AlbumKey]>, Box<[SongKey]>) {
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

//	#[inline(always)]
//	// Creates a [`Keychain`] from raw [`Box<usize>`]'s.
//	pub(crate) const unsafe fn from_boxes_raw(
//		artists: Box<[usize]>,
//		albums: Box<[usize]>,
//		songs: Box<[usize]>,
//	) -> Self {
//		// SAFETY: The `Key` types _must_ be `#[repr(transparent)]`
//		unsafe {
//			Self {
//				artists: std::mem::transmute::<Box<[usize]>, Box<[ArtistKey]>>(artists),
//				albums: std::mem::transmute::<Box<[usize]>, Box<[AlbumKey]>>(albums),
//				songs: std::mem::transmute::<Box<[usize]>, Box<[SongKey]>>(songs),
//			}
//		}
//	}

	#[inline(always)]
	/// Returns `true` if all inner [`Box`]'s are empty.
	pub fn is_empty(&self) -> bool {
		self.artists.is_empty() && self.albums.is_empty() && self.songs.is_empty()
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	// Asserts `*::new()`, and `Default::default()` are the same as `*::zero()`.
	// Multiple places in the codebase rely on this behavior.
	fn default_is_new_is_zero() {
		assert_eq!(Keychain::default(), Keychain::new());
		assert_eq!(ArtistKey::default(), ArtistKey::new());
		assert_eq!(AlbumKey::default(), AlbumKey::new());
		assert_eq!(SongKey::default(), SongKey::new());

		assert_eq!(ArtistKey::new(), ArtistKey::zero());
		assert_eq!(AlbumKey::new(), AlbumKey::zero());
		assert_eq!(SongKey::new(), SongKey::zero());
	}

	#[test]
	// Asserts all `from()` functions result in the same output.
	fn from() {
		assert_eq!(ArtistKey::from(0_u8), ArtistKey::zero());
		assert_eq!(ArtistKey::from(0_u16), ArtistKey::zero());
		assert_eq!(ArtistKey::from(0_u32), ArtistKey::zero());
		assert_eq!(ArtistKey::from(0_u64), ArtistKey::zero());
		assert_eq!(ArtistKey::from(0_usize), ArtistKey::zero());

		assert_eq!(AlbumKey::from(0_u8), AlbumKey::zero());
		assert_eq!(AlbumKey::from(0_u16), AlbumKey::zero());
		assert_eq!(AlbumKey::from(0_u32), AlbumKey::zero());
		assert_eq!(AlbumKey::from(0_u64), AlbumKey::zero());
		assert_eq!(AlbumKey::from(0_usize), AlbumKey::zero());

		assert_eq!(SongKey::from(0_u8), SongKey::zero());
		assert_eq!(SongKey::from(0_u16), SongKey::zero());
		assert_eq!(SongKey::from(0_u32), SongKey::zero());
		assert_eq!(SongKey::from(0_u64), SongKey::zero());
		assert_eq!(SongKey::from(0_usize), SongKey::zero());
	}

	#[test]
	// Asserts all comparison impls are correct.
	fn cmp() {
		for i in 0..=10_usize {
			assert_eq!(i, ArtistKey::from(i));
			assert_eq!(i, ArtistKey::from(i).inner());
			assert_eq!(i, AlbumKey::from(i));
			assert_eq!(i, AlbumKey::from(i).inner());
			assert_eq!(i, SongKey::from(i));
			assert_eq!(i, SongKey::from(i).inner());
		}
	}

	#[test]
	// Asserts `.inner()` is correct.
	fn inner() {
		for i in 0..=10_usize {
			let (a, b, c) = (ArtistKey::from(i), AlbumKey::from(i), SongKey::from(i));
			assert_eq!(i, a.inner());
			assert_eq!(a.inner(), b.inner());
			assert_eq!(a.inner(), c.inner());
			assert_eq!(b.inner(), c.inner());
		}
	}
}
