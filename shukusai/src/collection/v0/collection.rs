//---------------------------------------------------------------------------------------------------- Use
use crate::collection::v0::{
    album::Album,
    artist::Artist,
    plural::{Albums, Artists, Songs},
    song::Song,
    Map,
};
use crate::collection::{AlbumKey, ArtistKey, Key, SongKey};
use crate::constants::{COLLECTION_VERSION, FESTIVAL, FRONTEND_SUB_DIR, HEADER, STATE_SUB_DIR};
use crate::sort::{AlbumSort, ArtistSort, SongSort};
use benri::lock;
use bincode::{Decode, Encode};
use const_format::formatcp;
use once_cell::sync::Lazy;
use rand::{prelude::SliceRandom, Rng, SeedableRng};
use readable::Unsigned;
use std::sync::{Arc, Mutex};

//---------------------------------------------------------------------------------------------------- Collection
disk::bincode2!(
    Collection,
    disk::Dir::Data,
    FESTIVAL,
    formatcp!("{FRONTEND_SUB_DIR}/{STATE_SUB_DIR}"),
    "collection",
    HEADER,
    0
);
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
/// Version 0 of `Collection`.
pub(crate) struct Collection {
    // Metadata about the `Collection` itself.
    /// Is this [`Collection`] empty?
    ///
    /// Meaning, are there absolutely no [`Artist`]'s, [`Album`]'s and [`Song`]'s?
    pub(crate) empty: bool,
    /// UNIX timestamp of the [`Collection`]'s creation date.
    pub(crate) timestamp: u64,
    /// How many [`Artist`]'s in this [`Collection`]?
    pub(crate) count_artist: Unsigned,
    /// How many [`Album`]'s in this [`Collection`]?
    pub(crate) count_album: Unsigned,
    /// How many [`Song`]'s in this [`Collection`]?
    pub(crate) count_song: Unsigned,
    /// How many unique [`Album`] covers are there in this [`Collection`]?
    pub(crate) count_art: Unsigned,

    // The "Map".
    /// A [`HashMap`] that knows all [`Artist`]'s, [`Album`]'s and [`Song`]'s.
    pub(crate) map: Map,

    // The "3 arrays".
    /// All the [`Artist`]'s in mostly random order.
    pub(crate) artists: Artists,
    /// All the [`Album`]'s in mostly random order.
    pub(crate) albums: Albums,
    /// All the [`Song`]'s in mostly random order.
    pub(crate) songs: Songs,

    // Sorted `Artist` keys.
    /// [`Artist`] A-Z.
    pub(crate) sort_artist_lexi: Box<[ArtistKey]>,
    /// [`Artist`] Z-A.
    pub(crate) sort_artist_lexi_rev: Box<[ArtistKey]>,
    /// [`Artist`] with most [`Album`]'s to least.
    pub(crate) sort_artist_album_count: Box<[ArtistKey]>,
    /// [`Artist`] with least [`Album`]'s to most.
    pub(crate) sort_artist_album_count_rev: Box<[ArtistKey]>,
    /// [`Artist`] with most [`Song`]'s to least.
    pub(crate) sort_artist_song_count: Box<[ArtistKey]>,
    /// [`Artist`] with least [`Song`]'s to most.
    pub(crate) sort_artist_song_count_rev: Box<[ArtistKey]>,
    /// [`Artist`] runtime least-most.
    pub(crate) sort_artist_runtime: Box<[ArtistKey]>,
    /// [`Artist`] runtime most-least.
    pub(crate) sort_artist_runtime_rev: Box<[ArtistKey]>,
    /// [`Artist`] name shortest-longest.
    pub(crate) sort_artist_name: Box<[ArtistKey]>,
    /// [`Artist`] name longest-shortest
    pub(crate) sort_artist_name_rev: Box<[ArtistKey]>,

