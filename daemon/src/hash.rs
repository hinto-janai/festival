//---------------------------------------------------------------------------------------------------- Use
use sha3::{Digest, Sha3_256};
use rand::Rng;
use zeroize::{ZeroizeOnDrop,Zeroize};

//---------------------------------------------------------------------------------------------------- __NAME__
#[cfg_attr(debug_assertions, derive(Debug,PartialEq))]
#[derive(Zeroize,ZeroizeOnDrop)]
pub struct Hash {
	// Contains hex-encoded hash input (with salt).
	hash: String,

	// The salt used.
	salt: Salt,
}

#[cfg_attr(debug_assertions, derive(Debug,PartialEq))]
#[derive(Clone,Zeroize,ZeroizeOnDrop)]
pub(super) struct Salt([u8; 32]);

impl Salt {
	fn new() -> Self {
		Self(rand::thread_rng().gen())
	}
}

impl Hash {
	/// Hash an input with a random salt.
	pub fn new(input: String) -> Self {
		Self::with_salt(input, Salt::new())
	}

	pub(super) fn with_salt(input: String, salt: Salt) -> Self {
		// Hash.
		let mut hasher = Sha3_256::new();
		hasher.update(input);
		hasher.update(salt.0);

		Self {
			// Hex encode.
			hash: hex::encode(hasher.finalize()),
			salt,
		}
	}

	/// Compare `self` with a hash of another `String`.
	pub fn same(&self, new_input: String) -> bool {
		let new = Hash::with_salt(new_input, self.salt.clone());
		let cmp = &self.hash == &new.hash;

		// Zero the memory.
		drop(new);

		cmp
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	fn h1() -> Hash {
		Hash::with_salt("h1".into(), Salt([0;32]))
	}

	fn h2() -> Hash {
		Hash::with_salt("h2".into(), Salt([0;32]))
	}

	#[test]
	fn same() {
		assert_eq!(h1(), h1());
		assert_ne!(h1(), h2());
	}

	#[test]
	fn hash() {
		assert_eq!(h1().hash.len(), 64);
		assert_eq!(h2().hash, "05db833ef2c1b99a2a345dfb0987e9917af31206142120025b248c939879dffe");

		assert_eq!(h2().hash.len(), 64);
		assert_eq!(h2().hash, "05db833ef2c1b99a2a345dfb0987e9917af31206142120025b248c939879dffe");
	}
}
