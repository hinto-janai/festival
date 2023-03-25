//! # Festival
//! [`Festival`](https://github.com/hinto-janai/festival)'s internals that powers all of its frontends.
//!
//! The crate [`festival`](https://crates.io/crates/festival) is being squatted, so instead, `Festival`'s
//! original name, [`shukusai`](https://crates.io/crates/shukusai), is used.
//!
//! `祝祭/shukusai` translated means: `Festival`.
//!
//! In documentation:
//!
//! - `shukusai` _specifically_ means `Festival`'s internals
//! - `Festival` means a frontend OR the project as a whole
//!
//! # Warning
//! **The internals are not stable. There's no _restrictive, type-safe_ public API.**
//!
//! **If you're implementing a frontend, you are expected to implement the `Kernel`'s messages correctly.**
//!
//! You can look at [`festival-gui`](https://github.com/hinto-janai/festival/festival-gui)'s code as an example,
//! and the [internal documentation](https://github.com/hinto-janai/festival/src) as reference.
//!
//! # API
//! The "API" between `shukusai` and the frontends are:
//! - [`KernelToFrontend`]
//! - [`FrontendToKernel`]
//!
//! Each frontend must implement the correct message passing behavior to/from the `Kernel` and other various things.
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
//! There are shared functions/data that `shukusai/Kernel` exposes, notably:
//! - [`Collection`] (and everything within it)
//! - [`KernelState`]
//! - [`Volume`]
//! - [`Key`] (and other keys)
//! - `CONSTANTS`
//! - `macros!()`
//! - etc...
//!
//! It is up to the frontend on how to use these functions/data.
//!
//! None of the data/message relationships are restrictive enough to be a public API,
//! and a lot of behavior depends on knowledge that _I_ have of the internals.
//! Since _I_ will most likely be creating all the frontends, there are no plans
//! to make a well-defined public API for now (it's a lot of work).

mod audio;
mod ccd;
mod macros;
mod search;
mod watch;

mod collection;
pub use collection::*;
mod constants;
pub use constants::*;
mod kernel;
pub use kernel::*;
mod logger;
pub use logger::*;

pub use rolock::RoLock as RoLock;
