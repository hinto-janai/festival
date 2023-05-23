//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
use benri::{
	debug_panic,
	log::*,
	sync::*,
};
use crate::collection::{
	Collection,
	Keychain,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use std::{
	sync::Arc,
	collections::HashMap,
};
use crate::kernel::SearchKind;
use super::msg::{
	SearchToKernel,
	KernelToSearch,
};
use crossbeam::channel::{Sender,Receiver};
use benri::time::{
	now,secs_f32,
};

//---------------------------------------------------------------------------------------------------- Constants
// How many `(String, Keychain)` results to
// hold in cache before resetting.
//
// This will be set to the `total_count` when
// a new `Collection` is received, meaning
// all `count_*` fields added together.
const DEFAULT_CACHE_SIZE: usize = 1000;

//---------------------------------------------------------------------------------------------------- Search thread.
// This represents the `Search` thread.
pub(crate) struct Search {
	cache:       HashMap<String, Keychain>, // Search index cache
	cache_t25:   HashMap<String, Keychain>, // Search index cache (Top25),
	cache_s70:   HashMap<String, Keychain>, // Search index cache (Sim70),
	collection:  Arc<Collection>,           // Pointer to `Collection`
	total_count: usize,                     // Local cache of all total `Collection` objects
	to_kernel:   Sender<SearchToKernel>,    // Channel TO `Kernel`
	from_kernel: Receiver<KernelToSearch>,  // Channel FROM `Kernel`
}

//---------------------------------------------------------------------------------------------------- Search functions.
impl Search {
	// Kernel starts `Search` with this.
	pub(crate) fn init(
		collection:  Arc<Collection>,
		to_kernel:   Sender<SearchToKernel>,
		from_kernel: Receiver<KernelToSearch>,
	) {
		// Init data.
		let search = Self {
			cache: HashMap::with_capacity(DEFAULT_CACHE_SIZE),
			cache_t25: HashMap::with_capacity(DEFAULT_CACHE_SIZE),
			cache_s70: HashMap::with_capacity(DEFAULT_CACHE_SIZE),
			collection,
			total_count: DEFAULT_CACHE_SIZE,
			to_kernel,
			from_kernel,
		};

		// Start `main()`.
		Self::main(search);
	}

	#[inline]
	fn search_sim70(&self, input: &str) -> Keychain {
		let now = now!();

		let mut artists: Box<[(f64, ArtistKey)]> = self.collection.artists
			.iter().enumerate()
			.map(|(i, x)| (strsim::jaro(&x.name.to_ascii_lowercase(), input), ArtistKey::from(i)))
			.filter(|(f, _)| *f >= 0.7)
			.collect();
		let mut albums:  Box<[(f64, AlbumKey)]> = self.collection.albums
			.iter().enumerate()
			.map(|(i, x)| (strsim::jaro(&x.title.to_ascii_lowercase(), input), AlbumKey::from(i)))
			.filter(|(f, _)| *f >= 0.7)
			.collect();
		let mut songs:   Box<[(f64, SongKey)]>  = self.collection.songs
			.iter().enumerate()
			.map(|(i, x)| (strsim::jaro(&x.title.to_ascii_lowercase(), input), SongKey::from(i)))
			.filter(|(f, _)| *f >= 0.7)
			.collect();

		// Sort by lowest-to-highest similarity value first.
		artists.sort_by(|a, b| Self::cmp_f64(a.0, b.0));
		albums.sort_by(|a, b| Self::cmp_f64(a.0, b.0));
		songs.sort_by(|a, b| Self::cmp_f64(a.0, b.0));

		// Collect just the Keys (reverse, highest sim first).
		let artists: Box<[ArtistKey]> = artists.iter().rev().map(|tuple| tuple.1).collect();
		let albums:  Box<[AlbumKey]>  = albums.iter().rev().map(|tuple| tuple.1).collect();
		let songs:   Box<[SongKey]>   = songs.iter().rev().map(|tuple| tuple.1).collect();

		// Return keychain.
		Keychain::from_boxes(artists, albums, songs)
	}

	#[inline]
	fn search_top25(&self, input: &str) -> Keychain {
		let now = now!();

		let mut artists: Box<[(f64, ArtistKey)]> = self.collection.artists
			.iter().enumerate()
			.map(|(i, x)| (strsim::jaro(&x.name.to_ascii_lowercase(), input), ArtistKey::from(i)))
			.collect();
		let mut albums:  Box<[(f64, AlbumKey)]> = self.collection.albums
			.iter().enumerate()
			.map(|(i, x)| (strsim::jaro(&x.title.to_ascii_lowercase(), input), AlbumKey::from(i)))
			.collect();
		let mut songs:   Box<[(f64, SongKey)]>  = self.collection.songs
			.iter().enumerate()
			.map(|(i, x)| (strsim::jaro(&x.title.to_ascii_lowercase(), input), SongKey::from(i)))
			.collect();

		// Sort by lowest-to-highest similarity value first.
		artists.sort_by(|a, b| Self::cmp_f64(a.0, b.0));
		albums.sort_by(|a, b| Self::cmp_f64(a.0, b.0));
		songs.sort_by(|a, b| Self::cmp_f64(a.0, b.0));

		// Collect just the Keys (reverse, highest sim first).
		let artists: Box<[ArtistKey]> = artists.iter().rev().map(|tuple| tuple.1).take(25).collect();
		let albums:  Box<[AlbumKey]>  = albums.iter().rev().map(|tuple| tuple.1).take(25).collect();
		let songs:   Box<[SongKey]>   = songs.iter().rev().map(|tuple| tuple.1).take(25).collect();

		// Return keychain.
		Keychain::from_boxes(artists, albums, songs)
	}

	#[inline]
	fn search_all(&self, input: &str) -> Keychain {
		let now = now!();

		let mut artists: Box<[(f64, ArtistKey)]> = self.collection.artists
			.iter().enumerate()
			.map(|(i, x)| (strsim::jaro(&x.name.to_ascii_lowercase(), input), ArtistKey::from(i)))
			.collect();
		let mut albums:  Box<[(f64, AlbumKey)]> = self.collection.albums
			.iter().enumerate()
			.map(|(i, x)| (strsim::jaro(&x.title.to_ascii_lowercase(), input), AlbumKey::from(i)))
			.collect();
		let mut songs:   Box<[(f64, SongKey)]>  = self.collection.songs
			.iter().enumerate()
			.map(|(i, x)| (strsim::jaro(&x.title.to_ascii_lowercase(), input), SongKey::from(i)))
			.collect();

		// Sort by lowest-to-highest similarity value first.
		artists.sort_by(|a, b| Self::cmp_f64(a.0, b.0));
		albums.sort_by(|a, b| Self::cmp_f64(a.0, b.0));
		songs.sort_by(|a, b| Self::cmp_f64(a.0, b.0));

		// Collect just the Keys (reverse, highest sim first).
		let artists: Box<[ArtistKey]> = artists.iter().rev().map(|tuple| tuple.1).collect();
		let albums:  Box<[AlbumKey]>  = albums.iter().rev().map(|tuple| tuple.1).collect();
		let songs:   Box<[SongKey]>   = songs.iter().rev().map(|tuple| tuple.1).collect();

		// Return keychain.
		Keychain::from_boxes(artists, albums, songs)
	}

	#[inline]
	// INVARIANT:
	// `.partial_cmp()` returns an `Option` because a
	// floating point might be a `NaN`, but, `strsim::jaro()`
	// will always return a value between `0.0 - 1.0`.
	//
	// `cmp_f64()` just returns `Less` on error
	// (which should never happen... right strsim?)
	pub(crate) fn cmp_f64(a: f64, b: f64) -> std::cmp::Ordering {
		match (a <= b, a >= b) {
			(false, true) => std::cmp::Ordering::Greater,
			(true, false) => std::cmp::Ordering::Less,
			(true, true) => std::cmp::Ordering::Equal,
			_ => {
				debug_panic!("cmp_f64() has failed, input: {a} - {b}");

				error!("cmp_f64() has failed, input: {a} - {b}");
				std::cmp::Ordering::Less
			},
		}
	}
}

//---------------------------------------------------------------------------------------------------- Main Search loop.
impl Search {
	fn main(mut self) {
		ok_debug!("Search");

		loop {
			// Block, wait for signal.
			let msg = recv!(self.from_kernel);

			// Match message and do action.
			use KernelToSearch::*;
			match msg {
				Search((input, kind))    => self.msg_sim(input, kind),
//				NewCache(string) => self.msg_cache(string),
//				NewCacheVec(vec) => self.msg_vec_cache(vec),
				DropCollection   => self.msg_drop(),

				// Other messages shouldn't be received here, e.g:
				// `DropCollection` should _always_ be first before `NewCollection`.
				// Something buggy is happening if we randomly get a new `NewCollection`.
				NewCollection(_) => {
					debug_panic!("Search - Incorrect message received - NewCollection");
					error!("Search - Incorrect message received - NewCollection");
				},
			}
		}
	}

	#[inline(always)]
	// Reset the cache if it's filled up.
	fn check_cache(&mut self, kind: SearchKind) {
		let cache = match kind {
			SearchKind::Sim70 => &mut self.cache_s70,
			SearchKind::Top25 => &mut self.cache_t25,
			SearchKind::All   => &mut self.cache,
		};

		if cache.len() > self.total_count {
			// Clear.
			debug!("Search - Cache length more than '{}', clearing.", self.total_count);
			cache.clear();
		}
	}

	#[inline(always)]
	fn get_cache(&self, input: &str, kind: SearchKind) -> Option<Keychain> {
		match kind {
			SearchKind::Sim70 => &self.cache_s70,
			SearchKind::Top25 => &self.cache_t25,
			SearchKind::All   => &self.cache,
		}.get(input).map(Clone::clone)
	}

	#[inline(always)]
	fn insert_cache(&mut self, input: String, keychain: Keychain, kind: SearchKind) {
		match kind {
			SearchKind::Sim70 => self.cache_s70.insert(input, keychain),
			SearchKind::Top25 => self.cache_t25.insert(input, keychain),
			SearchKind::All   => self.cache.insert(input, keychain),
		};
	}

	#[inline(always)]
	fn msg_sim(&mut self, mut input: String, kind: SearchKind) {
		let now = now!();
		input.make_ascii_lowercase();

		let keychain = match self.get_cache(&input, kind) {
			Some(k) => {
				trace!("Search - cache ... {}", secs_f32!(now));
				k
			},
			None => {
				let k = match kind {
					SearchKind::Sim70 => self.search_sim70(&input),
					SearchKind::Top25 => self.search_top25(&input),
					SearchKind::All   => self.search_all(&input),
				};
				self.check_cache(kind);
				self.insert_cache(input, k.clone(), kind);
				trace!("Search - {kind:?} ... {}", secs_f32!(now));
				k
			},
		};

		// Send to Kernel.
		send!(self.to_kernel, SearchToKernel::Resp(keychain));
	}

//	#[inline(always)]
//	// We got a `String` key from a recently
//	// created `Collection`, add it to cache.
//	fn msg_cache(&mut self, input: String) {
//		trace!("Search - Adding input to cache: {}", &input);
//		let result = self.search(&input);
//		self.add_to_cache(input, result);
//	}
//
//	#[inline(always)]
//	// We got a `Vec` of `String` keys, add it to cache.
//	fn msg_vec_cache(&mut self, inputs: Vec<String>) {
//		for input in inputs {
//			trace!("Search - Adding Vec<input> to cache: {}", &input);
//			let result = self.search(&input);
//			self.add_to_cache(input, result);
//		}
//	}

	#[inline(always)]
	fn msg_drop(&mut self) {
		// Drop pointer.
		self.collection = Collection::dummy();

		// Reset cache.
		self.cache.clear();

		// Hang until we get the new one.
		debug!("Search - Dropped Collection, waiting...");

		// Listen to `Kernel`.
		loop {
			match recv!(self.from_kernel) {
				// We got the new `Collection` pointer.
				KernelToSearch::NewCollection(arc) => {
					ok_debug!("Search - New Collection");
					self.collection = arc;
					self.total_count = {
						self.collection.count_artist.usize() +
						self.collection.count_album.usize() +
						self.collection.count_song.usize()
					};
					return
				},
				_ => {
					debug_panic!("Search - Incorrect message received");
					error!("Search - Incorrect message received");
				},
			}

		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
