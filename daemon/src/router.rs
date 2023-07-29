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
	config::{AUTH,config,Config,ConfigBuilder},
	statics::{
		ConnectionToken,
		RESETTING,
		TOTAL_CONNECTIONS,
		TOTAL_REQUESTS,
	},
	hash::Hash,
	ptr::CollectionPtr,
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
	atomic_add,
	atomic_store,
	recv,send,
	debug_panic,
};
use std::sync::atomic::{
	AtomicPtr,
	Ordering,
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

	// Create `task` <-> `router` back channel
	// for `Collection` reset results.
	let (to_router, mut from_task) = tokio::sync::mpsc::channel::<Arc<Collection>>(1);

	// These last forever.
	let (
		LISTENER,
		TO_ROUTER,
		FROM_TASK,
	): (
		&'static TcpListener,
		&'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
		&'static mut tokio::sync::mpsc::Receiver::<Arc<Collection>>,
	) = (
		Box::leak(Box::new(listener)),
		Box::leak(Box::new(to_router)),
		Box::leak(Box::new(from_task)),
	);

	// Wait until `Kernel` has given us `Arc<Collection>`.
	let mut collection = loop {
		match recv!(FROM_KERNEL) {
			KernelToFrontend::NewCollection(c) => break c,
			_ => debug_panic!("wrong kernel msg"),
		}
	};

	// Create the 1 and only "global" `CollectionPtr`.
	// See `ptr.rs` for why this exists.
	let ptr = std::ptr::addr_of_mut!(collection);
	let ptr = AtomicPtr::new(ptr);
	let ptr = CollectionPtr(ptr);
	let mut COLLECTION_PTR: &'static CollectionPtr = Box::leak(Box::new(ptr));

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
			// Hang until `TCP` connection or
			// we received an `Arc<Collection>`
			// from one of the `tasks` we spawned.
			let (stream, addr) = loop {
				tokio::select! {
					biased; // Top-to-bottom.

					// Accept TCP stream, and get peer's IP.
					l = LISTENER.accept() => {
						match l {
							Ok((s, a)) => { atomic_add!(TOTAL_CONNECTIONS, 1); break (s, a) },
							Err(e)     => error!("Router - TCP stream error: {e}"),
						}
					},

					// We received a new `Collection` from a `task`.
					c = FROM_TASK.recv() => {
						if let Some(mut c) = c {
							info!("Router - New Collection received");

							// Set the atomic pointer to a dummy.
							let mut dummy = Collection::dummy();
							let ptr = std::ptr::addr_of_mut!(dummy);
							COLLECTION_PTR.0.store(ptr, Ordering::SeqCst); //-------> newly spawned tasks
                                                                                   // will be `.arc()`'ing
							                                                       // and receiving the
							// Overwrite the "real" `Arc<Collection>`              // dummy collection
							collection = c;                                        // until...
							                                                       //
                                                                                   //
							// Ok, we're safe, atomically update the pointer back. //
							let ptr = std::ptr::addr_of_mut!(collection);          //
							COLLECTION_PTR.0.store(ptr, Ordering::SeqCst); // <------
						} else {
							debug_panic!("Router - New Collection message but it was None");
						}
					},
				}
			};

			// Only accept IPv4.
			let addr = match addr {
				std::net::SocketAddr::V4(addr) => {
					info!("Router - New connection: [{}]", addr.ip());
					addr
				},
				std::net::SocketAddr::V6(addr) => {
					warn!("Router - Skipping IPv6 connection: [{}]", addr.ip());
					sleep_on_fail().await;
					continue;
				},
			};

			let ip = addr.ip();

			// If we have an exclusive IP list, deny non-contained IP connections.
			if let Some(ips) = &CONFIG.exclusive_ips {
				if !ips.contains(ip) {
					info!("Router - IP not in exclusive list, skipping [{ip}]");
					sleep_on_fail().await;
					continue;
				}
			}

			// If we are past the connection limit, wait until some
			// tasks are done before serving new connections.
			if let Some(max) = CONFIG.max_connections {
				if crate::statics::connections() > max {
					// Only log once.
					warn!("Router - Past max connections [{max}], waiting before serving [{ip}]...");

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

			tokio::task::spawn(async move {
				https(
					ConnectionToken::new(),
					stream,
					addr,
					ACCEPTOR,
					COLLECTION_PTR,
					TO_KERNEL,
					FROM_KERNEL,
					TO_ROUTER,
				).await;
			});
		}
	// Else If `HTTP`, start main `HTTP` loop (the exact same, but without TLS).
	} else {
		listening!();

		loop {
			let (stream, addr) = impl_loop!();

			tokio::task::spawn(async move {
				http(
					ConnectionToken::new(),
					stream,
					addr,
					COLLECTION_PTR,
					TO_KERNEL,
					FROM_KERNEL,
					TO_ROUTER,
				).await;
			});
		}
	}
}

