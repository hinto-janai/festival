//---------------------------------------------------------------------------------------------------- Use
use crate::collection::{AlbumKey, ArtistKey, Collection, Key, Keychain, SongKey};

//---------------------------------------------------------------------------------------------------- Ancillary Collection data validation
/// Check if a [`Key`] is a valid index into the [`Collection`]
///
/// - `true` == valid
/// - `false` == invalid
pub fn key<K: Into<Key>>(c: &Collection, key: K) -> bool {
    c.get(key.into()).is_some()
}

/// Check if an [`ArtistKey`] is a valid index into the [`Collection`]
///
/// - `true` == valid
/// - `false` == invalid
pub fn artist<K: Into<ArtistKey>>(c: &Collection, key: K) -> bool {
    c.artists.get(key.into()).is_some()
}

/// Check if an [`AlbumKey`] is a valid index into the [`Collection`]
///
/// - `true` == valid
/// - `false` == invalid
pub fn album<K: Into<AlbumKey>>(c: &Collection, key: K) -> bool {
    c.albums.get(key.into()).is_some()
}

/// Check if a [`SongKey`] is a valid index into the [`Collection`]
///
/// - `true` == valid
/// - `false` == invalid
pub fn song<K: Into<SongKey>>(c: &Collection, key: K) -> bool {
    c.songs.get(key.into()).is_some()
}

/// Check if a [`Keychain`] contains valid indices into the [`Collection`]
///
/// If the [`Keychain`] is completely empty, this returns `true`.
///
/// - `true` == valid
/// - `false` == invalid
pub fn keychain(c: &Collection, keychain: &Keychain) -> bool {
    let artists_len = c.artists.len();
    let albums_len = c.albums.len();
    let songs_len = c.songs.len();

    if artists_len == 0 || albums_len == 0 || songs_len == 0 {
        return false;
    }

    let artists_max = match keychain.artists.iter().max() {
        Some(key) => key.inner(),
        None => 0,
    };
    let albums_max = match keychain.albums.iter().max() {
        Some(key) => key.inner(),
        None => 0,
    };
    let songs_max = match keychain.songs.iter().max() {
        Some(key) => key.inner(),
        None => 0,
    };

    !(artists_len < artists_max || albums_len < albums_max || songs_len < songs_max)
}

///// Check if a [`Queue`] contains valid indices into the [`Collection`]
/////
///// If the [`Queue`] is completely empty, this returns `true`.
/////
///// - `true` == valid
///// - `false` == invalid
//pub fn queue(c: &Collection, queue: &Queue) -> bool {
//	match queue.iter().max() {
//		Some(key) => c.get(*key).is_some(),
//		None => true,
//	}
//}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        let collection = Collection::new();
        let c = &collection;

        assert_eq!(c.artists.len(), 0);
        assert_eq!(c.albums.len(), 0);
        assert_eq!(c.songs.len(), 0);

        let ar = ArtistKey::from(1_u8);
        let al = AlbumKey::from(1_u8);
        let s = SongKey::from(1_u8);
        let k = Key::from_keys(ar, al, s);

        assert!(!key(c, k));
        assert!(!artist(c, ar));
        assert!(!album(c, al));
        assert!(!song(c, s));

        let kc = Keychain {
            artists: Box::new([ar]),
            albums: Box::new([al]),
            songs: Box::new([s]),
        };
        assert!(!keychain(c, &kc));

        let kc = Keychain::new();
        assert!(!keychain(c, &kc));
    }
}
