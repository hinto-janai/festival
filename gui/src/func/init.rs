//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{
	ICON,
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
	FESTIVAL_DBUS,
	FESTIVAL,
};
use shukusai::kernel::{
	AudioState,
	ResetState,
	FrontendToKernel,
	KernelToFrontend,
};
use shukusai::collection::{
	Collection,
	Keychain,
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
		let style = Style {
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
		};

		style
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

	#[inline(always)]
	fn init_media_control(
		to_kernel: Sender<FrontendToKernel>,
		raise: Arc<AtomicBool>,
		should_exit: Arc<AtomicBool>,
	) -> souvlaki::MediaControls {
		#[cfg(target_os = "windows")]
		let hwnd = todo!();

		#[cfg(not(target_os = "windows"))]
		let hwnd = None;

		let config = souvlaki::PlatformConfig {
			dbus_name: FESTIVAL_DBUS,
			display_name: FESTIVAL,
			hwnd,
		};

		let mut media_controls = souvlaki::MediaControls::new(config).unwrap();

		media_controls.attach(move |event| {
			use souvlaki::{SeekDirection, MediaControlEvent::*};
			use shukusai::kernel::Seek;
			match event {
				Play                  => send!(to_kernel, FrontendToKernel::Play),
				Pause|Stop            => send!(to_kernel, FrontendToKernel::Pause),
				Toggle                => send!(to_kernel, FrontendToKernel::Toggle),
				Next                  => send!(to_kernel, FrontendToKernel::Next),
				Previous              => send!(to_kernel, FrontendToKernel::Previous),
				Seek(direction)       => {
					match direction {
						SeekDirection::Forward  => send!(to_kernel, FrontendToKernel::Seek((Seek::Forward, 5))),
						SeekDirection::Backward => send!(to_kernel, FrontendToKernel::Seek((Seek::Backward, 5))),
					}
				},
				SeekBy(direction, time) => {
					match direction {
						SeekDirection::Forward  => send!(to_kernel, FrontendToKernel::Seek((Seek::Forward, time.as_secs()))),
						SeekDirection::Backward => send!(to_kernel, FrontendToKernel::Seek((Seek::Backward, time.as_secs()))),
					}
				},
				SetPosition(time)      => send!(to_kernel, FrontendToKernel::Seek((Seek::Absolute, time.0.as_secs()))),
				OpenUri(string)       => warn!("GUI - Ignoring OpenURI({string})"),
				Raise                 => atomic_store!(raise, true),
				Quit                  => atomic_store!(should_exit, true),
			}
		}).unwrap();

		media_controls
	}

	//---------------------------------------------------------------------------------------------------- `egui/eframe` options & init
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
			Ok(s)  => { info!("GUI [1/8] - Settings from disk"); s },
			Err(e) => { warn!("GUI [1/8] - Settings failed from disk: {}", e); Settings::new() },
		};

		// Read `State` from disk.
		let state = match State::from_file() {
			Ok(s)  => { info!("GUI [2/8] - State from disk"); s },
			Err(e) => { warn!("GUI [2/8] - State failed from disk: {}", e); State::new() },
		};

		// Send signal to `Kernel` for `AudioState` if set.
		if settings.restore_state {
			info!("GUI [3/8] - Restoring AudioState");
			send!(to_kernel, FrontendToKernel::RestoreAudioState);
		} else {
			info!("GUI [3/8] - Skipping AudioState");
		}

		// Media controls.
		let raise          = Arc::new(AtomicBool::new(false));
		let should_exit    = Arc::new(AtomicBool::new(false));
		let media_controls = Self::init_media_control(
			to_kernel.clone(),
			Arc::clone(&raise),
			Arc::clone(&should_exit),
		);
		info!("GUI [4/8] - Media controls");

		// Style
		cc.egui_ctx.set_style(Self::init_style());
		info!("GUI [5/8] - Style");

		// Visuals
		cc.egui_ctx.set_visuals(Self::init_visuals());
		info!("GUI [6/8] - Visuals");

		// Fonts
		cc.egui_ctx.set_fonts(Self::init_fonts());
		info!("GUI [7/8] - Fonts");

		// Done.
		info!("GUI [8/8] - Init");
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

			// Media controls.
			media_controls,
			raise,

			// AudioState.
			audio_state: AudioState::new(),
			audio_seek: 0,
			audio_leeway: now!(),
			last_song: None,

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
			should_exit,

			resetting_collection: false,
			kernel_returned: false,

			debug_screen: false,
			debug_info: DebugInfo::new(),
		}
	}
}
