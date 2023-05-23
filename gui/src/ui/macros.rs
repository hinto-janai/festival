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
/// Add a clickable `Album` art button that opens the parent directory.
macro_rules! album_button {
	($self:ident, $album:ident, $key:ident, $ui:ident, $ctx:ident, $size:expr) => {
		// ImageButton.
		let img_button = egui::ImageButton::new($album.texture_id($ctx), egui::vec2($size, $size));

		let resp = $ui.add(img_button);

		if resp.clicked() {
			$crate::album!($self, $key);
		} else if resp.secondary_clicked() {
			match open::that(&$album.path) {
				Ok(_) => log::info!("GUI - Opening path: {}", $album.path.display()),
				Err(e) => log::warn!("GUI - Could not open path: {e}"),
			}
		}
	};

	// Same as above, adds optional text.
	($self:ident, $album:ident, $key:ident, $ui:ident, $ctx:ident, $size:tt, $text:expr) => {
		// ImageButton.
		let img_button = egui::ImageButton::new($album.texture_id($ctx), egui::vec2($size, $size));

		let resp = $ui.add(img_button).on_hover_text(&$text);

		if resp.clicked() {
			$crate::album!($self, $key);
		} else if resp.secondary_clicked() {
			match open::that(&$album.path) {
				Ok(_) => log::info!("GUI - Opening path: {}", $album.path.display()),
				Err(e) => log::warn!("GUI - Could not open path: {e}"),
			}
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
