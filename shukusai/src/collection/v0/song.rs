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
#[derive(Clone,Debug,Default,Hash,PartialEq,PartialOrd,Encode,Decode)]
/// Struct holding [`Song`] metadata, with a pointer to the [`Album`] it belongs to
///
/// This struct holds all the metadata about a particular [`Song`].
///
/// It contains a [`SongKey`] that is the index of the owning [`Album`], in the [`Collection`].
pub(crate) struct Song {
	// User-facing data.
	/// Title of the [`Song`].
	pub(crate) title: String,
	/// Key to the [`Album`].
	pub(crate) album: AlbumKey,
	/// Total runtime of this [`Song`].
	pub(crate) runtime: Runtime,
	/// Sample rate of this [`Song`].
	pub(crate) sample_rate: u32,
	/// The track number of this [`Song`].
	pub(crate) track: Option<u32>,
	/// The disc number of this [`Song`].
	pub(crate) disc: Option<u32>,
	/// The [`PathBuf`] this [`Song`] is located at.
	pub(crate) path: PathBuf,
}

impl Into<crate::collection::Song> for Song {
	fn into(self) -> crate::collection::Song {
		let Self {
			title,
			album,
			runtime,
			sample_rate,
			track,
			disc,
			path
		} = self;

		let title_lowercase = title.to_lowercase().into();
		let title = title.into();

		crate::collection::Song {
			// INVARIANT: must be set correctly in the broader `Collection::into()`
			key: SongKey::zero(),

			// Could chase PATHs and recover this
			// but that's slow and this info isn't crucial.
			mime: "".into(),
			extension: "".into(),

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
