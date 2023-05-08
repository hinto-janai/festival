//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use crate::constants::ALBUM_ART_SIZE_DEFAULT;

//---------------------------------------------------------------------------------------------------- Settings
#[derive(Clone,Debug,Default,PartialEq,Serialize,Deserialize,Encode,Decode)]
/// Does the user want a certain amount of `Album`'s per row or a static pixel size?
pub enum AlbumSizing {
	#[default]
	/// Album art will be `x` pixels wide
	Pixel,

	/// `x` amount of albums per row
	///
	/// (pixel size will scale to fit them)
	Row,
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod test {
//  #[test]
//  fn _() {
//  }
//}
