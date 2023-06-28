// This sets the git `COMMIT` environment variable.
fn main() {
	println!("cargo:rerun-if-changed=../../.git/refs/heads/");

	let output = std::process::Command::new("git")
		.arg("rev-parse")
		.arg("HEAD")
		.output()
		.unwrap();

	let commit = String::from_utf8(output.stdout).unwrap();

	println!("cargo:rustc-env=COMMIT={commit}");
}
