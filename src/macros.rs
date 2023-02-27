// | MACRO   | PURPOSE                                                  | EQUIVALENT CODE                                            |
// |---------|----------------------------------------------------------|------------------------------------------------------------|
// | lock    | Lock an [Arc<Mutex>]                                     | a.lock().unwrap()                                          |
// | lock2   | Lock a field inside a struct, both Arc<Mutex>            | a.lock().unwrap().b.lock().unwrap()                        |
// | arc_mut | Create a new [Arc<Mutex>]                                | std::sync::Arc::new(std::sync::Mutex::new(my_value))       |
// | sleep   | Sleep the current thread for x milliseconds              | std::thread::sleep(std::time::Duration::from_millis(1000)) |
// | flip    | Flip a bool in place                                     | my_bool = !my_bool                                         |
// | ok      | FORWARDS input to info!() appended with green "... OK"   | info!("{} {}", my_msg, "... OK")                           |
// | skip    | FORWARDS input to info!() appended with white "... SKIP" | info!("{} {}", my_msg, "... SKIP")                         |
// | fail    | FORWARDS input to error!() appended with red "... FAIL"  | error!("{} {}", my_msg, "... FAIL")                        |
//
// [lock2!()] works like this: "lock2!(my_struct, my_field)"
// and expects it be a [Struct]-[field] relationship, e.g:
// ```
// let struct = Arc::new(Mutex::new(Struct {
//     field: Arc::new(Mutex::new(true)),
// }));
// assert!(*lock2!(struct, field) == true);
// ```
//
// The equivalent code is: "struct.lock().unwrap().field.lock().unwrap()"

macro_rules! lock {
	($arc_mutex:expr) => {
		$arc_mutex.lock().expect("Failed to lock Mutex")
	};
}
pub(crate) use lock;

macro_rules! lock2 {
	($arc_mutex:expr, $arc_mutex_two:ident) => {
		$arc_mutex.lock().expect("Failed to lock Mutex").$arc_mutex_two.lock().expect("Failed to lock Mutex")
	};
}
pub(crate) use lock2;

macro_rules! arc_mut {
	($arc_mutex:expr) => {
		std::sync::Arc::new(std::sync::Mutex::new($arc_mutex))
	};
}
pub(crate) use arc_mut;

macro_rules! sleep {
    ($millis:expr) => {
		std::thread::sleep(std::time::Duration::from_millis($millis))
    };
}
pub(crate) use sleep;

macro_rules! flip {
	($b:expr) => {
		match $b {
			true|false => $b = !$b,
		}
	};
}
pub(crate) use flip;

macro_rules! ok {
	($($tts:tt)*) => {
			log::info!("{} {} {}{}{}", $($tts)*, "...", "\x1b[1;92m", "OK", "\x1b[0m");
		}
	}
pub(crate) use ok;

macro_rules! ok_debug {
	($($tts:tt)*) => {
			log::debug!("{} {} {}{}{}", $($tts)*, "...", "\x1b[1;92m", "OK", "\x1b[0m");
		}
	}
pub(crate) use ok_debug;

macro_rules! ok_trace {
	($($tts:tt)*) => {
			log::trace!("{} {} {}{}{}", $($tts)*, "...", "\x1b[1;92m", "OK", "\x1b[0m");
		}
	}
pub(crate) use ok_trace;

macro_rules! skip {
	($($tts:tt)*) => {
			log::info!("{} {} {}{}{}", $($tts)*, "...", "\x1b[1;97m", "SKIP", "\x1b[0m");
		}
	}
pub(crate) use skip;

macro_rules! fail {
	($($tts:tt)*) => {
			log::error!("{} {} {}{}{}", $($tts)*, "...", "\x1b[1;91m", "FAIL", "\x1b[0m");
		}
	}
pub(crate) use fail;

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	#[test]
	fn lock() {
		use std::sync::{Arc,Mutex};
		let arc_mutex = Arc::new(Mutex::new(false));
		*lock!(arc_mutex) = true;
		assert!(*lock!(arc_mutex) == true);
	}

	#[test]
	fn lock2() {
		struct Ab {
			a: Arc<Mutex<bool>>,
		}
		use std::sync::{Arc,Mutex};
		let arc_mutex = Arc::new(Mutex::new(
			Ab {
				a: Arc::new(Mutex::new(false)),
			}
		));
		*lock2!(arc_mutex,a) = true;
		assert!(*lock2!(arc_mutex,a) == true);
	}

	#[test]
	fn arc_mut() {
		let a = arc_mut!(false);
		assert!(*lock!(a) == false);
	}

	#[test]
	fn flip() {
		let mut b = true;
		flip!(b);
		assert!(b == false);
	}
}
