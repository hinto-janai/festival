//---------------------------------------------------------------------------------------------------- Use
use crate::collection::{Collection, Keychain};
use crate::search::SearchKind;
use std::sync::Arc;

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum SearchToKernel {
    // Here's the search (similarity) response.
    Resp(Keychain),
}

pub(crate) enum KernelToSearch {
    Search((String, SearchKind)), // Start a search on string input.
    //	NewCache(String),               // Here's a new `String` key from a recently created `Collection`, add it to your cache.
    //	NewCacheVec(Vec<String>),       // Here's a `Vec` of `String` keys, add it to cache
    DropCollection,                 // Drop your pointer.
    NewCollection(Arc<Collection>), // Here's a new `Collection` pointer.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
