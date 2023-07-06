//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use crate::collection::{
	album::Album,
	artist::Artist,
	song::Song,
	plural::{Artists,Albums,Songs},
	Map,
	Key,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use crate::sort::{
	ArtistSort,
	AlbumSort,
	SongSort,
};


//use disk::{Json,json_file};
use crate::constants::{
	FESTIVAL,
	HEADER,
	COLLECTION_VERSION,
	FRONTEND_SUB_DIR,
	STATE_SUB_DIR,
};
use rand::{
	Rng,
	SeedableRng,
	prelude::SliceRandom,
};
use std::sync::{Arc,Mutex};
use benri::{
	lock,
};
use readable::{
	Unsigned,
};
use once_cell::sync::Lazy;
use std::marker::PhantomData;
use const_format::formatcp;

//---------------------------------------------------------------------------------------------------- Lazy
// `RNG`: Global RNG state for `Collection`'s `rand_*` functions.
static RNG: Lazy<Mutex<rand::rngs::SmallRng>> = Lazy::new(|| Mutex::new(rand::rngs::SmallRng::from_entropy()));

// This is an empty, dummy `Collection`.
pub(crate) static DUMMY_COLLECTION: Lazy<Arc<Collection>> = Lazy::new(|| Arc::new(Collection::new()));

