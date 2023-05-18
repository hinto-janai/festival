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
macro_rules! tab {
	($self:ident, $tab:expr) => {
		$self.last_tab = Some($self.state.tab);
		$self.state.tab = $tab;
	}
}

#[macro_export]
/// Adds a clickable `Album` art button that opens the parent directory.
macro_rules! album_button {
	($self:ident, $album:ident, $key:ident, $ui:ident, $ctx:ident, $size:tt) => {
		// ImageButton.
		let img_button = ImageButton::new($album.texture_id($ctx), egui::vec2($size, $size));

		let resp = $ui.add(img_button);

		if resp.clicked() {
			$self.state.album = Some($key.into());
			$crate::tab!($self, crate::data::Tab::View);
		} else if resp.secondary_clicked() {
			match open::that(&$album.path) {
				Ok(_) => log::info!("GUI - Opening path: {}", $album.path.display()),
				Err(e) => log::warn!("GUI - Could not open path: {e}"),
			}
		}
	}
}
pub(crate) use album_button;

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
