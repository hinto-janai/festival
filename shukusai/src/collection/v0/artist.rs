//---------------------------------------------------------------------------------------------------- Use
use crate::collection::{AlbumKey, ArtistKey, SongKey};
use bincode::{Decode, Encode};
use readable::Runtime;
use std::marker::PhantomData;

//----------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
/// Struct holding [`Artist`] metadata, with pointers to [`Album`]\(s\)
///
/// This struct holds all the metadata about a particular [`Artist`].
///
/// It contains an [`Vec`] of [`AlbumKey`]\(s\) that are the indices of the associated [`Album`]\(s\), in the [`Collection`].
pub(crate) struct Artist {
    /// The [`Artist`]'s name.
    pub(crate) name: String,
    /// Total runtime.
    pub(crate) runtime: Runtime,
    /// Keys to the associated [`Album`]\(s\).
    pub(crate) albums: Vec<AlbumKey>,
    /// Keys to every [`Song`] by this [`Artist`].
    ///
    /// The order is [`Album`] release order, then [`Song`] track order.
    pub(crate) songs: Box<[SongKey]>,
}

impl Into<crate::collection::Artist> for Artist {
    fn into(self) -> crate::collection::Artist {
        let Self {
            name,
            runtime,
            albums,
            songs,
        } = self;

        let name_lowercase = name.to_lowercase().into();
        let name = name.into();

        crate::collection::Artist {
            // INVARIANT: must be set correctly in the broader `Collection::into()`
            key: ArtistKey::zero(),

            name,
            name_lowercase,
            runtime,
            albums,
            songs,
        }
    }
}
