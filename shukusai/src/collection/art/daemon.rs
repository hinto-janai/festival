//---------------------------------------------------------------------------------------------------- Use
use once_cell::sync::Lazy;
use std::path::PathBuf;

//---------------------------------------------------------------------------------------------------- Unknown Art (lazy) Constant
/// The unknown [`Album`] art size in pixels: 500x500 pixels.
pub const ALBUM_ART_SIZE: usize = 500;

pub(crate) const UNKNOWN_ALBUM_BYTES: &[u8] = include_bytes!("../../../../assets/images/art/unknown.png");

//---------------------------------------------------------------------------------------------------- Art
#[derive(Default)]
/// An `enum` that is _always_ an image.
///
/// Some [`Album`]'s may not have art. In this case, we'd like to show a "unknown" image anyway.
pub enum Art {
	/// Art exists, and is stored at this PATH, not by the user, but by us.
	///
	/// (We saved a copy here, in `image/`).
	///
	/// This image is not resized at all, it is the
	/// full resolution extracted from the [`Song`]
	Known(PathBuf),
	#[default]
	/// A gray background, white question-mark image representing an unknown image.
	///
	/// This image's width/height is guaranteed to be [`ALBUM_ART_SIZE`].
	Unknown,
}

//---------------------------------------------------------------------------------------------------- Art `Ord`
impl Ord for Art {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match (self, other) {
			(Self::Unknown,  Self::Unknown)  => std::cmp::Ordering::Equal,
			(Self::Known(_), Self::Known(_)) => std::cmp::Ordering::Equal,
			(Self::Known(_), Self::Unknown)  => std::cmp::Ordering::Greater,
			(Self::Unknown,  Self::Known(_)) => std::cmp::Ordering::Less,
		}
	}
}
impl PartialOrd for Art {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

//---------------------------------------------------------------------------------------------------- Art `Eq`
impl Eq for Art {}
impl PartialEq for Art {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Unknown, Self::Unknown) => true,
			(Self::Known(_), Self::Known(_)) => true,
			_ => false,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Art Impl
impl Art {
	#[inline(always)]
	/// Returns [`Self::Unknown`].
	pub(crate) const fn new() -> Self {
		Self::Unknown
	}
}

impl std::fmt::Debug for Art {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Known(p) => write!(f, "Art::Known({:?})", p.display()),
			Self::Unknown  => write!(f, "Art::Unknown"),
		}
	}
}

//---------------------------------------------------------------------------------------------------- Art Clone
impl Clone for Art {
	fn clone(&self) -> Self {
		match self {
			Self::Bytes(vec) => Self::Bytes(vec.clone()),
			_ => Self::Unknown,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Art Bincode
impl bincode::Encode for Art {
	fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> std::result::Result<(), bincode::error::EncodeError> {
		match self {
			Self::Bytes(field_0) => {
				<u32 as bincode::Encode>::encode(&(1u32), encoder)?;
				bincode::Encode::encode(field_0, encoder)?;
				Ok(())
			},
			_ => {
				<u32 as bincode::Encode>::encode(&(2u32), encoder)?;
				Ok(())
			},
		}
	}
}
impl bincode::Decode for Art {
	fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> std::result::Result<Self, bincode::error::DecodeError> {
		let variant_index = <u32 as bincode::Decode>::decode(decoder)?;
		match variant_index {
			1u32 => {
				Ok(Self::Bytes(bincode::Decode::decode(decoder)?))
			},
			_ => Ok(Self::Unknown),
		}
	}
}
impl<'de> bincode::BorrowDecode<'de> for Art {
	fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(decoder: &mut D) -> std::result::Result<Self, bincode::error::DecodeError> {
		let variant_index = <u32 as bincode::Decode>::decode(decoder)?;
		match variant_index {
			1u32 => {
				Ok(Self::Bytes(bincode::BorrowDecode::borrow_decode(decoder)?))
			},
			_ => Ok(Self::Unknown),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	// Assert `UNKNOWN_ART` size is correct.
	fn unknown() {
		assert_eq!(UNKNOWN_ALBUM.size(), [ALBUM_ART_SIZE; 2]);
	}

	#[test]
	// Assert the `ALBUM_SIZE` casts are lossless.
	fn cast() {
		assert_eq!(ALBUM_ART_SIZE_U32 as usize, ALBUM_ART_SIZE);
		assert_eq!(ALBUM_ART_SIZE_U16 as usize, ALBUM_ART_SIZE);
	}
}
