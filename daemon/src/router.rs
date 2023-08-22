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
	ok,
	atomic_add,
	atomic_store,
	recv,send,
	debug_panic,
};
use std::net::SocketAddr;
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

	// Create `task` <-> `router` back channel for `sys_*`.
	let (to_router_sys, mut from_task_sys) = tokio::sync::mpsc::channel::<()>(1);

	// Create `task` <-> `router` back channel
	// for `Collection` reset results.
	let (to_router_collection, mut from_task_collection) = tokio::sync::mpsc::channel::<Arc<Collection>>(1);

	// These last forever.
	let (
		LISTENER,
		TO_ROUTER_SYS,
		FROM_TASK_SYS,
		TO_ROUTER_COLLECTION,
		FROM_TASK_COLLECTION,
	): (
		&'static TcpListener,
		&'static tokio::sync::mpsc::Sender::<()>,
		&'static mut tokio::sync::mpsc::Receiver::<()>,
		&'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
		&'static mut tokio::sync::mpsc::Receiver::<Arc<Collection>>,
	) = (
		Box::leak(Box::new(listener)),
		Box::leak(Box::new(to_router_sys)),
		Box::leak(Box::new(from_task_sys)),
		Box::leak(Box::new(to_router_collection)),
		Box::leak(Box::new(from_task_collection)),
	);

	// Wait until `Kernel` has given us `Arc<Collection>`.
	let mut collection = loop {
		match recv!(FROM_KERNEL) {
			KernelToFrontend::NewCollection(c) => break c,
			_ => debug_panic!("wrong kernel msg"),
		}
	};

	// Create RPC cache.
	crate::rpc::cache_set_all(&collection).await;

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
					c = FROM_TASK_COLLECTION.recv() => {
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

					// We got `CTRL+C` in terminal.
					_ = tokio::signal::ctrl_c() => {
						debug!("Router - received CTRL+C");
						let c = Arc::clone(&collection);
						tokio::task::spawn(async move {
							crate::shutdown::shutdown(TO_KERNEL, FROM_KERNEL, c).await;
						});
					},

					// We got `sys_shutdown`.
					_ = FROM_TASK_SYS.recv() => {
						debug!("Router - received `sys_shutdown`");
						let c = Arc::clone(&collection);
						tokio::task::spawn(async move {
							crate::shutdown::shutdown(TO_KERNEL, FROM_KERNEL, c).await;
						});
					},
				}
			};

			if crate::statics::shutting_down() {
				continue;
			}

			let connection_token = ConnectionToken::new();

			// If we are past the connection limit, wait until some
			// tasks are done before serving new connections.
			if let Some(max) = CONFIG.max_connections {
				if crate::statics::connections() > max {
					// Only log once.
					warn!("Router - Past max connections [{max}], waiting before serving [{}]...", addr.ip());

					while crate::statics::connections() > max {
						tokio::time::sleep(std::time::Duration::from_millis(10)).await;
					}
				}
			}

			(stream, addr, connection_token)
		}}
	}

	// Prints `protocol`, `ip`, and `port` in color.
	macro_rules! listening {
		() => {{
			let protocol = if CONFIG.tls { "https" } else { "http" };

			let port = match LISTENER.local_addr() {
				Ok(a) => if let SocketAddr::V4(a) = a {
					a.port()
				} else {
					addr.port()
				}
				_ => addr.port(),
			};

			const PURPLE: &str = "\x1b[1;95m";
			const YELLOW: &str = "\x1b[1;93m";
			const BLUE:   &str = "\x1b[1;94m";
			const WHITE:  &str = "\x1b[1;97m";
			const OFF:    &str = "\x1b[0m";
			let listening = format!("| festivald listening on {PURPLE}{protocol}{OFF}://{YELLOW}{}{OFF}:{BLUE}{port}{OFF} |", addr.ip());
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
			let (stream, addr, connection_token) = impl_loop!();

			tokio::task::spawn(async move {
				https(
					connection_token,
					stream,
					addr,
					ACCEPTOR,
					COLLECTION_PTR,
					TO_KERNEL,
					FROM_KERNEL,
					TO_ROUTER_SYS,
					TO_ROUTER_COLLECTION,
				).await;
			});
		}
	// Else If `HTTP`, start main `HTTP` loop (the exact same, but without TLS).
	} else {
		listening!();

		loop {
			let (stream, addr, connection_token) = impl_loop!();

			tokio::task::spawn(async move {
				http(
					connection_token,
					stream,
					addr,
					COLLECTION_PTR,
					TO_KERNEL,
					FROM_KERNEL,
					TO_ROUTER_SYS,
					TO_ROUTER_COLLECTION,
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
	addr:           SocketAddr,
	COLLECTION_PTR: &'static CollectionPtr,
	TO_KERNEL:      &'static Sender<FrontendToKernel>,
	FROM_KERNEL:    &'static Receiver<KernelToFrontend>,
	TO_ROUTER_S:    &'static tokio::sync::mpsc::Sender::<()>,
	TO_ROUTER_C:    &'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
) {
	// For why this is `CollectionPtr` instead
	// of `Arc<Collection>`, see `ptr.rs`.
	if let Err(e) = Http::new()
		.serve_connection(stream, service_fn(|r| route(r, addr, COLLECTION_PTR, TO_KERNEL, FROM_KERNEL, TO_ROUTER_S, TO_ROUTER_C)))
		.await
	{
		error!("Router - HTTP error for [{}]: {e}", addr.ip());
	}
}

//---------------------------------------------------------------------------------------------------- Handle HTTPS
// Handle HTTPS requests.
async fn https(
	_c:             ConnectionToken,
	stream:         TcpStream,
	addr:           SocketAddr,
	acceptor:       &'static TlsAcceptor,
	COLLECTION_PTR: &'static CollectionPtr,
	TO_KERNEL:      &'static Sender<FrontendToKernel>,
	FROM_KERNEL:    &'static Receiver<KernelToFrontend>,
	TO_ROUTER_S:    &'static tokio::sync::mpsc::Sender::<()>,
	TO_ROUTER_C:    &'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
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
		.serve_connection(stream, service_fn(|r| route(r, addr, COLLECTION_PTR, TO_KERNEL, FROM_KERNEL, TO_ROUTER_S, TO_ROUTER_C)))
		.await
	{
		error!("Router - HTTPS error for [{}]: {e}", addr.ip());
	}
}

//---------------------------------------------------------------------------------------------------- Handle Routes
// Route requests to other functions.
async fn route(
	req:            Request<Body>,
	addr:           SocketAddr,
	COLLECTION_PTR: &'static CollectionPtr,
	TO_KERNEL:      &'static Sender<FrontendToKernel>,
	FROM_KERNEL:    &'static Receiver<KernelToFrontend>,
	TO_ROUTER_S:    &'static tokio::sync::mpsc::Sender::<()>,
	TO_ROUTER_C:    &'static tokio::sync::mpsc::Sender::<Arc<Collection>>,
) -> Result<Response<Body>, anyhow::Error> {
	atomic_add!(TOTAL_REQUESTS, 1);

	//-------------------------------------------------- Only accept IPv4
	let addr = match addr {
		std::net::SocketAddr::V4(addr) => {
			debug!("Router - New connection: [{}]", addr.ip());
			addr
		},
		std::net::SocketAddr::V6(addr) => {
			warn!("Router - Skipping IPv6 connection: [{}]", addr.ip());
			sleep_on_fail().await;
			return Ok(resp::server_err("IPv4 not supported"));
		},
	};

	crate::seen::add(&addr).await;

	trace!("Router - HTTP Request: {req:#?}");

	//-------------------------------------------------- Exclusive IP list
	let ip = addr.ip();
	if let Some(ips) = &config().exclusive_ips {
		if !ips.contains(ip) {
			info!("Router - IP not in exclusive list, skipping [{ip}]");
			sleep_on_fail().await;
			return Ok(resp::forbidden("IP not in exclusive list"));
		}
	}

	//-------------------------------------------------- Authorization
	let (mut parts, body) = req.into_parts();

	//-------------------------------------------------- JSON-RPC
	if parts.method == hyper::Method::POST {
		crate::rpc::handle(parts, body, addr, COLLECTION_PTR, TO_KERNEL, FROM_KERNEL, TO_ROUTER_S, TO_ROUTER_C).await
	//-------------------------------------------------- REST
	} else if crate::rest::REST_ENDPOINTS.contains({
		let mut uri = parts.uri.path().split("/");
		uri.next();
		&uri.next().unwrap_or_else(|| "")
	}) {
		if config().rest {
			crate::rest::handle(parts, addr, COLLECTION_PTR).await
		} else {
			Ok(resp::forbidden("REST is disabled"))
		}
	//-------------------------------------------------- Documentation
	} else if config().docs {
		let Some(path) = crate::docs::DOCS_PATH.get() else {
			return Ok(resp::server_err("Documentation failed to build"));
		};

		let sta = hyper_staticfile::Static::new(&path);
		let req = Request::from_parts(parts, body);

		let Ok(resolve) = hyper_staticfile::resolve(&path, &req).await else {
			return Ok(resp::not_found(crate::rest::ERR_END));
		};

		match resolve {
			hyper_staticfile::ResolveResult::Found { .. } => (),
			_ => return Ok(resp::not_found(crate::rest::ERR_END)),
		}

		match hyper_staticfile::ResponseBuilder::new()
			.request(&req)
			.build(resolve)
		{
			Ok(r) => {
				// Check auth.
				if !config().no_auth_docs {
					if let Some(hash) = AUTH.get() {
						if !auth_ok(&req.into_parts().0, hash).await {
							if crate::seen::seen(&addr).await {
								sleep_on_fail().await;
							}
							return Ok(resp::unauthorized("Unauthorized"));
						}
					}
				}

				Ok(r)
			},
			_     => Ok(resp::not_found(crate::rest::ERR_END)),
		}
	//-------------------------------------------------- Unknown endpoint.
	} else {
		Ok(resp::not_found(crate::rest::ERR_END))
	}
}

//---------------------------------------------------------------------------------------------------- Auth
// Verify authentication, ask for it, or ignore
// if none is set in our config.
pub async fn auth_ok(parts: &Parts, hash: &rpc::hash::Hash) -> bool {
	match parts.headers.get(AUTHORIZATION) {
		// AUTH header exists.
		Some(s) => {
			// Attempt to turn into UTF-8 string.
			let string = match String::from_utf8(s.as_bytes().into()) {
				Ok(s)  => s,
				Err(e) => return false,
			};

			// Check if the hash matches our existing one.
			hash.same(string)
		},

		// AUTH header doesn't exist, reject this request.
		None => false,
	}
}

//---------------------------------------------------------------------------------------------------- Sleep
// Sleep for a random while.
// Used for timing out requests, preventing timing attacks, etc.
pub async fn sleep_on_fail() {
	use rand::{Rng,thread_rng};

	if let Some(end) = config().sleep_on_fail {
		let millis = thread_rng().gen_range(0..end);
		trace!("Router - Sleeping for {millis} millis");
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
