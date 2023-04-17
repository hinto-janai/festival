//---------------------------------------------------------------------------------------------------- Use
use crate::{
	constants::*,
};
use crate::data::{
	State,
	Settings,
};
use crate::ui::{
	Tab,
};
use shukusai::kernel::{
	Kernel,
	KernelState,
	ResetState,
	FrontendToKernel,
	KernelToFrontend,
};
use shukusai::collection::{
	Collection,
};
use shukusai::key::{
	AlbumKey,
	Keychain,
};
use shukusai::sort::{
};
use benri::{
	now,
	log::*,
	panic::*,
	sync::*,
};
use log::{
	info,
	warn,
	error
};
use egui::{
	Style,Visuals,Color32,
	TopBottomPanel,SidePanel,CentralPanel,
	TextStyle,FontId,FontData,FontDefinitions,FontFamily,FontTweak,
};
use crossbeam_channel::{Sender,Receiver};
use std::path::PathBuf;
use std::sync::{
	Arc,
	Mutex,
	atomic::AtomicBool,
};
use rolock::RoLock;
use disk::Toml;
use disk::Bincode;
use std::time::Instant;
use super::AlbumSizing;

//---------------------------------------------------------------------------------------------------- GUI struct. This hold ALL data.
pub struct Gui {
	/// To `Kernel`.
	pub to_kernel: Sender<FrontendToKernel>,
	/// From `Kernel`.
	pub from_kernel: Receiver<KernelToFrontend>,

	/// The `Collection` and misc state.
	pub collection: Arc<Collection>,
	pub kernel_state: RoLock<KernelState>,
	pub reset_state: RoLock<ResetState>,

	/// `GUI` settings.
	pub settings: Settings,
	/// `GUI` settings (old).
	pub og_settings: Settings,

	/// `GUI` state.
	pub state: State,
	/// `GUI` settings (old).
	pub og_state: State,

	// RFD state.
	/// If a RFD window is currently open.
	pub rfd_open: Arc<AtomicBool>,
	/// If a file was selected with RFD.
	pub rfd_new: Arc<Mutex<Option<PathBuf>>>,
	/// A buffer of the indicies of the PATHs the user wants deleted.
	pub deleted_paths: Vec<usize>,

	// Search state.
	/// If we're currently searching.
	pub searching: bool,
	/// Our current search input.
	pub search_string: String,
	/// The search result [`Keychain`] we got from `Kernel`.
	pub search_result: Keychain,

	// Local cached variables.
	/// A cached, formatted version of [`Collection::count_artist`]
	pub count_artist: String,
	/// A cached, formatted version of [`Collection::count_album`]
	pub count_album: String,
	/// A cached, formatted version of [`Collection::count_song`]
	pub count_song: String,

	/// Are we currently in the process of exiting?
	pub exiting: bool,
	/// To prevent showing a flash of the spinner
	/// when exiting really quickly, this `Instant`
	/// needs to rack up some time before showing the spinner.
	pub exit_instant: Instant,

	/// Are we in the middle of resetting the [`Collection`]?
	pub resetting_collection: bool,
}

//---------------------------------------------------------------------------------------------------- GUI convenience functions.
impl Gui {
	#[inline(always)]
	// Sets the [`egui::Ui`]'s `Visual` from our current `Settings`
	//
	// This should be called at the beginning of every major `Ui` frame.
	pub fn set_visuals(&mut self, ui: &mut egui::Ui) {
		// Accent color.
		let mut visuals = ui.visuals_mut();
		visuals.selection.bg_fill = self.settings.accent_color;
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
		self.og_state = self.state; // `copy`-able
	}

