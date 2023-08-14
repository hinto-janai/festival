//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use anyhow::anyhow;
use log::{error,info,warn,debug,trace};
use disk::{Bincode2,Json};
use strum::{
	AsRefStr,
	Display,
	EnumCount,
	EnumIter,
	EnumString,
	EnumVariantNames,
	IntoStaticStr,
};

//---------------------------------------------------------------------------------------------------- __NAME__
#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[derive(AsRefStr,Display,EnumCount,EnumIter,EnumString,EnumVariantNames,IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
/// Enum of all the the "resources" provided by the REST API.
pub enum Resource {
	Collection,
	Playlist,
	Artist,
	Album,
	Song,
	Art,
}

impl Resource {
	pub fn from_str_not_c(s: &str) -> Option<Self> {
		match s {
			"playlist" => Some(Self::Playlist),
			"artist"   => Some(Self::Artist),
			"album"    => Some(Self::Album),
			"song"     => Some(Self::Song),
			"art"      => Some(Self::Art),
			_ => None,
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
