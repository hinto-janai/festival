// Some macros for `ui` and some that need `self` and egui's `ui`.
//
// These UI layouts appear in many places, thus reusable macros are here.
//
// These are macros instead of functions because
// `self/ui` mutable borrow rules prevent it.
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
			shukusai::kernel::FrontendToKernel::AddQueueSong(($key, shukusai::audio::Append::Front, true))
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
/// - `Play` signal is sent if queue is empty (and empty_autoplay is true)
/// - A toast should pop up showing we added the song to the queue
macro_rules! add_song {
	($self:ident, $song_title:expr, $key:expr) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::AddQueueSong(($key, shukusai::audio::Append::Back, false))
		);
		if $self.settings.empty_autoplay && $self.audio_state.queue.is_empty() {
			::benri::send!($self.to_kernel, shukusai::kernel::FrontendToKernel::Play);
		}
		$crate::toast!($self, format!("Added [{}] to queue", $song_title));
	}
}

#[macro_export]
/// Append an `Album` to the end of the queue.
macro_rules! add_album {
	($self:ident, $album_title:expr, $key:expr) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::AddQueueAlbum(($key, shukusai::audio::Append::Back, false, 0))
		);
		if $self.settings.empty_autoplay && $self.audio_state.queue.is_empty() {
			::benri::send!($self.to_kernel, shukusai::kernel::FrontendToKernel::Play);
		}
		$crate::toast!($self, format!("Added [{}] to queue", $album_title));
	}
}

#[macro_export]
/// Append all the `Album`'s of this `Artist` to the end of the queue.
///
/// This indicates:
/// - Queue should be not be cleared
/// - `Play` signal is sent if queue is empty (and empty_autoplay is true)
/// - A toast should pop up showing we added the `Artist` to the queue
macro_rules! add_artist {
	($self:ident, $artist:expr, $key:expr) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::AddQueueArtist(($key, shukusai::audio::Append::Back, false, 0))
		);
		if $self.settings.empty_autoplay && $self.audio_state.queue.is_empty() {
			::benri::send!($self.to_kernel, shukusai::kernel::FrontendToKernel::Play);
		}
		$crate::toast!($self, format!("Added [{}] to queue", $artist.name));
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
			shukusai::kernel::FrontendToKernel::AddQueueAlbum(($key, shukusai::audio::Append::Front, true, 0))
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
			shukusai::kernel::FrontendToKernel::AddQueueAlbum(($key, shukusai::audio::Append::Front, true, $offset))
		);
		::benri::send!($self.to_kernel, shukusai::kernel::FrontendToKernel::Play);
	}
}

