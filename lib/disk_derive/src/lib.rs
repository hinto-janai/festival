use proc_macro::TokenStream;
use quote::{quote,ToTokens};
use syn::{parse_macro_input, DeriveInput};
use darling::{FromDeriveInput,FromMeta};
use serde::{Serialize,Deserialize};


enum Dir {
	Project,
	Cache,
	Config,
	Data,
	DataLocal,
	Preference,
}

//---------------------------------------------------------------------------------------------------- Options
#[derive(Default, FromDeriveInput)]
#[darling(default, attributes(toml_file), attributes(bincode_file))]
struct Options {
	dir: String,
	sub_dirs: String,
}

//---------------------------------------------------------------------------------------------------- TOML
#[proc_macro_derive(TomlFile, attributes(toml_file))]
pub fn derive_toml(input: TokenStream) -> TokenStream {
	// Parse the input tokens into a syntax tree
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let lowercase = name.to_string().to_lowercase();

	let options = Options::from_derive_input(&input).expect("Wrong options");

	let dir = options.dir;
	let sub_dirs = options.sub_dirs;

	// Build the output, possibly using quasi-quotation
	let expanded = quote! {
		impl disk::Toml for #name {
			const FILE_NAME: &'static str = "#lowercase";
			const DIRECTORY: Dir = #dir;
			const SUB_DIRECTORIES: &'static str = "#sub_dirs";
		}
	};

	// Hand the output tokens back to the compiler
	TokenStream::from(expanded)
}

//---------------------------------------------------------------------------------------------------- Bincode
#[proc_macro_derive(BincodeFile)]
pub fn derive_bincode(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let lowercase = name.to_string().to_lowercase();

	let expanded = quote! {
		impl disk::Bincode for #name {
			const FILE_NAME: &'static str = "#lowercase";
			const DIRECTORY: disk::Dir = disk::Dir::Data;
			const SUB_DIRECTORIES: &'static str = "";
		}
	};

	TokenStream::from(expanded)
}

//---------------------------------------------------------------------------------------------------- JSON
#[proc_macro_derive(JsonFile)]
pub fn derive_json(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let lowercase = name.to_string().to_lowercase();

	let expanded = quote! {
		impl disk::Json for #name {
			const FILE_NAME: &'static str = "#lowercase";
			const DIRECTORY: disk::Dir = disk::Dir::Data;
			const SUB_DIRECTORIES: &'static str = "";
		}
	};

	TokenStream::from(expanded)
}
