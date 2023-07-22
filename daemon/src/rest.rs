//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use crate::hash::Hash;
use std::sync::Arc;
use std::net::SocketAddrV4;
use crate::config::Config;
use hyper::{
	Request,
	Response,
	body::Body,
};
use http::request::Parts;
use hyper::header::{
	CONTENT_LENGTH,
	CONTENT_TYPE,
	CONTENT_DISPOSITION,
};

//---------------------------------------------------------------------------------------------------- Constants
// Tells browsers to view files.
const VIEW_IN_BROWSER:     &str = "inline";
// Tells browsers to download files.
const DOWNLOAD_IN_BROWSER: &str = "attachment";

//---------------------------------------------------------------------------------------------------- REST Handler
pub async fn handle(
	parts:  Parts,
	body:   Body,
	addr:   SocketAddrV4,
	config: &'static Config,
) -> Result<Response<Body>, anyhow::Error> {
//	Ok(Response::new(Body::from(format!("{}", parts.uri))))
	const I: &'static [u8] = include_bytes!("/tmp/img.jpg");

	Ok(http::response::Builder::new()
//		.header(CONTENT_TYPE, "audio/flac")
		.header(CONTENT_TYPE, "image/jpg")
		.header(CONTENT_LENGTH, "3792807")
		.header(CONTENT_DISPOSITION, format!(r#"{VIEW_IN_BROWSER}; filename="hello world.mp3""#))
		.body(Body::from(I))?
	)
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
