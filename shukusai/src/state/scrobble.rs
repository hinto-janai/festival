//---------------------------------------------------------------------------------------------------- Use
use crate::{
    collection::{Album, AlbumKey, Artist, ArtistKey, Collection, Song, SongKey},
    constants::{FESTIVAL, FRONTEND_SUB_DIR, HEADER, SCROBBLE_VERSION, STATE_SUB_DIR},
};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

//---------------------------------------------------------------------------------------------------- Scrobble
disk::bincode2!(
    Scrobble,
    disk::Dir::Data,
    FESTIVAL,
    formatcp!("{FRONTEND_SUB_DIR}/{STATE_SUB_DIR}"),
    "scrobble",
    HEADER,
    SCROBBLE_VERSION
);
/// TODO
//                         song     seconds
//                           v        v
pub struct Scrobble(BTreeMap<SongKey, u64>);

impl Scrobble {
    pub(crate) fn new(song_key_max: usize) -> Self {
        Self(
            (0..song_key_max)
                .map(|key| (SongKey::from(key), 0))
                .collect(),
        )
    }

    pub(crate) fn inner(&self) -> &BTreeMap<SongKey, u64> {
        &self.0
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
