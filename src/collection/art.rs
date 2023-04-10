//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use egui_extras::image::RetainedImage;
use super::Album;

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
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
