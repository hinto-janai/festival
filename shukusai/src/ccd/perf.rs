//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{FESTIVAL, FRONTEND_SUB_DIR, TXT_SUB_DIR};

use const_format::formatcp;
use serde::{Deserialize, Serialize};

//---------------------------------------------------------------------------------------------------- __NAME__
disk::json!(
    Perf,
    disk::Dir::Data,
    FESTIVAL,
    formatcp!("{FRONTEND_SUB_DIR}/{TXT_SUB_DIR}"),
    "perf"
);
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
/// File representing some stats and performance of creating a [`Collection`]
///
/// This gets written in the `festival/txt` folder as `perf.json`.
pub struct Perf {
    // The specific timings of each step in `CCD`.
    pub(crate) phases: Phases,
    /// How many objects we allocated in our `Collection`.
    pub objects: Objects,
    /// Total size of `Collection` and time it took to create it,
    /// from the user's perspective and from `CCD`'s perspective.
    pub total: Total,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
/// How many objects we allocated in our `Collection`.
pub(crate) struct Phases {
    pub(super) deconstruct: f32,
    pub(super) walkdir: f32,
    pub(super) metadata: f32,
    pub(super) fix: f32,
    pub(super) sort: f32,
    pub(super) map: f32,
    pub(super) prepare: f32,
    pub(super) resize: f32,
    pub(super) clone: f32,
    pub(super) convert: f32,
    pub(super) textures: f32,
    pub(super) playlists: f32,
    pub(super) disk: f32,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
/// Object count.
pub struct Objects {
    /// Artist count
    pub artists: usize,
    /// Album count
    pub albums: usize,
    /// Song count
    pub songs: usize,
    /// Art count
    pub art: usize,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
/// Total size of `Collection` and time it took to create it,
/// from the user's perspective and from `CCD`'s perspective.
pub struct Total {
    /// Collection byte size
    pub bytes: u64,
    /// Collection creation user time
    pub user: f32,
    /// Collection creation CCD (internal) time
    pub sys: f32,
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
