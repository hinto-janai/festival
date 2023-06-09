//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use std::marker::PhantomData;
use super::{
	ArtistKey,
	SongKey,
};
use super::art::{
	Art,
};
use readable::{
	Runtime,
	Unsigned,
	Date,
};
use std::path::PathBuf;

//---------------------------------------------------------------------------------------------------- Album
#[derive(Clone,Debug,Default,PartialEq,PartialOrd,Encode,Decode)]
/// Struct holding [`Album`] metadata, with pointers to an [`Artist`] and [`Song`]\(s\)
///
/// This struct holds all the metadata about a particular [`Album`].
///
/// It contains an [`ArtistKey`] that is the index of the owning [`Artist`], in the [`Collection`].
///
/// It also contains [`SongKey`]\(s\) that are the indices of [`Song`]\(s\) belonging to this [`Album`], in the [`Collection`].
pub struct Album {
	// User-facing data.
	/// Title of the [`Album`].
	pub title: String,
	/// Key to the [`Artist`].
	pub artist: ArtistKey,
	/// Human-readable release date of this [`Album`].
	pub release: Date,
	/// Total runtime of this [`Album`].
	pub runtime: Runtime,
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
	/// Key\(s\) to the [`Song`]\(s\).
	pub songs: Vec<SongKey>,
	/// How many discs are in this `Album`?
	/// (Most will only have 1).
	pub discs: u32,

	/// The parent `PATH` of this `Album`.
	///
	/// This is always taken from the 1st `Song` that is inserted
	/// into this `Album`, so if the other `Song`'s are in different
	/// parent directories, this will not be fully accurate.
	pub path: PathBuf,

	/// The `Album`'s art.
	///
	/// `Frontend`'s don't need to access this field
	/// directly, instead, use `album.art_or()`.
	pub art: Art, // Always initialized after `CCD`.

	// Reserved fields that should SOMEDAY be implemented.
	/// The [`Album`]'s genre.
	pub _genre: PhantomData<Option<String>>,
	/// Boolean representing if this is a compilation or not.
	pub _compilation: PhantomData<bool>,

	// Reserved fields and their `size_of()`.
	pub(crate) _reserved1: PhantomData<Box<[usize]>>, // 16
	pub(crate) _reserved2: PhantomData<Box<[usize]>>, // 16
	pub(crate) _reserved3: PhantomData<Box<[usize]>>, // 16
	pub(crate) _reserved4: PhantomData<Box<[usize]>>, // 16
	pub(crate) _reserved5: PhantomData<String>,       // 24
	pub(crate) _reserved6: PhantomData<usize>,        // 8
}

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

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
