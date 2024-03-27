//---------------------------------------------------------------------------------------------------- Use
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumString, EnumVariantNames, IntoStaticStr};

//---------------------------------------------------------------------------------------------------- __NAME__
/// The table in the `Search` tab can show results
/// as the `Song` title, `Album` title, or `Artist` name.
///
/// This selects which one it is.
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
pub enum SearchSort {
    Song,
    #[default]
    Album,
    Artist,
}

impl SearchSort {
    /// No [`String`] allocation.
    pub fn human(&self) -> &'static str {
        match self {
            Self::Song => "Song",
            Self::Album => "Album",
            Self::Artist => "Artist",
        }
    }

    /// Returns an iterator over all the variants.
    pub fn iter() -> std::slice::Iter<'static, Self> {
        [Self::Song, Self::Album, Self::Artist].iter()
    }

    /// Returns the next sequential [`SearchSort`] variant.
    ///
    /// This returns the _first_ if at the _last_.
    pub fn next(&self) -> Self {
        match self {
            Self::Song => Self::Album,
            Self::Album => Self::Artist,
            Self::Artist => Self::Song,
        }
    }

    /// Returns the previous sequential [`SongSort`] variant.
    ///
    /// This returns the _last_ if at the _first_.
    pub fn previous(&self) -> Self {
        match self {
            Self::Song => Self::Artist,
            Self::Album => Self::Song,
            Self::Artist => Self::Album,
        }
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Asserts each variant:
    // 1. Gives a different string
    // 2. `.next()` gives a different variant
    // 3. `.prev()` gives a different variant
    fn diff() {
        let mut set1 = std::collections::HashSet::new();
        let mut set2 = std::collections::HashSet::new();
        let mut set3 = std::collections::HashSet::new();

        for i in SearchSort::iter() {
            assert!(set1.insert(i.human()));
            assert!(set2.insert(i.next()));
            assert!(set3.insert(i.previous()));
        }
    }
}
