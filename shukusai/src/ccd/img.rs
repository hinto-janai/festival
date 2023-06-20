//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use image::codecs::png::PngEncoder;
use image::codecs::jpeg::JpegEncoder;
use std::io::BufWriter;
use image::ImageEncoder;
use image::ColorType;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use fast_image_resize as fir;
use fir::{
	Image,
	Resizer,
	ResizeAlg,
	FilterType,
	PixelType,
};
use std::num::NonZeroU32;
use std::sync::Arc;
use crate::collection::ALBUM_ART_SIZE;
use benri::{
	debug_panic,
	sync::*,
	time::*,
};
use log::{warn,trace,error};
use benri::log::fail;
use crate::frontend::egui::gui_context;

//---------------------------------------------------------------------------------------------------- Album Art Constants.
pub(crate) const ALBUM_ART_SIZE_NUM: NonZeroU32 = match NonZeroU32::new(ALBUM_ART_SIZE as u32) {
	Some(n) => n,
	None    => panic!(),
};

//---------------------------------------------------------------------------------------------------- Image Manipulation Functions.
// Image pipeline, from raw/unedited bytes to an actually displayable `egui::RetainedImage`:
// ```
// echo $RAW_BYTES                \
//     | bytes_to_dyn_image()     \
//     | resize_dyn_image()       \
//     | rgb_bytes_to_color_img() \
//     | color_img_to_retained()  \
//     | retained_load_texture()
// ```
//
// Image pipeline, from known good edited bytes to `egui::RetainedImage`:
// ```
// echo $KNOWN_BYTES              \
//     | rgb_bytes_to_color_img() \
//     | color_img_to_retained()  \
//     | retained_load_texture()
// ```
// All these functions take input of the previous function
// and their output goes straight into the next.
//
// Pipe syntax is for easy reading.
// Ignore that they have arguments and side effects.

//-------------------------- CONVENIENCE WRAPPER FUNCTIONS
// These 2 just apply the pipeline above.
// The real functions are below.

// Input: abritary image bytes.
// Output: `500x500` RGB image bytes.
#[inline(always)]
pub(crate) fn art_from_raw(bytes: Box<[u8]>, resizer: &mut fir::Resizer) -> Result<Box<[u8]>, anyhow::Error> {
	// `.buffer()` must be called on `fir::Image`
	// before passing it to the next function.
	// It's cheap, it just returns a `&[u8]`.
	resize_dyn_image(bytes_to_dyn_image(bytes)?, resizer)
}

#[inline(always)]
pub(crate) fn art_raw_to_egui(bytes: Box<[u8]>) -> egui_extras::RetainedImage {
	color_img_to_retained(rgb_bytes_to_color_img(bytes))
}

#[inline(always)]
pub(crate) fn art_from_known(bytes: Box<[u8]>) -> egui_extras::RetainedImage {
	color_img_to_retained(
		rgb_bytes_to_color_img(bytes)
	)
}

//-------------------------- Real functions.
#[inline(always)]
pub(crate) fn create_resizer() -> fir::Resizer {
	// Fastest but pixels are noticably jagged.
//	fir::Resizer::new(ResizeAlg::Nearest)

	// Better quality when downscaling, same as `Nearest` when upscaling.
	fir::Resizer::new(ResizeAlg::Convolution(FilterType::Box))

	// Sharper than `Box` when downscaling, bad when upscaling.
//	fir::Resizer::new(ResizeAlg::Convolution(FilterType::Hamming))

	// Slowest, supposedly best but I don't think it's noticable.
//	fir::Resizer::new(ResizeAlg::Convolution(FilterType::Lanczos3))
}

#[inline(always)]
// FIXME:
// This function is really slow.
// The image probably doesn't need to be dynamic.
// We should only expect RGB/RGBA images.
//
// This is the `heaviest` function within the entire `new_collection()` function.
// It accounts for around 70% of the total time spent making the `Collection`.
fn bytes_to_dyn_image(bytes: Box<[u8]>) -> Result<image::DynamicImage, anyhow::Error> {
	match image::load_from_memory(&bytes) {
		Ok(img) => Ok(img),
		Err(e)  => {
			use image::error::ImageError::*;
			match e {
				Decoding(e)    => { debug_panic!("{e}"); bail!(e); },
				Encoding(e)    => { debug_panic!("{e}"); bail!(e); },
				Parameter(e)   => { debug_panic!("{e}"); bail!(e); },
				Limits(e)      => { debug_panic!("{e}"); bail!(e); },
				IoError(e)     => { debug_panic!("{e}"); bail!(e); },
				Unsupported(e) => { bail!(e); },
			}
		},
	}
}

#[inline(always)]
// The bool returned represents:
fn resize_dyn_image(img: image::DynamicImage, resizer: &mut fir::Resizer) -> Result<Box<[u8]>, anyhow::Error> {
	// Make sure the image width/height is not 0.
	debug_assert!(img.width() != 0);
	debug_assert!(img.height() != 0);

	let width = match NonZeroU32::new(img.width()) {
		Some(w) => w,
		None    => bail!("Album art width was 0"),
	};
	let height = match NonZeroU32::new(img.height()) {
		Some(w) => w,
		None    => bail!("Album art height was 0"),
	};

	// Convert image to RGB, then into a `fir::Image`.
	let old_img = Image::from_vec_u8(width, height, img.into_rgb8().into_raw(), PixelType::U8x3)?;


	// Create the image we'll resize into.
	let mut new_img = Image::new(ALBUM_ART_SIZE_NUM, ALBUM_ART_SIZE_NUM, PixelType::U8x3);

	// Get image view.
	// Images might not always be perfect squares.
	// This sets the resizer to crop a square out of
	// the middle instead squashing the aspect ratio.
	let mut old = old_img.view();
	old.set_crop_box_to_fit_dst_size(ALBUM_ART_SIZE_NUM, ALBUM_ART_SIZE_NUM, Some((0.5, 0.5)));

	// Resize old into new.
	if let Err(e) = resizer.resize(&old, &mut new_img.view_mut()) {
		fail!("CCD - Failed to resize art: {e}");
		bail!(e);
	}

	Ok(new_img.into_vec().into_boxed_slice())
}

