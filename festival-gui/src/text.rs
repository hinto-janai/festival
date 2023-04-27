//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{
	ALBUMS_PER_ROW_MIN,
	ALBUMS_PER_ROW_MAX,
	ALBUM_ART_SIZE_MIN,
	ALBUM_ART_SIZE_MAX,
	SEARCH_MAX_LEN,
};
use const_format::formatcp;

//---------------------------------------------------------------------------------------------------- Static text, mostly for `.on_hover_text()`
//---------------------------------------------------------------------------------------------------- Left Tab
pub const INCREMENT_ALBUM_SIZE: &str = "Increase the album art size";
pub const DECREMENT_ALBUM_SIZE: &str = "Decrease the album art size";

pub const VOLUME_SLIDER: &str = "Increase/decrease audio volume";

//---------------------------------------------------------------------------------------------------- Bottom Bar
pub const SAVING: &str = "Festival is still saving a recently created Collection";

//---------------------------------------------------------------------------------------------------- Settings Tab
pub const RESET:             &str = "Reset changes";
pub const SAVE:              &str = "Save changes to disk";
pub const ALBUM_SORT_ORDER:  &str = "Which method to sort the albums by";
pub const ALBUM_ART_SIZE:    &str = "How big the album art cover should be in the [Albums] tab";
pub const STATIC_PIXEL_SIZE: &str = formatcp!(
	"Always show album art at a static pixel size regardless of the window size ({}-{})",
	ALBUM_ART_SIZE_MIN as usize,
	ALBUM_ART_SIZE_MAX as usize,
);
pub const ALBUM_PER_ROW:     &str = formatcp!("Show [x] amount of albums per row, scaling the pixel size to fit ({ALBUMS_PER_ROW_MIN}-{ALBUMS_PER_ROW_MAX})");
pub const RESTORE_STATE:     &str = "Restore playback state from the last session when opening Festival";
pub const ACCENT_COLOR:      &str = "Which accent color to use (RGB)";
pub const COLLECTION:        &str = "The main music Collection that stores all (meta)data about the audio files";
pub const ADD_FOLDER:        &str = "Add a maximum of 10 folders";
pub const REMOVE_FOLDER:     &str = "Remove this folder";
pub const RESET_COLLECTION:  &str = "Scan the folders listed and create a new Collection";
pub const STATS:             &str = "Stats about your current Collection";

//---------------------------------------------------------------------------------------------------- Search Tab
// This is inaccurate because `char` != `u8` but meh.
pub const SEARCH_MAX: &str = formatcp!("Search character limit has been reached ({SEARCH_MAX_LEN})");

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
