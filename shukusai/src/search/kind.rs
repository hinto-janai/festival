//---------------------------------------------------------------------------------------------------- Use
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumString, EnumVariantNames, IntoStaticStr};

//---------------------------------------------------------------------------------------------------- Sort Constants
/// [`SearchKind::All`]
pub const ALL: &str = "View all the results, sorted from most similar to least";
/// [`SearchKind::Sim60`]
pub const SIM_60: &str = "View only the results that are at least 60% similar";
/// [`SearchKind::Sim70`]
pub const SIM_70: &str = "View only the results that are at least 70% similar";
/// [`SearchKind::Sim80`]
pub const SIM_80: &str = "View only the results that are at least 80% similar";
/// [`SearchKind::Top25`]
pub const TOP_25: &str = "View only the top 25 similar results";
/// [`SearchKind::Top5`]
pub const TOP_5: &str = "View only the top 5 similar results";
/// [`SearchKind::Top1`]
pub const TOP_1: &str = "View only the top 1 similar results";

//---------------------------------------------------------------------------------------------------- SearchKind
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
/// The different kinds of searches you can request from `Kernel`
pub enum SearchKind {
    /// String similarity, returns all calculated keys
    /// in order from most similar to least.
    All,
    /// [`Self::All`], but only returns the results that are at least 60% similar
    Sim60,
    #[default]
    /// [`Self::All`], but only returns the results that are at least 70% similar
    Sim70,
    /// [`Self::All`], but only returns the results that are at least 80% similar
    Sim80,
    /// [`Self::All`], but only returns the top 25 results
    Top25,
    /// [`Self::All`], but only returns the top 5 results
    Top5,
    /// [`Self::All`], but only returns the top 1 results
    Top1,
}

impl SearchKind {
    #[inline]
    /// Returns formatted, human readable versions.
    pub const fn human(&self) -> &'static str {
        match self {
            Self::Sim60 => SIM_60,
            Self::Sim70 => SIM_70,
            Self::Sim80 => SIM_80,
            Self::Top25 => TOP_25,
            Self::Top5 => TOP_5,
            Self::Top1 => TOP_1,
            Self::All => ALL,
        }
    }

    /// Returns the next sequential [`Self`] variant.
    ///
    /// This returns the _first_ if at the _last_.
    pub fn next(&self) -> Self {
        match self {
            Self::All => Self::Sim60,
            Self::Sim60 => Self::Sim70,
            Self::Sim70 => Self::Sim80,
            Self::Sim80 => Self::Top25,
            Self::Top25 => Self::Top5,
            Self::Top5 => Self::Top1,
            Self::Top1 => Self::All,
        }
    }

    /// Returns the previous sequential [`Self`] variant.
    ///
    /// This returns the _last_ if at the _first_.
    pub fn previous(&self) -> Self {
        match self {
            Self::All => Self::Top1,
            Self::Sim60 => Self::All,
            Self::Sim70 => Self::Sim60,
            Self::Sim80 => Self::Sim70,
            Self::Top25 => Self::Sim80,
            Self::Top5 => Self::Top25,
            Self::Top1 => Self::Top5,
        }
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use strum::*;

    #[test]
    // Asserts each variant:
    // 1. Gives a different string
    // 2. `.next()` gives a different variant
    // 3. `.prev()` gives a different variant
    fn diff() {
        let mut set1 = std::collections::HashSet::new();
        let mut set2 = std::collections::HashSet::new();
        let mut set3 = std::collections::HashSet::new();

        for i in SearchKind::iter() {
            assert!(set1.insert(i.human()));
            assert!(set2.insert(i.next()));
            assert!(set3.insert(i.previous()));
        }
    }
}
