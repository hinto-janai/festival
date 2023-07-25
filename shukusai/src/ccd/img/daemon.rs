//---------------------------------------------------------------------------------------------------- Use
use benri::ok_trace;
use log::warn;
use crate::collection::{Art,Album};
use crate::ccd::msg::CcdToKernel;
use crossbeam::channel::{Sender};
use std::path::{Path,PathBuf};

//---------------------------------------------------------------------------------------------------- Image()
// These functions are for the images in `~/.local/share/festival/${FRONTEND}/image`.
#[inline(always)]
pub(crate) fn save_image_and_convert(
	key: usize,
	album: &mut Album,
	base_path: &Path,
) {
	if let Art::Bytes(bytes) = &mut album.art {
		let Some(infer) = infer::get(&bytes) else {
			warn!("CCD ... Album [{}], key [{key}]: failed to infer art image type", album.title);
			return;
		};

		let mut path = PathBuf::from(base_path);
		path.push(format!("{key}.{}", infer.extension()));

		match std::fs::write(&path, &bytes) {
			Ok(_) => {
				ok_trace!("CCD ... Image: {}", &path.display());
				album.art = Art::Known { path, mime: infer.mime_type().into(), len: bytes.len() };
			},
			Err(e) => { warn!("CCD ... Image {e}: {}", path.display()); album.art = Art::Unknown; },
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	use super::*;
//}
