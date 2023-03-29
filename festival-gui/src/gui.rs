//---------------------------------------------------------------------------------------------------- Use
use super::{
	constants::*,
	tab::Tab,
	settings::Settings,
	state::State,
};
use shukusai::kernel::{
	Kernel,
	KernelState,
	FrontendToKernel,
	KernelToFrontend,
};
use shukusai::collection::{
	Collection,
};
use shukusai::sort::{
};
use shukusai::{
	ok,
	ok_debug,
	lock_read,
	mass_panic,
};
use log::{
	info,
	warn,
	error
};
use egui::{
	Style,Visuals,
	TopBottomPanel,SidePanel,CentralPanel,
	FontData,FontDefinitions,FontFamily,FontTweak,
};
use crossbeam_channel::{Sender,Receiver};
use std::sync::Arc;
use rolock::RoLock;
use disk::Bincode;

//---------------------------------------------------------------------------------------------------- GUI struct. This hold ALL data.
pub struct Gui {
	// `Kernel` channels.
	pub to_kernel: Sender<FrontendToKernel>,
	pub from_kernel: Receiver<KernelToFrontend>,

	// `shukusai` data.
	pub collection: Arc<Collection>,
	pub kernel_state: RoLock<KernelState>,

	// `GUI` settings.
	pub og_settings: Settings,
	pub settings: Settings,

	// `GUI` state.
	pub og_state: State,
	pub state: State,
}

//---------------------------------------------------------------------------------------------------- GUI convenience functions.
impl Gui {
	#[inline(always)]
	/// Set the original [`Settings`] to reflect live [`Settings`].
	pub fn set_settings(&mut self) {
		self.og_settings = self.settings.clone();
	}

	#[inline(always)]
	/// Set the original [`State`] to reflect live [`State`].
	pub fn set_state(&mut self) {
		self.og_state = self.state.clone();
	}

	#[inline(always)]
	/// Reset [`Settings`] to the original.
	pub fn reset_settings(&mut self) {
		self.settings = self.og_settings.clone();
	}

	#[inline(always)]
	/// Reset [`State`] to the original.
	pub fn reset_state(&mut self) {
		self.state = self.og_state.clone();
	}

	#[inline]
	/// Returns true if either [`Settings`] or [`State`] have diffs.
	pub fn diff(&self) -> bool {
		(self.state == self.og_state) && (self.settings == self.og_settings)
	}

	#[inline(always)]
	/// Returns true if [`Settings`] and the old version are not `==`.
	pub fn diff_settings(&self) -> bool {
		self.settings == self.og_settings
	}

	#[inline(always)]
	/// Returns true if [`State`] and the old version are not `==`.
	pub fn diff_state(&self) -> bool {
		self.state == self.og_state
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
			..Default::default()
		};

		ok_debug!("GUI Init - Style");
		style
	}

	#[inline(always)]
	fn init_visuals() -> egui::Visuals {
		let visuals = Visuals {
			slider_trailing_fill: true,
			..Visuals::dark()
		};

		ok_debug!("GUI Init - Visuals");
		visuals
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
			min_window_size: Some(egui::Vec2::from(APP_MIN_RESOLUTION)),
			initial_window_size: Some(egui::Vec2::from(APP_MIN_RESOLUTION)),
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
			Ok(s)  => s,
			Err(e) => { warn!("GUI - Settings failed from disk: {}", e); Settings::new() },
		};

		// Read `State` from disk.
		let state = match State::from_file() {
			Ok(s)  => s,
			Err(e) => { warn!("GUI - State failed from disk: {}", e); State::new() },
		};

		let mut app = Self {
			// `Kernel` channels.
			to_kernel,
			from_kernel,

			// `shukusai` data.
			collection: Collection::dummy(),
			kernel_state: KernelState::dummy(),

			// `GUI` settings.
			og_settings: settings.clone(),
			settings,

			// `GUI` state.
			og_state: state.clone(),
			state,
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
