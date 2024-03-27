//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{
    ALBUMS_PER_ROW_MAX, ALBUMS_PER_ROW_MIN, ALBUM_ART_SIZE_MAX, ALBUM_ART_SIZE_MIN,
    PIXELS_PER_POINT_MAX, PIXELS_PER_POINT_MIN, PIXELS_PER_POINT_UNIT, SETTINGS_VERSION,
    STATE_VERSION,
};
use crate::data::Gui;
use crate::data::{AlbumSizing, StateRestore};
use benri::{log::*, sync::*};
use disk::{Bincode2, Json};
use log::{error, info, warn};
use shukusai::{
    audio::Volume,
    collection::{Collection, Keychain},
    constants::PLAYLIST_VERSION,
    kernel::FrontendToKernel,
    state::{AUDIO_STATE, PLAYLISTS},
};
use std::sync::Arc;

//---------------------------------------------------------------------------------------------------- GUI `Drop` impl
// On Windows/Linux, ALT+F4 will trigger `eframe::App::on_close_event()` and cause `Gui`
// to run its exiting routine, however on macOS, the equivalent Control+Q does not trigger this.
//
// Destructors are run however, so this impl is specifically for macOS when Control+Q'ing.
//
// A `Collection` that is being saved to disk is not
// respected here, although at least `GUI` state is saved.
//#[cfg(target_os = "macos")]
impl Drop for Gui {
    fn drop(&mut self) {
        eframe::App::on_close_event(self);
    }
}

