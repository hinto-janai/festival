#[cfg(windows)]
fn main() -> std::io::Result<()> {
	// Include `VCVCRUNTIME140.dll`.
	static_vcruntime::metabuild();

	// Set `File Explorer` icon and other Windows metadata.
	let mut res = winres::WindowsResource::new();
	res.set_icon("../assets/images/icon/icon.ico");
	res.set_language(0x0009 /* english */);

	// This is the name of the program when right
	// clicking it in the taskbar... for some reason.
	res.set("FileDescription", "Festival");
	res.set("ProductName",     "Festival");
	res.set("LegalCopyright",  "Copyright (c) 2023 hinto-janai");
	res.compile()
}

#[cfg(unix)]
fn main() {}
