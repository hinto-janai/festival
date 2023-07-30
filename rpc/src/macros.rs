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
/// Implement an anonymous map of JSON.
macro_rules! impl_struct_anon {
	($struct:ident, $type:ty) => {
		#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
		#[serde(transparent)]
		#[repr(transparent)]
		pub struct $struct(pub $type);
	}
}

#[macro_export]
/// Implement a named map of JSON with a lifetime: `'a`.
macro_rules! impl_struct_lt {
	($struct:ident, $($( #[$attrs:meta] )* $field:ident: $type:ty),*) => {
		#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
		pub struct $struct<'a> {
			$(
				$( #[$attrs] )*
				pub $field: $type,
			)*
		}
	}
}

#[macro_export]
/// Implement an anonymous map of JSON with a lifetime: `'a`.
macro_rules! impl_struct_anon_lt {
	($struct:ident, $type:ty) => {
		#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
		#[serde(transparent)]
		#[repr(transparent)]
		pub struct $struct<'a>(#[serde(borrow)] $type);
	}
}