    // Sorted `Album` keys.
    /// [`Artist`] A-Z, [`Album`] oldest-latest.
    pub(crate) sort_album_release_artist_lexi: Box<[AlbumKey]>,
    /// [`Artist`] Z-A, [`Album`] oldest-latest.
    pub(crate) sort_album_release_artist_lexi_rev: Box<[AlbumKey]>,
    /// [`Artist`] A-Z, [`Album`] latest-oldest.
    pub(crate) sort_album_release_rev_artist_lexi: Box<[AlbumKey]>,
    /// [`Artist`] Z-A, [`Album`] latest-oldest.
    pub(crate) sort_album_release_rev_artist_lexi_rev: Box<[AlbumKey]>,
    /// [`Artist`] A-Z, [`Album`] A-Z.
    pub(crate) sort_album_lexi_artist_lexi: Box<[AlbumKey]>,
    /// [`Artist`] Z-A, [`Album`] A-Z.
    pub(crate) sort_album_lexi_artist_lexi_rev: Box<[AlbumKey]>,
    /// [`Artist`] A-Z, [`Album`] Z-A.
    pub(crate) sort_album_lexi_rev_artist_lexi: Box<[AlbumKey]>,
    /// [`Artist`] Z-A, [`Album`] Z-A.
    pub(crate) sort_album_lexi_rev_artist_lexi_rev: Box<[AlbumKey]>,
    /// [`Album`] A-Z.
    pub(crate) sort_album_lexi: Box<[AlbumKey]>,
    /// [`Album`] Z-A.
    pub(crate) sort_album_lexi_rev: Box<[AlbumKey]>,
    /// [`Album`] oldest to latest.
    pub(crate) sort_album_release: Box<[AlbumKey]>,
    /// [`Album`] latest to oldest.
    pub(crate) sort_album_release_rev: Box<[AlbumKey]>,
    /// [`Album`] shortest to longest.
    pub(crate) sort_album_runtime: Box<[AlbumKey]>,
    /// [`Album`] longest to shortest.
    pub(crate) sort_album_runtime_rev: Box<[AlbumKey]>,
    /// [`Album`] title shortest to longest.
    pub(crate) sort_album_title: Box<[AlbumKey]>,
    /// [`Album`] title longest to shortest.
    pub(crate) sort_album_title_rev: Box<[AlbumKey]>,

    // Sorted `Song` keys.
    /// [`Artist`] A-Z, [`Album`] oldest-latest, [`Song`] track_number
    pub(crate) sort_song_album_release_artist_lexi: Box<[SongKey]>,
    /// [`Artist`] Z-A, [`Album`] oldest-latest, [`Song`] track_number
    pub(crate) sort_song_album_release_artist_lexi_rev: Box<[SongKey]>,
    /// [`Artist`] A-Z, [`Album`] latest-oldest, [`Song`] track_number
    pub(crate) sort_song_album_release_rev_artist_lexi: Box<[SongKey]>,
    /// [`Artist`] Z-A, [`Album`] latest-oldest, [`Song`] track_number
    pub(crate) sort_song_album_release_rev_artist_lexi_rev: Box<[SongKey]>,
    /// [`Artist`] A-Z, [`Album`] A-Z, [`Song`] track_number.
    pub(crate) sort_song_album_lexi_artist_lexi: Box<[SongKey]>,
    /// [`Artist`] Z-A, [`Album`] A-Z, [`Song`] track_number.
    pub(crate) sort_song_album_lexi_artist_lexi_rev: Box<[SongKey]>,
    /// [`Artist`] A-Z, [`Album`] Z-A, [`Song`] track_number.
    pub(crate) sort_song_album_lexi_rev_artist_lexi: Box<[SongKey]>,
    /// [`Artist`] Z-A, [`Album`] Z-A, [`Song`] track_number.
    pub(crate) sort_song_album_lexi_rev_artist_lexi_rev: Box<[SongKey]>,
    /// [`Song`] A-Z.
    pub(crate) sort_song_lexi: Box<[SongKey]>,
    /// [`Song`] Z-A.
    pub(crate) sort_song_lexi_rev: Box<[SongKey]>,
    /// [`Song`] oldest to latest.
    pub(crate) sort_song_release: Box<[SongKey]>,
    /// [`Song`] latest to oldest.
    pub(crate) sort_song_release_rev: Box<[SongKey]>,
    /// [`Song`] shortest to longest.
    pub(crate) sort_song_runtime: Box<[SongKey]>,
    /// [`Song`] longest to shortest.
    pub(crate) sort_song_runtime_rev: Box<[SongKey]>,
    /// [`Song`] title shortest to longest.
    pub(crate) sort_song_title: Box<[SongKey]>,
    /// [`Song`] title longest to shortest.
    pub(crate) sort_song_title_rev: Box<[SongKey]>,
}