//---------------------------------------------------------------------------------------------------- Handle HTTP
// Handle HTTP requests.
async fn http(
	_c:             ConnectionToken,
	stream:         TcpStream,
	addr:           SocketAddrV4,
	COLLECTION_PTR: &'static CollectionPtr,
	TO_KERNEL:      &'static Sender<FrontendToKernel>,
	FROM_KERNEL:    &'static Receiver<KernelToFrontend>,
	TO_ROUTER:      &'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
) {
	// For why this is `CollectionPtr` instead
	// of `Arc<Collection>`, see `ptr.rs`.
	if let Err(e) = Http::new()
		.serve_connection(stream, service_fn(|r| route(r, addr, COLLECTION_PTR, TO_KERNEL, FROM_KERNEL, TO_ROUTER)))
		.await
	{
		error!("Task - HTTP error for [{}]: {e}", addr.ip());
	}
}

//---------------------------------------------------------------------------------------------------- Handle HTTPS
// Handle HTTPS requests.
async fn https(
	_c:             ConnectionToken,
	stream:         TcpStream,
	addr:           SocketAddrV4,
	acceptor:       &'static TlsAcceptor,
	COLLECTION_PTR: &'static CollectionPtr,
	TO_KERNEL:      &'static Sender<FrontendToKernel>,
	FROM_KERNEL:    &'static Receiver<KernelToFrontend>,
	TO_ROUTER:      &'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
) {
	let stream = match acceptor.accept(stream).await {
		Ok(s)  => s,
		Err(e) => {
			error!("TLS error for [{}]: {e}", addr.ip());
			return;
		},
	};

	// For why this is `CollectionPtr` instead
	// of `Arc<Collection>`, see `ptr.rs`.
	if let Err(e) = Http::new()
		.serve_connection(stream, service_fn(|r| route(r, addr, COLLECTION_PTR, TO_KERNEL, FROM_KERNEL, TO_ROUTER)))
		.await
	{
		error!("Task - HTTPS error for [{}]: {e}", addr.ip());
	}
}

//---------------------------------------------------------------------------------------------------- Handle Routes
// Route requests to other functions.
async fn route(
	req:            Request<Body>,
	addr:           SocketAddrV4,
	COLLECTION_PTR: &'static CollectionPtr,
	TO_KERNEL:      &'static Sender<FrontendToKernel>,
	FROM_KERNEL:    &'static Receiver<KernelToFrontend>,
	TO_ROUTER:      &'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
) -> Result<Response<Body>, anyhow::Error> {
	atomic_add!(TOTAL_REQUESTS, 1);

	let (mut parts, body) = req.into_parts();

//	println!("{parts:#?}");
//	println!("{body:#?}");

	// AUTHORIZATION.
	if let Some(resp) = auth(&mut parts).await {
		return Ok(resp);
	}

	if parts.uri == "/" && parts.method == hyper::Method::POST {
		crate::rpc::handle(parts, body, addr, COLLECTION_PTR, TO_KERNEL, FROM_KERNEL, TO_ROUTER).await
	} else if parts.method == hyper::Method::GET {
		if config().rest {
			crate::rest::handle(parts, COLLECTION_PTR).await
		} else {
			Ok(resp::not_found("REST is disabled"))
		}
	} else {
		Ok(resp::not_found("Invalid request"))
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
					return Some(resp::unauthorized("Authorization value is non-utf8"));
				},
			};

			// Check if the hash matches our existing one.
			if !hash.same(string) {
				sleep_on_fail().await;
				return Some(resp::unauthorized("Authorization failed"));
			}
		},

		// AUTH header doesn't exist, reject this request.
		None => {
			sleep_on_fail().await;
			return Some(resp::unauthorized("Missing authorization"));
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
		trace!("Task - Sleeping for {millis} millis");
		tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
