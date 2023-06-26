//! # `shukusai`
//! [`Festival`](https://github.com/hinto-janai/festival)'s internals, `shukusai`.
//!
//! **The internals are not stable and not meant for public usage.**
//!
//! Regardless, it is somewhat documented (mostly for my future self).

//---------------------------------------------------------------------------------------------------- Lints
#![allow(
	clippy::len_zero,
	clippy::type_complexity,
	clippy::module_inception,
)]

#![deny(
	nonstandard_style,
	unused_unsafe,
	unused_mut,
)]

#![forbid(
	future_incompatible,
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
	unused_parens,
	unused_labels,
	while_true,
	keyword_idents,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	unreachable_pub,
	single_use_lifetimes,
	variant_size_differences,
)]

// There are some `as` casts but they are:
// - handled with `.try_into()`
// - are `u32 as usize/u64`
// - are gated by `#[cfg(...)]`
#[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
compile_error!("shukusai is only compatible with 64-bit/32bit CPUs");

#[cfg(not(any(
	target_os = "windows",
	target_os = "macos",
	target_os = "linux",
)))]
compile_error!("shukusai is only tested on Window/macOS/Linux");

//---------------------------------------------------------------------------------------------------- Private `shukusai` internals.
mod ccd;
mod watch;

//---------------------------------------------------------------------------------------------------- Public Re-exports.
pub use readable;

//---------------------------------------------------------------------------------------------------- Hidden Re-exports.
#[doc(hidden)]
pub use const_format::assertcp as const_assert;
#[doc(hidden)]
pub use const_format::formatcp as const_format;

//---------------------------------------------------------------------------------------------------- Public modules.
/// Panic.
pub mod panic;

/// Logger.
pub mod logger;

/// Thread.
pub mod thread;

/// Constants.
pub mod constants;

/// Audio.
pub mod audio;

/// Search.
pub mod search;

/// Global state.
pub mod state;

/// The main music `Collection` and its inner data
pub mod collection;

/// `Kernel`, the messenger and coordinator
///
/// This is the "API" that all frontends must implement
/// in order to communicate with `Festival`'s internals.
///
/// Your `Frontend` will communicate with `Kernel`, and
/// `Kernel` will talk with the rest of `shukusai`'s internals.
///
/// Messages are sent via `crossbeam::channel`'s with these messages:
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

/// `Frontend`-specific compatibility layers
pub mod frontend;

/// Ancillary `Collection` data validation
///
/// Since the `Collection` uses indices instead of references,
/// it means that there is no lifetime associated with them.
///
/// If a new `Collection` is received, the already existing ancillary data
/// that was pointing to the old one may not be correct, e.g:
/// ```rust,ignore
/// let key = ArtistKey::from(123);
/// assert!(collection.artists.len() > 123);
/// collection.artists[key]; // OK
///
/// let collection = recv_new_collection();
/// collection.artists[key]; // This may or may not panic.
/// ```
/// Even if the key ends up existing, it most likely is pointing at the wrong thing.
///
/// This module provides some common validation methods
/// that checks inputs against an existing `Collection`.
///
/// These functions are used when `Kernel` is loading up the `Collection`
/// and `State` from disk, where there is never a 100% lifetime guarantee between the two.
///
/// These functions are also used for `GUI` configuration settings
/// that hold keys and other misc data like that.
///
/// All methods are free functions that require a `Collection`.
pub mod validate;
