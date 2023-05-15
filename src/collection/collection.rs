//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
use log::{error,warn,info,debug,trace};
use serde::{Serialize,Deserialize};
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
use crate::kernel::Kernel;
use std::collections::HashMap;
//use disk::{Json,json_file};
use crate::constants::{
	FESTIVAL,
	FESTIVAL_HEADER,
	COLLECTION_VERSION,
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
	Time,
	Unsigned,
};
use once_cell::sync::Lazy;
use std::marker::PhantomData;

//---------------------------------------------------------------------------------------------------- Lazy
// `RNG`: Global RNG state for `Collection`'s `rand_*` functions.
static RNG: Lazy<Mutex<rand::rngs::SmallRng>> = Lazy::new(|| Mutex::new(rand::rngs::SmallRng::from_entropy()));

// This is an empty, dummy `Collection`.
pub(crate) static DUMMY_COLLECTION: Lazy<Arc<Collection>> = Lazy::new(|| Arc::new(Collection::new()));

//---------------------------------------------------------------------------------------------------- Collection
disk::bincode2!(Collection, disk::Dir::Data, FESTIVAL, "", "collection", FESTIVAL_HEADER, COLLECTION_VERSION);
#[derive(Clone,Debug,PartialEq,Encode,Decode)]
//#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Encode,Decode)]
/// The main music `Collection`
///
/// This is the `struct` that holds all the (meta)data about the user's music.
///
/// This holds:
/// - The "3 Vecs", holding _all_ [`Artist`]'s, [`Album`]'s, and [`Song`]'s.
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
/// [`str`] input instead of raw indicies or a [`Key`]. This is more flexible
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
/// The "3 Vecs" are (basically) in random order due to how `Collection` is created.
///
/// Iterating directly on them is not very useful, so use the pre-calculated sorted keys.
///
/// The sorted key fields all start with `sort_`.
///
/// `lexi` is shorthand for `lexicographically`, as defined [here.](https://doc.rust-lang.org/stable/std/primitive.str.html#impl-Ord-for-str)
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

	// The "3 Vecs".
	/// All the [`Artist`]'s in mostly random order.
	pub artists: Artists,
	/// All the [`Album`]'s in mostly random order.
	pub albums: Albums,
	/// All the [`Song`]'s in mostly random order.
	pub songs: Songs,

	// Sorted `Artist` keys.
	/// [`Artist`] `lexi`.
	pub sort_artist_lexi: Box<[ArtistKey]>,
	/// [`Artist`] with most [`Album`]'s to least.
	pub sort_artist_album_count: Box<[ArtistKey]>,
	/// [`Artist`] with most [`Song`]'s to least.
	pub sort_artist_song_count: Box<[ArtistKey]>,

	// Sorted `Album` keys.
	/// [`Artist`] `lexi`, [`Album`]'s oldest release to latest.
	pub sort_album_release_artist_lexi: Box<[AlbumKey]>,
	/// [`Artist`] `lexi`, [`Album`]'s `lexi`.
	pub sort_album_lexi_artist_lexi: Box<[AlbumKey]>,
	/// [`Album`] lexi.
	pub sort_album_lexi: Box<[AlbumKey]>,
	/// [`Album`] oldest to latest.
	pub sort_album_release: Box<[AlbumKey]>,
	/// [`Album`] shortest to longest.
	pub sort_album_runtime: Box<[AlbumKey]>,

	// Sorted `Song` keys.
	/// [`Artist`] lexi, [`Album`] release, [`Song`] track_number
	pub sort_song_album_release_artist_lexi: Box<[SongKey]>,
	/// [`Artist`] lexi, [`Album`] lexi, [`Song`] track_number.
	pub sort_song_album_lexi_artist_lexi: Box<[SongKey]>,
	/// [`Song`] lexi.
	pub sort_song_lexi: Box<[SongKey]>,
	/// [`Song`] oldest to latest.
	pub sort_song_release: Box<[SongKey]>,
	/// [`Song`] shortest to longest.
	pub sort_song_runtime: Box<[SongKey]>,

	// Reserved fields and their `size_of()`.
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
	pub(crate) _reserved17: PhantomData<String>, // 24
	pub(crate) _reserved18: PhantomData<String>,
	pub(crate) _reserved19: PhantomData<usize>, // 8
	pub(crate) _reserved20: PhantomData<usize>,
	pub(crate) _reserved21: PhantomData<usize>,
	pub(crate) _reserved22: PhantomData<usize>,
	pub(crate) _reserved23: PhantomData<bool>, // 1
	pub(crate) _reserved24: PhantomData<bool>,
}

