//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use crossbeam::channel::Sender;
use benri::send;
use std::sync::atomic::AtomicBool;
use log::warn;
use crate::constants::{
	FESTIVAL_DBUS,
	FESTIVAL,
};

//---------------------------------------------------------------------------------------------------- Media Controls
// FIXME:
// - Windows image URI doesn't work
// - Windows previous/next doesn't work
pub(super) fn init_media_controls(to_audio: Sender<souvlaki::MediaControlEvent>) -> Result<souvlaki::MediaControls, anyhow::Error> {
	#[cfg(windows)]
	let hwnd = {
		let dummy_window = windows::DummyWindow::new()?;
		let hwnd = Some(dummy_window.handle.0 as _);

		// Leak this, it should be open forever.
		std::mem::forget(dummy_window);

		hwnd
	};

	#[cfg(unix)]
	let hwnd = None;

	let config = souvlaki::PlatformConfig {
		dbus_name: FESTIVAL_DBUS,
		display_name: FESTIVAL,
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

// Taken from https://github.com/Sinono3/souvlaki/blob/master/examples/print_events.rs
#[cfg(windows)]
mod windows {
	use anyhow::anyhow;
	use std::io::Error;
	use std::mem;

	use windows::core::PCWSTR;
	use windows::w;
	use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
	use windows::Win32::System::LibraryLoader::GetModuleHandleW;
	use windows::Win32::UI::WindowsAndMessaging::{
		CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetAncestor,
		IsDialogMessageW, PeekMessageW, RegisterClassExW, TranslateMessage, GA_ROOT, MSG,
		PM_REMOVE, WINDOW_EX_STYLE, WINDOW_STYLE, WM_QUIT, WNDCLASSEXW,
	};

	pub(super) struct DummyWindow {
		pub(super) handle: HWND,
	}

	impl DummyWindow {
		pub(super) fn new() -> Result<DummyWindow, anyhow::Error> {
			let class_name = w!("SimpleTray");

			let handle_result = unsafe {
				let instance = GetModuleHandleW(None).map_err(|e| (anyhow!("Getting module handle failed: {e}")))?;

				let wnd_class = WNDCLASSEXW {
					cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
					hInstance: instance,
					lpszClassName: class_name,
					lpfnWndProc: Some(Self::wnd_proc),
					..Default::default()
				};

				if RegisterClassExW(&wnd_class) == 0 {
					return Err(anyhow!(
						"Registering class failed: {}",
						Error::last_os_error()
					));
				}

				let handle = CreateWindowExW(
					WINDOW_EX_STYLE::default(),
					class_name,
					w!(""),
					WINDOW_STYLE::default(),
					0,
					0,
					0,
					0,
					None,
					None,
					instance,
					None,
				);

				if handle.0 == 0 {
					Err(anyhow!(
						"Message only window creation failed: {}",
						Error::last_os_error()
					))
				} else {
					Ok(handle)
				}
			};

			handle_result.map(|handle| DummyWindow { handle })
		}

		extern "system" fn wnd_proc(
			hwnd: HWND,
			msg: u32,
			wparam: WPARAM,
			lparam: LPARAM,
		) -> LRESULT {
			unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
		}
	}

	impl Drop for DummyWindow {
		fn drop(&mut self) {
			unsafe {
				DestroyWindow(self.handle);
			}
		}
	}

	pub(super) fn pump_event_queue() -> bool {
		unsafe {
			let mut msg: MSG = std::mem::zeroed();
			let mut has_message = PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool();
			while msg.message != WM_QUIT && has_message {
				if !IsDialogMessageW(GetAncestor(msg.hwnd, GA_ROOT), &msg).as_bool() {
					TranslateMessage(&msg);
					DispatchMessageW(&msg);
				}

				has_message = PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool();
			}

			msg.message == WM_QUIT
		}
	}
}
