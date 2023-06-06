// Some macros for `ui` and some that need `self` and egui's `ui`.
//
// These UI layouts appear in many places, thus reusable macros are here.
//
// These are macros instead of functions because
// `self/ui` mutable borrow rules prevent it.

//---------------------------------------------------------------------------------------------------- Use
use crate::data::Gui;
use egui::{
	Ui,Context,Frame,
	Label,ImageButton,
};
use shukusai::collection::{
	Album,AlbumKey,
};
use log::warn;

//---------------------------------------------------------------------------------------------------- Misc

//---------------------------------------------------------------------------------------------------- `self/ui`-based
#[macro_export]
/// Set the last tab, jump to a tab.
macro_rules! tab {
	($self:ident, $tab:expr) => {
		$self.state.last_tab = Some($self.state.tab);
		$self.state.tab      = $tab;
	}
}

#[macro_export]
/// Set an `Album`, set the last tab, jump to the view tab.
macro_rules! album {
	($self:ident, $key:expr) => {
		$self.state.album    = Some($key.into());
		$self.state.last_tab = Some($self.state.tab);
		$self.state.tab      = $crate::data::Tab::View;
	}
}

#[macro_export]
/// Set an `Artist`, set the last tab, jump to the Artist view sub-tab.
macro_rules! artist {
	($self:ident, $key:expr) => {
		$self.state.artist   = Some($key.into());
		$self.state.last_tab = Some($self.state.tab);
		$self.state.tab      = $crate::data::Tab::Artists;
		$self.settings.artist_sub_tab = $crate::data::ArtistSubTab::View;
	}
}

#[macro_export]
/// Send a single `Song` to `Kernel` to play.
///
/// This indicates:
/// - Queue should be cleared
/// - `Song` clicked should be immediate played
macro_rules! play_song {
	($self:ident, $key:expr) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::AddQueueSong(($key, shukusai::kernel::Append::Front, true))
		);
		::benri::send!($self.to_kernel, shukusai::kernel::FrontendToKernel::Play);
	}
}

#[macro_export]
/// Append a single `Song` to the end of the queue.
///
/// This indicates:
/// - Queue should be not be cleared
/// - `Song` clicked should be added to the back of the queue
/// - No `Play/Pause` signal is sent
/// - A toast should pop up showing we added the song to the queue
macro_rules! add_song {
	($self:ident, $song_title:expr, $key:expr) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::AddQueueSong(($key, shukusai::kernel::Append::Back, false))
		);
		$crate::toast!($self, format!("Added [{}] to queue", $song_title));
	}
}


#[macro_export]
/// Send an `Album` to `Kernel` to play.
///
/// This indicates:
/// - Queue should be cleared
/// - The first `Song` in the `Album` should be immediate played
/// - All the `Song`'s in the `Album` should be added to the queue
macro_rules! play_album {
	($self:ident, $key:expr) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::AddQueueAlbum(($key, shukusai::kernel::Append::Front, true, 0))
		);
		::benri::send!($self.to_kernel, shukusai::kernel::FrontendToKernel::Play);
	}
}

#[macro_export]
/// Send an `Album` to `Kernel` to play, skipping arbitrarily deep into it.
///
/// This implements the most used and expected
/// behavior when clicking a song in the `View` tab, or any
/// UI that shows an `Album` and it's respective `Songs`:
///
/// - Queue should be cleared
/// - The _clicked_ `Song` should be immediate played
/// - All the `Song`'s in the `Album` should be added to the queue
/// - Going backwards should be possible, even if the clicked `Song`
///   is not the first, e.g 5/12th track.
///
/// We must enumerate our lists, so we know the skip offset to send to `Kernel`.
macro_rules! play_album_offset {
	($self:ident, $key:expr, $offset:expr) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::AddQueueAlbum(($key, shukusai::kernel::Append::Front, true, $offset))
		);
		::benri::send!($self.to_kernel, shukusai::kernel::FrontendToKernel::Play);
	}
}

