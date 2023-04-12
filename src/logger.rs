//---------------------------------------------------------------------------------------------------- Use
use std::io::Write;
use benri::log::ok;
use log::info;
use compact_str::format_compact;

//---------------------------------------------------------------------------------------------------- Logger init function
#[inline(always)]
/// Initializes the logger.
///
/// This enables console logging on all the internals of `Festival`.
///
/// Functionality is provided by [`log`].
///
/// The levels are:
/// - ERROR
/// - WARN
/// - INFO
/// - DEBUG
/// - TRACE
///
/// # Panics
/// This must only be called _once_.
pub fn init_logger(filter: log::LevelFilter) {
	// If `RUST_LOG` isn't set, override it and disables
	// all library crate logs except for `festival` & `shukusai`.
	let mut env = String::new();
	match std::env::var("RUST_LOG") {
		Ok(e) => { std::env::set_var("RUST_LOG", &e); env = e; },
		// TODO:
		// Support frontend names without *festival*.
		_     => std::env::set_var("RUST_LOG", format_compact!("off,shukusai={},festival={}", filter, filter)),
	}

	let now = std::time::Instant::now();

	env_logger::Builder::new().format(move |buf, record| {
		let mut style = buf.style();
		let level = match record.level() {
			log::Level::Debug => { style.set_color(env_logger::fmt::Color::Blue);    "DEBUG" },
			log::Level::Trace => { style.set_color(env_logger::fmt::Color::Magenta); "TRACE" },
			log::Level::Info  => { style.set_color(env_logger::fmt::Color::White);   "INFO"  },
			log::Level::Warn  => { style.set_color(env_logger::fmt::Color::Yellow);  "WARN"  },
			log::Level::Error => { style.set_color(env_logger::fmt::Color::Red);     "ERROR" },
		};
		writeln!(
			buf,
			// Longest PATH in the repo: `festival-gui/src/data/settings.rs` - `33` characters
			// Longest file in the repo: `703 src/ccd/metadata.rs`           - `3` digits
			//
			// Use `utils/longest.sh` to find this.
			//
			//        Longest PATH ---|         |--- Longest file
			//                        |         |
			//                        v         v
			"| {: >5} | {: >10.3} | {: >33} @ {: <3} | {}",
			style.set_bold(true).value(level),
			buf.style().set_dimmed(true).value(now.elapsed().as_secs_f32()),
			buf.style().set_dimmed(true).value(record.file_static().unwrap_or("???")),
			buf.style().set_dimmed(true).value(record.line().unwrap_or(0)),
			record.args(),
		)
	}).write_style(env_logger::WriteStyle::Always).parse_default_env().init();

	// This has to be updated with the longest PATH/file too.
//	println!("| LEVEL |       TIME |                                   WHERE | MESSAGE |");
//	println!("|-------|------------|-----------------------------------------|---------|");

	if env.is_empty() {
		info!("Log Level (Flag) ... {}", filter);
	} else {
		info!("Log Level (ENV) ... {}", env);
	}

	ok!("Logger");
}
