// Generic HTTP responses.

//---------------------------------------------------------------------------------------------------- Use
use hyper::{
	Response,
	Body,
	header::{
		SERVER,
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
use std::borrow::Cow;
use serde::Serialize;
use crate::config::config;
use crate::constants::FESTIVALD_SERVER;

//---------------------------------------------------------------------------------------------------- Constants
// Tells browsers to view files.
const VIEW_IN_BROWSER: &str = "inline";
// Tells browsers to download files.
const DOWNLOAD_IN_BROWSER: &str = "attachment";
// Zip file MIME.
const MIME_ZIP: &str = "application/zip";

//---------------------------------------------------------------------------------------------------- REST Responses
pub fn rest_ok(bytes: Vec<u8>, name: &str, mime: &str) -> Response<Body> {
	match Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, mime)
		.header(CONTENT_LENGTH, bytes.len())
		.header(CONTENT_DISPOSITION, if config().direct_download { format!(r#"{DOWNLOAD_IN_BROWSER}; filename="{name}""#) } else { format!(r#"{VIEW_IN_BROWSER}; filename="{name}""#) })
		.body(Body::from(bytes))
	{
		Ok(r)  => r,
		Err(e) => server_err("Internal server error"),
	}
}

// Streaming body.
pub fn rest_stream(body: hyper::body::Body, name: &str, mime: &str, len: Option<u64>) -> Response<Body> {
	let mut b = Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, mime)
		.header(CONTENT_DISPOSITION, if config().direct_download { format!(r#"{DOWNLOAD_IN_BROWSER}; filename="{name}""#) } else { format!(r#"{VIEW_IN_BROWSER}; filename="{name}""#) });

	let mut b = if let Some(len) = len {
		b.header(CONTENT_LENGTH, len)
	} else {
		b
	};

	match b.body(body) {
		Ok(r)  => r,
		Err(e) => server_err("Internal server error"),
	}
}

// Streaming body for zip.
pub fn rest_zip(body: hyper::body::Body, name: &str, len: Option<u64>) -> Response<Body> {
	let mut b = Builder::new()
		.status(StatusCode::OK)
		.header(CONTENT_TYPE, MIME_ZIP)
		.header(CONTENT_DISPOSITION, format!(r#"{DOWNLOAD_IN_BROWSER}; filename="{name}""#));

	let mut b = if let Some(len) = len {
		b.header(CONTENT_LENGTH, len)
	} else {
		b
	};

	match b.body(body) {
		Ok(r)  => r,
		Err(e) => server_err("Internal server error"),
	}
}

pub fn rest_ok_msg(msg: &'static str) -> Response<Body> {
	// SAFETY: This `.unwraps()` are safe. The content is static.
	Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, "text/html; charset=UTF-8")
		.header(CONTENT_LENGTH, msg.len())
		.header(CONTENT_DISPOSITION, VIEW_IN_BROWSER)
		.body(Body::from(msg))
		.unwrap()
}

//---------------------------------------------------------------------------------------------------- REST Error Responses
// Unknown requests (404)
pub fn not_found(msg: &'static str) -> Response<Body> {
	// SAFETY: This `.unwraps()` are safe. The content is static.
	Builder::new()
		.status(StatusCode::NOT_FOUND)
		.header(SERVER, FESTIVALD_SERVER)
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
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, TEXT_PLAIN_UTF_8.essence_str())
		.header(CONTENT_LENGTH, msg.len())
		.header(WWW_AUTHENTICATE, r#"Basic realm="Acesss to REST API", charset="UTF-8""#)
		.body(Body::from(msg))
		.unwrap()
}

// Forbidden request (403)
pub fn forbidden(msg: &'static str) -> Response<Body> {
	// SAFETY: This `.unwraps()` are safe. The content is static.
	Builder::new()
		.status(StatusCode::FORBIDDEN)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, TEXT_PLAIN_UTF_8.essence_str())
		.header(CONTENT_LENGTH, msg.len())
		.body(Body::from(msg))
		.unwrap()
}

// Method now allowed (405)
pub fn method_not_allowed(msg: &'static str) -> Response<Body> {
	// SAFETY: This `.unwraps()` are safe. The content is static.
	Builder::new()
		.status(StatusCode::METHOD_NOT_ALLOWED)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, TEXT_PLAIN_UTF_8.essence_str())
		.header(CONTENT_LENGTH, msg.len())
		.body(Body::from(msg))
		.unwrap()
}

// Internal server error (500)
pub fn server_err(msg: &'static str) -> Response<Body> {
	// SAFETY: This `.unwraps()` are safe. The content is static.
	Builder::new()
		.status(StatusCode::INTERNAL_SERVER_ERROR)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, TEXT_PLAIN_UTF_8.essence_str())
		.header(CONTENT_LENGTH, msg.len())
		.body(Body::from(msg))
		.unwrap()
}

// We're in the middle of a `Collection` reset.
pub fn resetting_rest() -> Response<Body> {
	const MSG: &str = "Currently resetting the Collection";
	// SAFETY: This `.unwraps()` are safe. The content is static.
	Builder::new()
		.status(StatusCode::SERVICE_UNAVAILABLE)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, TEXT_PLAIN_UTF_8.essence_str())
		.header(CONTENT_LENGTH, MSG.len())
		.body(Body::from(MSG))
		.unwrap()
}

//---------------------------------------------------------------------------------------------------- JSON-RPC Responses
pub fn result_ok<'a>(id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	let r = json_rpc::Response::result(Cow::<rpc::resp::Status>::Owned(rpc::resp::Status(())), id.clone());
	let r = match serde_json::to_vec_pretty(&r) {
		Ok(r)  => r,
		Err(e) => return internal_error(id),
	};

	match Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, r.len())
		.body(Body::from(r))
	{
		Ok(r)  => r,
		Err(_) => internal_error(id),
	}
}

pub fn result<'a, T>(t: T, id: Option<json_rpc::Id<'a>>) -> Response<Body>
where
	T: Clone + Serialize,
{
	let r = json_rpc::Response::result(Cow::Borrowed(&t), id.clone());
	let r = match serde_json::to_vec_pretty(&r) {
		Ok(r)  => r,
		Err(e) => return internal_error(id),
	};

	match Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, r.len())
		.body(Body::from(r))
	{
		Ok(r)  => r,
		Err(_) => internal_error(id),
	}
}