#[macro_export]
/// Play a specific index in the `Queue`.
///
/// This indicates:
/// - Queue should NOT be cleared
/// - The corresponding `Song` relating to the sent index should be immediate played
macro_rules! play_queue_index {
	($self:ident, $index:expr) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::SetQueueIndex($index)
		);
		::benri::send!($self.to_kernel, shukusai::kernel::FrontendToKernel::Play);
	}
}

#[macro_export]
/// Set a search string, set the last tab, jump to the search tab.
macro_rules! search {
	($self:ident, $key:expr, $shift:expr) => {
		let s = $crate::ui::update::KeyPress::from_egui_key(&$key).to_string();

		$self.state.search_string = match $shift {
			true  => s.to_uppercase(),
			false => s,
		};

		$self.search_jump = true;
		$crate::tab!($self, Tab::Search);
	}
}

#[macro_export]
/// Open's an `Album` directory in a file explorer.
macro_rules! open {
	($self:ident, $album:expr) => {
		match open::that(&$album.path) {
			Ok(_) => {
				log::info!("GUI - Opening path: {}", $album.path.display());
				$crate::toast!($self, format!("Opening [{}]'s directory", $album.title));
			},
			Err(e) => {
				log::warn!("GUI - Could not open path: {e}");
				$crate::toast_err!($self, format!("Opening [{}]'s directory", $album.title));
			},
		}
	}
}

#[macro_export]
/// Clear the queue and stop playback.
macro_rules! clear_stop {
	($self:ident) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::Clear(false),
		);
	}
}

#[macro_export]
/// Add a clickable `Album` art button that:
/// - Primary click: sets it to view
/// - Secondary click: adds it to the queue
/// - Middle click: opens its directory in a file explorer
macro_rules! album_button {
	($self:ident, $album:expr, $key:expr, $ui:ident, $ctx:ident, $size:expr, $text:expr) => {
		// ImageButton.
		let img_button = egui::ImageButton::new($album.texture_id($ctx), egui::vec2($size, $size));

		// Should be compiled out.
		let resp = if $text.is_empty() {
			$ui.add(img_button)
		} else {
			$ui.add(img_button).on_hover_text($text)
		};

		if resp.clicked() {
			$crate::album!($self, $key);
		} else if resp.secondary_clicked() {
			::benri::send!(
				$self.to_kernel,
				shukusai::kernel::FrontendToKernel::AddQueueAlbum(($key, shukusai::kernel::Append::Back, false, 0))
			);
			::benri::send!($self.to_kernel, shukusai::kernel::FrontendToKernel::Play);
			$crate::toast!($self, format!("Added [{}] to queue", $album.title));
		} else if resp.middle_clicked() {
			$crate::open!($self, $album);
		}
	};
}

#[macro_export]
/// Reduces the default rounding settings for the scope's `ui`.
macro_rules! no_rounding {
	($ui:ident) => {
		{
			// Reduce rounding corners.
			let widgets = &mut $ui.visuals_mut().widgets;
			widgets.hovered.rounding  = egui::Rounding::none();
			widgets.inactive.rounding = egui::Rounding::none();
			widgets.active.rounding   = egui::Rounding::none();
			// Reduced padding.
			$ui.spacing_mut().button_padding.x -= 2.0;
		}
	}
}

#[macro_export]
/// Make a `egui_notify` toast.
macro_rules! toast {
	($self:ident, $str:expr) => {{
		$self.toasts.dismiss_all_toasts();
		$self.toasts.basic($str)
			.set_closable(true)
			.set_duration(Some(std::time::Duration::from_secs(5)));
	}}
}

#[macro_export]
/// Make a `success` `egui_notify` toast.
macro_rules! toast_ok {
	($self:ident, $str:expr) => {{
		$self.toasts.dismiss_all_toasts();
		$self.toasts.success($str)
			.set_closable(true)
			.set_duration(Some(std::time::Duration::from_secs(5)));
	}}
}

#[macro_export]
/// Make a `error` `egui_notify` toast.
macro_rules! toast_err {
	($self:ident, $str:expr) => {{
		$self.toasts.dismiss_all_toasts();
		$self.toasts.error($str)
			.set_closable(true)
			.set_duration(Some(std::time::Duration::from_secs(5)));
	}}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