impl Into<crate::collection::Collection> for Collection {
    fn into(self) -> crate::collection::Collection {
        let Self {
            empty,
            timestamp,
            count_artist,
            count_album,
            count_song,
            count_art,

            map,
            artists,
            albums,
            songs,

            sort_artist_lexi,
            sort_artist_lexi_rev,
            sort_artist_album_count,
            sort_artist_album_count_rev,
            sort_artist_song_count,
            sort_artist_song_count_rev,
            sort_artist_runtime,
            sort_artist_runtime_rev,
            sort_artist_name,
            sort_artist_name_rev,

            sort_album_release_artist_lexi,
            sort_album_release_artist_lexi_rev,
            sort_album_release_rev_artist_lexi,
            sort_album_release_rev_artist_lexi_rev,
            sort_album_lexi_artist_lexi,
            sort_album_lexi_artist_lexi_rev,
            sort_album_lexi_rev_artist_lexi,
            sort_album_lexi_rev_artist_lexi_rev,
            sort_album_lexi,
            sort_album_lexi_rev,
            sort_album_release,
            sort_album_release_rev,
            sort_album_runtime,
            sort_album_runtime_rev,
            sort_album_title,
            sort_album_title_rev,

            sort_song_album_release_artist_lexi,
            sort_song_album_release_artist_lexi_rev,
            sort_song_album_release_rev_artist_lexi,
            sort_song_album_release_rev_artist_lexi_rev,
            sort_song_album_lexi_artist_lexi,
            sort_song_album_lexi_artist_lexi_rev,
            sort_song_album_lexi_rev_artist_lexi,
            sort_song_album_lexi_rev_artist_lexi_rev,
            sort_song_lexi,
            sort_song_lexi_rev,
            sort_song_release,
            sort_song_release_rev,
            sort_song_runtime,
            sort_song_runtime_rev,
            sort_song_title,
            sort_song_title_rev,
        } = self;

        let artists: crate::collection::Artists = artists.into();
        let albums: crate::collection::Albums = albums.into();
        let songs: crate::collection::Songs = songs.into();
        let map = crate::collection::Map::from_3_vecs(&artists.0, &albums.0, &songs.0);

        crate::collection::Collection {
            empty,
            timestamp,
            count_artist,
            count_album,
            count_song,
            count_art,

            map,
            artists,
            albums,
            songs,

            sort_artist_lexi,
            sort_artist_lexi_rev,
            sort_artist_album_count,
            sort_artist_album_count_rev,
            sort_artist_song_count,
            sort_artist_song_count_rev,
            sort_artist_runtime,
            sort_artist_runtime_rev,
            sort_artist_name,
            sort_artist_name_rev,

            sort_album_release_artist_lexi,
            sort_album_release_artist_lexi_rev,
            sort_album_release_rev_artist_lexi,
            sort_album_release_rev_artist_lexi_rev,
            sort_album_lexi_artist_lexi,
            sort_album_lexi_artist_lexi_rev,
            sort_album_lexi_rev_artist_lexi,
            sort_album_lexi_rev_artist_lexi_rev,
            sort_album_lexi,
            sort_album_lexi_rev,
            sort_album_release,
            sort_album_release_rev,
            sort_album_runtime,
            sort_album_runtime_rev,
            sort_album_title,
            sort_album_title_rev,

            sort_song_album_release_artist_lexi,
            sort_song_album_release_artist_lexi_rev,
            sort_song_album_release_rev_artist_lexi,
            sort_song_album_release_rev_artist_lexi_rev,
            sort_song_album_lexi_artist_lexi,
            sort_song_album_lexi_artist_lexi_rev,
            sort_song_album_lexi_rev_artist_lexi,
            sort_song_album_lexi_rev_artist_lexi_rev,
            sort_song_lexi,
            sort_song_lexi_rev,
            sort_song_release,
            sort_song_release_rev,
            sort_song_runtime,
            sort_song_runtime_rev,
            sort_song_title,
            sort_song_title_rev,
        }
    }
}

