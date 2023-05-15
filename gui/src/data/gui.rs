//---------------------------------------------------------------------------------------------------- Use
use crate::{
	constants::*,
};
use crate::data::{
	State,
	Settings,
};
use super::{
	DebugInfo,
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
	AlbumKey,
	Keychain,
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
use crossbeam::channel::{Sender,Receiver};
use std::path::PathBuf;
use std::sync::{
	Arc,
	Mutex,
	atomic::AtomicBool,
	atomic::AtomicU8,
};
use rolock::RoLock;
use disk::{Bincode2,Toml,Json};
use std::time::Instant;
use super::AlbumSizing;
use garde::Validate;

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
	/// If the user types English from anywhere,
	/// we switch to the `Search` tab, input the
	/// `String` and set this [`bool`] so that
	/// the GUI knows to `request_focus()` the search `TextEdit`.
	pub search_focus: bool,
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

	// Exit state.
	/// Are we currently in the process of exiting?
	pub exiting: bool,
	/// To prevent showing a flash of the spinner
	/// when exiting really quickly, this `Instant`
	/// needs to rack up some time before showing the spinner.
	pub exit_instant: Instant,
	/// How long before we force quit without saving.
	pub exit_countdown: Arc<AtomicU8>,

	// Reset/Collection state.
	/// Are we in the middle of resetting the [`Collection`]?
	pub resetting_collection: bool,
	/// This is a [`bool`] that is `false` until `Kernel`
	/// responds with any message after it's done startup.
	///
	/// Once we get our first message from `Kernel`, this
	/// will always be `true`. This is used for things like
	/// the initial album art spinner screen.
	pub kernel_returned: bool,

	// Debug screen.
	/// Are we showing the debug screen?
	///
	/// (User pressed CTRL+SHIFT+D)
	pub debug_screen: bool,
	/// The debug info displayed on the debug screen.
	pub debug_info: DebugInfo,
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
	/// Set the current [`Settings`] to disk.
	pub fn save_settings(&mut self) {
		self.set_settings();
		// TODO: handle save error.
		match self.settings.save_atomic() {
			Ok(_)  => ok_debug!("GUI - Settings save"),
			Err(e) => error!("GUI - Settings could not be saved to disk: {e}"),
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
		let k = lockr!(self.kernel_state);

		// PERF:
		// Comparison seems to be slower than un-conditional
		// assignment for small `copy`-able structs like `AudioState`,
		// so don't even check for diffs, just always copy.
		self.state.audio = k.audio;
	}

	#[inline(always)]
	/// Perform all the necessary steps to add a folder
	/// to add to the Collection (spawns RFD thread).
	pub fn add_folder(&self) {
		if atomic_load!(self.rfd_open) {
			warn!("GUI - Add folder requested, but RFD is already open");
		} else {
			crate::data::spawn_rfd_thread(
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

	#[inline(always)]
	/// Copies the data from our current [`Collection`],
	/// formats it, and assigns it to [`Self`]'s `count_*` fields.
	fn format_count_assign(&mut self) {
		self.count_artist = format!("Artists: {}", self.collection.count_artist);
		self.count_album  = format!("Albums: {}", self.collection.count_album);
		self.count_song   = format!("Songs: {}", self.collection.count_song);
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
				(TextStyle::Small,                 FontId::new(10.0, FontFamily::Monospace)),
				(TextStyle::Name("Medium".into()), FontId::new(15.0, FontFamily::Monospace)),
				(TextStyle::Body,                  FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Button,                FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Monospace,             FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Name("25".into()),     FontId::new(25.0, FontFamily::Monospace)),
				(TextStyle::Name("30".into()),     FontId::new(30.0, FontFamily::Monospace)),
				(TextStyle::Name("35".into()),     FontId::new(35.0, FontFamily::Monospace)),
				(TextStyle::Heading,               FontId::new(40.0, FontFamily::Monospace)),
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

		for (i, (font, bytes)) in FONT_ARRAY.iter().enumerate() {
			fonts.font_data.insert(font.to_string(), FontData::from_static(bytes));

			fonts.families.get_mut(&FontFamily::Monospace)
				.expect("Failed to get: egui::FontFamily::Monospace")
				.insert(i, font.to_string());
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
			Ok(s)  => { ok!("GUI - Settings from disk"); s.validate(&()).map(|_| s).unwrap_or_default() },
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
			search_focus: false,
			search_string: String::new(),
			search_result: Keychain::new(),

			// Local cache.
			count_artist: "Artists: 0".to_string(),
			count_album: "Albums: 0".to_string(),
			count_song: "Songs: 0".to_string(),

			exiting: false,
			exit_instant: now!(),
			exit_countdown: Arc::new(AtomicU8::new(EXIT_COUNTDOWN)),

			resetting_collection: false,
			kernel_returned: false,

			debug_screen: false,
			debug_info: DebugInfo::new(),
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