//---------------------------------------------------------------------------------------------------- GUI convenience functions.
impl Gui {
    // Sets a new `Collection`, all the related flags, and does validation.
    pub fn new_collection(&mut self, c: Arc<Collection>) {
        info!("GUI ... New Collection received");
        self.collection = c;
        self.resetting_collection = false;
        self.kernel_returned = true;
        self.cache_collection();

        // Validation.
        use shukusai::validate;

        if validate::keychain(&self.collection, &self.state.search_result)
            && validate::album(&self.collection, self.state.album.unwrap_or_default())
            && validate::artist(&self.collection, self.state.artist.unwrap_or_default())
            && validate::keychain(&self.collection, &self.og_state.search_result)
            && validate::album(&self.collection, self.og_state.album.unwrap_or_default())
            && validate::artist(&self.collection, self.og_state.artist.unwrap_or_default())
        {
            ok!("GUI - State{STATE_VERSION} validation");
        } else {
            fail!("GUI - State{STATE_VERSION} validation");
            self.state = crate::data::State::new();
        }

        // Update playlist.
        self.og_playlists = PLAYLISTS.read().clone();

        // Update audio state.
        self.audio_state = AUDIO_STATE.read().clone();

        // Update state.
        //
        // INVARIANT: this must be done to ensure our
        // state keys are not referencing invalid keys.
        self.state_restore
            .update_state(&mut self.state, &self.collection);
    }

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
        match self.settings.save_atomic() {
            Ok(md) => {
                ok_debug!("GUI - Settings{SETTINGS_VERSION} save: {md}");
                Ok(())
            }
            Err(e) => {
                error!("GUI - Settings{SETTINGS_VERSION} could not be saved to disk: {e}");
                Err(e)
            }
        }
    }

    /// Set the current [`State`] to disk.
    pub fn save_state(&mut self) {
        self.set_state();
        match self.state.save_atomic() {
            Ok(md) => ok_debug!("GUI - State{STATE_VERSION} save: {md}"),
            Err(e) => error!("GUI - State{STATE_VERSION} could not be saved to disk: {e}"),
        }
    }

    /// Set the current Playlists to disk.
    pub fn save_playlists(&mut self) -> Result<(), anyhow::Error> {
        self.set_playlists();
        match self.og_playlists.save_atomic() {
            Ok(md) => {
                ok_debug!("GUI - Playlists{PLAYLIST_VERSION} save: {md}");
                Ok(())
            }
            Err(e) => {
                error!("GUI - Playlists{PLAYLIST_VERSION} could not be saved to disk: {e}");
                Err(e)
            }
        }
    }

    /// Set the original [`Settings`] to reflect live [`Settings`].
    pub fn set_settings(&mut self) {
        self.og_settings = self.settings.clone();
    }

    /// Reset [`Settings`] to the original.
    pub fn reset_settings(&mut self) {
        self.settings = self.og_settings.clone();
    }

    /// Reset all [`Settings`] to the default.
    pub fn default_settings(&mut self) {
        self.settings = Default::default();
        self.og_settings = Default::default();
    }

    /// Set the original [`State`] to reflect live [`State`].
    pub fn set_state(&mut self) {
        self.og_state = self.state.clone();
    }

    /// Reset [`State`] to the original.
    pub fn reset_state(&mut self) {
        self.state = self.og_state.clone();
    }

    /// Reset all [`State`] to the default.
    pub fn default_state(&mut self) {
        self.state = Default::default();
        self.og_state = Default::default();
    }

    /// Set the original [`Playlists`] to reflect live [`Playlists`].
    pub fn set_playlists(&mut self) {
        self.og_playlists = PLAYLISTS.read().clone();
    }

    /// Reset [`Playlists`] to the original.
    pub fn reset_playlists(&mut self) {
        *PLAYLISTS.write() = self.og_playlists.clone();
    }

    /// Reset all [`Playlists`] to the default.
    pub fn default_playlists(&mut self) {
        *PLAYLISTS.write() = Default::default();
        self.og_playlists = Default::default();
    }

    /// Returns true if either [`Settings`] or [`State`] or Playlists have diffs.
    pub fn diff(&self) -> bool {
        self.diff_settings() || self.diff_state() || self.diff_playlists()
    }

    /// Returns true if [`Settings`] and the old version are not `==`.
    pub fn diff_settings(&self) -> bool {
        self.settings != self.og_settings
    }

    /// Returns true if [`State`] and the old version are not `==`.
    pub fn diff_state(&self) -> bool {
        self.state != self.og_state
    }

    /// Returns true if [`shukusai::state::PLAYLISTS`] and the old version are not `==`.
    pub fn diff_playlists(&self) -> bool {
        self.og_playlists != *shukusai::state::PLAYLISTS.read()
    }

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

    /// Perform all the necessary steps to add a folder
    /// to add to the Collection (spawns RFD thread).
    pub fn add_folder(&self) {
        if atomic_load!(self.rfd_open) {
            warn!("GUI - Add folder requested, but RFD is already open");
        } else {
            crate::func::spawn_rfd_thread(Arc::clone(&self.rfd_open), Arc::clone(&self.rfd_new));
        }
    }

    /// Perform all the necessary steps to reset
    /// the [`Collection`] and enter the proper state.
    pub fn reset_collection(&mut self) {
        info!("GUI - Resetting Collection");

        // INVARIANT:
        // We must ensure our state does not reference invalid keys
        // after the `Collection` reset or else index panic.
        //
        // This call must be combined when an eventual `into_state`
        // when we receive the new `Collection`.
        self.state_restore = StateRestore::from_state(&mut self.state, &self.collection);

        // Drop our real `Collection`.
        self.collection = Collection::dummy();

        // Send signal to `Kernel`.
        if self.settings.collection_paths.is_empty() {
            match dirs::audio_dir() {
                Some(p) => {
                    info!(
                        "GUI - Collection reset requested but no PATHs, adding: {}",
                        p.display()
                    );
                    self.settings.collection_paths.push(p);
                }
                None => {
                    warn!("GUI - Collection reset requested but no PATHs and could not find user's audio PATH");
                }
            }
        }
        send!(
            self.to_kernel,
            FrontendToKernel::NewCollection(self.settings.collection_paths.clone())
        );

        // Go into collection mode.
        self.resetting_collection = true;
    }

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
            }
            AlbumSizing::Row => {
                let new = self.settings.albums_per_row + 1;
                if new > ALBUMS_PER_ROW_MAX {
                    self.settings.albums_per_row = ALBUMS_PER_ROW_MAX;
                } else {
                    self.settings.albums_per_row = new;
                }
            }
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
            }
            AlbumSizing::Row => {
                let new = self.settings.albums_per_row - 1;
                if new >= ALBUMS_PER_ROW_MIN {
                    self.settings.albums_per_row = new;
                }
            }
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
        self.count_album = format!("Albums: {}", self.collection.count_album);
        self.count_song = format!("Songs: {}", self.collection.count_song);
        self.count_art = format!("Art: {}", self.collection.count_art);
    }

    /// Add and set `Volume`.
    pub fn add_volume(&mut self, v: u8) {
        let v = self.state.volume + v;

        if v > 100 {
            self.state.volume = 100;
        } else {
            self.state.volume = v;
        }

        atomic_store!(shukusai::state::VOLUME, self.state.volume);
    }

    /// Subtract and set `Volume`.
    pub fn sub_volume(&mut self, v: u8) {
        self.state.volume = self.state.volume.saturating_sub(v);

        atomic_store!(shukusai::state::VOLUME, self.state.volume);
    }

    /// Add and set `pixels_per_point`
    ///
    /// Returns the new `Some` if we added, else `None` if at max.
    pub fn increment_pixels_per_point(&mut self) -> Option<f32> {
        let new = self.settings.pixels_per_point + PIXELS_PER_POINT_UNIT;
        let new = if new > PIXELS_PER_POINT_MAX {
            PIXELS_PER_POINT_MAX
        } else {
            new
        };

        if new <= PIXELS_PER_POINT_MAX {
            self.settings.pixels_per_point = new;
            Some(self.settings.pixels_per_point)
        } else {
            None
        }
    }

    /// Subtract and set `pixels_per_point`
    ///
    /// Returns the new `Some` if we subtracted, else `None` if at min.
    pub fn decrement_pixels_per_point(&mut self) -> Option<f32> {
        let new = self.settings.pixels_per_point - PIXELS_PER_POINT_UNIT;
        let new = if new < PIXELS_PER_POINT_MIN {
            PIXELS_PER_POINT_MIN
        } else {
            new
        };

        if new >= PIXELS_PER_POINT_MIN {
            self.settings.pixels_per_point = new;
            Some(self.settings.pixels_per_point)
        } else {
            None
        }
    }
}
