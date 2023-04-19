//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use benri::{
//};
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};

//---------------------------------------------------------------------------------------------------- __NAME__
pub const ALPHABET_KEY_PRESSES: [egui::Key; 27] = [
	egui::Key::A,
	egui::Key::A,
	egui::Key::B,
	egui::Key::C,
	egui::Key::D,
	egui::Key::E,
	egui::Key::F,
	egui::Key::G,
	egui::Key::H,
	egui::Key::I,
	egui::Key::J,
	egui::Key::K,
	egui::Key::L,
	egui::Key::M,
	egui::Key::N,
	egui::Key::O,
	egui::Key::P,
	egui::Key::Q,
	egui::Key::R,
	egui::Key::S,
	egui::Key::T,
	egui::Key::U,
	egui::Key::V,
	egui::Key::W,
	egui::Key::X,
	egui::Key::Y,
	egui::Key::Z,
];

pub enum KeyPress {}

impl KeyPress {
	pub fn from_egui_key(key: &egui::Key) -> &'static str {
		use egui::Key::*;
		match key {
			A => "a",
			B => "b",
			C => "c",
			D => "d",
			E => "e",
			F => "f",
			G => "g",
			H => "h",
			I => "i",
			J => "j",
			K => "k",
			L => "l",
			M => "m",
			N => "n",
			O => "o",
			P => "p",
			Q => "q",
			R => "r",
			S => "s",
			T => "t",
			U => "u",
			V => "v",
			W => "w",
			X => "x",
			Y => "y",
			Z => "z",
			_ => "",
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