impl Collection {
	//-------------------------------------------------- New.
	// Creates an empty [`Collection`].
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
			sort_artist_album_count: Box::new([]),
			sort_artist_song_count: Box::new([]),

			sort_album_release_artist_lexi: Box::new([]),
			sort_album_lexi_artist_lexi: Box::new([]),
			sort_album_lexi: Box::new([]),
			sort_album_release: Box::new([]),
			sort_album_runtime: Box::new([]),

			sort_song_album_release_artist_lexi: Box::new([]),
			sort_song_album_lexi_artist_lexi: Box::new([]),
			sort_song_lexi: Box::new([]),
			sort_song_release: Box::new([]),
			sort_song_runtime: Box::new([]),

			_reserved1: PhantomData,
			_reserved2: PhantomData,
			_reserved4: PhantomData,
			_reserved5: PhantomData,
			_reserved6: PhantomData,
			_reserved7: PhantomData,
			_reserved8: PhantomData,
			_reserved9: PhantomData,
			_reserved10: PhantomData,
			_reserved11: PhantomData,
			_reserved12: PhantomData,
			_reserved13: PhantomData,
			_reserved14: PhantomData,
			_reserved15: PhantomData,
			_reserved16: PhantomData,
			_reserved17: PhantomData,
			_reserved18: PhantomData,
			_reserved19: PhantomData,
			_reserved20: PhantomData,
			_reserved21: PhantomData,
			_reserved22: PhantomData,
			_reserved23: PhantomData,
			_reserved24: PhantomData,
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

