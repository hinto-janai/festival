// Top-level convenience macros.

//---------------------------------------------------------------------------------------------------- Use

//---------------------------------------------------------------------------------------------------- __NAME__
// Exit the whole program with an error message WITHOUT running destructors.
#[macro_export]
macro_rules! exit {
	($($msg:tt)*) => {{
		::std::eprintln!("festival-cli error: {}", ::std::format_args!($($msg)*));
		::std::process::exit(1);
	}}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
