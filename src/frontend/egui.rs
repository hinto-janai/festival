//---------------------------------------------------------------------------------------------------- Use
use std::sync::atomic::AtomicBool;
use std::thread::Thread;
use once_cell::sync::OnceCell;

//---------------------------------------------------------------------------------------------------- __NAME__
/// `egui` frontend should set this to `true` before
/// starting `eframe::App::update` and `false` after it finishes.
///
/// This allows `shukusai` to coordinate `egui::Context` locks better.
pub static GUI_UPDATING: AtomicBool = AtomicBool::new(true);

/// The main `egui` frontend thread should set this value.
///
/// This allows `shukusai` to:
/// - wake-up the GUI thread when appropriate
/// - access `Context` state
pub static GUI_CONTEXT: OnceCell<egui::Context> = OnceCell::new();

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
