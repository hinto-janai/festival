//---------------------------------------------------------------------------------------------------- Use
use clap::{Args,Subcommand};
use const_format::formatcp;
use crate::cli::BIN;
use std::borrow::Cow;

//---------------------------------------------------------------------------------------------------- Impl Macro
macro_rules! impl_rpc {
	($($doc:literal, $method:ident => $method_rpc:expr),*) => {
		$(
			#[derive(clap::Args)]
			#[doc = $doc]
			pub struct $method;

			pub fn request<'a>(id: json_rpc::Id<'a>) -> json_rpc::Request<'a, rpc::Method, ()> {
				json_rpc::Request::new(
					Cow::Owned($method_rpc),
					None,
					Some(id),
				)
			}
		)*
	}
}

macro_rules! impl_rpc_param {
	($doc:literal, $method:ident, $($param_doc:literal, $param_name:ident: $param_type:ty),*) => {
		#[derive(clap::Args)]
		#[command(arg_required_else_help(true))]
		#[command(override_usage = formatcp!("{BIN} data {} [--param <value>]", $method_lit))]
		#[doc = $doc]
		pub struct $method {
			$(
				#[arg(long, verbatim_doc_comment)]
				#[doc = $param_doc]
				$param_name: $param_type,
			)*
		}
	}
}

//---------------------------------------------------------------------------------------------------- Impl Method, No Params
impl_rpc! {
	"collection_new",
	CollectionNew => rpc::Method::CollectionNew
}

//---------------------------------------------------------------------------------------------------- Impl Method with Params
//impl_rpc_param! {
//}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
