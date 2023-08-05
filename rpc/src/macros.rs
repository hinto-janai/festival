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
		pub struct $struct<'a>(#[serde(borrow)] pub $type);
	}
}

//---------------------------------------------------------------------------------------------------- Impl macros for clap (request/params)
#[macro_export]
macro_rules! impl_rpc {
	($method_doc:literal, $method_link:literal, $method_name:ident => $method_enum:expr) => {
		#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        #[derive(clap::Args)]
		#[command(about = $method_doc, long_about = include_str!(concat!("../../cli/mdbook/src/json-rpc/", $method_link, ".md")))]
		pub struct $method_name;

		impl $method_name {
	        pub fn request<'a>(id: Option<json_rpc::Id<'a>>) -> json_rpc::Request<'a, crate::method::Method, ()> {
	            json_rpc::Request::new(
	                Cow::Owned($method_enum),
	                None,
	                id,
	            )
	        }
		}
	}
}

#[macro_export]
macro_rules! impl_rpc_param {
	($method_doc:literal, $method_link:literal, $method_name:ident => $method_enum:expr, $($param_doc:literal, $( #[$attrs:meta] )* $param:ident: $param_type:ty),*) => {
		#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        #[derive(clap::Args)]
		#[command(about = $method_doc, long_about = include_str!(concat!("../../cli/mdbook/src/json-rpc/", $method_link, ".md")))]
		pub struct $method_name {
			$(
				#[doc = $param_doc]
				#[arg(long, verbatim_doc_comment)]
				$( #[$attrs] )*
				pub $param: $param_type,
			)*
		}

		impl $method_name {
	        pub fn request<'a>(&'a self, id: Option<json_rpc::Id<'a>>) -> json_rpc::Request<'a, crate::method::Method, Self> {
	            json_rpc::Request::new(
	                Cow::Owned($method_enum),
	                Some(Cow::Borrowed(&self)),
	                id,
	            )
	        }
		}
	}
}
