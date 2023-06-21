//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use benri::debug_panic;

//---------------------------------------------------------------------------------------------------- MIME constants.
pub(crate) const SUPPORTED_AUDIO_MIME_TYPES: [&str; 22] = [
	// AAC
	"audio/aac",
	"audio/x-aac",
	// ALAC
	"audio/m4a",
	"audio/x-m4a",
	// FLAC
	"audio/flac",
	"audio/x-flac",
	// MP3
	"audio/mp3",
	"audio/mpeg",
	"audio/mpeg3",
	"audio/x-mp3",
	"audio/x-mpeg",
	"audio/x-mpeg3",
	// OGG/Vorbis
	"audio/ogg",
	"audio/vorbis",
	"audio/x-ogg",
	"audio/x-vorbis",
	// Opus
	"audio/opus",
	"audio/x-opus",
	// PCM (wav, aiff)
	"audio/wav",
	"audio/x-wav",
	"audio/aiff",
	"audio/x-aiff",
];

pub(crate) const SUPPORTED_IMG_MIME_TYPES: [&str; 3] = [
	"image/jpg",
	"image/jpeg",
	"image/png",
];

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub(super) enum Codec {
	Aac,
	Alac,
	Flac,
	Mp3,
	Ogg, // Vorbis.
	Opus,
	Wav,
	Aiff,
}

impl Codec {
	// INVARIANT:
	// This expects input to be one of the
	// `str`'s from the above `SUPPORTED_AUDIO_MIME_TYPES`.
	//
	// Else, it assumes `mp3`.
	pub(super) fn from_supported(s: &str) -> Self {
		match s {
			// AAC
			"audio/aac"|"audio/x-aac" => Self::Aac,
			// ALAC
			"audio/m4a"|"audio/x-m4a" => Self::Alac,
			// FLAC
			"audio/flac"|"audio/x-flac" => Self::Flac,
			// MP3
			"audio/mp3"|"audio/mpeg"|"audio/mpeg3"|"audio/x-mp3"|"audio/x-mpeg"|"audio/x-mpeg3" => Self::Mp3,
			// OGG/Vorbis
			"audio/ogg"|"audio/vorbis"|"audio/x-ogg"|"audio/x-vorbis" => Self::Ogg,
			// Opus
			"audio/opus"|"audio/x-opus" => Self::Opus,
			// PCM (wav, aiff)
			"audio/wav"|"audio/x-wav" => Self::Wav,
			"audio/aiff"|"audio/x-aiff" => Self::Aiff,

			_ => {
				debug_panic!("input to from_supported() was wrong");
				Self::Mp3
			},
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	// Detect MIME types.
	fn detect(mime: &str, extension: &str) {
		let infer = infer::get_from_path(format!("assets/audio/rain.{}", extension)).unwrap().unwrap().mime_type();
		let guess  = mime_guess::MimeGuess::from_path(format!("assets/audio/rain.{}", extension)).first_raw().unwrap();

		eprintln!("INFER: {}\nGUESS: {}", infer, guess);

		assert!(infer == format!("audio/{}", mime) || infer == format!("audio/x-{}", mime));
		assert!(guess == format!("audio/{}", mime) || guess == format!("audio/x-{}", mime));
		assert!(super::SUPPORTED_AUDIO_MIME_TYPES.contains(&infer));
		assert!(super::SUPPORTED_AUDIO_MIME_TYPES.contains(&guess));
	}

	#[test]
	fn detect_aac() { detect("aac", "aac"); }
	#[test]
	fn detect_alac() { detect("m4a", "m4a"); }
	#[test]
	fn detect_flac() { detect("flac", "flac"); }
	#[test]
	fn detect_mp3() { detect("mpeg", "mp3"); }
	#[test]
	fn detect_ogg() { detect("ogg", "ogg"); }
	#[test]
	fn detect_wav() { detect("wav", "wav"); }
	#[test]
	fn detect_aiff() { detect("aiff", "aiff"); }
}