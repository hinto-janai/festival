#[cfg(windows)]
fn main() -> std::io::Result<()> {
	// Include `VCVCRUNTIME140.dll`.
	static_vcruntime::metabuild();

	// Set `File Explorer` icon.
	let mut res = winres::WindowsResource::new();
	res.set_icon("../assets/images/icon/icon.ico");
	res.compile()
}

#[cfg(unix)]
fn main() {}
