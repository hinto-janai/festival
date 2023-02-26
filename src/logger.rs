//---------------------------------------------------------------------------------------------------- Use
use std::io::Write;
use crate::macros::ok;
use log::info;

//---------------------------------------------------------------------------------------------------- Logger init function
#[inline(always)]
pub fn init_logger(filter: log::LevelFilter) {
	// Disables all library crate logs except for [festival].
	std::env::set_var("RUST_LOG", format!("off,festival={}", filter));
	let now = std::time::Instant::now();
	env_logger::Builder::new().format(move |buf, record| {
		let mut style = buf.style();
		let level = match record.level() {
			log::Level::Error => { style.set_color(env_logger::fmt::Color::Red); "ERROR" },
			log::Level::Warn => { style.set_color(env_logger::fmt::Color::Yellow); "WARN" },
			log::Level::Info => { style.set_color(env_logger::fmt::Color::White); "INFO" },
			log::Level::Debug => { style.set_color(env_logger::fmt::Color::Blue); "DEBUG" },
			log::Level::Trace => { style.set_color(env_logger::fmt::Color::Magenta); "TRACE" },
		};
		writeln!(
			buf,
//			"[{}] [{}] [{}] [{}:{}] {}",
			"[{}] [{}] [{}:{}] {}",
			style.set_bold(true).value(level),
			buf.style().set_dimmed(true).value(format!("{:.3}", now.elapsed().as_secs_f32())),
//			buf.style().set_dimmed(true).value(chrono::offset::Local::now().format("%F %T%.3f")),
			buf.style().set_dimmed(true).value(record.file().unwrap_or("???")),
			buf.style().set_dimmed(true).value(record.line().unwrap_or(0)),
			record.args(),
		)
	}).write_style(env_logger::WriteStyle::Always).parse_default_env().format_timestamp_millis().init();
	info!("Log Level ... {}", filter);
	ok!("Logger");
}
