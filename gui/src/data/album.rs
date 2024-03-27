//---------------------------------------------------------------------------------------------------- Use
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumString, EnumVariantNames, IntoStaticStr};

//---------------------------------------------------------------------------------------------------- Settings
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    AsRefStr,
    Display,
    EnumCount,
    EnumIter,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
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