pub fn error<'a>(code: i32, msg: &'static str, id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	let e = json_rpc::error::ErrorObject {
		code: json_rpc::error::ErrorCode::ServerError(code),
		message: Cow::Borrowed(msg),
		data: None,
	};
	let r = json_rpc::Response::<()>::error(e, id.clone());
	let r = match serde_json::to_vec_pretty(&r) {
		Ok(r)  => r,
		Err(e) => return internal_error(id),
	};

	match Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, r.len())
		.body(Body::from(r))
	{
		Ok(r)  => r,
		Err(_) => internal_error(id),
	}
}

// We're in the middle of a `Collection` reset.
pub fn resetting<'a>(code: i32, msg: &'static str, id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	let e = json_rpc::error::ErrorObject {
		code: json_rpc::error::ErrorCode::ServerError(code),
		message: Cow::Borrowed(msg),
		data: None,
	};
	let r = json_rpc::Response::<()>::error(e, id.clone());
	let r = match serde_json::to_vec_pretty(&r) {
		Ok(r)  => r,
		Err(e) => return internal_error(id),
	};

	match Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, r.len())
		.body(Body::from(r))
	{
		Ok(r)  => r,
		Err(_) => internal_error(id),
	}
}

pub fn result_cache<'a>(string: &str, id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	let id_str = match &id {
		Some(id) => {
			match id {
				json_rpc::Id::Null   => r#"  "id": null"#.to_string(),
				json_rpc::Id::Num(n) => format!(r#"  "id": {n}"#),
				json_rpc::Id::Str(s) => format!(r#"  "id": "{s}""#),
			}
		},
		None => r#"  "id": null"#.to_string(),
	};

	// We popped off the `"id": null\n}` off the generic
	// cache and now we must add back the real ID.
	let mut r = string.to_string();
	r += &id_str;
	r += "\n}";

	match Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, r.len())
		.body(Body::from(r))
	{
		Ok(r)  => r,
		Err(_) => internal_error(id),
	}
}

//---------------------------------------------------------------------------------------------------- JSON-RPC specific error response
pub fn parse_error<'a>(id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	// SAFETY: These `.unwraps()` are safe. The content is static.

	let s = serde_json::to_vec_pretty(&json_rpc::Response::<()>::parse_error(id)).unwrap();

	Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, s.len())
		.body(Body::from(s))
		.unwrap()
}

pub fn invalid_request<'a>(id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	// SAFETY: These `.unwraps()` are safe. The content is static.

	let s = serde_json::to_vec_pretty(&json_rpc::Response::<()>::invalid_request(id)).unwrap();

	Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, s.len())
		.body(Body::from(s))
		.unwrap()
}

pub fn method_not_found<'a>(id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	// SAFETY: These `.unwraps()` are safe. The content is static.

	let s = serde_json::to_vec_pretty(&json_rpc::Response::<()>::method_not_found(id)).unwrap();

	Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, s.len())
		.body(Body::from(s))
		.unwrap()
}

pub fn invalid_params<'a>(id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	// SAFETY: These `.unwraps()` are safe. The content is static.

	let s = serde_json::to_vec_pretty(&json_rpc::Response::<()>::invalid_params(id)).unwrap();

	Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, s.len())
		.body(Body::from(s))
		.unwrap()
}

pub fn internal_error<'a>(id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	// SAFETY: These `.unwraps()` are safe. The content is static.

	let s = serde_json::to_vec_pretty(&json_rpc::Response::<()>::internal_error(id)).unwrap();

	Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, s.len())
		.body(Body::from(s))
		.unwrap()
}

// Unauthorized JSON-RPC request, HTTP code is still OK.
pub fn unauth_rpc<'a>(code: i32, msg: &'static str, id: Option<json_rpc::Id<'a>>) -> Response<Body> {
	let e = json_rpc::error::ErrorObject {
		code: json_rpc::error::ErrorCode::ServerError(code),
		message: Cow::Borrowed(msg),
		data: None,
	};

	let r = json_rpc::Response::<()>::error(e, id.clone());
	let r = match serde_json::to_vec_pretty(&r) {
		Ok(r)  => r,
		Err(e) => return internal_error(id),
	};

	match Builder::new()
		.status(StatusCode::OK)
		.header(SERVER, FESTIVALD_SERVER)
		.header(CONTENT_TYPE, APPLICATION_JSON.essence_str())
		.header(CONTENT_LENGTH, r.len())
		.header(WWW_AUTHENTICATE, r#"Basic realm="Access to JSON-RPC API""#)
		.body(Body::from(r))
	{
		Ok(r)  => r,
		Err(_) => internal_error(id),
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