//---------------------------------------------------------------------------------------------------- Collection
disk::bincode2!(Collection, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/{STATE_SUB_DIR}"), "collection", HEADER, COLLECTION_VERSION);
#[derive(Clone,Debug,PartialEq,Encode,Decode)]
/// The main music `Collection`
///
/// This is the `struct` that holds all the (meta)data about the user's music.
///
/// This holds:
/// - The "3 Slices", holding _all_ [`Artist`]'s, [`Album`]'s, and [`Song`]'s.
/// - The "Map", a searchable [`HashMap`]
/// - Pre-computed, sorted keys
/// - Metadata about the [`Collection`] itself
///
/// ### Index
/// The fastest & preferred way to access the [`Collection`] is via indexing.
///
/// To properly index the [`Collection`], for example, an [`Album`], you can use the `[]` operators, however,
/// it must be type-safe. It _cannot_ be a [`usize`], it must be the proper type of [`Key`].
///
/// Example:
/// ```rust,ignore
/// let my_usize = 0;
/// let key = AlbumKey::from(my_usize);
///
/// // NOT type-safe, compile error!.
/// collection.albums[my_usize];
///
/// // Type-safe, compiles.
/// collection.albums[key];
/// ```
///
/// ### Search
/// Instead of directly indexing the [`Collection`], you can opt to use:
/// - [`Collection::artist`]
/// - [`Collection::album`]
/// - [`Collection::song`]
///
/// These three functions are akin to a [`HashMap::get`] and accept arbitrary
/// [`str`] input instead of raw indices or a [`Key`]. This is more flexible
/// but obviously is much slower if you already know the proper index.
///
/// A [`Song`] or [`Album`] cannot be directly searched without an [`Artist`]
/// due to collisions. For example, what should be returned given a [`Collection`] like this?
/// - Artist_A | Album_C | Song_E
/// - Artist_B | Album_D | Song_E
/// - Artist_C | Album_D | Song_E
///
/// There's a collision of both [`Album`]'s AND [`Song`]'s.
///
/// [`Artist`]'s will always be unique, so it acts as an identifier.
///
/// ### Sort
/// The "3 Slices" are (basically) in random order due to how `Collection` is created.
///
/// Iterating directly on them is not very useful, so use the pre-calculated sorted keys.
///
/// The sorted key fields all start with `sort_`.
///
/// ### Frontend
/// As a `Frontend`, you will never produce a [`Collection`] yourself, rather,
/// you ask [`Kernel`] to produce one for you. You will receive an immutable `Arc<Collection>`.
///
/// ### Late initialization
/// Waiting on [`Kernel`] to hand you the _real_ [`Collection`] may take a while.
///
/// This prevents your `Frontend` from finishing initializing beforehand, so instead, use [`Collection::dummy`]
/// to _cheaply_ obtain an empty, dummy [`Collection`], then wait for [`Kernel`]'s signal later on.
pub struct Collection {
	// Metadata about the `Collection` itself.
	/// Is this [`Collection`] empty?
	///
	/// Meaning, are there absolutely no [`Artist`]'s, [`Album`]'s and [`Song`]'s?
	pub empty: bool,
	/// UNIX timestamp of the [`Collection`]'s creation date.
	pub timestamp: u64,
	/// How many [`Artist`]'s in this [`Collection`]?
	pub count_artist: Unsigned,
	/// How many [`Album`]'s in this [`Collection`]?
	pub count_album: Unsigned,
	/// How many [`Song`]'s in this [`Collection`]?
	pub count_song: Unsigned,
	/// How many unique [`Album`] covers are there in this [`Collection`]?
	pub count_art: Unsigned,

	// The "Map".
	/// A [`HashMap`] that knows all [`Artist`]'s, [`Album`]'s and [`Song`]'s.
	pub map: Map,

	// The "3 arrays".
	/// All the [`Artist`]'s in mostly random order.
	pub artists: Artists,
	/// All the [`Album`]'s in mostly random order.
	pub albums: Albums,
	/// All the [`Song`]'s in mostly random order.
	pub songs: Songs,

	// Sorted `Artist` keys.
	/// [`Artist`] A-Z.
	pub sort_artist_lexi: Box<[ArtistKey]>,
	/// [`Artist`] Z-A.
	pub sort_artist_lexi_rev: Box<[ArtistKey]>,
	/// [`Artist`] with most [`Album`]'s to least.
	pub sort_artist_album_count: Box<[ArtistKey]>,
	/// [`Artist`] with least [`Album`]'s to most.
	pub sort_artist_album_count_rev: Box<[ArtistKey]>,
	/// [`Artist`] with most [`Song`]'s to least.
	pub sort_artist_song_count: Box<[ArtistKey]>,
	/// [`Artist`] with least [`Song`]'s to most.
	pub sort_artist_song_count_rev: Box<[ArtistKey]>,
	/// [`Artist`] runtime least-most.
	pub sort_artist_runtime: Box<[ArtistKey]>,
	/// [`Artist`] runtime most-least.
	pub sort_artist_runtime_rev: Box<[ArtistKey]>,
	/// [`Artist`] name shortest-longest.
	pub sort_artist_name: Box<[ArtistKey]>,
	/// [`Artist`] name longest-shortest
	pub sort_artist_name_rev: Box<[ArtistKey]>,

	// Sorted `Album` keys.
	/// [`Artist`] A-Z, [`Album`] oldest-latest.
	pub sort_album_release_artist_lexi: Box<[AlbumKey]>,
	/// [`Artist`] Z-A, [`Album`] oldest-latest.
	pub sort_album_release_artist_lexi_rev: Box<[AlbumKey]>,
	/// [`Artist`] A-Z, [`Album`] latest-oldest.
	pub sort_album_release_rev_artist_lexi: Box<[AlbumKey]>,
	/// [`Artist`] Z-A, [`Album`] latest-oldest.
	pub sort_album_release_rev_artist_lexi_rev: Box<[AlbumKey]>,
	/// [`Artist`] A-Z, [`Album`] A-Z.
	pub sort_album_lexi_artist_lexi: Box<[AlbumKey]>,
	/// [`Artist`] Z-A, [`Album`] A-Z.
	pub sort_album_lexi_artist_lexi_rev: Box<[AlbumKey]>,
	/// [`Artist`] A-Z, [`Album`] Z-A.
	pub sort_album_lexi_rev_artist_lexi: Box<[AlbumKey]>,
	/// [`Artist`] Z-A, [`Album`] Z-A.
	pub sort_album_lexi_rev_artist_lexi_rev: Box<[AlbumKey]>,
	/// [`Album`] A-Z.
	pub sort_album_lexi: Box<[AlbumKey]>,
	/// [`Album`] Z-A.
	pub sort_album_lexi_rev: Box<[AlbumKey]>,
	/// [`Album`] oldest to latest.
	pub sort_album_release: Box<[AlbumKey]>,
	/// [`Album`] latest to oldest.
	pub sort_album_release_rev: Box<[AlbumKey]>,
	/// [`Album`] shortest to longest.
	pub sort_album_runtime: Box<[AlbumKey]>,
	/// [`Album`] longest to shortest.
	pub sort_album_runtime_rev: Box<[AlbumKey]>,
	/// [`Album`] title shortest to longest.
	pub sort_album_title: Box<[AlbumKey]>,
	/// [`Album`] title longest to shortest.
	pub sort_album_title_rev: Box<[AlbumKey]>,

	// Sorted `Song` keys.
	/// [`Artist`] A-Z, [`Album`] oldest-latest, [`Song`] track_number
	pub sort_song_album_release_artist_lexi: Box<[SongKey]>,
	/// [`Artist`] Z-A, [`Album`] oldest-latest, [`Song`] track_number
	pub sort_song_album_release_artist_lexi_rev: Box<[SongKey]>,
	/// [`Artist`] A-Z, [`Album`] latest-oldest, [`Song`] track_number
	pub sort_song_album_release_rev_artist_lexi: Box<[SongKey]>,
	/// [`Artist`] Z-A, [`Album`] latest-oldest, [`Song`] track_number
	pub sort_song_album_release_rev_artist_lexi_rev: Box<[SongKey]>,
	/// [`Artist`] A-Z, [`Album`] A-Z, [`Song`] track_number.
	pub sort_song_album_lexi_artist_lexi: Box<[SongKey]>,
	/// [`Artist`] Z-A, [`Album`] A-Z, [`Song`] track_number.
	pub sort_song_album_lexi_artist_lexi_rev: Box<[SongKey]>,
	/// [`Artist`] A-Z, [`Album`] Z-A, [`Song`] track_number.
	pub sort_song_album_lexi_rev_artist_lexi: Box<[SongKey]>,
	/// [`Artist`] Z-A, [`Album`] Z-A, [`Song`] track_number.
	pub sort_song_album_lexi_rev_artist_lexi_rev: Box<[SongKey]>,
	/// [`Song`] A-Z.
	pub sort_song_lexi: Box<[SongKey]>,
	/// [`Song`] Z-A.
	pub sort_song_lexi_rev: Box<[SongKey]>,
	/// [`Song`] oldest to latest.
	pub sort_song_release: Box<[SongKey]>,
	/// [`Song`] latest to oldest.
	pub sort_song_release_rev: Box<[SongKey]>,
	/// [`Song`] shortest to longest.
	pub sort_song_runtime: Box<[SongKey]>,
	/// [`Song`] longest to shortest.
	pub sort_song_runtime_rev: Box<[SongKey]>,
	/// [`Song`] title shortest to longest.
	pub sort_song_title: Box<[SongKey]>,
	/// [`Song`] title longest to shortest.
	pub sort_song_title_rev: Box<[SongKey]>,

	// Reserved fields and their `size_of()`.

	// SOMEDAY:
	// These will probably be `sort_*` but
	// direct pointers instead of indices.
	pub(crate) _reserved1: PhantomData<Box<[usize]>>, // 16
	pub(crate) _reserved2: PhantomData<Box<[usize]>>,
	pub(crate) _reserved4: PhantomData<Box<[usize]>>,
	pub(crate) _reserved5: PhantomData<Box<[usize]>>,
	pub(crate) _reserved6: PhantomData<Box<[usize]>>,
	pub(crate) _reserved7: PhantomData<Box<[usize]>>,
	pub(crate) _reserved8: PhantomData<Box<[usize]>>,
	pub(crate) _reserved9: PhantomData<Box<[usize]>>,
	pub(crate) _reserved10: PhantomData<Box<[usize]>>,
	pub(crate) _reserved11: PhantomData<Box<[usize]>>,
	pub(crate) _reserved12: PhantomData<Box<[usize]>>,
	pub(crate) _reserved13: PhantomData<Box<[usize]>>,
	pub(crate) _reserved14: PhantomData<Box<[usize]>>,
	pub(crate) _reserved15: PhantomData<Box<[usize]>>,
	pub(crate) _reserved16: PhantomData<Box<[usize]>>,
	pub(crate) _reserved17: PhantomData<Box<[usize]>>,
	pub(crate) _reserved18: PhantomData<Box<[usize]>>,
	pub(crate) _reserved19: PhantomData<Box<[usize]>>,
	pub(crate) _reserved20: PhantomData<Box<[usize]>>,
	pub(crate) _reserved21: PhantomData<Box<[usize]>>,
	pub(crate) _reserved22: PhantomData<Box<[usize]>>,
	pub(crate) _reserved23: PhantomData<Box<[usize]>>,
	pub(crate) _reserved24: PhantomData<Box<[usize]>>,
	pub(crate) _reserved25: PhantomData<Box<[usize]>>,
	pub(crate) _reserved26: PhantomData<Box<[usize]>>,
	pub(crate) _reserved27: PhantomData<Box<[usize]>>,
	pub(crate) _reserved28: PhantomData<Box<[usize]>>,
	pub(crate) _reserved29: PhantomData<Box<[usize]>>,
	pub(crate) _reserved30: PhantomData<Box<[usize]>>,
	pub(crate) _reserved31: PhantomData<Box<[usize]>>,
	pub(crate) _reserved32: PhantomData<Box<[usize]>>,
	pub(crate) _reserved33: PhantomData<Box<[usize]>>,
	pub(crate) _reserved34: PhantomData<Box<[usize]>>,
	pub(crate) _reserved35: PhantomData<Box<[usize]>>,
	pub(crate) _reserved36: PhantomData<Box<[usize]>>,
	pub(crate) _reserved37: PhantomData<Box<[usize]>>,
	pub(crate) _reserved38: PhantomData<Box<[usize]>>,
	pub(crate) _reserved39: PhantomData<Box<[usize]>>,
	pub(crate) _reserved40: PhantomData<Box<[usize]>>,
	pub(crate) _reserved41: PhantomData<Box<[usize]>>,
	pub(crate) _reserved42: PhantomData<Box<[usize]>>,

	// Misc reserved fields.
	pub(crate) _reserved43: PhantomData<String>,    // 24
	pub(crate) _reserved44: PhantomData<Box<[u8]>>, // 16
	pub(crate) _reserved45: PhantomData<usize>,     // 8
	pub(crate) _reserved46: PhantomData<usize>,
	pub(crate) _reserved47: PhantomData<usize>,
	pub(crate) _reserved48: PhantomData<usize>,
	pub(crate) _reserved49: PhantomData<bool>, // 1
	pub(crate) _reserved50: PhantomData<bool>,
}

impl Collection {
	//-------------------------------------------------- New.
	/// Creates an empty [`Collection`].
	pub fn new() -> Self {
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

			// We don't use `..Default::default()` because
			// we want to _explicit_ about the values here.
			_reserved1: PhantomData, _reserved2: PhantomData, _reserved4: PhantomData, _reserved5: PhantomData,
			_reserved6: PhantomData, _reserved7: PhantomData, _reserved8: PhantomData, _reserved9: PhantomData,
			_reserved10: PhantomData, _reserved11: PhantomData, _reserved12: PhantomData, _reserved13: PhantomData,
			_reserved14: PhantomData, _reserved15: PhantomData, _reserved16: PhantomData, _reserved17: PhantomData,
			_reserved18: PhantomData, _reserved19: PhantomData, _reserved20: PhantomData, _reserved21: PhantomData,
			_reserved22: PhantomData, _reserved23: PhantomData, _reserved24: PhantomData, _reserved25: PhantomData,
			_reserved26: PhantomData, _reserved27: PhantomData, _reserved28: PhantomData, _reserved29: PhantomData,
			_reserved30: PhantomData, _reserved31: PhantomData, _reserved32: PhantomData, _reserved33: PhantomData,
			_reserved34: PhantomData, _reserved35: PhantomData, _reserved36: PhantomData, _reserved37: PhantomData,
			_reserved38: PhantomData, _reserved39: PhantomData, _reserved40: PhantomData, _reserved41: PhantomData,
			_reserved42: PhantomData, _reserved43: PhantomData, _reserved44: PhantomData, _reserved45: PhantomData,
			_reserved46: PhantomData, _reserved47: PhantomData, _reserved48: PhantomData, _reserved49: PhantomData,
			_reserved50: PhantomData,
		}
	}

	#[inline(always)]
	/// Obtain an empty, dummy [`Collection`] wrapped in an [`Arc`].
	///
	/// This is useful when you need to initialize but don't want
	/// to wait on [`Kernel`] to hand you the _real_ `Arc<Collection>`.
	///
	/// Details on the fields:
	/// - All [`Vec`]'s are empty
	/// - All search functions will return [`Option::None`]
	/// - The `timestamp` and `count_*` fields are set to `0`
	/// - `empty` is set to `true`
	///
	/// This [`Collection`] is [`Arc::clone`]'ed from a lazily
	/// evaluated, empty [`Collection`] that has static lifetime.
	pub fn dummy() -> Arc<Self> {
		Arc::clone(&DUMMY_COLLECTION)
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
	pub fn artist<S: AsRef<str>>(&self, artist_name: S) -> Option<(&Artist, ArtistKey)> {
		if let Some((key, _)) = self.map.0.get(artist_name.as_ref()) {
			return Some((&self.artists[key], *key))
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
	pub fn album<S: AsRef<str>>(
		&self,
		artist_name: S,
		album_title: S,
	) -> Option<(&Album, AlbumKey)> {
		if let Some((_key, albums)) = self.map.0.get(artist_name.as_ref()) {
			if let Some((key, _)) = albums.0.get(album_title.as_ref()) {
				return Some((&self.albums[key], *key))
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
	pub fn song<S: AsRef<str>>(
		&self,
		artist_name: S,
		album_title: S,
		song_title: S,
	) -> Option<(&Song, Key)> {
		if let Some((artist_key, albums)) = self.map.0.get(artist_name.as_ref()) {
			if let Some((album_key, songs)) = albums.0.get(album_title.as_ref()) {
				if let Some(song_key) = songs.0.get(song_title.as_ref()) {
					let key = Key::from_keys(*artist_key, *album_key, *song_key);
					return Some((&self.songs[song_key], key))
				}
			}
		}

		None
	}

	//-------------------------------------------------- Bulk.
	/// Returns an iterator that starts from the input [`Song`]
	/// and includes every [`Song`] after that one.
	///
	/// e.g: If we input `song_1` to an array of [`song_0`, `song_1`, `song_2`, `song_3`]
	/// it would return an iterator that starts at `song_1` and ends at `song_3`.
	///
	/// This is useful for starting a queue including songs of an [`Album`]
	/// but not necessarily starting from the first [`Song`].
	pub fn song_tail<K: Into<SongKey>>(&self, key: K) -> std::iter::Peekable<std::slice::Iter<'_, SongKey>> {
		let key = key.into();
		let (album, _) = self.album_from_song(key);
		let mut iter = album.songs.iter().peekable();

		// The input `SongKey` should _always_ be found
		// in the owning `Album`'s `song` field.
		while let Some(song) = iter.peek() {
			if key == *song {
				return iter;
			}

			iter.next();
		}

		panic!("{key:?} did not exist in the album {album:#?}");
	}

	//-------------------------------------------------- Indexing.
	#[inline]
	/// Directly index the [`Collection`] with a [`Key`].
	///
	/// # Panics:
	/// The [`ArtistKey`], [`AlbumKey`] and [`SongKey`] within
	/// the [`Key`] must be valid indices into the [`Collection`].
	pub fn index<K: Into<Key>>(&self, key: K) -> (&Artist, &Album, &Song) {
		let (artist, album, song) = key.into().into_usize();
		(&self.artists.0[artist], &self.albums.0[album], &self.songs.0[song])
	}

	/// Walk through the relational data from a
	/// [`SongKey`] and return the full tuple.
	#[inline]
	pub fn walk<K: Into<SongKey>>(&self, key: K) -> (&Artist, &Album, &Song) {
		let song   = &self.songs[key.into()];
		let album  = &self.albums[song.album];
		let artist = &self.artists[album.artist];

		(artist, album, song)
	}

	/// Get all [`Album`]'s from the same [`Artist`] of this [`AlbumKey`].
	#[inline]
	pub fn other_albums<K: Into<AlbumKey>>(&self, key: K) -> &[AlbumKey] {
		&self.artists[self.albums[key.into()].artist].albums
	}

	/// Get all [`Song`]'s from the same [`Album`] of this [`SongKey`].
	#[inline]
	pub fn other_songs<K: Into<SongKey>>(&self, key: K) -> &[SongKey] {
		&self.albums[self.songs[key.into()].album].songs
	}

	/// Get all the [`SongKey`]'s belonging to this [`ArtistKey`].
	#[inline]
	pub fn all_songs<K: Into<ArtistKey>>(&self, key: K) -> Box<[SongKey]> {
		self.artists[key.into()].albums
			.iter()
			.flat_map(|a| self.albums[a].songs.iter())
			.copied()
			.collect()
	}

	/// Get the next [`Album`] belonging to this [`Artist`].
	///
	/// This:
	/// - Iterates via release date
	/// - Wraps around if at the last element
	#[inline]
	pub fn next_album<K: Into<AlbumKey>>(&self, key: K) -> AlbumKey {
		let key = key.into();
		let other_albums = self.other_albums(key);

		let index = other_albums
			.iter()
			.position(|i| i == key)
			.unwrap_or(0);

		if let Some(key) = other_albums.get(index + 1) {
			*key
		} else {
			other_albums[0]
		}
	}

	/// Get the previous [`Album`] belonging to this [`Artist`].
	///
	/// This:
	/// - Iterates via release date
	/// - Wraps around if at the first element
	#[inline]
	pub fn previous_album<K: Into<AlbumKey>>(&self, key: K) -> AlbumKey {
		let key          = key.into();
		let other_albums = self.other_albums(key);
		let len          = other_albums.len();

		let index = other_albums
			.iter()
			.position(|i| i == key)
			.unwrap_or(0);

		if let Some(index) = index.checked_sub(1) {
			other_albums[index]
		} else {
			other_albums[len - 1]
		}
	}

	/// Get the next [`Song`] belonging to this [`Album`].
	///
	/// This:
	/// - Iterates via track order
	/// - Wraps around if at the last element
	#[inline]
	pub fn next_song<K: Into<SongKey>>(&self, key: K) -> SongKey {
		let key = key.into();
		let other_songs = self.other_songs(key);

		let index = other_songs
			.iter()
			.position(|i| i == key)
			.unwrap_or(0);

		if let Some(key) = other_songs.get(index + 1) {
			*key
		} else {
			other_songs[0]
		}
	}

	/// Get the previous [`Song`] belonging to this [`Album`].
	///
	/// This:
	/// - Iterates via track order
	/// - Wraps around if at the first element
	#[inline]
	pub fn previous_song<K: Into<SongKey>>(&self, key: K) -> SongKey {
		let key         = key.into();
		let other_songs = self.other_songs(key);
		let len         = other_songs.len();

		let index = other_songs
			.iter()
			.position(|i| i == key)
			.unwrap_or(0);

		if let Some(index) = index.checked_sub(1) {
			other_songs[index]
		} else {
			other_songs[len - 1]
		}
	}

	#[inline]
	/// [`slice::get`] the [`Collection`] with a [`Key`].
	///
	/// # Errors:
	/// The [`ArtistKey`], [`AlbumKey`] and [`SongKey`] within
	/// the [`Key`] must be valid indices into the [`Collection`].
	pub fn get<K: Into<Key>>(&self, key: K) -> Option<(&Artist, &Album, &Song)> {
		let (artist, album, song) = key.into().into_usize();

		let artist = match self.artists.0.get(artist) {
			Some(a) => a,
			None    => return None,
		};

		let album = match self.albums.0.get(album) {
			Some(a) => a,
			None    => return None,
		};

		let song = match self.songs.0.get(song) {
			Some(a) => a,
			None    => return None,
		};

		Some((artist, album, song))
	}

	//-------------------------------------------------- Key traversal (index).
	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`AlbumKey`].
	///
	/// # Panics:
	/// The [`AlbumKey`] must be a valid index.
	pub fn artist_from_album<K: Into<AlbumKey>>(&self, key: K) -> (&Artist, ArtistKey) {
		let album = &self.albums[key.into()];
		(&self.artists[album.artist], album.artist)
	}

	#[inline(always)]
	/// Obtain an [`Album`], but from a [`SongKey`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn album_from_song<K: Into<SongKey>>(&self, key: K) -> (&Album, AlbumKey) {
		let song = &self.songs[key.into()];
		(&self.albums[song.album], song.album)
	}

	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`SongKey`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn artist_from_song<K: Into<SongKey>>(&self, key: K) -> (&Artist, ArtistKey) {
		self.artist_from_album(self.songs[key.into()].album)
	}

	//-------------------------------------------------- Key traversal (`.get()`).
	#[inline]
	/// Obtain an [`Artist`], but from a [`AlbumKey`].
	///
	/// # Errors:
	/// The [`AlbumKey`] must be a valid index.
	pub fn get_artist_from_album<K: Into<AlbumKey>>(&self, key: K) -> Option<&Artist> {
		let artist = match self.albums.get(key.into()) {
			Some(a) => a.artist,
			None    => return None,
		};

		self.artists.get(artist)
	}

	#[inline]
	/// Obtain an [`Album`], but from a [`SongKey`].
	///
	/// # Errors:
	/// The [`SongKey`] must be a valid index.
	pub fn get_album_from_song<K: Into<SongKey>>(&self, key: K) -> Option<&Album> {
		let album = match self.songs.get(key.into()) {
			Some(a) => a.album,
			None    => return None,
		};

		self.albums.get(album)
	}

	#[inline]
	/// Obtain an [`Artist`], but from a [`SongKey`].
	///
	/// # Errors:
	/// The [`SongKey`] must be a valid index.
	pub fn get_artist_from_song<K: Into<SongKey>>(&self, key: K) -> Option<&Artist> {
		let album = match self.songs.get(key.into()) {
			Some(a) => a.album,
			None    => return None,
		};

		self.get_artist_from_album(album)
	}

	//-------------------------------------------------- Sorting
	/// Access `sort_artist` fields in the [`Collection`] as an iterator via a [`ArtistSort`].
	pub fn artist_iter(&self, sort: ArtistSort) -> std::slice::Iter<'_, ArtistKey> {
		use ArtistSort::*;
		match sort {
			Lexi          => &self.sort_artist_lexi,
			LexiRev       => &self.sort_artist_lexi_rev,
			AlbumCount    => &self.sort_artist_album_count,
			AlbumCountRev => &self.sort_artist_album_count_rev,
			SongCount     => &self.sort_artist_song_count,
			SongCountRev  => &self.sort_artist_song_count_rev,
			Runtime       => &self.sort_artist_runtime,
			RuntimeRev    => &self.sort_artist_runtime_rev,
			Name          => &self.sort_artist_name,
			NameRev       => &self.sort_artist_name_rev,
		}.iter()
	}

	/// Access `sort_album` fields in the [`Collection`] as an iterator via a [`AlbumSort`].
	pub fn album_iter(&self, sort: AlbumSort) -> std::slice::Iter<'_, AlbumKey> {
		use AlbumSort::*;
		match sort {
			ReleaseArtistLexi       => &self.sort_album_release_artist_lexi,
			ReleaseArtistLexiRev    => &self.sort_album_release_artist_lexi_rev,
			ReleaseRevArtistLexi    => &self.sort_album_release_rev_artist_lexi,
			ReleaseRevArtistLexiRev => &self.sort_album_release_rev_artist_lexi_rev,
			LexiArtistLexi          => &self.sort_album_lexi_artist_lexi,
			LexiArtistLexiRev       => &self.sort_album_lexi_artist_lexi_rev,
			LexiRevArtistLexi       => &self.sort_album_lexi_rev_artist_lexi,
			LexiRevArtistLexiRev    => &self.sort_album_lexi_rev_artist_lexi_rev,
			Lexi                    => &self.sort_album_lexi,
			LexiRev                 => &self.sort_album_lexi_rev,
			Release                 => &self.sort_album_release,
			ReleaseRev              => &self.sort_album_release_rev,
			Runtime                 => &self.sort_album_runtime,
			RuntimeRev              => &self.sort_album_runtime_rev,
			Title                   => &self.sort_album_title,
			TitleRev                => &self.sort_album_title_rev,
		}.iter()
	}

	/// Access `sort_song` fields in the [`Collection`] as an iterator via a [`SongSort`].
	pub fn song_iter(&self, sort: SongSort) -> std::slice::Iter<'_, SongKey> {
		use SongSort::*;
		match sort {
			AlbumReleaseArtistLexi       => &self.sort_song_album_release_artist_lexi,
			AlbumReleaseArtistLexiRev    => &self.sort_song_album_release_artist_lexi_rev,
			AlbumReleaseRevArtistLexi    => &self.sort_song_album_release_rev_artist_lexi,
			AlbumReleaseRevArtistLexiRev => &self.sort_song_album_release_rev_artist_lexi_rev,
			AlbumLexiArtistLexi          => &self.sort_song_album_lexi_artist_lexi,
			AlbumLexiArtistLexiRev       => &self.sort_song_album_lexi_artist_lexi_rev,
			AlbumLexiRevArtistLexi       => &self.sort_song_album_lexi_rev_artist_lexi,
			AlbumLexiRevArtistLexiRev    => &self.sort_song_album_lexi_rev_artist_lexi_rev,
			Lexi                         => &self.sort_song_lexi,
			LexiRev                      => &self.sort_song_lexi_rev,
			Release                      => &self.sort_song_release,
			ReleaseRev                   => &self.sort_song_release_rev,
			Runtime                      => &self.sort_song_runtime,
			RuntimeRev                   => &self.sort_song_runtime_rev,
			Title                        => &self.sort_song_title,
			TitleRev                     => &self.sort_song_title_rev,
		}.iter()
	}

	//-------------------------------------------------- Random
	/// Get a random _valid_ [`ArtistKey`].
	///
	/// If you provide a `Some<ArtistKey>`, this function
	/// will _not_ return that same [`ArtistKey`].
	///
	/// # Notes
	/// - If there is only 1 [`ArtistKey`], `ArtistKey(0)` will always be returned.
	/// - If there are _no_ [`Artist`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_artist(&self, key: Option<ArtistKey>) -> Option<ArtistKey> {
		match self.count_artist.usize() {
			0 => return None,
			1 => return Some(ArtistKey::zero()),
			_ => (),
		}

		if let Some(key) = key {
			loop {
				let rand_usize: usize = lock!(RNG).gen_range(0..self.count_artist.usize());
				if rand_usize != key {
					return Some(ArtistKey::from(rand_usize))
				}
			}
		}

		let rand_usize: usize = lock!(RNG).gen_range(0..self.count_artist.usize());
		Some(ArtistKey::from(rand_usize))
	}

	/// Get a random _valid_ [`AlbumKey`].
	///
	/// If you provide a `Some<AlbumKey>`, this function
	/// will _not_ return that same [`AlbumKey`].
	///
	/// # Notes
	/// - If there is only 1 [`AlbumKey`], `AlbumKey(0)` will always be returned.
	/// - If there are _no_ [`Album`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_album(&self, key: Option<AlbumKey>) -> Option<AlbumKey> {
		match self.count_album.usize() {
			0 => return None,
			1 => return Some(AlbumKey::zero()),
			_ => (),
		}

		if let Some(key) = key {
			loop {
				let rand_usize: usize = lock!(RNG).gen_range(0..self.count_album.usize());
				if rand_usize != key {
					return Some(AlbumKey::from(rand_usize))
				}
			}
		}

		let rand_usize: usize = lock!(RNG).gen_range(0..self.count_album.usize());
		Some(AlbumKey::from(rand_usize))
	}

	/// Get a random _valid_ [`SongKey`].
	///
	/// If you provide a `Some<SongKey>`, this function
	/// will _not_ return that same [`SongKey`].
	///
	/// # Notes
	/// - If there is only 1 [`SongKey`], `SongKey(0)` will always be returned.
	/// - If there are _no_ [`Song`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_song(&self, key: Option<SongKey>) -> Option<SongKey> {
		match self.count_song.usize() {
			0 => return None,
			1 => return Some(SongKey::zero()),
			_ => (),
		}

		if let Some(key) = key {
			loop {
				let rand_usize: usize = lock!(RNG).gen_range(0..self.count_song.usize());
				if rand_usize != key {
					return Some(SongKey::from(rand_usize))
				}
			}
		}

		let rand_usize: usize = lock!(RNG).gen_range(0..self.count_song.usize());
		Some(SongKey::from(rand_usize))
	}

	/// Get a [`Vec`] of random _valid_ [`ArtistKey`]'s.
	///
	/// The length of the returned [`Vec`] will match [`Collection::count_artist`]'s length.
	///
	/// # Notes
	/// - If there are _no_ [`Artist`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_artists(&self) -> Option<Box<[ArtistKey]>> {
		match self.count_artist.usize() {
			0 => None,
			1 => Some(Box::new([ArtistKey::zero()])),
			_ => {
				let mut vec: Box<[ArtistKey]> = (0..self.count_artist.usize())
					.map(ArtistKey::from)
					.collect();
				vec.shuffle(&mut *lock!(RNG));
				Some(vec)
			}
		}
	}

	/// Get a [`Vec`] of random _valid_ [`AlbumKey`]'s.
	///
	/// The length of the returned [`Vec`] will match [`Collection::count_album`]'s length.
	///
	/// # Notes
	/// - If there are _no_ [`Album`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_albums(&self) -> Option<Box<[AlbumKey]>> {
		match self.count_album.usize() {
			0 => None,
			1 => Some(Box::new([AlbumKey::zero()])),
			_ => {
				let mut vec: Box<[AlbumKey]> = (0..self.count_album.usize())
					.map(AlbumKey::from)
					.collect();
				vec.shuffle(&mut *lock!(RNG));
				Some(vec)
			}
		}
	}

	/// Get a [`Vec`] of random _valid_ [`SongKey`]'s.
	///
	/// The length of the returned [`Vec`] will match [`Collection::count_song`]'s length.
	///
	/// # Notes
	/// - If there are _no_ [`Song`]'s in the [`Collection`], [`Option::None`] is returned.
	pub fn rand_songs(&self) -> Option<Box<[SongKey]>> {
		match self.count_song.usize() {
			0 => None,
			1 => Some(Box::new([SongKey::zero()])),
			_ => {
				let mut vec: Box<[SongKey]> = (0..self.count_song.usize())
					.map(SongKey::from)
					.collect();
				vec.shuffle(&mut *lock!(RNG));
				Some(vec)
			}
		}
	}

	/// Format this [`Collection`]'s core metadata in JSON format.
	///
	/// This output is not meant to be relied on (yet).
	///
	/// It it mostly for quick displaying and debugging
	/// purposes and may be changed at any time.
	///
	/// If you're reading the file directly via [`Collection::from_file()`],
	/// you will have some extra metadata, like byte count and absolute path.
	///
	/// You can pass this in the optional arguments
	/// and it will be added to the JSON output.
	pub(crate) fn json(
		&self,
		path:    Option<std::path::PathBuf>,
		bytes:   Option<u64>,
		header:  Option<String>,
		version: Option<u8>,
	) -> String {
		let path = match path {
			Some(p) => format!("\n        \"path\": \"{}\",", p.display()),
			_ => String::new(),
		};
		let bytes = match bytes {
			Some(b) => format!("\n        \"bytes\": {b},"),
			_ => String::new(),
		};
		let header = match header {
			Some(h) => format!("\n        \"header\": \"{h}\","),
			_ => String::new(),
		};
		let version = match version {
			Some(v) => format!("\n        \"version\": {v},"),
			_ => String::new(),
		};

		// Due to formatting, the indentation is gonna get weird.

//--- Base string.
let mut s = format!(
r#"{{
    "metadata": {{{path}{bytes}{header}{version}
        "empty": {},
        "timestamp": {},
        "artists": {},
        "albums": {},
        "songs": {},
        "art": {}
    }},
"#,
self.empty,
self.timestamp,
self.count_artist.inner(),
self.count_album.inner(),
self.count_song.inner(),
self.count_art.inner(),
);

//--- Artists.
		s += r#"    "artists": ["#;
		let mut iter = self.artists.iter().peekable();
		let mut key  = 0;
		while let Some(a) = iter.next() {
			let comma = if iter.peek().is_none() { "\n" } else { "," };
			s +=
&format!(
r#"
        {{
            "key": {key},
            "name": "{}",
            "runtime": {},
            "albums": {},
            "songs": {}
        }}{comma}"#,
a.name.replace('\"', "\\\""),
a.runtime.inner(),
a.albums.len(),
a.songs.len(),
);
			key += 1;
		}
		s += "    ],\n";

//--- Albums.
		s += r#"    "albums": ["#;
		let mut iter = self.albums.iter().peekable();
		let mut key  = 0;
		while let Some(a) = iter.next() {
			let comma = if iter.peek().is_none() { "\n" } else { "," };
			s +=
&format!(
r#"
        {{
            "key": {key},
            "title": "{}",
            "artist": {},
            "release": "{}",
            "runtime": {},
            "songs": {},
            "discs": {},
            "path": "{}",
            "art": "{:?}"
        }}{comma}"#,
a.title.replace('\"', "\\\""),
a.artist.inner(),
a.release,
a.runtime.inner(),
a.songs.len(),
a.discs,
a.path.display(),
a.art,
);
			key += 1;
		}
		s += "    ],\n";

//--- Songs.
		s += r#"    "songs": ["#;
		let mut iter = self.songs.iter().peekable();
		let mut key  = 0;
		while let Some(a) = iter.next() {
			let comma = if iter.peek().is_none() { "\n" } else { "," };
			let track = if let Some(t) = a.track { t.to_string() } else { "null".to_string() };
			let disc  = if let Some(d) = a.disc  { d.to_string() } else { "null".to_string() };
			s +=
&format!(
r#"
        {{
            "key": {key},
            "title": "{}",
            "album": {},
            "runtime": {},
            "sample_rate": {},
            "track": {track},
            "disc": {disc},
            "path": "{}"
        }}{comma}"#,
a.title.replace('\"', "\\\""),
a.album.inner(),
a.runtime.inner(),
a.sample_rate,
a.path.display(),
);
			key += 1;
		}
		s += "    ]\n}";

		s
	}
}

