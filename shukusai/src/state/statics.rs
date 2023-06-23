//---------------------------------------------------------------------------------------------------- Use
use std::sync::atomic::{
	AtomicBool,
	AtomicU8,
};
use crate::audio::Volume;
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

//---------------------------------------------------------------------------------------------------- Volume.
/// The global [`Volume`] level that `Audio` will play samples at
///
/// HACK:
/// This is a giant giant giant workaround for preventing a sort of DoS
/// situation when it comes to sending many `Volume` signals to `Kernel`
/// which eventually get forwarded to `Audio`.
///
/// `Audio` only has so much time to process signals before it must
/// continue on the decode/demux the next sample, otherwise audio
/// playback would end up choppy.
///
/// `GUI` however, can send many many many `Volume` signals in a short
/// period of time (simply dragging the volume slider), which creates
/// a sort of overflow of queue messages sent to `Kernel`, to `Audio`.
///
/// This causes a buggy looking delay for `GUI`, since it copies the global
/// `AUDIO_STATE`'s `Volume`, which may not actually be updated, since `Audio`
/// hasn't processed those signals yet.
///
/// To get around this, we completely avoid the `Kernel` <-> `Audio` message pass
/// and `Frontend`'s should simply mutate this value, which `Audio` will unconditionally
/// use every loop iteration.
///
/// The `Volume` signal is still available, but `Audio` will just mutate this value
/// accordingly, instead of locking the global `AUDIO_STATE` (which takes too much time sometimes).
///
/// If this value is > 100, it will be set to 100.
pub static VOLUME: AtomicU8 = AtomicU8::new(Volume::const_default().inner());

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
