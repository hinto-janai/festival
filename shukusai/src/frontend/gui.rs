//---------------------------------------------------------------------------------------------------- Use
use once_cell::sync::OnceCell;
use std::sync::atomic::AtomicBool;

//---------------------------------------------------------------------------------------------------- __NAME__
/// `GUI` frontend should set this to `true` before
/// starting `eframe::App::update` and `false` after it finishes.
///
/// This allows `shukusai` to coordinate `egui::Context` locks better.
pub static GUI_UPDATING: AtomicBool = AtomicBool::new(true);

/// The main `GUI` frontend thread should set this value.
///
/// This allows `shukusai` to:
/// - wake-up the GUI thread when appropriate
/// - access `Context` state
pub static GUI_CONTEXT: OnceCell<egui::Context> = OnceCell::new();

/// Convenience function that unsafely `.get()`'s the [`GUI_CONTEXT`].
///
/// # Safety
/// This causes UB if [`GUI_CONTEXT`] is not initialized
/// since `.get_unchecked()` is used.
pub fn gui_context() -> &'static egui::Context {
    unsafe { GUI_CONTEXT.get_unchecked() }
}

/// Convenience function that requests the
/// `GUI` to update a frame using [`GUI_CONTEXT`]
///
/// # Safety
/// This causes UB if [`GUI_CONTEXT`] is not initialized
/// since `.get_unchecked()` is used.
pub fn gui_request_update() {
    unsafe {
        GUI_CONTEXT.get_unchecked().request_repaint();
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
