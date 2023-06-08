//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use crossbeam::channel::Sender;
use benri::send;
use std::sync::atomic::AtomicBool;
use log::warn;

//---------------------------------------------------------------------------------------------------- Media Controls
/// The user sent a signal via the OS Media Control's that the main window should be raised.
pub static MEDIA_CONTROLS_RAISE: AtomicBool = AtomicBool::new(false);

/// The user sent a signal via the OS Media Control's that we should exit (all of Festival).
pub static MEDIA_CONTROLS_SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

pub(super) fn init_media_controls(to_audio: Sender<souvlaki::MediaControlEvent>) -> Result<souvlaki::MediaControls, anyhow::Error> {
	#[cfg(target_os = "windows")]
	let hwnd = todo!();

	#[cfg(not(target_os = "windows"))]
	let hwnd = None;

	let config = souvlaki::PlatformConfig {
		dbus_name: crate::FESTIVAL_DBUS,
		display_name: crate::FESTIVAL,
		hwnd,
	};

	let Ok(mut media_controls) = souvlaki::MediaControls::new(config) else {
		bail!("souvlaki::MediaControls::new() failed");
	};

	match media_controls.attach(move |event| {
		// We don't want to kill all of `Festival`
		// if the media controls die, it's not that important.
		if let Err(e) = to_audio.send(event) {
			warn!("Media controls failed to send to Audio: {e}");
		}
	}) {
		Ok(_)  => Ok(media_controls),
		Err(e) => Err(anyhow!("souvlaki::MediaControls::attach() failed")),
	}
}
