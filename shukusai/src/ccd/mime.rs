//---------------------------------------------------------------------------------------------------- Use
use benri::debug_panic;

//---------------------------------------------------------------------------------------------------- MIME constants.
// SOMEDAY: Fix AIFF (symphonia doesn't parse metadata correctly)
pub(crate) const SUPPORTED_AUDIO_MIME_TYPES: [&str; 26] = [
	// AAC
	"audio/aac",
	"audio/x-aac",
	// ADPCM
	"audio/adpcm",
	"audio/x-adpcm",
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
//	"audio/aiff",
//	"audio/x-aiff",
	// Wavpack
	"audio/wavpack",
	"audio/x-wavpack",
	"audio/wavpack-correction",
	"audio/x-wavpack-correction",
];

pub(crate) const SUPPORTED_IMG_MIME_TYPES: [&str; 9] = [
	"image/jpg",
	"image/jpeg",
	"image/png",
	"image/bmp",
	"image/ico", "image/x-icon", "image/vnd.microsoft.icon", // thanks microsoft.
	"image/tiff",
	"image/webp",
//	"image/avif",
];

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub(super) enum Codec {
	Aac,
	Adpcm,
	Alac,
	Flac,
	Mp3,
	Ogg, // Vorbis.
	Opus,
	Wav,
//	Aiff,
	Wavpack,
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
			// ADPCM
			"audio/adpcm"|"audio/x-adpcm" => Self::Adpcm,
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
//			"audio/aiff"|"audio/x-aiff" => Self::Aiff,
			// Wavpack
			"audio/wavpack"|"audio/x-wavpack" => Self::Wavpack,

			_ => {
				debug_panic!("input to from_supported() was wrong");
				Self::Mp3
			},
		}
	}
}
