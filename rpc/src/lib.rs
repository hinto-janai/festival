mod macros;

mod method;
pub use method::Method;
pub use method::Rpc;

/// Structs that represent the parameters of a [`Method`]
pub mod param;

/// Structs that represent the responses expected from a given [`Method`]
pub mod resp;

/// REST resources
pub mod resource;

/// Base64 operations
pub mod base64;
/// Hashing operations
pub mod hash;
