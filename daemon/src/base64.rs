//---------------------------------------------------------------------------------------------------- Use
use base64::{
	Engine,
	engine::general_purpose::STANDARD,
};
use zeroize::Zeroize;

//---------------------------------------------------------------------------------------------------- Base64 Encode
/// 1. Takes in `String` input
/// 2. Prefixes it with `Basic `
/// 3. Encodes it in `base64`
pub fn encode_with_authorization_basic_header(mut input: String) -> String {
	let mut encode = STANDARD.encode(input.as_bytes());

	input.zeroize();

	let basic = format!("Basic {encode}");

	encode.zeroize();

	basic
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn encode() {
		const INPUT: &str = "my_username:my_password";
		const EXPECTED: &str = "Basic bXlfdXNlcm5hbWU6bXlfcGFzc3dvcmQ=";

		let encoded = encode_with_authorization_basic_header(INPUT.into());

		assert_eq!(encoded, EXPECTED);
	}
}
