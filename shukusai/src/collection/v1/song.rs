//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use std::path::PathBuf;
use readable::Runtime;
use std::marker::PhantomData;
use crate::collection::{
	AlbumKey,
	SongKey,
};
use std::sync::Arc;

//----------------------------------------------------------------------------------------------------
#[derive(Clone,Debug,Hash,PartialEq,PartialOrd,Encode,Decode)]
/// Struct holding [`Song`] metadata, with a pointer to the [`Album`] it belongs to
///
/// This struct holds all the metadata about a particular [`Song`].
///
/// It contains a [`SongKey`] that is the index of the owning [`Album`], in the [`Collection`].
pub struct Song {
	// User-facing data.
	/// Title of the [`Song`].
	pub title: Arc<str>,
	/// Title of the [`Song`] in "Unicode Derived Core Property" lowercase.
	pub title_lowercase: Arc<str>,
	/// Key to the [`Album`].
	pub album: AlbumKey,
	/// Total runtime of this [`Song`].
	pub runtime: Runtime,
	/// Sample rate of this [`Song`].
	pub sample_rate: u32,
	/// The track number of this [`Song`].
	pub track: Option<u32>,
	/// The disc number of this [`Song`].
	pub disc: Option<u32>,
	/// The [`PathBuf`] this [`Song`] is located at.
	pub path: PathBuf,
}

impl Into<crate::collection::Song> for Song {
	fn into(self) -> crate::collection::Song {
		let Self {
			title,
			title_lowercase,
			album,
			runtime,
			sample_rate,
			track,
			disc,
			path
		} = self;

		crate::collection::Song {
			// INVARIANT: must be set correctly in the broader `Collection::into()`
			key: SongKey::zero(),

			title,
			title_lowercase,
			album,
			runtime,
			sample_rate,
			track,
			disc,
			path
		}
	}
}
