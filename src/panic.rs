//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::kernel::Kernel;
use crate::FESTIVAL;
use serde::{Serialize,Deserialize};
use benri::mass_panic;
use disk::Plain;
use std::fmt::Write;

//----------------------------------------------------------------------------------------------------
/// Set `shukusai`'s custom panic hook.
pub(crate) fn set_panic_hook() {
	std::panic::set_hook(Box::new(|panic_info| {
		// Set stack-trace (bunch of <???> on release builds, so ignore.)
		#[cfg(debug_assertions)]
		let stack_trace = std::backtrace::Backtrace::force_capture();
		#[cfg(not(debug_assertions))]
		let stack_trace = "<Release builds stack symbols were stripped>";

		// Re-format panic info.
		let panic_info = format!(
"{:#?}\n\n{:#?}\n
info:
   OS      | {} {}
   args    | {:?}
   build   | {}
   commit  | {}   threads | {}
   version | {}
   elapsed | {} seconds\n
stack backtrace:\n{}",
			panic_info,
			std::thread::current(),
			std::env::consts::OS,
			std::env::consts::ARCH,
			std::env::args_os(),
			crate::constants::BUILD,
			crate::constants::COMMIT,
			crate::thread::threads_available(),
			crate::constants::FESTIVAL_NAME_VER,
			crate::logger::init_instant().elapsed().as_secs_f64(),
			stack_trace,
		);
		// Attempt to write panic info to disk.
		let panic = crate::panic::Panic(panic_info.clone());
		let path  = crate::panic::Panic::absolute_path();
		let save  = panic.save();
		match (save, path) {
			(Ok(_), Ok(p)) => eprintln!("\nmass_panic!() - Saved panic log to: {}\n", p.display()),
			(Ok(_), _)     => eprintln!("\nmass_panic!() - Saved panic log in festival folder.\n"),
			_              => eprintln!("\nmass_panic!() - Could not save panic log\n"),
		}
		// Exit all threads.
		mass_panic!(panic_info);
	}));
}

//----------------------------------------------------------------------------------------------------
disk::plain!(Panic, disk::Dir::Data, FESTIVAL, "", "panic");
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
#[serde(transparent)]
/// File representing a `panic!()` log.
///
/// This gets written in the `festival` folder as `panic.txt`.
pub struct Panic(pub(crate) String);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
