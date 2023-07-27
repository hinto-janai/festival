//---------------------------------------------------------------------------------------------------- Use
use zeroize::Zeroize;
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use std::sync::Arc;
use std::path::PathBuf;
use std::net::{
	Ipv4Addr,
	SocketAddrV4,
};
use hyper::{
	body::Body,
	server::conn::Http,
	service::service_fn,
	http::{
		Request,
		Response,
		StatusCode,
	},
};
use http::{
	header::{
		AUTHORIZATION,
		CONTENT_TYPE,
		CONTENT_LENGTH,
		WWW_AUTHENTICATE,
	},
	response::Builder,
	request::Parts,
};
use mime::{
	TEXT_PLAIN_UTF_8,
};
use crossbeam::channel::{
	Sender,Receiver,
};
use shukusai::{
	collection::Collection,
	kernel::{
		FrontendToKernel,
		KernelToFrontend,
	},
	constants::DASH,
};
use crate::{
	config::AUTH,
	statics::ConnectionToken,
	config::{config,Config,ConfigBuilder},
	hash::Hash,
};
use tokio_native_tls::{
	TlsAcceptor,
	TlsStream,
};
use tokio::net::{
	TcpListener,
	TcpStream,
};
use crate::resp;
use benri::{
	recv,send,
	debug_panic,
};

//---------------------------------------------------------------------------------------------------- Router
#[tokio::main]
pub async fn init(
	CONFIG:      &'static Config,
	TO_KERNEL:   &'static Sender<FrontendToKernel>,
	FROM_KERNEL: &'static Receiver<KernelToFrontend>,
) {
	// Bind to address.
	let addr     = SocketAddrV4::new(CONFIG.ip, CONFIG.port);
	let listener = match TcpListener::bind(addr).await {
		Ok(l)  => l,
		Err(e) => crate::exit!("could not bind to [{addr}]: {e}"),
	};

	// Instead of branching everytime for HTTP/HTTPS or
	// using dynamic dispatch or an enum and matching it,
	// we'll just "implement" the main loop "twice".
	//
	// We branch below _once_ depending on HTTP/HTTPS, then
	// we enter this loop. No matching every request, no
	// dynamic dispatch :)
	macro_rules! impl_loop {
		() => {{
			// Begin router loop.
			// Accept TCP stream, and get peer's IP.
			let (stream, addr) = match listener.accept().await {
				Ok(ok) => ok,
				Err(e) => { error!("tcp stream error: {e}"); continue; },
			};

			// Only accept IPv4.
			let addr = match addr {
				std::net::SocketAddr::V4(addr) => {
					info!("new connection: [{}]", addr.ip());
					addr
				},
				std::net::SocketAddr::V6(addr) => {
					warn!("skipping ipv6 connection: [{}]", addr.ip());
					sleep_on_fail().await;
					continue;
				},
			};

			let ip = addr.ip();

			// If we have an exclusive IP list, deny non-contained IP connections.
			if let Some(ips) = &CONFIG.exclusive_ips {
				if !ips.contains(ip) {
					info!("ip not in exclusive list, skipping [{ip}]");
					sleep_on_fail().await;
					continue;
				}
			}

			// If we are past the connection limit, wait until some
			// tasks are done before serving new connections.
			if let Some(max) = CONFIG.max_connections {
				if crate::statics::connections() > max {
					// Only log once.
					warn!("past max connections [{max}], waiting before serving [{ip}]...");

					while crate::statics::connections() > max {
						tokio::time::sleep(std::time::Duration::from_millis(10)).await;
					}
				}
			}

			(stream, addr)
		}}
	}

	// Prints `protocol`, `ip`, and `port` in color.
	macro_rules! listening {
		() => {{
			let protocol = if CONFIG.tls { "https" } else { "http" };

			const PURPLE: &str = "\x1b[1;95m";
			const YELLOW: &str = "\x1b[1;93m";
			const BLUE:   &str = "\x1b[1;94m";
			const WHITE:  &str = "\x1b[1;97m";
			const OFF:    &str = "\x1b[0m";
			let listening = format!("festivald listening on {PURPLE}{protocol}{OFF}://{YELLOW}{}{OFF}:{BLUE}{}{OFF}", addr.ip(), addr.port());
			println!("{WHITE}{0}{OFF}\n{listening}\n{WHITE}{0}{OFF}", "=".repeat(listening.len() - 33));
		}}
	}

	// Wait until `Kernel` has given us `Arc<Collection>`.
	let collection = loop {
		match recv!(FROM_KERNEL) {
			KernelToFrontend::NewCollection(c) => break c,
			_ => debug_panic!("wrong kernel msg"),
		}
	};
//	drop(collection);
//
//	// TODO: for testing, cleanup later.
//	send!(TO_KERNEL, FrontendToKernel::NewCollection(vec![PathBuf::from("/home/main/git/festival/assets/audio")]));
//	let collection = loop {
//		match recv!(FROM_KERNEL) {
//			KernelToFrontend::NewCollection(c) => break c,
//			_ => debug_panic!("wrong kernel msg"),
//		}
//	};
//	send!(TO_KERNEL, FrontendToKernel::AddQueueArtist((64_u8.into(), Default::default(), false, 0)));
//	send!(TO_KERNEL, FrontendToKernel::Play);
//	send!(TO_KERNEL, FrontendToKernel::Volume(shukusai::audio::Volume::new(1)));

	// If `HTTPS`, start main `HTTPS` loop.
	if CONFIG.tls {
		// Sanity-checks.
		let path_cert = match &CONFIG.certificate {
			Some(p) => p,
			None    => crate::exit!("TLS enabled but no certificate PATH provided"),
		};

		let path_key = match &CONFIG.key {
			Some(p) => p,
			None    => crate::exit!("TLS enabled but no key PATH provided"),
		};

		let ACCEPTOR: &'static TlsAcceptor = match crate::cert::get_tls_acceptor(&path_cert, &path_key) {
			Ok(a)  => a,
			Err(e) => crate::exit!("failed to create TLS acceptor: {e}"),
		};

		listening!();

		loop {
			let (stream, addr) = impl_loop!();

			let collection = Arc::clone(&collection);
			tokio::task::spawn(async move {
				https(ConnectionToken::new(), stream, addr, ACCEPTOR, collection).await;
			});
		}
	// Else If `HTTP`, start main `HTTP` loop (the exact same, but without TLS).
	} else {
		listening!();

		loop {
			let (stream, addr) = impl_loop!();

			let collection = Arc::clone(&collection);
			tokio::task::spawn(async move {
				http(ConnectionToken::new(), stream, addr, collection).await;
			});
		}
	}
}