impl Collection {
    //-------------------------------------------------- Converts v0 from disk into current.
    pub(crate) fn disk_into() -> Result<crate::collection::Collection, anyhow::Error> {
        use disk::Bincode2;
        // SAFETY: memmap is used.
        unsafe { Self::from_file_memmap().map(Into::into) }
    }

    //-------------------------------------------------- New.
    /// Creates an empty [`Collection`].
    pub(crate) fn new() -> Self {
        Self {
            empty: true,
            timestamp: 0,
            count_artist: Unsigned::zero(),
            count_album: Unsigned::zero(),
            count_song: Unsigned::zero(),
            count_art: Unsigned::zero(),

            map: Map::new(),
            artists: Artists::new(),
            albums: Albums::new(),
            songs: Songs::new(),

            sort_artist_lexi: Box::new([]),
            sort_artist_lexi_rev: Box::new([]),
            sort_artist_album_count: Box::new([]),
            sort_artist_album_count_rev: Box::new([]),
            sort_artist_song_count: Box::new([]),
            sort_artist_song_count_rev: Box::new([]),
            sort_artist_runtime: Box::new([]),
            sort_artist_runtime_rev: Box::new([]),
            sort_artist_name: Box::new([]),
            sort_artist_name_rev: Box::new([]),

            sort_album_release_artist_lexi: Box::new([]),
            sort_album_release_artist_lexi_rev: Box::new([]),
            sort_album_release_rev_artist_lexi: Box::new([]),
            sort_album_release_rev_artist_lexi_rev: Box::new([]),
            sort_album_lexi_artist_lexi: Box::new([]),
            sort_album_lexi_artist_lexi_rev: Box::new([]),
            sort_album_lexi_rev_artist_lexi: Box::new([]),
            sort_album_lexi_rev_artist_lexi_rev: Box::new([]),
            sort_album_lexi: Box::new([]),
            sort_album_lexi_rev: Box::new([]),
            sort_album_release: Box::new([]),
            sort_album_release_rev: Box::new([]),
            sort_album_runtime: Box::new([]),
            sort_album_runtime_rev: Box::new([]),
            sort_album_title: Box::new([]),
            sort_album_title_rev: Box::new([]),

            sort_song_album_release_artist_lexi: Box::new([]),
            sort_song_album_release_artist_lexi_rev: Box::new([]),
            sort_song_album_release_rev_artist_lexi: Box::new([]),
            sort_song_album_release_rev_artist_lexi_rev: Box::new([]),
            sort_song_album_lexi_artist_lexi: Box::new([]),
            sort_song_album_lexi_artist_lexi_rev: Box::new([]),
            sort_song_album_lexi_rev_artist_lexi: Box::new([]),
            sort_song_album_lexi_rev_artist_lexi_rev: Box::new([]),
            sort_song_lexi: Box::new([]),
            sort_song_lexi_rev: Box::new([]),
            sort_song_release: Box::new([]),
            sort_song_release_rev: Box::new([]),
            sort_song_runtime: Box::new([]),
            sort_song_runtime_rev: Box::new([]),
            sort_song_title: Box::new([]),
            sort_song_title_rev: Box::new([]),
        }
    }

