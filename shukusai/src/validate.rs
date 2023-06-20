//---------------------------------------------------------------------------------------------------- Use
use crate::collection::{
	Collection,Keychain,Key,
	ArtistKey,AlbumKey,SongKey,
};

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

	!(
		c.artists.len() < artists_max ||
		c.albums.len() < albums_max   ||
		c.songs.len()  < songs_max
	)
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
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
