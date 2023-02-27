//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};

#[cfg(not(target_pointer_width = "64"))]
compile_error!("Festival is only built for 64-bit systems.");

//---------------------------------------------------------------------------------------------------- Macros to implement common traits.
macro_rules! impl_common {
	($type:ty) => {
		impl $type {
			#[inline(always)]
			pub const fn new() -> Self {
				Self(0)
			}
			#[inline(always)]
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
		impl From<u64> for $type {
			#[inline(always)]
			fn from(index: u64) -> Self {
				Self(index as usize)
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
	}
}

macro_rules! impl_from_tuple {
	($one:ty, $two:ty, $three:ty) => {
		impl From<($one, $two, $three)> for CollectionKey {
			#[inline(always)]
			fn from(tuple: ($one, $two, $three)) -> Self {
				Self {
					artist: ArtistKey(tuple.0 as usize),
					album: AlbumKey(tuple.1 as usize),
					song: SongKey(tuple.2 as usize),
				}
			}
		}
	}
}

//---------------------------------------------------------------------------------------------------- CollectionKey
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct CollectionKey {
	pub artist: ArtistKey,
	pub album: AlbumKey,
	pub song: SongKey,
}

impl CollectionKey {
	#[inline(always)]
	pub const fn new() -> Self {
		Self {
			artist: ArtistKey::new(),
			album: AlbumKey::new(),
			song: SongKey::new(),
		}
	}

	#[inline(always)]
	pub const fn to_tuple(&self) -> (usize, usize, usize) {
		(self.artist.inner(), self.album.inner(), self.song.inner())
	}
}

// Converts any tuple of 3 integers that can losslessly `.into()` a `u64`.
//
// Since the target will (probably...) always be `x86_64`,
// the cast from `u64` to `usize` is (probably...) always safe.
impl<A, B, C> From<(A, B, C)> for CollectionKey
where
	A: Into<u64>,
	B: Into<u64>,
	C: Into<u64>,
{
	#[inline(always)]
	fn from(tuple: (A, B, C)) -> Self {
		Self {
			artist: ArtistKey(tuple.0.into() as usize),
			album: AlbumKey(tuple.1.into() as usize),
			song: SongKey(tuple.2.into() as usize),
		}
	}
}

//---------------------------------------------------------------------------------------------------- ArtistKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct ArtistKey(usize);

impl_common!(ArtistKey);

//---------------------------------------------------------------------------------------------------- AlbumKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct AlbumKey(usize);

impl_common!(AlbumKey);

//---------------------------------------------------------------------------------------------------- SongKey
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct SongKey(usize);

impl_common!(SongKey);

//---------------------------------------------------------------------------------------------------- QueueKey
// Used to index `Queue` which is just a `Vec<CollectionKey>`, e.g:
// ```
// 1. user clicks 'remove song #4 from queue'
// 2. gui sends QueueKey(3) to helper
// 3. helper deletes queue[3]
// ```
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct QueueKey(usize);

impl_common!(QueueKey);

//---------------------------------------------------------------------------------------------------- PlaylistKey
// Same as `QueueKey` but for `Playlist`.
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct PlaylistKey(usize);

impl_common!(PlaylistKey);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
