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
//! **The internals are not stable.**
//!
//! **If you're implementing a frontend, you are expected to implement the `Kernel`'s messages correctly.**
//!
//! You can look at [`festival-gui`](https://github.com/hinto-janai/festival/festival-gui)'s code as an example,
//! and the [internal documentation](https://github.com/hinto-janai/festival/src) as reference.
//!
//! # API
//! The "API" between `shukusai` and the frontends are:
//! - [`kernel::KernelToFrontend`]
//! - [`kernel::FrontendToKernel`]
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
//! - Save and manage its own state/settings
//! - Properly implement the messages `To/From` the `Kernel`
//! - Properly handle shared data
//!
//! # Shared Data
//! There are shared functions/data that `shukusai` exposes, notably:
//! - [`collection::Collection`] (and everything within it)
//! - [`kernel::KernelState`]
//! - [`kernel::Volume`]
//! - [`key::Key`] (and other keys)
//! - `CONSTANTS`
//! - etc...
//!
//! It is up to the frontend on how to use these functions/data.
//!
//! A lot of the correct behavior implementation depends on knowledge that _I_ have of the internals.
//! Since _I_ will most likely be creating all the frontends, there are no plans
//! to fully flesh out this documentation for now (it's a lot of work).

//---------------------------------------------------------------------------------------------------- Lints
#![forbid(
	future_incompatible,
	let_underscore,
	break_with_label_and_loop,
	coherence_leak_check,
	deprecated,
	duplicate_macro_attributes,
	exported_private_dependencies,
	for_loops_over_fallibles,
	large_assignments,
	overlapping_range_endpoints,
	private_in_public,
	semicolon_in_expressions_from_macros,
	redundant_semicolons,
	unconditional_recursion,
	unreachable_patterns,
	unused_allocation,
	unused_braces,
	unused_comparisons,
	unused_doc_comments,
	unused_labels,
	unused_unsafe,
	while_true,
	keyword_idents,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	unreachable_pub,
	single_use_lifetimes,
	variant_size_differences,
)]
#![deny(
	nonstandard_style,
	unused_mut,
)]

#[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
compile_error!("shukusai is only compatible with 64-bit/32bit CPUs");
#[cfg(not(any(
	target_os = "windows",
	target_os = "macos",
	target_os = "linux",
	target_family = "wasm",
)))]
compile_error!("shukusai is only compatible with Window/macOS/Linux/WASM");

//---------------------------------------------------------------------------------------------------- Private `shukusai` internals.
mod audio;
mod ccd;
mod search;
mod watch;

//---------------------------------------------------------------------------------------------------- Public Re-exports.
pub use disk;
pub use readable;
pub use rolock;

//---------------------------------------------------------------------------------------------------- Hidden Re-exports.
#[doc(hidden)]
pub use const_format::assertcp as const_assert;
#[doc(hidden)]
pub use const_format::formatcp as const_format;

//---------------------------------------------------------------------------------------------------- Public `/` stuff.
mod constants;
pub use constants::*;
mod logger;
pub use logger::*;

//---------------------------------------------------------------------------------------------------- Public modules.
/// Custom panic hook + backtrace log
///
/// The first thing `Kernel` will do when you spawn it
/// with `Kernel::bios()` is set a custom [`panic!()`] hook.
///
/// Since it is unsafe to carry on operating if any one of the threads
/// within `shukusai` panics, all threads are forcefully exited if any
/// single thread panics, even outside of `shukusai`.
///
/// But before that, a full stack backtrace is printed to console
/// and is also written to disk in the `festival` folder as `panic.txt`.
pub mod panic;

/// The main music `Collection` and it's inner data
pub mod collection;

/// `Key`'s to index the `Collection` in a type-safe way
pub mod key;

/// `Kernel`, the messenger and coordinator
///
/// This is the "API" that all frontends must implement
/// in order to communicate with `Festival`'s internals.
///
/// Your `Frontend` will communicate with `Kernel`, and
/// `Kernel` will talk with the rest of `shukusai`'s internals.
///
/// Messages are sent via `crossbeam_channel`'s with these messages:
/// - [`kernel::KernelToFrontend`]
/// - [`kernel::FrontendToKernel`]
pub mod kernel;

/// Various sorting methods for the `Collection`
///
/// These `enum`'s just represent `Collection` fields and are used for convenience:
/// ```rust,ignore
/// // These two both return the same data.
/// // The enum can be useful when programming frontend stuff.
///
/// collection.album_sort(AlbumSort::ReleaseArtistLexi);
///
/// collection.sort_album_release_artist_lexi;
/// ```
pub mod sort;

/// `Queue` and `Playlist`
///
/// Both `Queue` and `Playlist` are practically the same thing:
///   - A slice of the `Collection`
///
/// They contain a bunch of `Key`'s that point
/// to "segments" of the `Collection` (it's a slice).
///
/// Both `Queue` and `Playlist` inner values are `VecDeque<Key>`.
pub mod slice;

/// Audio Signals to `Kernel`
///
/// These are structs that represent files that represent a signal.
///
/// These structs implement `disk::Empty` so that they can easily be created with `touch()`.
///
/// It holds no data but the file existing represents a signal to `Kernel`.
///
/// ## Usage
/// ```rust,ignore
/// Play::touch().unwrap()
/// ```
/// This creates a file with the:
/// - Lowercase struct name in the
/// - `signal` subdirectory of the
/// - `festival` folder which is in the
/// - OS data folder
///
/// Example: `~/.local/share/festival/signal/play`.
///
/// `Kernel` will immediately respond to the signal, in this example,
/// `Kernel` will start audio playback, then delete the file that was created.
pub mod signal;
