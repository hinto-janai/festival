// This code is mostly taken from <https://github.com/hyperium/hyper-tls/blob/master/src/stream.rs>.
//
// It's a convenient wrapper around either
// a `HTTP` or `HTTPS` connection without
// `Box<dyn ...>`, although theres probably
// overhead on every `match`.
//
// Either way, it means we only have to write the
// networking code once, and it'll work for both
// `HTTP`/`HTTPS`.

//---------------------------------------------------------------------------------------------------- Use
use std::fmt;
use std::io;
use std::io::IoSlice;
use std::pin::Pin;
use std::task::{Context, Poll};
use hyper::client::connect::{Connected, Connection};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio_native_tls::TlsStream;

//---------------------------------------------------------------------------------------------------- MaybeTlsStream
/// A stream that might be protected with TLS.
pub enum MaybeTlsStream<T> {
	/// A stream over plain text.
	No(T),
	/// A stream protected with TLS.
	Yes(TlsStream<T>),
}

//---------------------------------------------------------------------------------------------------- Impl
impl<T> MaybeTlsStream<T> {
	pub fn is_tls(&self) -> bool {
		match self {
			Self::No(_) => false,
			Self::Yes(_) => true,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Trait impl
impl<T: fmt::Debug> fmt::Debug for MaybeTlsStream<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MaybeTlsStream::No(s) => f.debug_tuple("No").field(s).finish(),
			MaybeTlsStream::Yes(s) => f.debug_tuple("Yes").field(s).finish(),
		}
	}
}

impl<T: AsyncRead + AsyncWrite + Unpin> AsyncRead for MaybeTlsStream<T> {
	#[inline]
	fn poll_read(
		self: Pin<&mut Self>,
		cx: &mut Context,
		buf: &mut ReadBuf,
	) -> Poll<Result<(), io::Error>> {
		match Pin::get_mut(self) {
			MaybeTlsStream::No(s) => Pin::new(s).poll_read(cx, buf),
			MaybeTlsStream::Yes(s) => Pin::new(s).poll_read(cx, buf),
		}
	}
}

impl<T: AsyncWrite + AsyncRead + Unpin> AsyncWrite for MaybeTlsStream<T> {
	#[inline]
	fn poll_write(
		self: Pin<&mut Self>,
		cx: &mut Context<'_>,
		buf: &[u8],
	) -> Poll<Result<usize, io::Error>> {
		match Pin::get_mut(self) {
			MaybeTlsStream::No(s) => Pin::new(s).poll_write(cx, buf),
			MaybeTlsStream::Yes(s) => Pin::new(s).poll_write(cx, buf),
		}
	}

	fn poll_write_vectored(
		self: Pin<&mut Self>,
		cx: &mut Context<'_>,
		bufs: &[IoSlice<'_>],
	) -> Poll<Result<usize, io::Error>> {
		match Pin::get_mut(self) {
			MaybeTlsStream::No(s) => Pin::new(s).poll_write_vectored(cx, bufs),
			MaybeTlsStream::Yes(s) => Pin::new(s).poll_write_vectored(cx, bufs),
		}
	}

	fn is_write_vectored(&self) -> bool {
		match self {
			MaybeTlsStream::No(s) => s.is_write_vectored(),
			MaybeTlsStream::Yes(s) => s.is_write_vectored(),
		}
	}

	#[inline]
	fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
		match Pin::get_mut(self) {
			MaybeTlsStream::No(s) => Pin::new(s).poll_flush(cx),
			MaybeTlsStream::Yes(s) => Pin::new(s).poll_flush(cx),
		}
	}

	#[inline]
	fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
		match Pin::get_mut(self) {
			MaybeTlsStream::No(s) => Pin::new(s).poll_shutdown(cx),
			MaybeTlsStream::Yes(s) => Pin::new(s).poll_shutdown(cx),
		}
	}
}

impl<T: AsyncRead + AsyncWrite + Connection + Unpin> Connection for MaybeTlsStream<T> {
	fn connected(&self) -> Connected {
		match self {
			MaybeTlsStream::No(s) => s.connected(),
			MaybeTlsStream::Yes(s) => s.get_ref().get_ref().get_ref().connected(),
		}
	}
}
