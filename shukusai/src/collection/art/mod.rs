// Each `Frontend` will handle art differently.
// These are the different implementations for each.

#[cfg(feature = "gui")]
mod gui;
#[cfg(feature = "gui")]
pub use gui::*;

#[cfg(not(feature = "gui"))]
#[cfg(feature = "daemon")]
mod daemon;
#[cfg(not(feature = "gui"))]
#[cfg(feature = "daemon")]
pub use daemon::*;
