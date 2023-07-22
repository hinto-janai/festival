//---------------------------------------------------------------------------------------------------- Use
use zeroize::Zeroize;
use anyhow::anyhow;
use tokio_native_tls::{
	TlsAcceptor,
	native_tls::{
		Identity,
		TlsAcceptor as TlsAcceptorNative,
	},
};
use std::path::{
	Path,PathBuf,
};

//----------------------------------------------------------------------------------------------------
pub fn get_tls_acceptor(path_cert: &Path, path_key: &Path) -> Result<&'static TlsAcceptor, anyhow::Error> {
	// Read cert.
	let mut cert = std::fs::read(path_cert)?;

	// Read key.
	let mut key = std::fs::read(path_key)?;

	let acceptor = TlsAcceptor::from(build_cert(cert, key)?);
	let acceptor: &'static TlsAcceptor = Box::leak(Box::new(acceptor));

	Ok(acceptor)
}

fn build_cert(mut cert: Vec<u8>, mut key: Vec<u8>) -> Result<TlsAcceptorNative, anyhow::Error> {
	// Build.
	let identity = Identity::from_pkcs8(&cert, &key)?;
	let acceptor = TlsAcceptorNative::builder(identity).build()?;

	// Zeroize cert + key.
	cert.zeroize();
	key.zeroize();

	Ok(acceptor)
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	const CERT: &[u8] =
r#"-----BEGIN CERTIFICATE-----
MIICIzCCAdWgAwIBAgIUMi3zZHqVrFt6WtPljnHvF7LLzGMwBQYDK2VwMIGGMQsw
CQYDVQQGEwJYWDESMBAGA1UECAwJU3RhdGVOYW1lMREwDwYDVQQHDAhDaXR5TmFt
ZTEUMBIGA1UECgwLQ29tcGFueU5hbWUxGzAZBgNVBAsMEkNvbXBhbnlTZWN0aW9u
TmFtZTEdMBsGA1UEAwwUQ29tbW9uTmFtZU9ySG9zdG5hbWUwHhcNMjMwNzIyMDAx
MzU2WhcNMzMwNzE5MDAxMzU2WjCBhjELMAkGA1UEBhMCWFgxEjAQBgNVBAgMCVN0
YXRlTmFtZTERMA8GA1UEBwwIQ2l0eU5hbWUxFDASBgNVBAoMC0NvbXBhbnlOYW1l
MRswGQYDVQQLDBJDb21wYW55U2VjdGlvbk5hbWUxHTAbBgNVBAMMFENvbW1vbk5h
bWVPckhvc3RuYW1lMCowBQYDK2VwAyEAJStbAI8AssO6yo2/ku0Rc8/PF8u/gvh/
pFAoDeRK5D6jUzBRMB0GA1UdDgQWBBTe7vW3YqPkj9V4vQX0NoGyRACh8TAfBgNV
HSMEGDAWgBTe7vW3YqPkj9V4vQX0NoGyRACh8TAPBgNVHRMBAf8EBTADAQH/MAUG
AytlcANBAHYqfk6KTfx6KuoP4b6imb9xRxvPMBd+JnZZcW/xZg5oJrc73hww3K/r
AzdeI/jDKmSh20ojFdQ4H7mp5KLrDgw=
-----END CERTIFICATE-----"#.as_bytes();

	const KEY: &[u8] =
r#"-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIPQGuEm4kDjqPAivMizibpPRSE/i95Mw9Tk6ZZd5E5cr
-----END PRIVATE KEY-----"#.as_bytes();

	#[test]
	fn cert() {
		build_cert(CERT.into(), KEY.into()).unwrap();
	}
}

