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

//---------------------------------------------------------------------------------------------------- Unknown Art (lazy) Constant
lazy_static::lazy_static! {
	pub(crate) static ref UNKNOWN_ALBUM: RetainedImage = RetainedImage::from_image_bytes("Unknown", include_bytes!("../../assets/images/art/unknown.png")).unwrap();
	pub(crate) static ref UNKNOWN_ALBUM_BYTES: &'static [u8] = include_bytes!("../../assets/images/art/unknown.png");
}

//---------------------------------------------------------------------------------------------------- Art
#[derive(Default)]
pub enum Art {
	Known(RetainedImage),
	#[default]
	Unknown,
}

impl Art {
	#[inline(always)]
	pub(crate) fn new() -> Self {
		Self::default()
	}
}

impl Art {
	#[inline]
	// Return the associated art or the default `[?]` image if `Unknown`
	pub fn art_or(&self) -> &RetainedImage {
		match &self {
			Self::Known(art) => art,
			Self::Unknown    => &*UNKNOWN_ALBUM,
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
