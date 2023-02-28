//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use crate::macros::*;
use crate::collection::{
	Collection,
	key::CollectionKeychain,
	key::ArtistKey,
	key::AlbumKey,
	key::SongKey,
};
use std::{
	sync::Arc,
	collections::HashMap,
};
use super::msg::{
	SearchToKernel,
	KernelToSearch,
};

//---------------------------------------------------------------------------------------------------- Search thread.
// This represents the `Search` thread.
struct Search {
	cache:       HashMap<String, CollectionKeychain>,        // Search index cache
	collection:  Arc<Collection>,                            // Pointer to `Collection`
	to_kernel:   crossbeam::channel::Sender<SearchToKernel>, // Channel TO `Kernel`
	from_kernel: std::sync::mpsc::Receiver<KernelToSearch>,  // Channel FROM `Kernel`
}

//---------------------------------------------------------------------------------------------------- Search functions.
impl Search {
	// Kernel starts `Search` with this.
	pub fn init(
		collection: Arc<Collection>,
		to_kernel: crossbeam::channel::Sender<SearchToKernel>,
		from_kernel: std::sync::mpsc::Receiver<KernelToSearch>,
	) {
		// Init data.
		let search = Self {
			cache: HashMap::with_capacity(50),
			collection,
			to_kernel,
			from_kernel,
		};

		// Start `main()`.
		Self::main(search);
	}

	fn search(&mut self, input: String) -> CollectionKeychain {
		// Return early if search input is in cache.
		if let Some(v) = self.cache.get(&input) {
			return v.clone()
		}

		// Search and collect results.
		let mut artists: Vec<(f64, ArtistKey)> = self.collection.artists.iter().map(|x| (strsim::jaro(&x.name, &input), ArtistKey::from(x.key))).collect();
		let mut albums:  Vec<(f64, AlbumKey)>  = self.collection.albums.iter().map(|x| (strsim::jaro(&x.title, &input), AlbumKey::from(x.key))).collect();
		let mut songs:   Vec<(f64, SongKey)>   = self.collection.songs.iter().map(|x| (strsim::jaro(&x.title, &input), SongKey::from(x.key))).collect();

		// Sort by highest similarity value first.
		artists.sort_by(|a, b| Self::cmp_f64(&a.0, &b.0));
		albums.sort_by(|a, b| Self::cmp_f64(&a.0, &b.0));
		songs.sort_by(|a, b| Self::cmp_f64(&a.0, &b.0));

		// Collect just the Keys.
		let artists: Vec<ArtistKey> = artists.iter().map(|tuple| tuple.1).collect();
		let albums:  Vec<AlbumKey>  = albums.iter().map(|tuple| tuple.1).collect();
		let songs:   Vec<SongKey>   = songs.iter().map(|tuple| tuple.1).collect();

		// Create keychain.
		let keychain = CollectionKeychain {
			artists,
			albums,
			songs,
		};

		// Add to cache.
		self.cache.insert(input, keychain.clone());

		// Return.
		keychain
	}

	// Invariant:
	// `.partial_cmp()` returns an `Option` because a
	// floating point might be a `NaN`, but, `strsim::jaro()`
	// will always return a value between `0.0 - 1.0`.
	//
	// `cmp_f64()` just returns `Less` on error
	// (which should never happen... right strsim?)
	fn cmp_f64(a: &f64, b: &f64) -> std::cmp::Ordering {
		match (*a <= *b, *a >= *b) {
			(false, true) => std::cmp::Ordering::Greater,
			(true, false) => std::cmp::Ordering::Less,
			(true, true) => std::cmp::Ordering::Equal,
			_ => {
				error!("strsim you have failed me");
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

			// Match message.
			use KernelToSearch::*;
			match msg {
				Search(input)      => Self::msg_search(&mut self, input),
				DropCollection     => Self::msg_drop(&mut self),
				CollectionArc(ptr) => Self::msg_ptr(&mut self),
			}
		}
	}

	#[inline(always)]
	fn msg_search(&mut self, input: String) {
		// Get result.
		let result = self.search(input);

		// Send to Kernel.
		send!(self.to_kernel, SearchToKernel::SearchResult(result));
	}

	#[inline(always)]
	fn msg_drop(&mut self) {}

	#[inline(always)]
	fn msg_ptr(&mut self) {}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
