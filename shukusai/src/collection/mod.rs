mod album;
mod art;
mod artist;
mod collection;
mod entry;
mod image;
mod key;
mod map;
mod plural;
mod song;
pub use crate::collection::image::*;
pub use album::*;
pub use art::*;
pub use artist::*;
pub use collection::*;
pub use entry::*;
pub use key::*;
pub use map::*;
pub use plural::*;
pub use song::*;

mod metadata;
pub use metadata::metadata;

// Previous Collection versions.
#[cfg(feature = "gui")]
pub(crate) mod v0;
#[cfg(feature = "gui")]
pub(crate) mod v1;
#[cfg(feature = "gui")]
pub(crate) mod v2;

/// `struct` representations for JSON output
pub mod json;
/// 1:1 copies of JSON-RPC calls
pub mod rpc;

// Pointer related code. To be used... eventually... maybe.
//mod decode;
//pub(crate) use decode::*;
//mod ptr;
//pub(crate) use ptr::*;

// Playlist/Queue code. To be used SOMEDAY.
//mod slice;
//pub use slice::*;
