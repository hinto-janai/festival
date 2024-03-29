//---------------------------------------------------------------------------------------------------- Use
use log::info;
use once_cell::sync::Lazy;
use std::io::Write;
use std::time::Instant;

//---------------------------------------------------------------------------------------------------- Start of logger.
// This will get initialized below.
/// Returns the init [`Instant`]
///
/// This returns the [`Instant`] of either:
/// - When [`init_logger()`] was first called
/// - When [`Kernel`] was first spawned
///
/// (which ever one came first)
pub static INIT_INSTANT: Lazy<Instant> = Lazy::new(Instant::now);

/// Returns the seconds since [`INIT_INSTANT`].
pub fn uptime() -> u64 {
    benri::secs!(INIT_INSTANT)
}

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
    // Initialize timer.
    let now = Lazy::force(&INIT_INSTANT);

    // If `RUST_LOG` isn't set, override it and disables
    // all library crate logs except for `festival` & `shukusai`.
    let mut env = String::new();
    match std::env::var("RUST_LOG") {
        Ok(e) => {
            std::env::set_var("RUST_LOG", &e);
            env = e;
        }
        // SOMEDAY:
        // Support frontend names without *festival*.
        _ => std::env::set_var(
            "RUST_LOG",
            format!("off,shukusai={},festival={}", filter, filter),
        ),
    }

    env_logger::Builder::new()
        .format(move |buf, record| {
            let mut style = buf.style();
            let level = match record.level() {
                log::Level::Debug => {
                    style.set_color(env_logger::fmt::Color::Blue);
                    "D"
                }
                log::Level::Trace => {
                    style.set_color(env_logger::fmt::Color::Magenta);
                    "T"
                }
                log::Level::Info => {
                    style.set_color(env_logger::fmt::Color::White);
                    "I"
                }
                log::Level::Warn => {
                    style.set_color(env_logger::fmt::Color::Yellow);
                    "W"
                }
                log::Level::Error => {
                    style.set_color(env_logger::fmt::Color::Red);
                    "E"
                }
            };
            writeln!(
                buf,
                // Longest PATH in the repo: `shukusai/src/collection/collection.rs` - `40` characters
                // Longest file in the repo: `shukusai/src/audio/audio.rs`           - `4` digits
                //
                // Use `utils/longest.sh` to find this.
                //
                //      Longest PATH ---|        |--- Longest file
                //                      |        |
                //                      v        v
                "| {} | {: >9.3} | {: >37} @ {: <4} | {}",
                style.set_bold(true).value(level),
                buf.style()
                    .set_dimmed(true)
                    .value(now.elapsed().as_secs_f32()),
                buf.style()
                    .set_dimmed(true)
                    .value(record.file_static().unwrap_or("???")),
                buf.style()
                    .set_dimmed(true)
                    .value(record.line().unwrap_or(0)),
                record.args(),
            )
        })
        .write_style(env_logger::WriteStyle::Always)
        .parse_default_env()
        .init();

    if env.is_empty() {
        info!("Log Level (Flag) ... {}", filter);
    } else {
        info!("Log Level (RUST_LOG) ... {}", env);
    }
}
