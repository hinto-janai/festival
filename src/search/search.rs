//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
use log::{info,error,warn,trace,debug};
use benri::{
	log::*,
	sync::*,
};
use crate::collection::{
	Collection,
};
use crate::key::{
	Keychain,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use std::{
	sync::Arc,
	collections::HashMap,
};
use super::msg::{
	SearchToKernel,
	KernelToSearch,
};
use crossbeam_channel::{Sender,Receiver};

//---------------------------------------------------------------------------------------------------- Search thread.
// This represents the `Search` thread.
pub(crate) struct Search {
	cache:       HashMap<String, Keychain>, // Search index cache
	collection:  Arc<Collection>,           // Pointer to `Collection`
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
			cache: HashMap::with_capacity(1000),
			collection,
			to_kernel,
			from_kernel,
		};

		// Start `main()`.
		Self::main(search);
	}

	fn sim(&mut self, input: String) -> Keychain {
		// Return early if search input is in cache.
		if let Some(v) = self.cache.get(&input) {
			return v.clone()
		}

		// Search and collect results.
		let mut artists: Vec<(f64, ArtistKey)> = self.collection.artists.iter().enumerate().map(|(i, x)| (strsim::jaro(&x.name, &input), ArtistKey::from(i))).collect();
		let mut albums:  Vec<(f64, AlbumKey)>  = self.collection.albums.iter().enumerate().map(|(i, x)| (strsim::jaro(&x.title, &input), AlbumKey::from(i))).collect();
		let mut songs:   Vec<(f64, SongKey)>   = self.collection.songs.iter().enumerate().map(|(i, x)| (strsim::jaro(&x.title, &input), SongKey::from(i))).collect();

		// Sort by highest similarity value first.
		artists.sort_by(|a, b| Self::cmp_f64(&a.0, &b.0));
		albums.sort_by(|a, b| Self::cmp_f64(&a.0, &b.0));
		songs.sort_by(|a, b| Self::cmp_f64(&a.0, &b.0));

		// Collect just the Keys.
		let artists: Vec<ArtistKey> = artists.iter().map(|tuple| tuple.1).collect();
		let albums:  Vec<AlbumKey>  = albums.iter().map(|tuple| tuple.1).collect();
		let songs:   Vec<SongKey>   = songs.iter().map(|tuple| tuple.1).collect();

		// Create keychain.
		let keychain = Keychain::from_vecs(artists, albums, songs);

		// (Maybe) clear cache.
		if self.cache.len() > 1000 {
			// Clear.
			self.cache.clear();
		}

		// Add to cache.
		self.cache.insert(input, keychain.clone());

		// Return.
		keychain
	}

	// INVARIANT:
	// `.partial_cmp()` returns an `Option` because a
	// floating point might be a `NaN`, but, `strsim::jaro()`
	// will always return a value between `0.0 - 1.0`.
	//
	// `cmp_f64()` just returns `Less` on error
	// (which should never happen... right strsim?)
	pub(crate) fn cmp_f64(a: &f64, b: &f64) -> std::cmp::Ordering {
		match (*a <= *b, *a >= *b) {
			(false, true) => std::cmp::Ordering::Greater,
			(true, false) => std::cmp::Ordering::Less,
			(true, true) => std::cmp::Ordering::Equal,
			_ => {
				error!("cmp_f64() has failed, input: {} - {}", a, b);
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
				SearchSim(input)   => self.msg_sim(input),
				DropCollection     => self = self.msg_drop(),

				// Other messages shouldn't be received here, e.g:
				// `DropCollection` should _always_ be first before `NewCollection`.
				// Something buggy is happening if we randomly get a new `NweCollection`.
				NewCollection(_) => error!("Search: Incorrect message received - NewCollection"),
			}
		}
	}

	#[inline(always)]
	fn msg_sim(&mut self, input: String) {
		// Get result.
		let result = self.sim(input);

		// Send to Kernel.
		send!(self.to_kernel, SearchToKernel::SearchSim(result));
	}

	#[inline(always)]
	fn msg_drop(mut self) -> Self {
		// Drop pointer.
		drop(self.collection);

		// Hang until we get the new one.
		debug!("Search: Dropped Collection, waiting...");

		// Ignore messages until it's a pointer.
		// (`Kernel` should only be sending a pointer at this point anyway).
		loop {
			if let KernelToSearch::NewCollection(arc) = recv!(self.from_kernel) {
				ok_debug!("Search: New Collection");
				self.collection = arc;
				return self
			}

			error!("Search: Incorrect message received");
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
