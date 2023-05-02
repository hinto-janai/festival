//---------------------------------------------------------------------------------------------------- Use
use disk::Json;
use crate::constants::FESTIVAL;
use crate::collection::Collection;
use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- __NAME__
disk::json!(Perf, disk::Dir::Data, FESTIVAL, "", "perf");
#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Serialize,Deserialize)]
/// File representing some stats and performance of creating a [`Collection`]
///
/// This gets written in the `festival` folder as `perf.json`.
pub(super) struct Perf {
	// The specific timings of each step in `CCD`.
	pub(super) phases: Phases,
	// How many objects we allocated in our `Collection`.
	pub(super) objects: Objects,
	// Total size of `Collection` and time it took to create it,
	// from the user's perspective and from `CCD`'s perspective.
	pub(super) total: Total,
}

impl std::fmt::Display for Perf {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let string = match disk::Json::to_string(self) {
			Ok(s) => s,
			Err(e) => e.to_string(),
		};
		write!(f, "Perf {}", string)
	}
}

#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Serialize,Deserialize)]
pub(super) struct Phases {
	pub(super) walkdir:     f32,
	pub(super) metadata:    f32,
	pub(super) fix:         f32,
	pub(super) sort:        f32,
	pub(super) map:         f32,
	pub(super) prepare:     f32,
	pub(super) resize:      f32,
	pub(super) clone:       f32,
	pub(super) convert:     f32,
	pub(super) textures:    f32,
	pub(super) to_kernel:   f32,
	pub(super) die:         f32,
	pub(super) disk:        f32,
	pub(super) deconstruct: f32,
}

#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Serialize,Deserialize)]
pub(super) struct Objects {
	pub(super) artists: usize,
	pub(super) albums: usize,
	pub(super) songs: usize,
	pub(super) art: usize,
}

#[derive(Copy,Clone,Debug,Default,PartialEq,PartialOrd,Serialize,Deserialize)]
pub(super) struct Total {
	pub(super) bytes: u64,
	pub(super) user: f32,
	pub(super) ccd: f32,
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
