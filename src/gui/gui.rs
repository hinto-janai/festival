//---------------------------------------------------------------------------------------------------- Use
use super::{
	constants::*,
	tab::Tab,
};
use crate::macros::*;
use log::{
	info,
	warn,
	error
};
use egui::{
	Style,Visuals,
	ScrollArea,Frame,
	Color32,Vec2,Stroke,Rounding,RichText,
	TopBottomPanel,SidePanel,CentralPanel,
	FontData,FontDefinitions,FontFamily,FontTweak,
};
use egui::widgets::{
	Slider,Button,SelectableLabel,Label,
};
use strum::{
	IntoEnumIterator,
};
use super::{GuiToKernel, KernelToGui};

//---------------------------------------------------------------------------------------------------- TODO
// TODO: tmp data
pub struct Img {
	pub vec: Vec<(&'static str, egui_extras::RetainedImage)>,
}

impl Img {
	fn new() -> Self {
		let now = std::time::Instant::now();
		let mut vec = vec![
				("さよーならあなた", egui_extras::RetainedImage::from_image_bytes("a.jpg", include_bytes!("../../images/art/a.jpg")).unwrap()),
				("ひかれあい", egui_extras::RetainedImage::from_image_bytes("b.jpg", include_bytes!("../../images/art/b.jpg")).unwrap()),
				("祝祭", egui_extras::RetainedImage::from_image_bytes("c.jpg", include_bytes!("../../images/art/c.jpg")).unwrap()),
				("祝祭ひとりでに", egui_extras::RetainedImage::from_image_bytes("d.jpg", include_bytes!("../../images/art/d.jpg")).unwrap()),
				("燦々", egui_extras::RetainedImage::from_image_bytes("e.jpg", include_bytes!("../../images/art/e.jpg")).unwrap()),
				("燦々ひとりでに", egui_extras::RetainedImage::from_image_bytes("f.jpg", include_bytes!("../../images/art/f.jpg")).unwrap()),
				("よすが", egui_extras::RetainedImage::from_image_bytes("g.jpg", include_bytes!("../../images/art/g.jpg")).unwrap()),
				("よすがひとりでに", egui_extras::RetainedImage::from_image_bytes("h.jpg", include_bytes!("../../images/art/h.jpg")).unwrap()),
				("タオルケットは穏やかな", egui_extras::RetainedImage::from_image_bytes("i.jpg", include_bytes!("../../images/art/i.jpg")).unwrap()),
		];
		Self {
			vec,
		}
	}
}

//---------------------------------------------------------------------------------------------------- GUI struct. This hold ALL data.
pub struct Gui {
	// TODO: tmp data.
	pub img: Img,
	pub v: f32,
	pub s: u8,
	pub list: Vec<(u8, &'static str, &'static str)>,
	pub name: &'static str,

	// TODO: This is real data, clean it up.
	pub tab: super::tab::Tab, // This should be in [State]
}

//---------------------------------------------------------------------------------------------------- GUI Init.
// Instead of having [Gui::new()] be 1000s of lines long,
// these private functions will be separate stuff.
impl Gui {
	#[inline(always)]
	fn init_style() -> egui::Style {
		let style = Style {
			..Default::default()
		};

		ok_debug!("GUI Init | Style");
		style
	}

	#[inline(always)]
	fn init_visuals() -> egui::Visuals {
		let visuals = Visuals {
			slider_trailing_fill: true,
			..Visuals::dark()
		};

		ok_debug!("GUI Init | Visuals");
		visuals
	}

