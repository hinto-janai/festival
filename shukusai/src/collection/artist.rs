//---------------------------------------------------------------------------------------------------- Use
use crate::collection::key::{AlbumKey, ArtistKey, SongKey};
use bincode::{Decode, Encode};
use readable::Runtime;
use serde::Serialize;
use std::marker::PhantomData;
use std::sync::Arc;

//----------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Encode, Decode)]
/// Struct holding [`Artist`] metadata, with pointers to [`Album`]\(s\)
///
/// This struct holds all the metadata about a particular [`Artist`].
///
/// It contains an [`Vec`] of [`AlbumKey`]\(s\) that are the indices of the associated [`Album`]\(s\), in the [`Collection`].
pub struct Artist {
    /// The [`Artist`]'s name.
    pub name: Arc<str>,
    #[serde(skip)]
    /// The [`Artist`]'s name in "Unicode Derived Core Property" lowercase.
    pub name_lowercase: Arc<str>,

    /// This [`Artist`]'s [`ArtistKey`].
    pub key: ArtistKey,

    #[serde(serialize_with = "crate::serde::runtime")]
    /// Total runtime.
    pub runtime: Runtime,

    // SOMEDAY:
    // This should be a Box<[AlbumKey]>.
    /// Keys to the associated [`Album`]\(s\).
    pub albums: Vec<AlbumKey>,

    /// Keys to every [`Song`] by this [`Artist`].
    ///
    /// The order is [`Album`] release order, then [`Song`] track order.
    pub songs: Box<[SongKey]>,
}

impl Default for Artist {
    fn default() -> Self {
        Self {
            name: "".into(),
            name_lowercase: "".into(),
            key: ArtistKey::zero(),
            runtime: Default::default(),
            albums: Vec::with_capacity(0),
            songs: Box::new([]),
        }
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;
}
