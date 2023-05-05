//---------------------------------------------------------------------------------------------------- Use
use std::sync::atomic::AtomicBool;

//---------------------------------------------------------------------------------------------------- __NAME__
/// `egui` frontend should set this to `true` before
/// starting `eframe::App::update` and `false` after it finishes.
///
/// This allows `shukusai` to coordinate `egui::Context` locks better.
pub static UPDATING: AtomicBool = AtomicBool::new(true);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
