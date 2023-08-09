//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};
use bincode::{Encode,Decode};
use strum::{
	AsRefStr,
	Display,
	EnumCount,
	EnumIter,
	EnumString,
	EnumVariantNames,
	IntoStaticStr,
};

//---------------------------------------------------------------------------------------------------- Volume.
#[derive(Copy,Clone,Default,Debug,Hash,Eq,Ord,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
#[derive(AsRefStr,Display,EnumCount,EnumIter,EnumString,EnumVariantNames,IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
/// The different ways you can append songs to the audio queue.
///
/// The [`Default`] is `Append::Front`.
pub enum Append {
	#[default]
	/// Add a single or multiple songs to the front.
	///
	/// Queue:
	/// - Before: `a, b, c`
	/// - Input: `1, 2, 3`
	/// - After: `1, 2, 3, a, b, c`
	Front,

	/// Add a single or multiple songs to the back.
	///
	/// - Before: `a, b, c`
	/// - Input: `1, 2, 3`
	/// - After: `a, b, c, 1, 2, 3`
	Back,

	/// Add a single or multiple songs starting at an index.
	///
	/// - Before: `a, b, c`
	/// - Input: `1, 2, 3` with index `1`
	/// - After: `a, 1, 2, 3, b, c`
	Index(usize),
}

#[derive(Copy,Clone,Default,Debug,Hash,Eq,Ord,PartialEq,PartialOrd,Serialize,Deserialize,Encode,Decode)]
#[derive(AsRefStr,Display,EnumCount,EnumIter,EnumString,EnumVariantNames,IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
/// For `festivald` + `festival-cli`.
pub enum Append2 {
	/// [`Append::Front`]
	#[default]
	Front,
	/// [`Append::Back`]
	Back,
	/// [`Append::Index`], the index is specified in a different field.
	///
	/// This is because `clap` gets weird with `enum`'s with values.
	///
	/// `festival-cli ... --append index ...` is allowed but
	/// it does not let us specify the value.
	Index,
}