	//-------------------------------------------------- Metadata functions.
	// Set the proper metadata for this `Collection`.
	pub(crate) fn set_metadata(mut self) -> Self {
		// Get `Vec` lengths.
		let artists = self.artists.len();
		let albums  = self.albums.len();
		let songs   = self.songs.len();

		// Set `empty`.
		if artists == 0 && albums == 0 && songs == 0 {
			self.empty = true;
		} else {
			self.empty = false;
		}

		// Set `count_*`.
		self.count_artist = Unsigned::from(artists);
		self.count_album  = Unsigned::from(albums);
		self.count_song   = Unsigned::from(songs);

		// Set `timestamp`.
		self.timestamp = benri::unix!();

		self
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
	pub fn artist(&self, artist_name: &str) -> Option<(&Artist, ArtistKey)> {
		if let Some((key, _)) = self.map.0.get(artist_name) {
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
	pub fn album(
		&self,
		artist_name: &str,
		album_title: &str
	) -> Option<(&Album, AlbumKey)> {
		if let Some((key, albums)) = self.map.0.get(artist_name) {
			if let Some((key, _)) = albums.0.get(album_title) {
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
	pub fn song(
		&self,
		artist_name: &str,
		album_title: &str,
		song_title: &str,
	) -> Option<(&Song, Key)> {
		if let Some((artist_key, albums)) = self.map.0.get(artist_name) {
			if let Some((album_key, songs)) = albums.0.get(album_title) {
				if let Some(song_key) = songs.0.get(song_title) {
					let key = Key::from_keys(*artist_key, *album_key, *song_key);
					return Some((&self.songs[song_key], key))
				}
			}
		}

		None
	}

	//-------------------------------------------------- Indexing.
	/// Directly index the [`Collection`] with a [`Key`].
	///
	/// # Panics:
	/// The [`ArtistKey`], [`AlbumKey`] and [`SongKey`] within
	/// the [`Key`] must be valid indicies into the [`Collection`].
	#[inline(always)]
	pub fn index(&self, key: Key) -> (&Artist, &Album, &Song) {
		let (artist, album, song) = key.inner_usize();
		(&self.artists.0[artist], &self.albums.0[album], &self.songs.0[song])
	}

	/// [`slice::get`] the [`Collection`] with a [`Key`].
	///
	/// # Errors:
	/// The [`ArtistKey`], [`AlbumKey`] and [`SongKey`] within
	/// the [`Key`] must be valid indicies into the [`Collection`].
	pub fn get(&self, key: Key) -> Option<(&Artist, &Album, &Song)> {
		let (artist, album, song) = key.inner_usize();

		let artists = match self.artists.0.get(artist) {
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

		Some((artists, album, song))
	}

	//-------------------------------------------------- Key traversal (index).
	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`AlbumKey`].
	///
	/// # Panics:
	/// The [`AlbumKey`] must be a valid index.
	pub fn artist_from_album(&self, key: AlbumKey) -> &Artist {
		&self.artists[self.albums[key].artist]
	}

	#[inline(always)]
	/// Obtain an [`Album`], but from a [`SongKey`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn album_from_song(&self, key: SongKey) -> &Album {
		&self.albums[self.songs[key].album]
	}

	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`SongKey`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn artist_from_song(&self, key: SongKey) -> &Artist {
		self.artist_from_album(self.songs[key].album)
	}

	//-------------------------------------------------- Key traversal (`.get()`).
	#[inline]
	/// Obtain an [`Artist`], but from a [`AlbumKey`].
	///
	/// # Errors:
	/// The [`AlbumKey`] must be a valid index.
	pub fn get_artist_from_album(&self, key: AlbumKey) -> Option<&Artist> {
		let artist = match self.albums.get(key) {
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
	pub fn get_album_from_song(&self, key: SongKey) -> Option<&Album> {
		let album = match self.songs.get(key) {
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
	pub fn get_artist_from_song(&self, key: SongKey) -> Option<&Artist> {
		let album = match self.songs.get(key) {
			Some(a) => a.album,
			None    => return None,
		};

		self.get_artist_from_album(album)
	}

	//-------------------------------------------------- Sorting
	/// Access a particular `sort_artist_` field in the [`Collection`] via a [`ArtistSort`].
	pub const fn artist_sort(&self, sort: ArtistSort) -> &[ArtistKey] {
		use ArtistSort::*;
		match sort {
			Lexi       => &self.sort_artist_lexi,
			AlbumCount => &self.sort_artist_album_count,
			SongCount  => &self.sort_artist_song_count,
		}
	}

	/// Access a particular `sort_album_` field in the [`Collection`] via a [`AlbumSort`].
	pub const fn album_sort(&self, sort: AlbumSort) -> &[AlbumKey] {
		use AlbumSort::*;
		match sort {
			ReleaseArtistLexi => &self.sort_album_release_artist_lexi,
			LexiArtistLexi    => &self.sort_album_lexi_artist_lexi,
			Lexi              => &self.sort_album_lexi,
			Release           => &self.sort_album_release,
			Runtime           => &self.sort_album_runtime,
		}
	}

	/// Access a particular `sort_song_` field in the [`Collection`] via a [`SongSort`].
	pub const fn song_sort(&self, sort: SongSort) -> &[SongKey] {
		use SongSort::*;
		match sort {
			AlbumReleaseArtistLexi => &self.sort_song_album_release_artist_lexi,
			AlbumLexiArtistLexi    => &self.sort_song_album_lexi_artist_lexi,
			Lexi                   => &self.sort_song_lexi,
			Release                => &self.sort_song_release,
			Runtime                => &self.sort_song_runtime,
		}
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
	use disk::*;

	#[test]
	fn serde() {
		let collection = Collection::new();
		collection.save().unwrap();
		let collection = Collection::from_file();
	}
}