    //-------------------------------------------------- Searching.
    #[inline]
    /// Search [`Collection`] for an [`Artist`].
    ///
    /// # Example:
    /// ```ignore
    /// collection.artist("hinto").unwrap();
    /// ```
    /// In the above example, we're searching for a:
    /// - [`Artist`] called `hinto`
    pub(crate) fn artist<S: AsRef<str>>(&self, artist_name: S) -> Option<(&Artist, ArtistKey)> {
        if let Some((key, _)) = self.map.0.get(artist_name.as_ref()) {
            return Some((&self.artists[key], *key));
        }

        None
    }

    #[inline]
    /// Search [`Collection`] for a [`Song`] in an [`Album`] by an [`Artist`].
    ///
    /// # Example:
    /// ```ignore
    /// collection.album("hinto", "festival").unwrap();
    /// ```
    /// In the above example, we're searching for a:
    /// - [`Album`] called `festival` by the
    /// - [`Artist`] called `hinto`
    pub(crate) fn album<S: AsRef<str>>(
        &self,
        artist_name: S,
        album_title: S,
    ) -> Option<(&Album, AlbumKey)> {
        if let Some((_key, albums)) = self.map.0.get(artist_name.as_ref()) {
            if let Some((key, _)) = albums.0.get(album_title.as_ref()) {
                return Some((&self.albums[key], *key));
            }
        }

        None
    }

    #[inline]
    /// Search [`Collection`] for a [`Song`] in an [`Album`] by an [`Artist`].
    ///
    /// # Example:
    /// ```ignore
    /// collection.song("hinto", "festival", "track_1").unwrap();
    /// ```
    /// In the above example, we're searching for a:
    /// - [`Song`] called `track_1` in an
    /// - [`Album`] called `festival` by the
    /// - [`Artist`] called `hinto`
    pub(crate) fn song<S: AsRef<str>>(
        &self,
        artist_name: S,
        album_title: S,
        song_title: S,
    ) -> Option<(&Song, Key)> {
        if let Some((artist_key, albums)) = self.map.0.get(artist_name.as_ref()) {
            if let Some((album_key, songs)) = albums.0.get(album_title.as_ref()) {
                if let Some(song_key) = songs.0.get(song_title.as_ref()) {
                    let key = Key::from_keys(*artist_key, *album_key, *song_key);
                    return Some((&self.songs[song_key], key));
                }
            }
        }

        None
    }

