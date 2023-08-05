//---------------------------------------------------------------------------------------------------- Use
use zeroize::{ZeroizeOnDrop,Zeroize};
use std::pin::Pin;

//---------------------------------------------------------------------------------------------------- Constants
const LEN: usize = 32;

//---------------------------------------------------------------------------------------------------- Hash
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Zeroize,ZeroizeOnDrop,PartialEq)]
// Contains `Basic <original input in base64>`
pub struct Auth(Pin<String>);

#[cfg(not(debug_assertions))]
impl std::fmt::Debug for Auth {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "crate::auth::Auth")
	}
}

impl Auth {
	/// Encode input in base64
	pub fn new(input: String) -> Self {
		Self(Pin::new(rpc::base64::encode_with_authorization_basic_header(input)))
	}
}


//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
}
