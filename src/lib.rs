//! The `Festival` internals that powers all the frontends.
//!
//! **Note: The internals are not, and will probably never be stable or publically exposed.**
//!
//! If you're implementing a frontend, you can look at `festival-gui`'s code as an example.
//!
//! The "API" between `Festival` and the frontends are:
//! - `KernelToFrontend`
//! - `FrontendToKernel`
//!
//! Which is defined in the `src/kernel/msg.rs` file.
//!
//! Each frontend must implement the correct message passing behavior to/from the `Kernel` and other various things.
//!
//! A simple frontend setup:
//! ```rust
//! fn main() {
//!     // Handle CLI arguments (and logging).
//!     // This calls into 'festival' code to initialize the logging
//!     // so that it doesn't need to be implemented for each frontend.
//!     cli::Cli::handle_args();
//!
//!     // Create `Kernel` <-> `GUI` channels.
//!     // These channels must _never_ be closed.
//!     let (kernel_to_gui, gui_recv)    = crossbeam_channel::unbounded::<festival::KernelToFrontend>();
//!     let (gui_to_kernel, kernel_recv) = crossbeam_channel::unbounded::<festival::FrontendToKernel>();
//!
//!     // Spawn `Kernel`.
//!     std::thread::spawn(move || festival::Kernel::bios(kernel_to_gui, kernel_recv));
//!
//!     // Start `GUI`.
//!     eframe::run_native(
//!         festival::FESTIVAL_NAME_VER,
//!         gui::Gui::options(),
//!         Box::new(|cc| Box::new(gui::Gui::init(cc, gui_to_kernel, gui_recv)))
//!     );
//! }
//! ```
//!
//! `Kernel` itself will handle:
//! - Logging initialization
//! - `Collection` management
//! - Pretty much everything
//!
//! The `Frontend` implementation must:
//! - Keep a channel to `Kernel` open at _all times_
//! - Properly implement the messages `To/From` the `Kernel`
//! - Properly handle shared data
//!
//! There is shared stuff that `Festival/Kernel` exposes:
//! - `Collection`
//! - `State`
//! - `CONSTANTS`
//! - `macros!()`
//! - etc...
//!
//! None of the data/message relationships are restrictive enough to be a public API,
//! and a lot of behavior depends on knowledge that _I_ have of the internals.
//! Since _I_ will most likely be creating all the frontends, there are no plans
//! to make a well-defined public API. That's a lot of work.

mod audio;
mod ccd;
mod macros;
mod search;
mod watch;

mod cli;
pub use cli::*;
mod collection;
pub use collection::*;
mod constants;
pub use constants::*;
mod kernel;
pub use kernel::*;
mod logger;
pub use logger::*;
