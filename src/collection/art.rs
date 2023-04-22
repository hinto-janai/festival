//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use egui::TextureId;
use egui_extras::image::RetainedImage;
use super::Album;

//---------------------------------------------------------------------------------------------------- Constant
/// The [`Album`] art size in pixels
///
/// 600x600 pixels.
///
/// Album art will _always_ be resized internally to this size.
pub const ALBUM_ART_SIZE: u16 = 600;

//---------------------------------------------------------------------------------------------------- Unknown Art (lazy) Constant
lazy_static::lazy_static! {
	pub(crate) static ref UNKNOWN_ALBUM: RetainedImage = RetainedImage::from_image_bytes("Unknown", include_bytes!("../../assets/images/art/unknown.png")).unwrap();
	pub(crate) static ref UNKNOWN_ALBUM_BYTES: &'static [u8] = include_bytes!("../../assets/images/art/unknown.png");
}

//---------------------------------------------------------------------------------------------------- Art
#[derive(Default)]
// An `enum` that is _always_ an image.
//
// Some [`Album`]'s may not have art. In this case, we'd like to show a "unknown" image anyway.
//
// This `enum` and the associate function [`Art::art_or()`] will always return
// a valid [`egui_extras::RetainedImage`], the real art if it exists, or an "unknown" image.
//
// The returned "unknown" image is actually just a pointer to the single image created with [`lazy_static`].
//
// The "unknown" image is from `assets/images/art/unknown.png`.
pub(crate) enum Art {
	Known(RetainedImage),
	#[default]
	Unknown,
}

//---------------------------------------------------------------------------------------------------- Art Serde
//use serde::{Serialize,Deserialize,Serializer,Deserializer};
//
//impl Serialize for Art {
//	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//		where
//		S: Serializer,
//	{
//		match self {
//			Self::Known(img) => {
//				match &img.texture.lock().as_ref().unwrap().tex_mngr.read().delta.set[0].1.image {
//					epaint::image::ImageData::Color(c) => {
//						use serde::ser::{Serialize, Serializer, SerializeSeq};
//						let mut seq = serializer.serialize_seq(Some(c.pixels.len())).unwrap();
//						for p in &c.pixels {
//							seq.serialize_element(&p.to_array());
//						}
//						seq.end()
//					},
//					_ => panic!(),
//				}
//			},
//			Self::Unknown => serializer.serialize_unit_variant("Art", 1, "Unknown"),
//		}
//	}
//}

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
	/// Return the associated art or the default `[?]` image if [`Art::Unknown`]
	pub(crate) fn art_or(&self) -> &RetainedImage {
		match self {
			Self::Known(art) => art,
			Self::Unknown    => &UNKNOWN_ALBUM,
		}
	}

	#[inline]
	/// Same as [`Art::art_or`] but with no backup image.
	pub(crate) fn get(&self) -> Option<&RetainedImage> {
		match self {
			Self::Known(art) => Some(art),
			Self::Unknown    => None,
		}
	}

	#[inline]
	/// Calls [`egui::extras::texture_id`].
	pub(crate) fn texture_id(&self, ctx: &egui::Context) -> egui::TextureId {
		match self {
			Self::Known(a) => a.texture_id(ctx),
			// TODO: `lazy_static` this id, no need to lock
			Self::Unknown  => UNKNOWN_ALBUM.texture_id(ctx),
		}
	}
}

impl std::fmt::Debug for Art {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Known(_) => write!(f, "Art::Known(RetainedImage)"),
			Self::Unknown  => write!(f, "Art::Unknown"),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	#[test]
	fn unknown_art() {
		// Make sure the `.unwrap()` doesn't panic.
		assert!(*UNKNOWN_ALBUM == *UNKNOWN_ALBUM);
	}
}