//---------------------------------------------------------------------------------------------------- Display
impl std::fmt::Display for Collection {
	/// Displays the [`Collection`] in a _slightly_ more human readable way.
	///
	/// [`Debug`] will flood the screen with recursive data, this does not.
	///
	/// This still shouldn't be used for any public facing interfaces.
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f,
"Collection:
    empty     | {}
    timestamp | {}
    artists   | {}
    albums    | {}
    songs     | {}",
			self.empty,
			self.timestamp,
			self.count_artist,
			self.count_album,
			self.count_song,
		)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use disk::Bincode2;
	use readable::{Runtime, Date};

	// Empty new `Collection`.
	const C1: Lazy<Collection> = Lazy::new(|| Collection::new());
	// Filled, user `Collection`.
	const C2: Lazy<Collection> = Lazy::new(|| Collection::from_path("../assets/shukusai/state/collection0_real.bin").unwrap());

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
		assert_eq!(C2.song("artist_1", "album_1", "mp3"), Some((&C2.songs[k], Key::from_raw(0, 0, 1))));
	}

	#[test]
	// Tests `index()`.
	fn index() {
		assert_eq!(
			C2.index(Key::zero()),
			(&C2.artists[ArtistKey::zero()], &C2.albums[AlbumKey::zero()], &C2.songs[SongKey::zero()])
		);
	}

	#[test]
	// Compares a pre-saved `Collection` against `Collection::new()`.
	fn collection_new() {
		let b1 = C1.to_bytes().unwrap();
		let b2 = C2.to_bytes().unwrap();

		assert_ne!(Lazy::force(&C1), Lazy::force(&C2));
		assert_ne!(b1, b2);
	}

	#[test]
	// Attempts to deserialize a non-empty `Collection`.
	fn collection_real() {
		// Assert metadata within the `Collection`.
		assert!(!C2.empty);
		assert_eq!(C2.count_artist, 3);
		assert_eq!(C2.count_album,  4);
		assert_eq!(C2.count_song,   7);
		assert_eq!(C2.count_art,    0);
		assert_eq!(C2.timestamp,    1688605697);

		// Artist 1/3
		let k = ArtistKey::from(0_u8);
		assert_eq!(C2.artists[k].name,         "artist_1");
		assert_eq!(C2.artists[k].runtime,      Runtime::from(4_u8));
		assert_eq!(C2.artists[k].albums.len(), 2);
		assert_eq!(C2.artists[k].songs.len(),  4);

		// Artist 2/3
		let k = ArtistKey::from(1_u8);
		assert_eq!(C2.artists[k].name,         "artist_2");
		assert_eq!(C2.artists[k].runtime,      Runtime::from(2_u8));
		assert_eq!(C2.artists[k].albums.len(), 1);
		assert_eq!(C2.artists[k].songs.len(),  2);

		// Artist 3/3
		let k = ArtistKey::from(2_u8);
		assert_eq!(C2.artists[k].name,         "artist_3");
		assert_eq!(C2.artists[k].runtime,      Runtime::from(1_u8));
		assert_eq!(C2.artists[k].albums.len(), 1);
		assert_eq!(C2.artists[k].songs.len(),  1);

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
		assert_eq!(C2.songs[k].path.as_os_str().to_str().unwrap(), "/home/main/git/festival/assets/audio/song_1.mp3");

		// Song 2/7
		let k = SongKey::from(1_u8);
		assert_eq!(C2.songs[k].title, "mp3");
		assert_eq!(C2.songs[k].sample_rate, 48_000);
		assert_eq!(C2.songs[k].path.as_os_str().to_str().unwrap(), "/home/main/git/festival/assets/audio/song_2.mp3");

		// Song 3/7
		let k = SongKey::from(2_u8);
		assert_eq!(C2.songs[k].title, "mp3");
		assert_eq!(C2.songs[k].sample_rate, 48_000);
		assert_eq!(C2.songs[k].path.as_os_str().to_str().unwrap(), "/home/main/git/festival/assets/audio/song_3.mp3");

		// Song 4/7
		let k = SongKey::from(3_u8);
		assert_eq!(C2.songs[k].title, "flac");
		assert_eq!(C2.songs[k].sample_rate, 48_000);
		assert_eq!(C2.songs[k].path.as_os_str().to_str().unwrap(), "/home/main/git/festival/assets/audio/song_4.flac");

		// Song 5/7
		let k = SongKey::from(4_u8);
		assert_eq!(C2.songs[k].title, "m4a");
		assert_eq!(C2.songs[k].sample_rate, 48_000);
		assert_eq!(C2.songs[k].path.as_os_str().to_str().unwrap(), "/home/main/git/festival/assets/audio/song_5.m4a");

		// Song 6/7
		let k = SongKey::from(5_u8);
		assert_eq!(C2.songs[k].title, "song_6");
		assert_eq!(C2.songs[k].sample_rate, 48_000);
		assert_eq!(C2.songs[k].path.as_os_str().to_str().unwrap(), "/home/main/git/festival/assets/audio/song_6.ogg");

		// Song 7/7
		let k = SongKey::from(6_u8);
		assert_eq!(C2.songs[k].title, "mp3");
		assert_eq!(C2.songs[k].sample_rate, 48_000);
		assert_eq!(C2.songs[k].path.as_os_str().to_str().unwrap(), "/home/main/git/festival/assets/audio/song_7.mp3");
	}
}
