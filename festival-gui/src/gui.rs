//---------------------------------------------------------------------------------------------------- Use
use super::{
	constants::*,
	tab::Tab,
	settings::Settings,
	state::State,
};
use shukusai::*;
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

//---------------------------------------------------------------------------------------------------- GUI struct. This hold ALL data.
pub struct Gui {
	// `Kernel` channels.
	pub(super) to_kernel: Sender<FrontendToKernel>,
	pub(super) from_kernel: Receiver<KernelToFrontend>,

	// `shukusai` data.
	pub(super) collection: Arc<Collection>,
	pub(super) k_state: RoLock<KernelState>,

	// `GUI` settings.
	pub(super) og_settings: Settings,
	pub(super) settings: Settings,

	// `GUI` state.
	pub(super) og_state: State,
	pub(super) state: State,
}

//---------------------------------------------------------------------------------------------------- GUI convenience functions.
impl Gui {
	#[inline]
	/// Set the original [`Settings`] to reflect live [`Settings`].
	pub(super) fn set_settings(&mut self) {
		self.og_settings = self.settings.clone();
	}

	#[inline]
	/// Set the original [`State`] to reflect live [`State`].
	pub(super) fn set_state(&mut self) {
		self.og_state = self.state.clone();
	}

	#[inline]
	/// Reset [`Settings`] to the original.
	pub(super) fn reset_settings(&mut self) {
		self.settings = self.og_settings.clone();
	}

	#[inline]
	/// Reset [`State`] to the original.
	pub(super) fn reset_state(&mut self) {
		self.state = self.og_state.clone();
	}
}

////---------------------------------------------------------------------------------------------------- GUI Init.
//// Instead of having [Gui::new()] be 1000s of lines long,
//// these private functions will be separate stuff.
//impl Gui {
//	#[inline(always)]
//	fn init_style() -> egui::Style {
//		let style = Style {
//			..Default::default()
//		};
//
//		ok_debug!("GUI Init | Style");
//		style
//	}
//
//	#[inline(always)]
//	fn init_visuals() -> egui::Visuals {
//		let visuals = Visuals {
//			slider_trailing_fill: true,
//			..Visuals::dark()
//		};
//
//		ok_debug!("GUI Init | Visuals");
//		visuals
//	}
//
//	#[inline(always)]
//	fn init_fonts() -> egui::FontDefinitions {
//		let mut fonts = FontDefinitions::default();
//		// TODO
//		fonts.font_data.insert("0".to_string(), FontData::from_static(FONT_SOURCECODE_PRO));
//		fonts.font_data.insert("1".to_string(), FontData::from_static(FONT_SOURCECODE_JP).tweak(
//			FontTweak {
//				y_offset_factor: -0.38, // Move it up
//				..Default::default()
//			},
//		));
////		fonts.font_data.insert("SourceCode-Pro".to_string(), FontData::from_static(FONT_SOURCECODE_PRO));
////		fonts.font_data.insert("SourceCode-CN".to_string(), FontData::from_static(FONT_SOURCECODE_CN));
////		fonts.font_data.insert("SourceCode-HK".to_string(), FontData::from_static(FONT_SOURCECODE_HK));
////		fonts.font_data.insert("SourceCode-TW".to_string(), FontData::from_static(FONT_SOURCECODE_TW));
////		fonts.font_data.insert("SourceCode-KR".to_string(), FontData::from_static(FONT_SOURCECODE_KR));
////		fonts.font_data.insert("SourceCode-JP".to_string(), FontData::from_static(FONT_SOURCECODE_JP));
////		fonts.font_data.insert("JuliaMono".to_string(), FontData::from_static(FONT_JULIAMONO));
//
//		for i in 0..=1 {
//			fonts.families.get_mut(&FontFamily::Monospace)
//				.expect("Failed to get: egui::FontFamily::Monospace")
//				.insert(i, i.to_string());
//			fonts.families.get_mut(&FontFamily::Proportional)
//				.expect("Failed to get: egui::FontFamily::Proportional")
//				.push(i.to_string());
//		}
//
//		ok_debug!("GUI Init | Fonts");
//		fonts
//	}
//}
//
////---------------------------------------------------------------------------------------------------- `egui/eframe` options & init
//impl Gui {
//	#[inline(always)]
//	// Sets the initial options for native rendering with eframe
//	pub(super) fn options() -> eframe::NativeOptions {
//		// Icon
//		let icon = image::load_from_memory(ICON).expect("Failed to read icon bytes").to_rgba8();
//		let (width, height) = icon.dimensions();
//		let icon_data = Some(eframe::IconData {
//			rgba: icon.into_raw(),
//			width,
//			height,
//		});
//
//		// The rest
//		let options = eframe::NativeOptions {
//			min_window_size: Some(egui::Vec2::from(APP_MIN_RESOLUTION)),
//			initial_window_size: Some(egui::Vec2::from(APP_MIN_RESOLUTION)),
//			follow_system_theme: false,
//			default_theme: eframe::Theme::Dark,
//			renderer: eframe::Renderer::Wgpu,
//			icon_data,
//			..Default::default()
//		};
//
//		ok!("eframe::NativeOptions");
//		options
//	}
//
//	#[inline(always)]
//	pub(super) fn init(
//		cc:          &eframe::CreationContext<'_>,
//		to_kernel:   Sender<FrontendToKernel>,
//		from_kernel: Receiver<KernelToFrontend>,
//	) -> Self {
//		info!("GUI Init starting...");
//
//		let mut app = Self {
//			to_kernel,
//			from_kernel,
//			img: Img::new(),
//			v: 0.0,
//			s: 1,
//			list: vec![
//				(1, "3:02", "Home Alone"),
//				(2, "3:22", "恋しい日々"),
//				(3, "3:20", "エメラルド"),
//				(4, "2:46", "ごあいさつ"),
//				(5, "2:09", "ジェットコースター"),
//				(6, "4:07", "序章"),
//				(7, "2:29", "ロマンス宣言"),
//				(8, "4:34", "ゆくえ"),
//				(9, "3:58", "サマーバケーション"),
//				(10, "3:45", "カーステレオから"),
//				(11, "4:38", "グレープフルーツ"),
//				(12, "3:21", "アーケード"),
//				(13, "4:18", "祝日"),
//			],
//			name: "祝祭",
//			tab: Tab::Albums,
//		};
//
//		// Style
//		cc.egui_ctx.set_style(Self::init_style());
//
//		// Visuals
//		cc.egui_ctx.set_visuals(Self::init_visuals());
//
//		// Fonts
//		cc.egui_ctx.set_fonts(Self::init_fonts());
//
//		// Done.
//		ok!("GUI Init");
//		app
//	}
//}
