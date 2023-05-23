//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{
	ALBUM_ART_SIZE_MAX,
	ALBUM_ART_SIZE_MIN,
	ALBUMS_PER_ROW_MAX,
	ALBUMS_PER_ROW_MIN,
};
use crate::data::{
	AlbumSizing,
};
use shukusai::kernel::{
	FrontendToKernel,
	KernelToFrontend,
};
use shukusai::collection::{
	Collection,
};
use shukusai::sort::{
	ArtistSort,AlbumSort,SongSort,
};
use benri::{
	debug_panic,
	log::*,
	sync::*,
};
use log::{
	info,
	warn,
	error
};
use std::sync::{
	Arc,
	atomic::AtomicBool,
	atomic::AtomicU8,
};
use disk::{Bincode2,Toml,Json};

//---------------------------------------------------------------------------------------------------- GUI convenience functions.
impl crate::data::Gui {
	// Sets a new `Collection`, all the related flags, and does validation.
	pub fn new_collection(&mut self, c: Arc<Collection>) {
		ok_debug!("GUI - New Collection");
		self.collection           = c;
		self.resetting_collection = false;
		self.kernel_returned      = true;
		self.cache_collection();

		// Validation.
		use shukusai::validate;

		if validate::keychain(&self.collection, &self.state.search_result) &&
			validate::album(&self.collection, self.state.album.unwrap_or_default()) &&
			validate::artist(&self.collection, self.state.artist.unwrap_or_default()) &&
			validate::keychain(&self.collection, &self.og_state.search_result) &&
			validate::album(&self.collection, self.og_state.album.unwrap_or_default()) &&
			validate::artist(&self.collection, self.og_state.artist.unwrap_or_default())
		{
			ok!("GUI - State validation");
		} else {
			fail!("GUI - State validation");
			self.state = crate::data::State::default();
		}
	}

	#[inline(always)]
	// Sets the [`egui::Ui`]'s `Visual` from our current `Settings`
	//
	// This should be called at the beginning of every major `Ui` frame.
	pub fn set_visuals(&mut self, ui: &mut egui::Ui) {
		// Accent color.
		let mut visuals = ui.visuals_mut();
		visuals.selection.bg_fill = self.settings.accent_color;
	}

	/// Set the current [`Settings`] to disk.
	pub fn save_settings(&mut self) -> Result<(), anyhow::Error> {
		self.set_settings();
		// TODO: handle save error.
		match self.settings.save_atomic() {
			Ok(md) => { ok_debug!("GUI - Settings save: {md}"); Ok(()) },
			Err(e) => { error!("GUI - Settings could not be saved to disk: {e}"); Err(e) },
		}
	}

	/// Set the current [`State`] to disk.
	pub fn save_state(&mut self) {
		self.set_state();
		// TODO: handle save error.
		match self.state.save_atomic() {
			Ok(md) => ok_debug!("GUI - State save: {md}"),
			Err(e) => error!("GUI - State could not be saved to disk: {e}"),
		}
	}

	#[inline(always)]
	/// Set the original [`Settings`] to reflect live [`Settings`].
	pub fn set_settings(&mut self) {
		self.og_settings = self.settings.clone();
	}

	#[inline(always)]
	/// Reset [`Settings`] to the original.
	///
	/// This also resets the [`egui::Visuals`].
	pub fn reset_settings(&mut self) {
		self.settings = self.og_settings.clone();
	}

	#[inline(always)]
	/// Set the original [`State`] to reflect live [`State`].
	pub fn set_state(&mut self) {
		self.og_state = self.state.clone();
	}

	#[inline(always)]
	/// Reset [`State`] to the original.
	pub fn reset_state(&mut self) {
		self.state = self.og_state.clone();
	}

	#[inline]
	/// Returns true if either [`Settings`] or [`State`] have diffs.
	pub fn diff(&self) -> bool {
		self.diff_settings() && self.diff_state()
	}

	#[inline(always)]
	/// Returns true if [`Settings`] and the old version are not `==`.
	pub fn diff_settings(&self) -> bool {
		self.settings != self.og_settings
	}

	#[inline(always)]
	/// Returns true if [`State`] and the old version are not `==`.
	pub fn diff_state(&self) -> bool {
		self.state != self.og_state
	}

//	#[inline(always)]
//	/// Copies the _audio_ values from [`KernelState`] into `self`.
//	pub fn copy_kernel_audio(&mut self) {
//		let k = lockr!(self.kernel_state);
//
//		// PERF:
//		// Comparison seems to be slower than un-conditional
//		// assignment for small `copy`-able structs like `AudioState`,
//		// so don't even check for diffs, just always copy.
//		self.audio = k.audio;
//	}

