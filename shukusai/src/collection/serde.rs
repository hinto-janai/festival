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
use serde::Serializer;

//---------------------------------------------------------------------------------------------------- Serialize.
pub fn runtime<S: Serializer>(r: &readable::Runtime, s: S) -> Result<S::Ok, S::Error> {
	s.serialize_u32(r.inner())
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
