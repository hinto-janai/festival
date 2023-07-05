//---------------------------------------------------------------------------------------------------- Use
use egui_extras::image::RetainedImage;
use once_cell::sync::Lazy;

//---------------------------------------------------------------------------------------------------- Constant
/// The [`Album`] art size in pixels
///
/// 500x500 pixels.
///
/// Album art will _always_ be resized internally to this size.
pub const ALBUM_ART_SIZE: usize = 500;

//---------------------------------------------------------------------------------------------------- Unknown Art (lazy) Constant
pub(crate) const UNKNOWN_ALBUM_BYTES: &[u8] = include_bytes!("../../../assets/images/art/unknown.png");
pub(crate) static UNKNOWN_ALBUM: Lazy<RetainedImage> = Lazy::new(|| RetainedImage::from_image_bytes("Unknown", UNKNOWN_ALBUM_BYTES).unwrap());

// INVARIANT:
// `egui` uses ID 0 for its own textures.
//
// `Kernel` _must_ initialize `UNKNOWN_ALBUM` before
// anything other texture so that this ID is correct.
pub(crate) const UNKNOWN_ALBUM_ID: egui::TextureId = egui::TextureId::Managed(1);

//---------------------------------------------------------------------------------------------------- Art
#[derive(Default)]
/// An `enum` that is _always_ an image.
///
/// Some [`Album`]'s may not have art. In this case, we'd like to show a "unknown" image anyway.
///
/// This `enum` and the associated function [`Album::art_or()`] will always return
/// a valid [`egui_extras::RetainedImage`], the real art if it exists, or an "unknown" image.
///
/// The returned "unknown" image is actually just a pointer to a single image.
///
/// The "unknown" image is from `assets/images/art/unknown.png`.
pub enum Art {
	/// This is a known-good, already resized [`RetainedImage`] that
	/// can be used in `egui`.
	///
	/// This image's width/height is guaranteed to be [`ALBUM_ART_SIZE`].
	Known(RetainedImage),
	/// This is raw image bytes that have not yet been transformed into [`Art::Known`].
	///
	/// This variant is never exposed to a `Frontend`, as `Kernel` turns all [`Art`]
	/// into either [`Art::Known`] or [`Art::Unknown`].
	Bytes(Box<[u8]>),
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
			(Self::Unknown, Self::Unknown) => std::cmp::Ordering::Equal,
			(Self::Bytes(_), Self::Bytes(_)) => std::cmp::Ordering::Equal,
			(Self::Known(_), Self::Known(_)) => std::cmp::Ordering::Equal,
			(Self::Known(_), _) => std::cmp::Ordering::Greater,
			(_, Self::Known(_)) => std::cmp::Ordering::Less,
			(Self::Bytes(_), _) => std::cmp::Ordering::Greater,
			(_, Self::Bytes(_)) => std::cmp::Ordering::Less,
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
			(Self::Bytes(b1), Self::Bytes(b2)) => b1 == b2,
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

impl Art {
	#[inline]
	// Return the associated art or the default `[?]` image if [`Art::Unknown`]
	pub(crate) fn art_or(&self) -> &RetainedImage {
		match self {
			Self::Known(art) => art,
			_ => &UNKNOWN_ALBUM,
		}
	}

	#[inline]
	// Same as [`Art::art_or`] but with no backup image.
	pub(crate) fn get(&self) -> Option<&RetainedImage> {
		match self {
			Self::Known(art) => Some(art),
			_ => None,
		}
	}

	#[inline]
	// Calls [`egui::extras::texture_id`].
	pub(crate) fn texture_id(&self, ctx: &egui::Context) -> egui::TextureId {
		match self {
			Self::Known(a) => a.texture_id(ctx),
			_ => UNKNOWN_ALBUM_ID,
		}
	}
}

impl std::fmt::Debug for Art {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Known(_) => write!(f, "Art::Known(RetainedImage)"),
			Self::Bytes(b) => write!(f, "Art::Bytes({})", b.len()),
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
// Same thing as above, but for `bincode`'s `Encode` & `Decode`
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
//#[cfg(test)]
//mod tests {
//	use super::*;
//
//	#[test]
//	fn unknown_art() {
//		// Make sure the `.unwrap()` doesn't panic.
//		assert!(UNKNOWN_ALBUM.size() == [ALBUM_ART_SIZE; 2]);
//	}
//}