//---------------------------------------------------------------------------------------------------- Handle HTTP
// Handle HTTP requests.
async fn http(
	_c:         ConnectionToken,
	stream:     TcpStream,
	addr:       SocketAddrV4,
	collection: Arc<Collection>,
) {
	if let Err(e) = Http::new()
		.serve_connection(stream, service_fn(|r| route(r, addr, Arc::clone(&collection))))
		.await
	{
		error!("HTTP error for [{}]: {e}", addr.ip());
	}
}

//---------------------------------------------------------------------------------------------------- Handle HTTPS
// Handle HTTPS requests.
async fn https(
	_c:         ConnectionToken,
	stream:     TcpStream,
	addr:       SocketAddrV4,
	acceptor:   &'static TlsAcceptor,
	collection: Arc<Collection>,
) {
	let stream = match acceptor.accept(stream).await {
		Ok(s)  => s,
		Err(e) => { error!("TLS error for [{}]: {e}", addr.ip()); return; },
	};

	if let Err(e) = Http::new()
		.serve_connection(stream, service_fn(|r| route(r, addr, Arc::clone(&collection))))
		.await
	{
		error!("HTTPS error for [{}]: {e}", addr.ip());
	}
}

//---------------------------------------------------------------------------------------------------- Handle Routes
// Route requests to other functions.
async fn route(
	req:        Request<Body>,
	addr:       SocketAddrV4,
	collection: Arc<Collection>,
) -> Result<Response<Body>, anyhow::Error> {
	let (mut parts, body) = req.into_parts();

//	println!("{parts:#?}");
//	println!("{body:#?}");

	// AUTHORIZATION.
	if let Some(resp) = auth(&mut parts).await {
		return Ok(resp);
	}

	if parts.uri == "/" && parts.method == hyper::Method::POST {
		crate::rpc::handle(parts, body, addr, collection).await
	} else if parts.method == hyper::Method::GET {
		if config().rest {
			crate::rest::handle(parts, collection).await
		} else {
			Ok(resp::not_found("rest is disabled"))
		}
	} else {
		Ok(resp::not_found("invalid request"))
	}
}

//---------------------------------------------------------------------------------------------------- Auth
// Verify authentication, ask for it, or ignore if none is set in our config.
async fn auth(parts: &mut Parts) -> Option<Response<Body>> {
	// If auth stuff isn't set in user's config, skip this.
	let Some(hash) = AUTH.get() else { return None };

	match parts.headers.remove(AUTHORIZATION) {
		// AUTH header exists.
		Some(s) => {
			// Attempt to turn into UTF-8 string.
			let string = match String::from_utf8(s.as_bytes().into()) {
				Ok(s)  => s,
				Err(e) => {
					sleep_on_fail().await;
					return Some(resp::unauthorized("authorization value is non-utf8"));
				},
			};

			// Check if the hash matches our existing one.
			if !hash.same(string) {
				sleep_on_fail().await;
				return Some(resp::unauthorized("authorization failed"));
			}
		},

		// AUTH header doesn't exist, reject this request.
		None => {
			sleep_on_fail().await;
			return Some(resp::unauthorized("missing authorization"));
		},
	}

	// If we're here, that means AUTH went OK.
	None
}

//---------------------------------------------------------------------------------------------------- Sleep
// Sleep for a random while.
// Used for timing out requests, preventing timing attacks, etc.
async fn sleep_on_fail() {
	use rand::{Rng,thread_rng};

	if let Some(end) = config().sleep_on_fail {
		let millis = thread_rng().gen_range(0..end);
		trace!("sleeping for {millis} millis");
		tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
	}
}

// Same as above but used for spawned `task`.
//
// Since this function is used before a task exits,
// we should drop the `Collection` as to not block any reset requests.
async fn sleep_on_fail_task(c: Arc<Collection>) {
	drop(c);
	sleep_on_fail().await;
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
