//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use crate::config::Config;
use crate::hash::Hash;
use std::sync::Arc;
use std::net::{
	Ipv4Addr,
	SocketAddrV4,
};
use hyper::{
	body::Body,
	server::conn::Http,
	service::service_fn,
};
use hyper::http::{Request, Response, StatusCode};
use std::convert::Infallible;
use crossbeam::channel::{
	Sender,Receiver,
};
use shukusai::{
	kernel::{
		FrontendToKernel,
		KernelToFrontend,
	},
	constants::DASH,
};
use crate::statics::ConnectionToken;
use tokio_native_tls::{
	TlsAcceptor,
	TlsStream,
};
use tokio::net::{
	TcpListener,
	TcpStream,
};

//---------------------------------------------------------------------------------------------------- Router
#[tokio::main]
pub async fn init(
//	to_kernel:   Sender<FrontendToKernel>,
//	from_kernel: Receiver<KernelToFrontend>,
	config:      Config,
)
	-> Result<(), anyhow::Error>
{
	// Bind to address.
	let addr     = SocketAddrV4::new(config.ip, config.port);
	let listener = match TcpListener::bind(addr).await {
		Ok(l)  => l,
		Err(e) => return Err(anyhow!("could not bind to [{addr}]: {e}")),
	};

	// These last forever.
//	let TO_KERNEL:   &'static Sender<FrontendToKernel>   = Box::leak(Box::new(to_kernel));
//	let FROM_KERNEL: &'static Receiver<KernelToFrontend> = Box::leak(Box::new(from_kernel));
	let CONFIG:      &'static Config                     = Box::leak(Box::new(config));

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
				std::net::SocketAddr::V4(addr) => { info!("new connection: [{}]", addr.ip()); addr },
				std::net::SocketAddr::V6(addr) => { warn!("skipping ipv6 connection: [{}]", addr.ip()); continue; },
			};

			let ip = addr.ip();

			// If we have an exclusive IP list, deny non-contained IP connections.
			if let Some(ips) = &CONFIG.exclusive_ips {
				if !ips.contains(ip) {
					println!("ip not in exclusive list, skipping [{ip}]");
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

	// If `HTTPs`
	if CONFIG.tls {
		// Sanity-checks.
		let path_cert = match &CONFIG.certificate {
			Some(p) => p,
			None    => return Err(anyhow!("TLS enabled but no certificate PATH provided")),
		};

		let path_key = match &CONFIG.key {
			Some(p) => p,
			None    => return Err(anyhow!("TLS enabled but no key PATH provided")),
		};

		let ACCEPTOR: &'static TlsAcceptor = crate::cert::get_tls_acceptor(&path_cert, &path_key)?;

		listening!();

		loop {
			let (stream, addr) = impl_loop!();

			tokio::task::spawn(async move {
				https(ConnectionToken::new(), stream, addr, CONFIG, ACCEPTOR).await;
			});
		}
	// Else If `HTTP`
	} else {
		listening!();

		loop {
			let (stream, addr) = impl_loop!();

			tokio::task::spawn(async move {
				http(ConnectionToken::new(), stream, addr, CONFIG).await;
			});
		}
	}

	Ok(())
}

//---------------------------------------------------------------------------------------------------- Handle HTTP
async fn http(
	_c:     ConnectionToken,
	stream: TcpStream,
	addr:   SocketAddrV4,
	config: &'static Config,
) {
	if let Err(e) = Http::new()
		.serve_connection(stream, service_fn(|r| route(r, addr, config)))
		.await
	{
		error!("HTTP error for [{}]: {e}", addr.ip());
	}
}

//---------------------------------------------------------------------------------------------------- Handle HTTPS
async fn https(
	_c:       ConnectionToken,
	stream:   TcpStream,
	addr:     SocketAddrV4,
	config:   &'static Config,
	acceptor: &'static TlsAcceptor,
) {
	let stream = match acceptor.accept(stream).await {
		Ok(s)  => s,
		Err(e) => { error!("TLS error for [{}]: {e}", addr.ip()); return; },
	};

	if let Err(e) = Http::new()
		.serve_connection(stream, service_fn(|r| route(r, addr, config)))
		.await
	{
		error!("HTTPS error for [{}]: {e}", addr.ip());
	}
}

//---------------------------------------------------------------------------------------------------- Handle Routes
async fn route(
	req:    Request<Body>,
	addr:   SocketAddrV4,
	config: &'static Config,
) -> Result<Response<Body>, anyhow::Error> {
	let (parts, body) = req.into_parts();

	if parts.uri == "/" && parts.method == hyper::Method::POST {
		crate::rpc::handle(parts, body, addr, config).await
	} else {
		crate::rest::handle(parts, body, addr, config).await
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
