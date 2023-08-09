mod auth;
mod cli;
mod config;
mod constants;
mod docs;
mod macros;
mod rpc;

fn main() {
	// Handle regular CLI arguments (exit if needed).
	let (config_cmd, rpc, debug, dry_run) = crate::cli::Cli::get();

	// Read config: `festival-cli.toml`.
	let mut config_builder = crate::config::ConfigBuilder::file_or();

	// Merge config + command-line.
	if let Some(mut config_cmd) = config_cmd {
		config_builder.merge(&mut config_cmd);
	}

	// Build config.
	let config = config_builder.build(debug);

	let Some(rpc) = rpc else {
		crate::exit!("missing method");
	};

	// Connect to `festivald`, send request, print response.
	crate::rpc::request(config, debug, dry_run, rpc);
}
