//! Disk: [`serde`](https://docs.rs/serde) + [`directories`](https://docs.rs/directories) + a whole bunch of file formats as [`Traits`](https://doc.rust-lang.org/book/ch10-02-traits.html).
//!
//! This crate is for writing/reading many different file formats (provided by `serde`) to/from disk locations that follow OS-specific specifications/conventions (provided by `directories`). All errors returned will be an [`anyhow::Error`].
//!
//! Simple example of data being saved on the user's disk for future use:
//! ```
//! use disk::prelude::*;       // Necessary imports to get things working.
//! use disk::{Toml,toml_file}; // <- TOML trait & macro.
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize,Deserialize)] // <- Your data MUST implement `serde`.
//! struct State {
//! 	string: String,
//! 	number: u32,
//! }
//! // To make this struct a file, use the following macro:
//! //
//! // |- 1. The file format used will be TOML.
//! // |
//! // |          |- 2. The struct "State" will be used.
//! // |          |
//! // |          |      |- 3. It will be saved in the OS Data directory.
//! // |          |      |
//! // |          |      |          |- 4. The main project directory is called "MyProject".
//! // |          |      |          |
//! // |          |      |          |            |- 6. It won't be in any sub-directories.
//! // |          |      |          |            |
//! // |          |      |          |            |   |- 7. The file name will be "state.toml".
//! // v          v      v          v            v   v
//!    toml_file!(State, Dir::Data, "MyProject", "", "state");
//!
//! // Now our `State` struct implements the `Toml` trait.
//! //
//! // The PATH would look something like:
//! // Windows | C:\Users\Alice\AppData\Roaming\My_Project\state.toml
//! // macOS   | /Users/Alice/Library/Application Support/My-Project/state.toml
//! // Linux   | /home/alice/.local/share/myproject/state.toml
//!
//! // I'd like to save this to disk, since I'll use it next time.
//! let my_state = State { string: "Hello".to_string(), number: 123 };
//!
//! // Since our `State` struct implements the `Toml` trait, it can do that:
//! match my_state.write() {
//! 	Ok(_) => println!("We saved to disk"),
//! 	Err(e) => eprintln!("We failed to save to disk"),
//! }
//!
//! // Let's create a new `State` by reading the file that we just created:
//! let new_state = State::from_file().expect("Failed to read disk");
//!
//! // These should be the same.
//! assert!(my_state == new_state);
//! ```
//! Manually implementing these traits is possible as well, it only requires 4 constants.
//!
//! The file extension (`.bin`, `.toml`, `.json`) is inferred based on what trait you use.
//! ```
//! impl disk::Toml for State {
//!     // Which OS directory it will be saved in.
//! 	const OS_DIRECTORY: disk::Dir = disk::Dir::Data;
//!     // Which the main project directory is called.
//! 	const PROJECT_DIRECTORY: &'static str = "MyProject";
//!     // If it should be in any sub-directories.
//!     const SUB_DIRECTORIES: &'static str = ""
//!     // What the saved filename will be.
//! 	const FILE_NAME: &'static str = "state";
//! }
//! ```
//!
//! Either a single or multiple sub-directories can be specified with a `/` delimiter.
//!
//! `\` is also allowed but ONLY if building on Windows.
//!
//! An empty string `""` means NO sub directories.
//! ```
//! # use disk::Dir::Data;
//! // Windows ... C:\Users\Alice\AppData\Roaming\My_Project\sub1\sub2\state.bin
//! bincode_file!(State, Data, "MyProject", r"sub1\sub2", "state");
//!
//! // macOS ... /Users/Alice/Library/Application Support/My-Project/sub1/sub2/state.json
//! json_file!(State, Data, "MyProject", "sub1/sub2", "state");
//!
//! // Linux ... /home/alice/.local/share/myproject/sub1/sub2/state.yml
//! yaml_file!(State, Data, "MyProject", "sub1/sub2", "state");
//!
//! // NO sub directory:
//! toml_file!(State, Data, "MyProject", "", "state");
//! ```
//
// The "project" directory is taken from the `CARGO_PKG_NAME` environment variable, which should match the `[package.name]` key in your `Cargo.toml`, for example:
// ```toml
// [package]
// name = "my_project"
// ```
// This would create a directory like so:
// ```text
// Windows | C:\Users\Alice\AppData\Roaming\My_Project\
// macOS   | /Users/Alice/Library/Application Support/My-Project/
// Linux   | /home/alice/.local/share/myproject/
// ```

mod common;
//pub use disk_derive::*;
pub mod prelude {
	pub use crate::common::Dir as Dir;
	pub use const_format::assertcp as const_assert;
	pub use const_format::formatcp as const_format;
}

#[cfg(feature = "bincode")]
mod bincode;
#[cfg(feature = "bincode")]
pub use crate::bincode::Bincode as Bincode;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use crate::json::Json as Json;

#[cfg(feature = "toml")]
mod toml;
#[cfg(feature = "toml")]
pub use crate::toml::Toml as Toml;

#[cfg(feature = "yaml")]
mod yaml;
#[cfg(feature = "yaml")]
pub use crate::yaml::Yaml as Yaml;

#[cfg(feature = "pickle")]
mod pickle;
#[cfg(feature = "pickle")]
pub use crate::pickle::Pickle as Pickle;

#[cfg(feature = "messagepack")]
mod messagepack;
#[cfg(feature = "messagepack")]
pub use crate::messagepack::MessagePack as MessagePack;

#[cfg(feature = "bson")]
mod bson;
#[cfg(feature = "bson")]
pub use crate::bson::Bson as Bson;

#[cfg(feature = "plain")]
mod plain;
#[cfg(feature = "plain")]
pub use crate::plain::Plain as Plain;
