/// `festival-gui`-specific
#[cfg(feature = "gui")]
pub mod gui;

/// `festivald`-specific
#[cfg(feature = "daemon")]
pub mod daemon;

/// `festival-web`-specific
#[cfg(feature = "web")]
pub mod web;
