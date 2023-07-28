//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use once_cell::sync::Lazy;
use std::path::{Path,PathBuf};

//---------------------------------------------------------------------------------------------------- Unknown Art (lazy) Constant
/// The unknown [`Album`] art size in pixels: 500x500 pixels.
pub const ALBUM_ART_SIZE: usize = 500;

pub(crate) const UNKNOWN_ALBUM_BYTES: &[u8] = include_bytes!("../../../../assets/images/art/unknown.png");

//---------------------------------------------------------------------------------------------------- Art
#[derive(Clone,Default,Debug,PartialEq,Eq,PartialOrd,Ord,Encode,Decode)]
/// An `enum` that is _always_ an image.
///
/// Some [`Album`]'s may not have art. In this case, we'd like to show a "unknown" image anyway.
///
/// This type is specifically for `festivald`, where the `Collection`
/// doesn't hold bytes, but the full resolution images are saved in `image/`.
///
/// Just like how `Bytes` should never exist in `GUI` after `Collection` creation,
/// `festivald` should _always_ either have a `Art::Known` or `Art::Unknown`.
pub enum Art {
	/// Art exists, and is stored at this PATH, not by the user, but by us.
	///
	/// (We saved a copy here, in `image/`).
	///
	/// This image is not resized at all, it is the
	/// full resolution extracted from the [`Song`]
	Known {
		/// Path to image
		path: PathBuf,
		/// Mime type
		mime: String,
		/// File extension
		extension: String,
		/// Byte length of image
		len: usize,
	},
	/// This is raw image bytes that have not yet been transformed into [`Art::Known`].
	///
	/// This variant is never exposed to a `Frontend`, as `Kernel` turns all [`Art`]
	/// into either [`Art::Known`] or [`Art::Unknown`].
	Bytes(Vec<u8>),
	#[default]
	/// A gray background, white question-mark image representing an unknown image.
	///
	/// This image's width/height is guaranteed to be [`ALBUM_ART_SIZE`].
	Unknown,
}

//---------------------------------------------------------------------------------------------------- Art Impl
impl Art {
	#[inline(always)]
	/// Returns [`Self::Unknown`].
	pub(crate) const fn new() -> Self {
		Self::Unknown
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	use super::*;
//}
