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
use mime::{
	TEXT_PLAIN_UTF_8,
	APPLICATION_JSON,
};

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

//---------------------------------------------------------------------------------------------------- JSON-RPC specific error response
pub fn parse_error<'a>(id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	// SAFETY: These `.unwraps()` are safe. The content is static.

	let s = serde_json::to_string(&json_rpc::Response::<()>::parse_error(id)).unwrap();

	Builder::new()
		.status(StatusCode::ACCEPTED)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, s.len())
		.body(Body::from(s))
		.unwrap()
}

pub fn invalid_request<'a>(id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	// SAFETY: These `.unwraps()` are safe. The content is static.

	let s = serde_json::to_string(&json_rpc::Response::<()>::invalid_request(id)).unwrap();

	Builder::new()
		.status(StatusCode::ACCEPTED)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, s.len())
		.body(Body::from(s))
		.unwrap()
}

pub fn method_not_found<'a>(id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	// SAFETY: These `.unwraps()` are safe. The content is static.

	let s = serde_json::to_string(&json_rpc::Response::<()>::method_not_found(id)).unwrap();

	Builder::new()
		.status(StatusCode::ACCEPTED)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, s.len())
		.body(Body::from(s))
		.unwrap()
}

pub fn invalid_params<'a>(id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	// SAFETY: These `.unwraps()` are safe. The content is static.

	let s = serde_json::to_string(&json_rpc::Response::<()>::invalid_params(id)).unwrap();

	Builder::new()
		.status(StatusCode::ACCEPTED)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, s.len())
		.body(Body::from(s))
		.unwrap()
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
