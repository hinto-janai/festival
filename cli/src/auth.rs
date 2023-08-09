//---------------------------------------------------------------------------------------------------- Use
use zeroize::{ZeroizeOnDrop,Zeroize};
use std::pin::Pin;
use serde::Serialize;

//---------------------------------------------------------------------------------------------------- Hash
#[derive(Zeroize,ZeroizeOnDrop,PartialEq,Debug,Serialize)]
// Contains `Basic <original input in base64>`
pub struct Auth(String);

impl Auth {
	/// Encode input in base64
	pub fn new(input: String) -> Self {
		Self(rpc::base64::encode_with_authorization_basic_header(input))
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
