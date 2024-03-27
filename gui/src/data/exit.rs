//---------------------------------------------------------------------------------------------------- Use
use std::sync::{atomic::AtomicBool, atomic::AtomicU8};

//---------------------------------------------------------------------------------------------------- Shared state between `GUI` and the `Exit` thread.
// How many seconds to wait for the `Collection`
// to be saved to disk before force-quitting the `GUI`.
const EXIT_COUNTDOWN_START: u8 = 30;

/// How long before we force quit without saving.
pub static EXIT_COUNTDOWN: AtomicU8 = AtomicU8::new(EXIT_COUNTDOWN_START);

/// The exit thread, or another thread has signaled
/// that the main `GUI` process should exit.
pub static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);