	#[inline(always)]
	fn init_fonts() -> egui::FontDefinitions {
		let mut fonts = FontDefinitions::default();
		// TODO
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

		ok_debug!("GUI Init | Fonts");
		fonts
	}
}

//---------------------------------------------------------------------------------------------------- egui/eframe options & init
impl Gui {
	#[inline(always)]
	// Sets the initial options for native rendering with eframe
	pub fn options() -> eframe::NativeOptions {
		// Icon
		let icon = image::load_from_memory(ICON).expect("Failed to read icon bytes").to_rgba8();
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
	pub fn init(
		cc: &eframe::CreationContext<'_>,
		to_kernel: crossbeam_channel::Sender<GuiToKernel>,
		from_kernel: std::sync::mpsc::Receiver<KernelToGui>,
	) -> Self {
		info!("GUI Init starting...");

		let mut app = Self {
			img: Img::new(),
			v: 0.0,
			s: 1,
			list: vec![
				(1, "3:02", "Home Alone"),
				(2, "3:22", "恋しい日々"),
				(3, "3:20", "エメラルド"),
				(4, "2:46", "ごあいさつ"),
				(5, "2:09", "ジェットコースター"),
				(6, "4:07", "序章"),
				(7, "2:29", "ロマンス宣言"),
				(8, "4:34", "ゆくえ"),
				(9, "3:58", "サマーバケーション"),
				(10, "3:45", "カーステレオから"),
				(11, "4:38", "グレープフルーツ"),
				(12, "3:21", "アーケード"),
				(13, "4:18", "祝日"),
			],
			name: "祝祭",
			tab: super::tab::Tab::Album,
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

//---------------------------------------------------------------------------------------------------- Main GUI event loop.
impl eframe::App for Gui {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		// Determine if there is a diff in settings.
//		let diff = self.settings != self.og;

		// Set global UI [Style/Visual]s
//		if diff {
//			ctx.set_visuals();
//		}

		// Set global available width/height.
		let rect   = ctx.available_rect();
		let width  = rect.width();
		let height = rect.height();

		// Size definitions of the major UI panels.
		let bottom_panel_height = height / 15.0;
		let side_panel_width    = width / 8.0;
		let side_panel_height   = height - (bottom_panel_height*2.0);

		// Bottom Panel
		Self::show_bottom(self, ctx, frame, width, bottom_panel_height);

		// Left Panel
		Self::show_left(self, ctx, frame, side_panel_width, side_panel_height);

		// Right Panel
		Self::show_right(self, ctx, frame, side_panel_width, side_panel_height);

		// Central Panel
		Self::show_central(self, ctx, frame, width, height);
	}
}

//---------------------------------------------------------------------------------------------------- Bottom Panel
impl Gui {
#[inline(always)]
fn show_bottom(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	TopBottomPanel::bottom("bottom").resizable(false).show(ctx, |ui| {
		ui.set_height(height);

		// Base unit for sizing UI.
		let unit = width / 15.0;

		ui.horizontal(|ui| {
			// Media control buttons
			ui.group(|ui| {
				ui.add_sized([unit, height], Button::new("⏪"));
				ui.add_sized([unit*1.5, height], Button::new("▶"));
				ui.add_sized([unit, height], Button::new("⏩"));
			});

			// Song time elapsed
			ui.add_sized([unit, height], Label::new("1:33 / 3:22"));

			// Slider (playback)
			ui.spacing_mut().slider_width = ui.available_width();
			ui.add_sized(
				[ui.available_width(), height],
				Slider::new(&mut self.v, 0.0..=100.0).smallest_positive(1.0).show_value(false)
			);
		});
	});
}}

//---------------------------------------------------------------------------------------------------- Left Panel
impl Gui {
#[inline(always)]
fn show_left(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	SidePanel::left("left").resizable(false).show(ctx, |ui| {
		ui.set_width(width);
//		ui.set_height(height);

		// Size definitions of the elements within the left panel.
		let half_height = height / 2.0;
		let tab_height  = half_height / 7.0;
		let tab_width   = width / 1.2;

		// Main UI
		ui.vertical_centered_justified(|ui| {

			// Display [SelectableLabel] for each [Tab].
			for tab in Tab::iter() {
				if ui.add_sized([tab_width, tab_height], SelectableLabel::new(self.tab == tab, tab.as_str())).clicked() {
					self.tab = tab;
				}
				ui.separator();
			}

			// Volume slider
			let slider_height = ui.available_height() - 20.0;

			ui.add_space(10.0);

			ui.spacing_mut().slider_width = slider_height;
			ui.visuals_mut().selection.bg_fill = Color32::from_rgb(200, 100, 100);

			ui.horizontal(|ui| {
				let unit = width / 10.0;
				ui.add_space(unit*4.0);
				ui.add(Slider::new(&mut self.v, 0.0..=100.0).smallest_positive(1.0).show_value(false).vertical().thickness(unit*2.0).circle_size(unit));
			});
		});
	});
}}

//---------------------------------------------------------------------------------------------------- Right Panel
impl Gui {
#[inline(always)]
fn show_right(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	SidePanel::right("right").resizable(false).show(ctx, |ui| {
		ui.set_width(width);

		// How big the albums (on the right side) should be.
		let ALBUM_SIZE = width / 1.4;

		ScrollArea::vertical().max_width(width).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
			ui.vertical_centered(|ui| {
			for (name, img) in &self.img.vec {
				ui.add_space(5.0);
				ui.scope(|ui| {
					ui.set_width(ALBUM_SIZE);
	 				Frame::window(&ctx.style()).rounding(Rounding::none()).inner_margin(1.0).show(ui, |ui| {
						let mut rect = ui.cursor();
						rect.max.x += 2.0;
						rect.max.y = rect.min.y + ALBUM_SIZE;
						if ui.put(rect, Button::new("").rounding(Rounding::none())).clicked() { self.name = *name };
						rect.max.x = rect.min.x;
						ui.allocate_ui_at_rect(rect, |ui| {
							ui.horizontal_centered(|ui| {
								img.show_size(ui, Vec2::new(ALBUM_SIZE, ALBUM_SIZE));
							});
						});
					});
				});
				if *name == self.name {
					ui.add(Label::new(RichText::new(name.to_string()).color(Color32::LIGHT_BLUE)));
				} else {
					ui.label(name.to_string());
				}
				ui.add_space(5.0);
			}
			});
		});
	});
}}

//---------------------------------------------------------------------------------------------------- Central Panel
impl Gui {
#[inline(always)]
fn show_central(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	CentralPanel::default().show(ctx, |ui| {
		use super::tab::Tab;
		match self.tab { // TODO - this should be self.state.tab
			Tab::Album    => Self::show_tab_album(self, ui, ctx, frame, width, height),
			_ => (),
		}
	});
}}
