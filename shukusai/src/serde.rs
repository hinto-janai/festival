// Some custom `serde` code for when `Collection` and
// the inner types need to be serialized with `serde`,
// and not `bincode`.
//
// This currently only exists so that converting
// `Collection` types directly into JSON is easy.
//
// This is _not_ meant for actual lossless conversions between
// `struct`'s and JSON, hence there is no `Deserialize` impl here.
//
// This is solely meant to represent these structures as JSON,
// which are needed for `festivald`, `festival-cli`, etc.
//
// This serialization must be stable, as the output is directly
// used in the `Request`, `Response` parts of the above two.
//
// These free functions exist because we want to represent
// some types in a more JSON-friendly manner, e.g, instead
// of serializing `readable::Runtime`'s inner string buffer
// as an array, length, etc, end-users of the JSON output
// probably only care about the `u32`.

//---------------------------------------------------------------------------------------------------- Use.
use crate::collection::Art;
use readable::{Date, Runtime, Unsigned};
use serde::Serializer;

//---------------------------------------------------------------------------------------------------- Readable
// Serialize as `u32`
pub fn runtime<S: Serializer>(r: &Runtime, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(r.inner())
}

// Serialize as `string`, "2018-04-25"
pub fn date<S: Serializer>(r: &Date, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(r.as_str())
}

// Serialize as `u64`
pub fn unsigned<S: Serializer>(r: &Unsigned, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u64(r.inner())
}

//---------------------------------------------------------------------------------------------------- Art
#[cfg(feature = "gui")]
// Serialize as a string.
//
// This will never be used but to keep
// everything unified it is implemented.
pub fn art<S: Serializer>(r: &Art, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(match r {
        Art::Known(_) => "Known",
        Art::Bytes(_) => "Bytes",
        Art::Unknown => "Unknown",
    })
}

#[cfg(feature = "daemon")]
// Serialize the byte length, or `null`, so as if it were `Option<usize>`.
pub fn art<S: Serializer>(r: &Art, s: S) -> Result<S::Ok, S::Error> {
    match r {
        Art::Known { len, .. } => s.serialize_u64(*len as u64),
        _ => s.serialize_none(),
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
