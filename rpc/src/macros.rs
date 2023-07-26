//---------------------------------------------------------------------------------------------------- Impl macros
#[macro_export]
/// Implement a named map of JSON.
macro_rules! impl_struct {
	($struct:ident, $($field:ident: $type:ty),*) => {
		#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
		pub struct $struct {
			$(
				pub $field: $type,
			)*
		}
	}
}

#[macro_export]
/// Implement a named map of JSON with a lifetime: `'a`.
macro_rules! impl_struct_lt {
	($struct:ident, $($field:ident: $type:ty),*) => {
		#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
		pub struct $struct<'a> {
			$(
				pub $field: $type,
			)*
		}
	}
}