#[macro_export]
/// Send an `Artist` to `Kernel` to play, skipping arbitrarily deep into it.
///
/// - Queue should be cleared
/// - The _clicked_ `Song` should be immediate played
/// - All the `Album` by the `Artist` should be added to the queue
/// - Going backwards should be possible, even if the clicked `Song`
///   is not the first, e.g 5/12th track.
macro_rules! play_artist_offset {
	($self:ident, $key:expr, $offset:expr) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::AddQueueArtist(($key, shukusai::audio::Append::Front, true, $offset))
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
/// Remove an index range from the `Queue`.
///
/// This indicates:
/// - If our current `Song` gets removed, skip to the next available one
macro_rules! remove_queue_range {
	($self:ident, $range:expr) => {
		::benri::send!(
			$self.to_kernel,
			shukusai::kernel::FrontendToKernel::RemoveQueueRange(($range, true))
		);
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
/// Adds a clickable `Song` label button that:
///
/// - Lists `track`, `runtime`, `title`
/// - Primary click: play the song/album/artist
/// - Secondary click: adds it to the queue
/// - Middle click: opens its directory in a file explorer
///
/// HACK:
/// This also takes in a optional `$artist`.
/// This dictates whether we want to add a whole `Album` or `Artist`
/// `Some(ArtistKey)` == `Artist`
/// `None`            == `Album`
///
/// This is for the `Artist/View` tab, where users would probably
/// expect all songs by that artist to be added when clicking a song.
///
/// HACK HACK:
/// This also takes in input for the `$queue` tab which
/// has some special needs (wider width, head, etc).
///
/// This macro is terrible.
/// Maintaining this 6 months in the future is going to be very painful.
macro_rules! song_button {
	($self:ident, $same:expr, $album:expr, $song:expr, $key:expr, $ui:ident, $offset:expr, $artist:expr, $queue_index:expr, $y_add:expr, $x_add:expr) => {
		let mut rect = $ui.cursor();
		rect.max.y = rect.min.y + $y_add;

		if $x_add != 0.0 {
			rect.max.x = rect.min.x + $x_add;
		}

		let resp = $ui.put(rect, egui::SelectableLabel::new($same, ""));

		let primary   = resp.clicked();
		let middle    = resp.middle_clicked();
		let secondary = resp.secondary_clicked();

		if middle || (primary && $self.modifiers.command) {
			$crate::open!($self, $album);
		} else if primary {
			if let Some(queue_index) = $queue_index {
				crate::play_queue_index!($self, queue_index);
			} else if let Some(artist_key) = $artist {
				$crate::play_artist_offset!($self, artist_key, $offset);
			} else {
				$crate::play_album_offset!($self, $song.album, $offset);
			}
		} else if secondary {
			$crate::add_song!($self, $song.title, $key);
		}

		// FIXME:
		// This is a little too eager to chop and doesn't
		// scale as the width gets longer, aka, we have enough
		// space since we expanded the GUI horizontally, but
		// this doesn't match it right.
		//
		// Chop song title with `Head`.
		let width = rect.max.x - rect.min.x;
		// HACK:
		// Even though all fonts are monospace, non-ASCII characters,
		// especially Chinese characters are really wide in width,
		// so the character leeway depends on this.
		//
		// HACK HACK:
		// Queue tab has wider song buttons, so
		// we must vary the width for that.
		let ascii = $song.title.is_ascii();
		let head_len = match $self.state.tab == crate::data::Tab::Queue {
			true  => if ascii { 17.0 } else { 28.0 },
			false => if ascii { 19.0 } else { 32.0 },
		};
		let head_len = (width / head_len) as usize;

		let head    = readable::HeadTail::head_dot(&$song.title, head_len);
		// If we chopped the title, show the full title on hover.
		let chopped = $song.title == head;

		let resp = $ui.allocate_ui_at_rect(rect, |ui| {
			ui.horizontal_centered(|ui| {
				match (chopped, $song.track) {
					(true,  Some(t)) => ui.add(egui::Label::new(format!("{: >3}{: >8}    {}", t, $song.runtime.as_str(), head))),
					(false, Some(t)) => ui.add(egui::Label::new(format!("{: >3}{: >8}    {}", t, $song.runtime.as_str(), head))).on_hover_text(&$song.title),
					(true,  None)    => ui.add(egui::Label::new(format!("{: >3}{: >8}    {}", "???", $song.runtime.as_str(), head))),
					(false, None)    => ui.add(egui::Label::new(format!("{: >3}{: >8}    {}", "???", $song.runtime.as_str(), head))).on_hover_text(&$song.title),
				}
			});
		});
	}
}

#[macro_export]
/// Adds a clickable `Album` art button that:
///
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

		let primary   = resp.clicked();
		let middle    = resp.middle_clicked();
		let secondary = resp.secondary_clicked();

		if middle || (primary && $self.modifiers.command) {
			$crate::open!($self, $album);
		} else if primary {
			$crate::album!($self, $key);
		} else if secondary {
			$crate::add_album!($self, $album.title, $key);
		}
	};
}

#[macro_export]
/// Same as `song_button!()` but a label.
macro_rules! song_label {
	($self:ident, $song:expr, $album:expr, $key:expr, $ui:ident, $label:expr) => {
		let resp = $ui.add($label.sense(Sense::click()));

		let primary   = resp.clicked();
		let middle    = resp.middle_clicked();
		let secondary = resp.secondary_clicked();

		if middle || (primary && $self.modifiers.command) {
			$crate::open!($self, $album);
		} else if primary {
			$crate::play_song!($self, $key);
		} else if secondary {
			$crate::add_song!($self, $song.title, $key);
		}
	}
}

#[macro_export]
/// Same as `album_button!()` but a label.
macro_rules! album_label {
	($self:ident, $album:expr, $key:expr, $ui:ident, $label:expr) => {
		let resp = $ui.add($label.sense(Sense::click()));

		let primary   = resp.clicked();
		let middle    = resp.middle_clicked();
		let secondary = resp.secondary_clicked();

		if middle || (primary && $self.modifiers.command) {
			$crate::open!($self, $album);
		} else if primary {
			$crate::album!($self, $key);
		} else if secondary {
			$crate::add_album!($self, $album.title, $key);
		}
	}
}

#[macro_export]
/// Add a clickable `Artist` label that:
/// - Primary click: sets it to `Artists` tab view
/// - Secondary click: adds all `Album`'s by that `Artist` to the queue
macro_rules! artist_label {
	($self:ident, $artist:expr, $key:expr, $ui:ident, $label:expr) => {
		let resp = $ui.add($label.sense(Sense::click()));

		if resp.clicked() {
			$crate::artist!($self, $key);
		} else if resp.secondary_clicked() {
			$crate::add_artist!($self, $artist, $key);
		}
	}
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
			.set_duration(Some(std::time::Duration::from_secs(10)));
	}}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