#[inline(always)]
// INVARIANT:
// Input to this function _must_ be bytes that are perfectly
// separated into `3` chunks, as in `[R,G,B ... R,G,B]`.
//
// The image size must also be `500x500` or this will cause `egui` to `panic!()`.
//
// Original `egui` function has an `assert!()`.
fn rgb_bytes_to_color_img(bytes: Box<[u8]>) -> egui::ColorImage {
	debug_assert!(bytes.len() % 3 == 0);

	egui::ColorImage {
		size: [ALBUM_ART_SIZE; 2],
		pixels: bytes.chunks_exact(3).map(|p| egui::Color32::from_rgb(p[0], p[1], p[2])).collect(),
	}
}

#[inline(always)]
fn color_img_to_retained(img: egui::ColorImage) -> egui_extras::RetainedImage {
	egui_extras::RetainedImage::from_color_image("", img)
}

// The image must be turned into a `texture` before
// it can properly be painted in `egui`.
//
// This `image` -> `texture` process is done
// lazily and only occurs when the `GUI` with a `Context`
// actually needs to load the image, aka:
//
// The GUI stutters a bit when loading the
// album art of a new `Collection`.
// This is not acceptable.
//
// To prevent this, we'll load the textures here so that
// `CCD` itself can load these images instead of `GUI`.
//
// FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME
// `CCD` locking `Context` to insert the textures actually freezes
// the `GUI`, so we sleep every loop so that we don't starve reads.
//
// This is so bad.
// There has to be a better way.
// FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME FIXME
//
// Update 2023-06-14:
// Switching `egui`'s internal lock to `std` instead of `parking_lot`
// makes it _alot_ better. There is still a tiny freeze but it's fine
// for now, we won't show the spinner near the end.
pub(super) fn alloc_textures(albums: &crate::collection::Albums) {
	// Get `Context`.
	let ctx = gui_context();

	// Wait until `GUI` has loaded at least 1 frame.
	while !atomic_load!(crate::frontend::egui::GUI_UPDATING) {
		std::hint::spin_loop();
	}

	// For each `Album`...
	for album in albums.iter() {
		// Continue only if this is a real `Art`.
		if let crate::collection::Art::Known(art) = &album.art {
			// INVARIANT:
			// As of `egui_extras 0.21.0`, this function makes sure
			// the inner image is allocated before returning the id.
			//
			// This behavior must exist for this to actually allocate the image.
			_ = art.texture_id(ctx);
		}
	}
}

// Since we are manually allocated textures, we must also free them.
// `epaint` internally increments (but never decrements) a counter
// when allocating a texture. This counter is used as an "id" for the
// texture, which can later be used to free it.
//
// INVARIANT: This must only be mutated in the below function, in a single thread.
static mut NEXT_TEXTURE_ID: u64 = 2;
// The above counter is our local version so that we
// know which counter (texture id) we are at.
//
// `egui` itself loads fonts and its texture data into the first
// slot (`0`), so our textures that we allocate will always start at 1.
// We must also _never_ free `0`, or `GUI` will turn into a black screen.
//
// We also internally use `1` for `UNKNOWN_IMAGE`.
pub(super) fn free_textures(tex_manager: &mut epaint::TextureManager) {
	// Increment our local number.
	let current_texture_count = tex_manager.num_allocated() as u64;

	// SAFETY: see above comment.
	let (start, end) = unsafe {
		let start = NEXT_TEXTURE_ID;
		let end   = NEXT_TEXTURE_ID + current_texture_count;
		NEXT_TEXTURE_ID += current_texture_count;
		(start, end)
	};

	// Free them.
	trace!("CCD - current_texture_count: {current_texture_count}, freeing: {start}..{end}");
	for id in start..end {
		tex_manager.free(epaint::TextureId::Managed(id));
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	const IMG_BYTES: &[u8] = include_bytes!("../../assets/images/icon/1024.png");

	#[test]
	// Makes sure we can take in random image bytes,
	// resize with `fir`, and transform into an `egui` image.
	fn _art_from_raw() {
		let mut resizer = super::create_resizer();
		art_from_raw(IMG_BYTES, &mut resizer).unwrap();
	}

	#[test]
	// Make sure known image bytes can be converted to an `egui` image.
	fn _art_from_known() {
		// Resizer.
		let mut resizer = super::create_resizer();

		// Bytes -> DynamicImage.
		let dyn_img = bytes_to_dyn_image(IMG_BYTES).unwrap();

		// DynamicImage -> FIR Image.
		let fir_img = resize_dyn_image(dyn_img, &mut resizer).unwrap();

		// Bytes of FIR Image should be in perfect `3` chunks (RGB).
		assert!(fir_img.len() % 3 == 0);

		let retained = art_from_known(&fir_img);
		assert!(retained.width() == 500);
		assert!(retained.height() == 500);
	}
}
