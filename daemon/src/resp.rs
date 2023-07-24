// Generic HTTP responses.

//---------------------------------------------------------------------------------------------------- Use
use hyper::{
	Response,
	Body,
	header::{
		CONTENT_LENGTH,
		CONTENT_TYPE,
		CONTENT_DISPOSITION,
		WWW_AUTHENTICATE,
	},
};
use http::{
	request::Parts,
	response::Builder,
	StatusCode,
};
use mime::TEXT_PLAIN_UTF_8;

//---------------------------------------------------------------------------------------------------- Responses
// Unknown requests (404)
pub fn not_found(msg: &'static str) -> Response<Body> {
	// SAFETY: This `.unwraps()` are safe. The content is static.
	Builder::new()
		.status(StatusCode::NOT_FOUND)
		.header(CONTENT_TYPE, TEXT_PLAIN_UTF_8.essence_str())
		.header(CONTENT_LENGTH, msg.len())
		.body(Body::from(msg))
		.unwrap()
}


// Unauthorized request (401)
pub fn unauthorized(msg: &'static str) -> Response<Body> {
	// SAFETY: This `.unwraps()` are safe. The content is static.
	Builder::new()
		.status(StatusCode::UNAUTHORIZED)
		.header(CONTENT_TYPE, TEXT_PLAIN_UTF_8.essence_str())
		.header(CONTENT_LENGTH, msg.len())
		.header(WWW_AUTHENTICATE, r#"Basic realm="User Visible Realm", charset="UTF-8""#)
		.body(Body::from(msg))
		.unwrap()
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