	#[inline(always)]
	/// Perform all the necessary steps to add a folder
	/// to add to the Collection (spawns RFD thread).
	pub fn add_folder(&self) {
		if atomic_load!(self.rfd_open) {
			warn!("GUI - Add folder requested, but RFD is already open");
		} else {
			crate::func::spawn_rfd_thread(
				Arc::clone(&self.rfd_open),
				Arc::clone(&self.rfd_new),
			);
		}
	}

	#[inline(always)]
	/// Perform all the necessary steps to reset
	/// the [`Collection`] and enter the proper state.
	pub fn reset_collection(&mut self) {
		// Drop our real `Collection`.
		self.collection = Collection::dummy();

		// Send signal to `Kernel`.
		if self.settings.collection_paths.is_empty() {
			match dirs::audio_dir() {
				Some(p) => {
					info!("GUI - Collection reset requested but no PATHs, adding: {}", p.display());
					self.settings.collection_paths.push(p);
				},
				None => {
					warn!("GUI - Collection reset requested but no PATHs and could not find user's audio PATH");
				}
			}
		}
		send!(self.to_kernel, FrontendToKernel::NewCollection(self.settings.collection_paths.clone()));

		// Go into collection mode.
		self.resetting_collection = true;

	}

	#[inline(always)]
	/// Caches some segments of [`Collection`] for local use
	/// so don't have to access it all the time.
	///
	/// This should be called after we received a new [`Collection`].
	pub fn cache_collection(&mut self) {
		self.format_count_assign();
	}

	/// Increments the [`Album`] art size.
	///
	/// - If `AlbumSizing::Pixel`, increment by `1.0`
	/// - If `AlbumSizing::Row`, increment by `1`
	///
	/// If over the max, this function does nothing.
	///
	/// If close to the max, this sets `self` to the max.
	pub fn increment_art_size(&mut self) {
		match self.settings.album_sizing {
			AlbumSizing::Pixel => {
				let new = self.settings.album_pixel_size + 1.0;
				if new > ALBUM_ART_SIZE_MAX {
					self.settings.album_pixel_size = ALBUM_ART_SIZE_MAX;
				} else {
					self.settings.album_pixel_size = new;
				}
			},
			AlbumSizing::Row => {
				let new = self.settings.albums_per_row + 1;
				if new > ALBUMS_PER_ROW_MAX {
					self.settings.albums_per_row = ALBUMS_PER_ROW_MAX;
				} else {
					self.settings.albums_per_row = new;
				}
			},
		}
	}

	/// Decrements the [`Album`] art size.
	///
	/// - If `AlbumSizing::Pixel`, decrement by `1.0`
	/// - If `AlbumSizing::Row`, decrement by `1`
	///
	/// If at the minimum, this function does nothing.
	pub fn decrement_art_size(&mut self) {
		match self.settings.album_sizing {
			AlbumSizing::Pixel => {
				let new = self.settings.album_pixel_size - 1.0;
				if new < ALBUM_ART_SIZE_MIN {
					self.settings.album_pixel_size = ALBUM_ART_SIZE_MIN;
				} else {
					self.settings.album_pixel_size = new;
				}
			},
			AlbumSizing::Row => {
				let new = self.settings.albums_per_row - 1;
				if new >= ALBUMS_PER_ROW_MIN {
					self.settings.albums_per_row = new;
				}
			},
		}
	}

	/// Returns true if the current setting `<=` the minimum size.
	pub fn album_size_is_min(&self) -> bool {
		match self.settings.album_sizing {
			AlbumSizing::Pixel => self.settings.album_pixel_size <= ALBUM_ART_SIZE_MIN,
			AlbumSizing::Row => self.settings.albums_per_row <= ALBUMS_PER_ROW_MIN,
		}
	}

	/// Returns true if the current setting `>=` the maximum size.
	pub fn album_size_is_max(&self) -> bool {
		match self.settings.album_sizing {
			AlbumSizing::Pixel => self.settings.album_pixel_size >= ALBUM_ART_SIZE_MAX,
			AlbumSizing::Row => self.settings.albums_per_row >= ALBUMS_PER_ROW_MAX,
		}
	}

	/// Copies the data from our current [`Collection`],
	/// formats it, and assigns it to [`Self`]'s `count_*` fields.
	pub fn format_count_assign(&mut self) {
		self.count_artist = format!("Artists: {}", self.collection.count_artist);
		self.count_album  = format!("Albums: {}", self.collection.count_album);
		self.count_song   = format!("Songs: {}", self.collection.count_song);
	}
}
