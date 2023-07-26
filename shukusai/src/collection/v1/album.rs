//---------------------------------------------------------------------------------------------------- Use
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
#[derive(Clone,Debug,PartialEq,PartialOrd,Encode,Decode)]
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
	pub title: Arc<str>,
	/// Title of the [`Album`] in "Unicode Derived Core Property" lowercase.
	pub title_lowercase: Arc<str>,
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
	//
	// SOMEDAY:
	// This should be a Box<[AlbumKey]>.
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
	/// `GUI` doesn't need to access this field
	/// directly, instead, use `album.art_or()`.
	///
	/// THIS TYPE IS DIFFERENT DEPENDING ON THE FRONTEND.
	pub art: Art,
}

impl Into<crate::collection::Album> for Album {
	fn into(self) -> crate::collection::Album {
		let Self {
			title,
			title_lowercase,
			artist,
			release,
			runtime,
			song_count,
			songs,
			discs,
			path,
			art,
		} = self;

		crate::collection::Album {
			// INVARIANT: must be set correctly in the broader `Collection::into()`
			key: AlbumKey::zero(),
			// We can't recover this info, assume user will rescan... eventually...
			genre: None,

			title,
			title_lowercase,
			artist,
			release,
			runtime,
			song_count,
			songs,
			discs,
			path,
			art,
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
