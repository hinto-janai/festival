//---------------------------------------------------------------------------------------------------- Use
use anyhow::{bail};
use fast_image_resize as fir;
use fir::{
	Image,
	ResizeAlg,
	PixelType,
};
use std::num::NonZeroU32;
use crate::collection::{
	ALBUM_ART_SIZE,
	ALBUM_ART_SIZE_U32,
};
use benri::{
	debug_panic,
	sync::*,
};
use log::{trace};
use benri::log::fail;
use crate::frontend::egui::gui_context;

//---------------------------------------------------------------------------------------------------- Album Art Constants.
pub(crate) const ALBUM_ART_SIZE_NUM: NonZeroU32 = match NonZeroU32::new(ALBUM_ART_SIZE_U32) {
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
	// Attempt `zune-jpeg` first.
	if let Some((width, height, bytes)) = zune_decode(&bytes) {
		if let Ok(bytes) = resize_image(width, height, bytes, resizer) {
			return Ok(bytes);
		}
	}

	// Fallback to `image`.
	match image_decode(bytes) {
		Ok((w, h, bytes)) => resize_image(w, h, bytes, resizer),
		Err(e)            => Err(e),
	}
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
	// Fastest but pixels are noticeably jagged.
	fir::Resizer::new(ResizeAlg::Nearest)

	// Better quality when downscaling, same as `Nearest` when upscaling.
//	fir::Resizer::new(ResizeAlg::Convolution(FilterType::Box))

	// Sharper than `Box` when downscaling, bad when upscaling.
//	fir::Resizer::new(ResizeAlg::Convolution(FilterType::Hamming))

	// Slowest, supposedly best but I don't think it's noticeable.
//	fir::Resizer::new(ResizeAlg::Convolution(FilterType::Lanczos3))
}

#[inline(always)]
// Uses `zune` to decode solely `JPG`.
//
// INVARIANT:
// `zune` outputs some bad data sometimes (the sacrifices
// we make for performance) but running it through the
// resizer somehow... makes this data good again.
//
// If this isn't run through the resizer, `egui` will start panicking:
// ```
// message: Some(
//     assertion failed: `(left == right)`
//       left: `250000`,
//      right: `83333`: Mismatch between texture size and texel count,
// ),
// location: Location {
//     file: "external/egui/crates/egui-wgpu/src/renderer.rs",
//     line: 509,
//    col: 17,
// },
// ```
// so the output of this function _must_ go through
// `fast_image_resize` regardless if the `width/height` are the same.
fn zune_decode(bytes: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
	let options = zune_core::options::DecoderOptions::new_cmd()
		.jpeg_set_out_colorspace(zune_core::colorspace::ColorSpace::RGB);
	let mut decoder = zune_jpeg::JpegDecoder::new_with_options(options, &bytes);
	if let Ok(bytes) = decoder.decode() {
		if let Some(info) = decoder.info() {
			return Some((info.width as u32, info.height as u32, bytes));
		}
	}

	None
}

#[inline(always)]
// Uses `image` to decode everything else.
//
// This the fallback used when the above `zune` fails, or
// if the resizer failed on `zune`'s bytes.
fn image_decode(bytes: Box<[u8]>) -> Result<(u32, u32, Vec<u8>), anyhow::Error> {
	match image::load_from_memory(&bytes) {
		Ok(img) => Ok((img.width(), img.height(), img.into_rgb8().into_raw())),
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
fn resize_image(width: u32, height: u32, bytes: Vec<u8>, resizer: &mut fir::Resizer) -> Result<Box<[u8]>, anyhow::Error> {
	// Make sure the image width/height is not 0.
	debug_assert!(width != 0);
	debug_assert!(height != 0);

	let width = match NonZeroU32::new(width) {
		Some(w) => w,
		None    => bail!("Album art width was 0"),
	};
	let height = match NonZeroU32::new(height) {
		Some(w) => w,
		None    => bail!("Album art height was 0"),
	};

	// Convert image to RGB, then into a `fir::Image`.
	let old_img = Image::from_vec_u8(width, height, bytes, PixelType::U8x3)?;

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
// TODO: Add test for zune.
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	// Assert we can:
	// 1. Take in all image formats
	// 2. Resize with `fir`
	// 3. Convert to an `egui` image
	fn __art_from_known() {
		let mut resizer = super::create_resizer();

		for ext in ["jpg", "png", "bmp", "ico", "tiff", "webp"] {
			let img = std::fs::read(format!("../assets/images/test/512.{ext}")).unwrap();
			let img = art_from_raw(img.into(), &mut resizer).unwrap();
			assert!(!img.is_empty());

			// Bytes of FIR Image should be in perfect `3` chunks (RGB).
			assert_eq!(img.len() % 3, 0);

			// Convert to `egui` image.
			let retained = art_from_known(img);
			assert_eq!(retained.width(), ALBUM_ART_SIZE);
			assert_eq!(retained.height(), ALBUM_ART_SIZE);
		}
	}
}
