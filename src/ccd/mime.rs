//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};

//---------------------------------------------------------------------------------------------------- MIME constants.
pub const SUPPORTED_AUDIO_MIME_TYPES: [&str; 22] = [
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
	// OGG/Opus/Vorbis
	"audio/ogg",
	"audio/opus",
	"audio/vorbis",
	"audio/x-ogg",
	"audio/x-opus",
	"audio/x-vorbis",
	// PCM (wav, aiff)
	"audio/wav",
	"audio/x-wav",
	"audio/aiff",
	"audio/x-aiff",
];

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
