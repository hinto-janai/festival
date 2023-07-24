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

//---------------------------------------------------------------------------------------------------- JSON-RPC Handler
pub async fn handle(
	parts:  Parts,
	body:   Body,
	addr:   SocketAddrV4,
) -> Result<Response<Body>, anyhow::Error> {
	let body = hyper::body::to_bytes(body).await?;

	if let Ok(a) = serde_json::from_slice::<json_rpc::Request<String, String>>(&body) {
		let string = a.to_string();
		println!("{string}");
		Ok(Response::new(Body::from(string)))
	} else {
		Err(anyhow!("oops"))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
