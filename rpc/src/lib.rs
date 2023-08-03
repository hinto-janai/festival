mod macros;

mod method;
pub use method::Method;

/// Structs that represent the parameters of a [`Method`]
pub mod param;

/// Structs that represent the responses expected from a given [`Method`]
pub mod resp;

/// REST resources
pub mod resource;
