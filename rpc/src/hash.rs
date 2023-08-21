//---------------------------------------------------------------------------------------------------- Use
use sha2::{Digest, Sha256};
use rand::Rng;
use zeroize::{ZeroizeOnDrop,Zeroize};
use std::pin::Pin;

//---------------------------------------------------------------------------------------------------- Constants
const LEN: usize = 32;

//---------------------------------------------------------------------------------------------------- Hash
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Zeroize,ZeroizeOnDrop,PartialEq)]
pub struct Hash {
	// Contains hash output (with salt).
	hash: PinBox,

	// The salt that was used.
	salt: PinBox,
}

#[cfg(not(debug_assertions))]
impl std::fmt::Debug for Hash {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "crate::hash::Hash")
	}
}

impl Hash {
	/// Hash an input with a random salt.
	pub fn new(input: String) -> Self {
		Self::with_salt(input, PinBox::rand())
	}

	pub(super) fn with_salt(input: String, salt: PinBox) -> Self {
		Self {
			hash: Self::hash(input, &salt),
			salt,
		}
	}

	fn hash(input: String, salt: &PinBox) -> PinBox {
		let mut hasher = Sha256::new();
		hasher.update(input);
		hasher.update(&*salt.0);
		PinBox(Box::pin(hasher.finalize().into()))
	}

	/// Compare `self` with a hash of another `String`.
	pub fn same(&self, new_input: String) -> bool {
		&self.hash == &Hash::hash(new_input, &self.salt)
	}
}

// Regular hash function.
// Used for `ZIP` names.
pub fn sha256(input: &str) -> String {
	let mut hasher = Sha256::new();
	hasher.update(input);
	hex::encode(hasher.finalize())
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Zeroize,ZeroizeOnDrop,PartialEq)]
pub(super) struct PinBox(Pin<Box<[u8; LEN]>>);

impl PinBox {
	fn zero() -> Self {
		Self(Box::pin([0; LEN]))
	}

	fn rand() -> Self {
		Self(Box::pin(rand::thread_rng().gen()))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	fn h1() -> Hash {
		Hash::with_salt("h1".into(), PinBox::zero())
	}

	fn h2() -> Hash {
		Hash::with_salt("h2".into(), PinBox::zero())
	}

	#[test]
	fn salt_rand() {
		if PinBox::rand() == PinBox::rand() {
			panic!("salt_rand()");
		}
	}

	#[test]
	fn same() {
		if h1() != h1() {
			panic!("same(), h1() != h1");
		}

		if h1() == h2() {
			panic!("same(), h1() == h2()");
		}
	}

	#[test]
	fn hash() {
		if !h1().same("h1".into()) {
			panic!(r#"!h1().same("h1".into())"#)
		}

		if !h2().same("h2".into()) {
			panic!(r#"!h2().same("h2".into())"#)
		}
	}
}
