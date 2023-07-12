//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use std::marker::PhantomData;
use crate::collection::{
	ArtistKey,
	SongKey,
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
pub(crate) struct Album {
	// User-facing data.
	/// Title of the [`Album`].
	pub(crate) title: String,
	/// Key to the [`Artist`].
	pub(crate) artist: ArtistKey,
	/// Human-readable release date of this [`Album`].
	pub(crate) release: Date,
	/// Total runtime of this [`Album`].
	pub(crate) runtime: Runtime,
	/// [`Song`] count of this [`Album`].
	pub(crate) song_count: Unsigned,
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
	pub(crate) songs: Vec<SongKey>,
	/// How many discs are in this `Album`?
	/// (Most will only have 1).
	pub(crate) discs: u32,

	/// The parent `PATH` of this `Album`.
	///
	/// This is always taken from the 1st `Song` that is inserted
	/// into this `Album`, so if the other `Song`'s are in different
	/// parent directories, this will not be fully accurate.
	pub(crate) path: PathBuf,

	/// The `Album`'s art.
	///
	/// `Frontend`'s don't need to access this field
	/// directly, instead, use `album.art_or()`.
	pub(crate) art: Art, // Always initialized after `CCD`.
}

impl Into<crate::collection::Album> for Album {
	fn into(self) -> crate::collection::Album {
		let Self {
			title,
			artist,
			release,
			runtime,
			song_count,
			songs,
			discs,
			path,
			art,
		} = self;

		let title_lowercase = title.to_lowercase().into();
		let title = title.into();

		crate::collection::Album {
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
