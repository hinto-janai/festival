/// `egui`-specific
#[cfg(feature = "gui")]
pub mod egui;

/// `festivald`-specific
#[cfg(feature = "daemon")]
pub mod daemon;

/// `festival-cli`-specific
#[cfg(feature = "cli")]
pub mod cli;

/// `festival-web`-specific
#[cfg(feature = "web")]
pub mod web;
