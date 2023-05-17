// Some macors for `ui` and some that need `self` and egui's `ui`.
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
/// Adds a clickable `Album` art button that opens the parent directory.
macro_rules! album_button {
	($self:ident, $album:ident, $key:ident, $ui:ident, $ctx:ident, $size:tt) => {
		// ImageButton.
		let img_button = ImageButton::new($album.texture_id($ctx), egui::vec2($size, $size));

		let resp = $ui.add(img_button);

		if resp.clicked() {
			$self.state.album = Some($key.clone());
		} else if resp.secondary_clicked() {
			// INVARIANT:
			// We're opening the parent directory
			// of the 1st song in this album by
			// directly indexing into it.
			//
			// The album _must_ have at least 1 song.
			let song = &$self.collection.songs[$album.songs[0]];

			match &song.path.parent() {
				Some(p) => {
					if let Err(e) = open::that(p) {
						warn!("GUI - Could not open path: {e}");
					}
				}
				None => warn!("GUI - Could not get parent path: {}", song.path.display()),
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
