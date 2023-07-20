//---------------------------------------------------------------------------------------------------- Use
use sha3::{Digest, Sha3_256};
use rand::Rng;

//---------------------------------------------------------------------------------------------------- __NAME__
/// Get random 32 bytes.
pub fn r32() -> [u8; 32] {
	rand::thread_rng().gen()
}

/// Hash an input with a salt.
pub fn sha3_256_salt(input: String, salt: &[u8; 32]) -> String {
	// Hash.
	let mut hasher = Sha3_256::new();
	hasher.update(input);
	hasher.update(salt);

	// Hex encode.
	hex::encode(hasher.finalize())
}

pub fn hash_same(hash: &str, salt: &[u8; 32], new_input: String) -> bool {
	hash == sha3_256_salt(new_input, salt)
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn same() {
		let input = "input".to_string();
		let salt  = [1; 32];
		let hash  = sha3_256_salt(input.clone(), &salt);
		assert!(hash_same(&hash, &salt, input));
		assert!(!hash_same(&hash, &salt, "inputt".into()));
	}

	#[test]
	fn rand() {
		assert_ne!(r32(), r32());
	}

	#[test]
	fn hash() {
		assert_eq!(sha3_256_salt("input".into(), &[1; 32]).len(), 64);
		assert_eq!(sha3_256_salt("input".into(), &[1; 32]), "045cae48d449c3c4e20a8f413c8e7f921b6858bf4ca7fa91f81f17f23736b8f0");

		assert_ne!(sha3_256_salt("input".into(), &[1; 32]), sha3_256_salt("input".into(), &[0; 32]));
		assert_ne!(sha3_256_salt("inputt".into(), &[1; 32]), sha3_256_salt("input".into(), &[1; 32]));
	}
}
