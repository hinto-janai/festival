mod auth;
mod cli;
mod config;
mod constants;
mod docs;
mod macros;
mod rpc;

fn main() {
	// Handle CLI arguments.
//	let (disable_watch, disable_media_controls, log, config_cmd) = {
//		if std::env::args_os().len() == 1 {
//			(false, false, None, None)
//		} else {
//			crate::cli::Cli::get()
//		}
//	};

	// Init logger.
//	shukusai::logger::init_logger(log.unwrap_or_else(|| log::LevelFilter::Error));

	crate::cli::Cli::get();
//    println!("{}", x.id);
}
