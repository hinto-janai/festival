//---------------------------------------------------------------------------------------------------- Use
use std::sync::atomic::AtomicBool;
use benri::atomic_load;

//---------------------------------------------------------------------------------------------------- Saving.
pub(crate) static SAVING: AtomicBool = AtomicBool::new(false);
#[inline(always)]
/// This [`bool`] represents if a [`Collection`] that was
/// recently created is still being written to the disk.
///
/// For performance reasons, when the `Frontend` asks [`Kernel`]
/// for a new [`Collection`], [`Kernel`] will return immediately upon
/// having an in-memory [`Collection`]. However, `shukusai` will
/// (in the background) be saving it disk.
///
/// If your `Frontend` exits around this time, it should probably hang
/// (for a reasonable amount of time) if this returns `true`, waiting
/// for the [`Collection`] to be saved to disk.
pub fn saving() -> bool {
	atomic_load!(SAVING)
}

//---------------------------------------------------------------------------------------------------- Media Controls
pub(crate) static MEDIA_CONTROLS_RAISE: AtomicBool = AtomicBool::new(false);
#[inline(always)]
/// The user sent a signal via the OS Media Control's that the main window should be raised.
pub fn media_controls_raise() -> bool {
	atomic_load!(MEDIA_CONTROLS_RAISE)
}

pub(crate) static MEDIA_CONTROLS_SHOULD_EXIT: AtomicBool = AtomicBool::new(false);
#[inline(always)]
/// The user sent a signal via the OS Media Control's that we should exit (all of Festival).
pub fn media_controls_should_exit() -> bool {
	atomic_load!(MEDIA_CONTROLS_SHOULD_EXIT)
}
