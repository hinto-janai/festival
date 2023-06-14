mod art;
mod collection;
mod album;
mod artist;
mod song;
mod plural;
mod map;
mod key;
mod slice;
pub use collection::*;
pub use art::*;
pub use album::*;
pub use artist::*;
pub use song::*;
pub use plural::*;
pub use map::*;
pub use key::*;
pub use slice::*;

mod metadata;
pub use metadata::metadata;

// Pointer related code. To be used... eventually... maybe.
//mod decode;
//pub(crate) use decode::*;
//mod ptr;
//pub(crate) use ptr::*;