	#[inline(always)]
	/// Reset [`State`] to the original.
	pub fn reset_state(&mut self) {
		self.state = self.og_state; // `copy`-able
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

	#[inline(always)]
	/// Copies the _audio_ values from [`KernelState`] into [`State`].
	pub fn copy_kernel_audio(&mut self) {
		let k = lock_read!(self.kernel_state);

		// PERF:
		// Comparison seems to be slower than un-conditional
		// assignment for small `copy`-able structs like `AudioState`,
		// so don't even check for diffs, just always copy.
		self.state.audio = k.audio;
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
				if new > ALBUM_ART_SIZE_MAX + 1.0 {
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
				if new < ALBUM_ART_SIZE_MIN - 1.0 {
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

	#[inline(always)]
	/// Copies the data from our current [`Collection`],
	/// formats it, and assigns it to [`Self`]'s `count_*` fields.
	fn format_count_assign(&mut self) {
		self.count_artist = format!("Artists: {}", self.collection.count_artist);
		self.count_album  = format!("Albums: {}", self.collection.count_album);
		self.count_song   = format!("Songs: {}", self.collection.count_song);
	}

	#[inline(always)]
	/// Sets the current [`State::album`] and switches the current tab to [`Tab::View`].
	pub fn set_album_tab_view(&mut self, album_key: AlbumKey) {
		self.state.album = Some(album_key);
		self.state.tab = Tab::View;
	}
}

//---------------------------------------------------------------------------------------------------- GUI Init.
// Instead of having [Gui::new()] be 1000s of lines long,
// these private functions will be separate stuff.
//
// See `Gui::init` at the bottom to see the function that "starts" the `GUI`.
impl Gui {
	#[inline(always)]
	fn init_style() -> egui::Style {
		let style = Style {
			text_styles: [
				(TextStyle::Small,     FontId::new(10.0, FontFamily::Monospace)),
				(TextStyle::Body,      FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Button,    FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Monospace, FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Heading,   FontId::new(40.0, FontFamily::Monospace)),
			].into(),
			spacing: SPACING.clone(),
			..Default::default()
		};

		ok_debug!("GUI Init - Style");
		style
	}

	#[inline(always)]
	fn init_visuals() -> egui::Visuals {
		ok_debug!("GUI Init - Visuals");
		VISUALS.clone()
	}

	#[inline(always)]
	fn init_fonts() -> egui::FontDefinitions {
		let mut fonts = FontDefinitions::default();
		// TODO:
		// Only 2 fonts for debugging.
		// Make sure all fonts are enabled at v1.0.0.
		fonts.font_data.insert("0".to_string(), FontData::from_static(FONT_SOURCECODE_PRO));
		fonts.font_data.insert("1".to_string(), FontData::from_static(FONT_SOURCECODE_JP).tweak(
			FontTweak {
				y_offset_factor: -0.38, // Move it up
				..Default::default()
			},
		));
//		fonts.font_data.insert("SourceCode-Pro".to_string(), FontData::from_static(FONT_SOURCECODE_PRO));
//		fonts.font_data.insert("SourceCode-CN".to_string(), FontData::from_static(FONT_SOURCECODE_CN));
//		fonts.font_data.insert("SourceCode-HK".to_string(), FontData::from_static(FONT_SOURCECODE_HK));
//		fonts.font_data.insert("SourceCode-TW".to_string(), FontData::from_static(FONT_SOURCECODE_TW));
//		fonts.font_data.insert("SourceCode-KR".to_string(), FontData::from_static(FONT_SOURCECODE_KR));
//		fonts.font_data.insert("SourceCode-JP".to_string(), FontData::from_static(FONT_SOURCECODE_JP));
//		fonts.font_data.insert("JuliaMono".to_string(), FontData::from_static(FONT_JULIAMONO));

		for i in 0..=1 {
			fonts.families.get_mut(&FontFamily::Monospace)
				.expect("Failed to get: egui::FontFamily::Monospace")
				.insert(i, i.to_string());
			fonts.families.get_mut(&FontFamily::Proportional)
				.expect("Failed to get: egui::FontFamily::Proportional")
				.push(i.to_string());
		}

		ok_debug!("GUI Init - Fonts");
		fonts
	}
}

//---------------------------------------------------------------------------------------------------- `egui/eframe` options & init
impl Gui {
	#[inline(always)]
	// Sets the initial options for native rendering with eframe
	pub fn options() -> eframe::NativeOptions {
		// Icon
		// SAFETY: This image is known at compile-time. It should never fail.
		let icon = image::load_from_memory(ICON).unwrap().to_rgba8();
		let (width, height) = icon.dimensions();
		let icon_data = Some(eframe::IconData {
			rgba: icon.into_raw(),
			width,
			height,
		});

		// The rest
		let options = eframe::NativeOptions {
			min_window_size: Some(egui::vec2(APP_RESOLUTION_MIN[0], APP_RESOLUTION_MIN[1])),
			initial_window_size: Some(egui::vec2(APP_RESOLUTION_MIN[0], APP_RESOLUTION_MIN[1])),
			follow_system_theme: false,
			default_theme: eframe::Theme::Dark,
			renderer: eframe::Renderer::Wgpu,
			icon_data,
			..Default::default()
		};

		ok!("eframe::NativeOptions");
		options
	}

	#[inline(always)]
	// This "starts" the `GUI` thread.
	pub fn init(
		cc:          &eframe::CreationContext<'_>,
		to_kernel:   Sender<FrontendToKernel>,
		from_kernel: Receiver<KernelToFrontend>,
	) -> Self {
		info!("GUI Init starting...");

		// Read `Settings` from disk.
		let settings = match Settings::from_file() {
			Ok(s)  => { ok!("GUI - Settings from disk"); s },
			Err(e) => { warn!("GUI - Settings failed from disk: {}", e); Settings::new() },
		};

		// Read `State` from disk.
		let state = match State::from_file() {
			Ok(s)  => { ok!("GUI - State from disk"); s },
			Err(e) => { warn!("GUI - State failed from disk: {}", e); State::new() },
		};
		let app = Self {
			// `Kernel` channels.
			to_kernel,
			from_kernel,

			// `shukusai` data.
			collection: Collection::dummy(),
			kernel_state: KernelState::dummy(),
			reset_state: ResetState::dummy(),

			// `GUI` settings.
			og_settings: settings.clone(),
			settings,

			// `GUI` state.
			og_state: state, // `copy`-able
			state,

			// `rfd`.
			rfd_open: Arc::new(AtomicBool::new(false)),
			rfd_new: Arc::new(Mutex::new(None)),
			deleted_paths: vec![],

			// Search state.
			searching: false,
			search_string: String::new(),
			search_result: Keychain::new(),

			// Local cache.
			count_artist: "Artists: 0".to_string(),
			count_album: "Albums: 0".to_string(),
			count_song: "Songs: 0".to_string(),

			exiting: false,
			exit_instant: now!(),

			resetting_collection: false,
		};

		// Style
		cc.egui_ctx.set_style(Self::init_style());

		// Visuals
		cc.egui_ctx.set_visuals(Self::init_visuals());

		// Fonts
		cc.egui_ctx.set_fonts(Self::init_fonts());

		// Done.
		ok!("GUI Init");
		app
	}
}
