//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use crate::constants::{
	VISUALS,
	SPACING,
	FONT_ARRAY,
	APP_RESOLUTION_MIN,
	EXIT_COUNTDOWN,
};
use crate::data::{
	State,
	Settings,
	DebugInfo,
};
use shukusai::{
	constants::{
		FESTIVAL_ICON,
		FESTIVAL_DBUS,
		FESTIVAL,
	},
	kernel::{
		FrontendToKernel,
		KernelToFrontend,
	},
	state::{
		AudioState,
		ResetState,
	},
	collection::{
		Collection,
		Keychain,
	},
};
use benri::{
	now,
	send,
	log::*,
	atomic_store,
};
use log::{
	info,
	warn,
};
use egui::{
	FontDefinitions,FontId,TextStyle,
	Style,FontData,FontFamily,
};
use crossbeam::channel::{
	Sender,Receiver
};
use std::sync::{
	Arc,
	Mutex,
	atomic::AtomicBool,
	atomic::AtomicU8,
};
use disk::{Bincode2,Toml,Json};

//---------------------------------------------------------------------------------------------------- GUI Init.
// Instead of having [Gui::new()] be 1000s of lines long,
// these private functions will be separate stuff.
//
// See `Gui::init` at the bottom to see the function that "starts" the `GUI`.
impl crate::data::Gui {
	#[inline(always)]
	fn init_style() -> egui::Style {
		Style {
			text_styles: [
				(TextStyle::Small,                 FontId::new(10.0, FontFamily::Monospace)),
				(TextStyle::Name("15".into()),     FontId::new(15.0, FontFamily::Monospace)),
				(TextStyle::Body,                  FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Button,                FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Monospace,             FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Name("25".into()),     FontId::new(25.0, FontFamily::Monospace)),
				(TextStyle::Name("30".into()),     FontId::new(30.0, FontFamily::Monospace)),
				(TextStyle::Name("35".into()),     FontId::new(35.0, FontFamily::Monospace)),
				(TextStyle::Heading,               FontId::new(40.0, FontFamily::Monospace)),
			].into(),

			spacing: SPACING.clone(),

			// Smooths out some resizing animations.
			// Default is `1.0 / 12.0` (very fast).
			animation_time: 0.25,

			// This gets rid of the delay for tooltips.
			// (It wants to wait until the pointer is still).
			interaction: egui::style::Interaction { show_tooltips_only_when_still: false, ..Default::default() },

			..Default::default()
		}
	}

	#[inline(always)]
	fn init_visuals() -> egui::Visuals {
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

		fonts
	}

	//---------------------------------------------------------------------------------------------------- `egui/eframe` options & init
	#[inline(always)]
	// Sets the initial options for native rendering with eframe
	pub fn options() -> eframe::NativeOptions {
		// Icon
		// SAFETY: This image is known at compile-time. It should never fail.
		let icon = image::load_from_memory(FESTIVAL_ICON).unwrap().to_rgba8();
		let (width, height) = icon.dimensions();
		let icon_data = Some(eframe::IconData {
			rgba: icon.into_raw(),
			width,
			height,
		});

		// The rest
		eframe::NativeOptions {
			min_window_size: Some(egui::vec2(APP_RESOLUTION_MIN[0], APP_RESOLUTION_MIN[1])),
			initial_window_size: Some(egui::vec2(APP_RESOLUTION_MIN[0], APP_RESOLUTION_MIN[1])),
			follow_system_theme: false,
			default_theme: eframe::Theme::Dark,
			renderer: eframe::Renderer::Wgpu,
			icon_data,
			..Default::default()
		}
	}

	#[inline(always)]
	// This "starts" the `GUI` thread.
	pub fn init(
		cc:          &eframe::CreationContext<'_>,
		to_kernel:   Sender<FrontendToKernel>,
		from_kernel: Receiver<KernelToFrontend>,
	) -> Self {
		// Read `Settings` from disk.
		let settings = match Settings::from_file() {
			Ok(s)  => { info!("GUI Init [1/8] ... Settings from disk"); s },
			Err(e) => { warn!("GUI Init [1/8] ... Settings failed from disk: {}", e); Settings::new() },
		};

		// Read `State` from disk.
		let state = match State::from_file() {
			Ok(s)  => { info!("GUI Init [2/8] ... State from disk"); s },
			Err(e) => { warn!("GUI Init [2/8] ... State failed from disk: {}", e); State::new() },
		};

		// Send signal to `Kernel` for `AudioState` if set.
		if settings.restore_state {
			info!("GUI Init [3/8] ... Restoring AudioState");
			send!(to_kernel, FrontendToKernel::RestoreAudioState);
		} else {
			info!("GUI Init [3/8] ... Skipping AudioState");
		}

		// Style
		cc.egui_ctx.set_style(Self::init_style());
		info!("GUI Init [5/8] ... Style");

		// Visuals
		cc.egui_ctx.set_visuals(Self::init_visuals());
		info!("GUI Init [6/8] ... Visuals");

		// Fonts
		cc.egui_ctx.set_fonts(Self::init_fonts());
		info!("GUI Init [7/8] ... Fonts");

		// Done.
		info!("GUI Init [8/8] ... Init");
		Self {
			// `Kernel` channels.
			to_kernel,
			from_kernel,

			// `shukusai` data.
			collection: Collection::dummy(),

			// `GUI` settings.
			og_settings: settings.clone(),
			settings,

			// `GUI` state.
			og_state: state.clone(),
			state,

			// AudioState.
			audio_state: AudioState::new(),
			audio_seek: 0,
			audio_leeway: now!(),
			last_song: None,

			reset_state: ResetState::new(),

			rect: egui::Rect { min: Default::default(), max: Default::default() },
			resize_leeway: now!(),

			// `egui_notify`
			toasts: egui_notify::Toasts::new(),

			// `rfd`.
			rfd_open: Arc::new(AtomicBool::new(false)),
			rfd_new: Arc::new(Mutex::new(None)),
			deleted_paths: vec![],

			// Search state.
			searching: false,
			search_jump: false,

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

			modifiers: Default::default(),
		}
	}
}