    //-------------------------------------------------- Indexing.
    #[inline]
    /// Directly index the [`Collection`] with a [`Key`].
    ///
    /// # Panics:
    /// The [`ArtistKey`], [`AlbumKey`] and [`SongKey`] within
    /// the [`Key`] must be valid indices into the [`Collection`].
    pub(crate) fn index<K: Into<Key>>(&self, key: K) -> (&Artist, &Album, &Song) {
        let (artist, album, song) = key.into().into_usize();
        (
            &self.artists.0[artist],
            &self.albums.0[album],
            &self.songs.0[song],
        )
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use disk::Bincode2;
    use readable::{Date, Runtime};

    // Empty new `Collection`.
    const C1: Lazy<Collection> = Lazy::new(|| {
        Collection::from_path("../assets/shukusai/state/collection0_new.bin").unwrap()
    });
    // Filled, user `Collection`.
    const C2: Lazy<Collection> = Lazy::new(|| {
        Collection::from_path("../assets/shukusai/state/collection0_real.bin").unwrap()
    });

    #[test]
    // Tests functions that depend on the correctness of the `Map`.
    fn map() {
        // Artist
        let k = ArtistKey::zero();
        assert_eq!(C2.artist("artist_1"), Some((&C2.artists[k], k)));

        // Album
        let k = AlbumKey::zero();
        assert_eq!(C2.album("artist_1", "album_1"), Some((&C2.albums[k], k)));

        // Song
        let k = SongKey::from(1_u8);
        assert_eq!(
            C2.song("artist_1", "album_1", "mp3"),
            Some((&C2.songs[k], Key::from_raw(0, 0, 1)))
        );
    }

    #[test]
    // Tests `index()`.
    fn index() {
        assert_eq!(
            C2.index(Key::zero()),
            (
                &C2.artists[ArtistKey::zero()],
                &C2.albums[AlbumKey::zero()],
                &C2.songs[SongKey::zero()]
            )
        );
    }

    #[test]
    // Compares `Collection::new()` against C1 & C2.
    fn cmp() {
        assert_eq!(Lazy::force(&C1), &Collection::new());
        assert_ne!(Lazy::force(&C1), Lazy::force(&C2));

        let b1 = C1.to_bytes().unwrap();
        let b2 = C2.to_bytes().unwrap();
        assert_ne!(b1, b2);
    }

    #[test]
    // Attempts to deserialize a non-empty `Collection`.
    fn real() {
        // Assert metadata within the `Collection`.
        assert!(!C2.empty);
        assert_eq!(C2.count_artist, 3);
        assert_eq!(C2.count_album, 4);
        assert_eq!(C2.count_song, 7);
        assert_eq!(C2.count_art, 4);
        assert_eq!(C2.timestamp, 1688690421);

        // Artist 1/3
        let k = ArtistKey::from(0_u8);
        assert_eq!(C2.artists[k].name, "artist_1");
        assert_eq!(C2.artists[k].runtime, Runtime::from(4_u8));
        assert_eq!(C2.artists[k].albums.len(), 2);
        assert_eq!(C2.artists[k].songs.len(), 4);

        // Artist 2/3
        let k = ArtistKey::from(1_u8);
        assert_eq!(C2.artists[k].name, "artist_2");
        assert_eq!(C2.artists[k].runtime, Runtime::from(2_u8));
        assert_eq!(C2.artists[k].albums.len(), 1);
        assert_eq!(C2.artists[k].songs.len(), 2);

        // Artist 3/3
        let k = ArtistKey::from(2_u8);
        assert_eq!(C2.artists[k].name, "artist_3");
        assert_eq!(C2.artists[k].runtime, Runtime::from(1_u8));
        assert_eq!(C2.artists[k].albums.len(), 1);
        assert_eq!(C2.artists[k].songs.len(), 1);

        // Albums 1/4
        let k = AlbumKey::from(0_u8);
        assert_eq!(C2.albums[k].title, "album_1");
        assert_eq!(C2.albums[k].release, Date::from_str("2018-04-25").unwrap());

        // Albums 2/4
        let k = AlbumKey::from(1_u8);
        assert_eq!(C2.albums[k].title, "album_2");
        assert_eq!(C2.albums[k].release, Date::from_str("2018-04-25").unwrap());

        // Albums 3/4
        let k = AlbumKey::from(2_u8);
        assert_eq!(C2.albums[k].title, "album_3");
        assert_eq!(C2.albums[k].release, Date::from_str("2018-04-25").unwrap());

        // Albums 4/4
        let k = AlbumKey::from(3_u8);
        assert_eq!(C2.albums[k].title, "album_4");
        assert_eq!(C2.albums[k].release, Date::from_str("2018-04-25").unwrap());

        // Song 1/7
        let k = SongKey::from(0_u8);
        assert_eq!(C2.songs[k].title, "mp3");
        assert_eq!(C2.songs[k].sample_rate, 48_000);
        assert_eq!(
            C2.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_1.mp3"
        );

        // Song 2/7
        let k = SongKey::from(1_u8);
        assert_eq!(C2.songs[k].title, "mp3");
        assert_eq!(C2.songs[k].sample_rate, 48_000);
        assert_eq!(
            C2.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_2.mp3"
        );

        // Song 3/7
        let k = SongKey::from(2_u8);
        assert_eq!(C2.songs[k].title, "mp3");
        assert_eq!(C2.songs[k].sample_rate, 48_000);
        assert_eq!(
            C2.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_3.mp3"
        );

        // Song 4/7
        let k = SongKey::from(3_u8);
        assert_eq!(C2.songs[k].title, "flac");
        assert_eq!(C2.songs[k].sample_rate, 48_000);
        assert_eq!(
            C2.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_4.flac"
        );

        // Song 5/7
        let k = SongKey::from(4_u8);
        assert_eq!(C2.songs[k].title, "m4a");
        assert_eq!(C2.songs[k].sample_rate, 48_000);
        assert_eq!(
            C2.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_5.m4a"
        );

        // Song 6/7
        let k = SongKey::from(5_u8);
        assert_eq!(C2.songs[k].title, "song_6");
        assert_eq!(C2.songs[k].sample_rate, 48_000);
        assert_eq!(
            C2.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_6.ogg"
        );

        // Song 7/7
        let k = SongKey::from(6_u8);
        assert_eq!(C2.songs[k].title, "mp3");
        assert_eq!(C2.songs[k].sample_rate, 48_000);
        assert_eq!(
            C2.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_7.mp3"
        );
    }

    #[test]
    // Asserts conversion works losslessly.
    fn convert() {
        let c: crate::collection::Collection = C2.clone().into();

        // Assert metadata within the `Collection`.
        assert!(!c.empty);
        assert_eq!(c.count_artist, 3);
        assert_eq!(c.count_album, 4);
        assert_eq!(c.count_song, 7);
        assert_eq!(c.count_art, 4);
        assert_eq!(c.timestamp, 1688690421);

        // Artist 1/3
        let k = ArtistKey::from(0_u8);
        assert_eq!(c.artists[k].name, "artist_1".into());
        assert_eq!(c.artists[k].runtime, Runtime::from(4_u8));
        assert_eq!(c.artists[k].albums.len(), 2);
        assert_eq!(c.artists[k].songs.len(), 4);

        // Artist 2/3
        let k = ArtistKey::from(1_u8);
        assert_eq!(c.artists[k].name, "artist_2".into());
        assert_eq!(c.artists[k].runtime, Runtime::from(2_u8));
        assert_eq!(c.artists[k].albums.len(), 1);
        assert_eq!(c.artists[k].songs.len(), 2);

        // Artist 3/3
        let k = ArtistKey::from(2_u8);
        assert_eq!(c.artists[k].name, "artist_3".into());
        assert_eq!(c.artists[k].runtime, Runtime::from(1_u8));
        assert_eq!(c.artists[k].albums.len(), 1);
        assert_eq!(c.artists[k].songs.len(), 1);

        // Albums 1/4
        let k = AlbumKey::from(0_u8);
        assert_eq!(c.albums[k].title, "album_1".into());
        assert_eq!(c.albums[k].release, Date::from_str("2018-04-25").unwrap());

        // Albums 2/4
        let k = AlbumKey::from(1_u8);
        assert_eq!(c.albums[k].title, "album_2".into());
        assert_eq!(c.albums[k].release, Date::from_str("2018-04-25").unwrap());

        // Albums 3/4
        let k = AlbumKey::from(2_u8);
        assert_eq!(c.albums[k].title, "album_3".into());
        assert_eq!(c.albums[k].release, Date::from_str("2018-04-25").unwrap());

        // Albums 4/4
        let k = AlbumKey::from(3_u8);
        assert_eq!(c.albums[k].title, "album_4".into());
        assert_eq!(c.albums[k].release, Date::from_str("2018-04-25").unwrap());

        // Song 1/7
        let k = SongKey::from(0_u8);
        assert_eq!(c.songs[k].title, "mp3".into());
        assert_eq!(c.songs[k].sample_rate, 48_000);
        assert_eq!(
            c.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_1.mp3"
        );

        // Song 2/7
        let k = SongKey::from(1_u8);
        assert_eq!(c.songs[k].title, "mp3".into());
        assert_eq!(c.songs[k].sample_rate, 48_000);
        assert_eq!(
            c.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_2.mp3"
        );

        // Song 3/7
        let k = SongKey::from(2_u8);
        assert_eq!(c.songs[k].title, "mp3".into());
        assert_eq!(c.songs[k].sample_rate, 48_000);
        assert_eq!(
            c.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_3.mp3"
        );

        // Song 4/7
        let k = SongKey::from(3_u8);
        assert_eq!(c.songs[k].title, "flac".into());
        assert_eq!(c.songs[k].sample_rate, 48_000);
        assert_eq!(
            c.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_4.flac"
        );

        // Song 5/7
        let k = SongKey::from(4_u8);
        assert_eq!(c.songs[k].title, "m4a".into());
        assert_eq!(c.songs[k].sample_rate, 48_000);
        assert_eq!(
            c.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_5.m4a"
        );

        // Song 6/7
        let k = SongKey::from(5_u8);
        assert_eq!(c.songs[k].title, "song_6".into());
        assert_eq!(c.songs[k].sample_rate, 48_000);
        assert_eq!(
            c.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_6.ogg"
        );

        // Song 7/7
        let k = SongKey::from(6_u8);
        assert_eq!(c.songs[k].title, "mp3".into());
        assert_eq!(c.songs[k].sample_rate, 48_000);
        assert_eq!(
            c.songs[k].path.as_os_str().to_str().unwrap(),
            "/home/main/git/festival/assets/audio/song_7.mp3"
        );
    }

    #[test]
    // Assert the memory layout is correct.
    // This must be correct or else `Bincode` won't be
    // able to decode things.
    //
    // A `cargo update` might include a change that
    // slightly changes the memory layout, which would
    // make the `Collection` decoding broken.
    //
    // We can rely on `std` to be stable, but not 3rd party crates (even my own).
    //
    // All recursive structures within `Collection` are tested here.
    fn layout() {
        use crate::collection::v0::Art;
        use crate::collection::Keychain;

        #[cfg(target_os = "linux")]
        const ALBUM_SIZE: usize = 320;
        #[cfg(target_os = "macos")]
        const ALBUM_SIZE: usize = 336;
        #[cfg(target_os = "windows")]
        const ALBUM_SIZE: usize = 344;

        #[cfg(target_os = "linux")]
        const ART_SIZE: usize = 128;
        #[cfg(target_os = "macos")]
        const ART_SIZE: usize = 144;
        #[cfg(target_os = "windows")]
        const ART_SIZE: usize = 144;

        #[cfg(target_os = "linux")]
        const SONG_SIZE: usize = 104;
        #[cfg(target_os = "macos")]
        const SONG_SIZE: usize = 104;
        #[cfg(target_os = "windows")]
        const SONG_SIZE: usize = 112;

        crate::assert_size_of! {
            // Collection
            Collection       => 976,
            Unsigned         => 48,
            Map              => 48,
            Artists          => 16,
            Albums           => 16,
            Songs            => 16,
            Box<[ArtistKey]> => 16,
            Box<[AlbumKey]>  => 16,
            Box<[SongKey]>   => 16,

            // Artist
            Artist           => 88,
            Runtime          => 24,
            Vec<AlbumKey>    => 24,

            // Album
            Album        => ALBUM_SIZE,
            Date         => 32,
            Vec<SongKey> => 24,
            Art          => ART_SIZE,

            // Song
            Song => SONG_SIZE,

            // Keys
            Key       => 24,
            Keychain  => 48,
            ArtistKey => 8,
            AlbumKey  => 8,
            SongKey   => 8
        }
    }
}
