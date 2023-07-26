//---------------------------------------------------------------------------------------------------- Use
use serde::Serialize;
use bincode::{Encode,Decode};
use std::marker::PhantomData;
use crate::collection::key::{
	ArtistKey,
	AlbumKey,
	SongKey,
};
use crate::collection::art::{
	Art,
};
use readable::{
	Runtime,
	Unsigned,
	Date,
};
use std::path::PathBuf;
use std::sync::Arc;

//---------------------------------------------------------------------------------------------------- Album
#[derive(Clone,Debug,PartialEq,PartialOrd,Serialize,Encode,Decode)]
/// Struct holding [`Album`] metadata, with pointers to an [`Artist`] and [`Song`]\(s\)
///
/// This struct holds all the metadata about a particular [`Album`].
///
/// It contains an [`ArtistKey`] that is the index of the owning [`Artist`], in the [`Collection`].
///
/// It also contains [`SongKey`]\(s\) that are the indices of [`Song`]\(s\) belonging to this [`Album`], in the [`Collection`].
pub struct Album {
	/// Title of the [`Album`].
	pub title: Arc<str>,
	#[serde(skip)]
	/// Title of the [`Album`] in "Unicode Derived Core Property" lowercase.
	pub title_lowercase: Arc<str>,

	/// This [`Album`]'s [`AlbumKey`].
	pub key: AlbumKey,
	/// Key to the [`Artist`].
	pub artist: ArtistKey,

	#[serde(serialize_with = "crate::collection::serde::date")]
	/// Human-readable release date of this [`Album`].
	pub release: Date,
	#[serde(serialize_with = "crate::collection::serde::runtime")]
	/// Total runtime of this [`Album`].
	pub runtime: Runtime,
	#[serde(serialize_with = "crate::collection::serde::unsigned")]
	/// [`Song`] count of this [`Album`].
	pub song_count: Unsigned,
	// This `Vec<SongKey>` is _always_ sorted based
	// off incrementing disc and track numbers, e.g:
	//
	// DISC 1:
	//   - 1. ...
	//   - 2. ...
	// DISC 2:
	//   - 1. ...
	//   - 2. ...
	//
	// So, doing `my_album.songs.iter()` will always
	// result in the correct `Song` order for `my_album`.
	//
	// SOMEDAY:
	// This should be a Box<[AlbumKey]>.
	/// Key\(s\) to the [`Song`]\(s\).
	pub songs: Vec<SongKey>,
	/// How many discs are in this `Album`?
	/// (Most will only have 1).
	pub discs: u32,

	#[serde(skip)]
	/// The parent `PATH` of this `Album`.
	///
	/// This is always taken from the 1st `Song` that is inserted
	/// into this `Album`, so if the other `Song`'s are in different
	/// parent directories, this will not be fully accurate.
	pub path: PathBuf,

	#[serde(serialize_with = "crate::collection::serde::art")]
	/// The `Album`'s art.
	///
	/// `GUI` doesn't need to access this field
	/// directly, instead, use `album.art_or()`.
	///
	/// THIS TYPE IS DIFFERENT DEPENDING ON THE FRONTEND.
	pub art: Art,

	/// This [`Album`]'s genre.
	pub genre: Option<String>,
}

#[cfg(feature = "gui")]
impl Album {
	#[inline(always)]
	/// Return the [`Album`] art.
	///
	/// Some [`Album`]'s may not have art. In this case, we'd like to show a "unknown" image anyway.
	///
	/// This function will always return a valid [`egui_extras::RetainedImage`], either:
	/// 1. The real [`Album`] art (if it exists)
	/// 2. An "unknown" image
	///
	/// The returned "unknown" image is actually just a pointer to a single lazily evaluated image.
	///
	/// The "unknown" image is from `assets/images/art/unknown.png`.
	pub fn art_or(&self) -> &egui_extras::RetainedImage {
		self.art.art_or()
	}

	#[inline(always)]
	/// Return the [`Album`] art wrapped in [`Option`].
	///
	/// Same as [`Album::art_or`] but with no "unknown" backup image.
	pub fn art(&self) -> Option<&egui_extras::RetainedImage> {
		self.art.get()
	}

	#[inline]
	/// Calls [`egui_extras::RetainedImage::texture_id`].
	pub fn texture_id(&self, ctx: &egui::Context) -> egui::TextureId {
		self.art.texture_id(ctx)
	}
}

impl Default for Album {
	fn default() -> Self {
		Self {
			key: AlbumKey::zero(),
			title: "".into(),
			title_lowercase: "".into(),
			artist: Default::default(),
			release: Default::default(),
			runtime: Default::default(),
			song_count: Default::default(),
			songs: Vec::with_capacity(0),
			discs: Default::default(),
			path: Default::default(),
			art: Default::default(),
			genre: Default::default(),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	const EXPECTED: &str =
r#"{
  "title": "",
  "key": 0,
  "artist": 0,
  "release": "????-??-??",
  "runtime": 0,
  "song_count": 0,
  "songs": [],
  "discs": 0,
  "art": null,
  "genre": null
}"#;

	#[test]
	#[cfg(feature = "daemon")]
	fn serde_json() {
		let d: String = serde_json::to_string_pretty(&Album::default()).unwrap();
		assert_eq!(EXPECTED, d);
	}
}
