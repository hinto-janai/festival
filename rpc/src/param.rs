//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use std::path::PathBuf;

//---------------------------------------------------------------------------------------------------- Impl macros
// Implement a named map of JSON.
macro_rules! impl_param {
	($struct:ident, $($field:ident: $type:ty),*) => {
		#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
		pub struct $struct {
			$(
				pub $field: $type,
			)*
		}
	}
}

// Implement a fixed size, anonymous JSON array.
macro_rules! impl_param_array {
	($struct:ident, $type:ty, $len:literal) => {
		#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
		#[serde(transparent)]
		#[repr(transparent)]
		pub struct $struct(pub [$type; $len]);
	}
}

// Implement a dynamically size, anonymous JSON array.
macro_rules! impl_param_vec {
	($struct:ident, $type:ty) => {
		#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
		#[serde(transparent)]
		#[repr(transparent)]
		pub struct $struct(pub Vec<$type>);
	}
}

//---------------------------------------------------------------------------------------------------- Param impl
impl_param!(Previous, threshold: Option<u32>);
impl_param!(Volume, volume: u8);
impl_param!(AddQueueSong, key: usize, append: String, clear: bool);
impl_param!(AddQueueAlbum, key: usize, append: String, clear: bool, offset: usize);
impl_param!(AddQueueArtist, key: usize, append: String, clear: bool, offset: usize);
impl_param!(Clear, r#continue: bool);
impl_param!(Seek, seek: String, second: u64);
impl_param!(Skip, skip: usize);
impl_param!(Back, back: usize);
impl_param!(SetQueueIndex, index: usize);
impl_param!(RemoveQueueRange, start: usize, end: usize, skip: bool);
impl_param!(Search, input: String, kind: String);

impl_param_vec!(NewCollection, PathBuf);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
