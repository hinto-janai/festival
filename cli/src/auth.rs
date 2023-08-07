//---------------------------------------------------------------------------------------------------- Use
use zeroize::{ZeroizeOnDrop,Zeroize};
use std::pin::Pin;

//---------------------------------------------------------------------------------------------------- Constants
const LEN: usize = 32;

//---------------------------------------------------------------------------------------------------- Hash
#[derive(Zeroize,ZeroizeOnDrop,PartialEq,Debug)]
// Contains `Basic <original input in base64>`
pub struct Auth(Pin<String>);

impl Auth {
	/// Encode input in base64
	pub fn new(input: String) -> Self {
		Self(Pin::new(rpc::base64::encode_with_authorization_basic_header(input)))
	}

	pub fn as_str(&self) -> &str {
		self.as_str()
	}
}


//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
}
