//---------------------------------------------------------------------------------------------------- Use.
use egui::epaint::{
	Rounding,
	Shadow,
	Stroke
};

use egui::{
	Color32,
	Visuals,
	style::Spacing,
};

use egui::style::{
	Selection,
	Widgets,
	WidgetVisuals,
};
use once_cell::sync::Lazy;
pub use const_format::assertcp as const_assert;
pub use const_format::formatcp as const_format;

//---------------------------------------------------------------------------------------------------- Version.
/// `Festival` version
///
/// This is the version of `Festival`, the `GUI`.
pub const FESTIVAL_VERSION: &str = {
	const_assert!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("v", env!("CARGO_PKG_VERSION"))
};

/// `Festival` + version
///
/// Just a string concatenating "Festival" and the current version, e.g: `Festival v0.0.1`
pub const FESTIVAL_NAME_VER: &str = {
	const_assert!(env!("CARGO_PKG_VERSION").len() != 0, "CARGO_PKG_VERSION is 0 length");
	concat!("Festival v", env!("CARGO_PKG_VERSION"))
};

//---------------------------------------------------------------------------------------------------- `egui` Visuals
/// This is based off [`Visuals::dark()`].
pub static VISUALS: Lazy<Visuals> = Lazy::new(|| {
	let selection = Selection {
		bg_fill: ACCENT_COLOR,
		stroke: Stroke::new(1.0, Color32::from_rgb(192, 222, 255)),
	};

	let widgets = Widgets {
		noninteractive: WidgetVisuals {
			weak_bg_fill: Color32::from_gray(20),
			bg_fill:      Color32::from_gray(20),
			bg_stroke:    Stroke::new(2.0, Color32::from_gray(40)), // separators, indentation lines
			fg_stroke:    Stroke::new(1.0, Color32::from_gray(140)), // normal text color
			rounding:     Rounding::same(10.0),
			expansion:    0.0,
		},
		inactive: WidgetVisuals {
			weak_bg_fill: Color32::from_gray(45),
			bg_fill:      Color32::from_gray(45),
//				weak_bg_fill: Color32::from_gray(60), // button background
//				bg_fill:      Color32::from_gray(60),      // checkbox background
			bg_stroke:    Default::default(),
			fg_stroke:    Stroke::new(1.0, Color32::from_gray(180)), // button text
			rounding:     Rounding::same(10.0),
			expansion:    0.0,
		},
		hovered: WidgetVisuals {
			weak_bg_fill: Color32::from_gray(60),
			bg_fill:      Color32::from_gray(60),
			bg_stroke:    Stroke::new(1.0, Color32::from_gray(150)), // e.g. hover over window edge or button
			fg_stroke:    Stroke::new(1.5, Color32::from_gray(240)), // egui-notify uses this for toast text
			rounding:     Rounding::same(10.0),
			expansion:    1.0,
		},
		active: WidgetVisuals {
			weak_bg_fill: Color32::from_gray(55),
			bg_fill:      Color32::from_gray(55),
			bg_stroke:    Stroke::new(1.0, Color32::WHITE),
			fg_stroke:    Stroke::new(2.0, Color32::WHITE),
			rounding:     Rounding::same(10.0),
			expansion:    1.0,
		},
		open: WidgetVisuals {
			weak_bg_fill: Color32::from_gray(27),
			bg_fill:      Color32::from_gray(27),
			bg_stroke:    Stroke::new(1.0, Color32::from_gray(60)),
			fg_stroke:    Stroke::new(1.0, Color32::from_gray(210)),
			rounding:     Rounding::same(10.0),
			expansion:    0.0,
		},
	};

    Visuals {
		dark_mode: true,
		override_text_color:     None,
		widgets,
		selection,
		hyperlink_color:         Color32::from_rgb(90, 170, 255),
		faint_bg_color:          Color32::from_additive_luminance(5), // visible, but barely so
		extreme_bg_color:        Color32::from_gray(10),            // e.g. TextEdit background
		code_bg_color:           Color32::from_gray(35), // egui-notify uses this for toast background
		warn_fg_color:           Color32::from_rgb(255, 143, 0), // orange
		error_fg_color:          Color32::from_rgb(255, 0, 0),  // red
		window_rounding:         Rounding::same(6.0),
		window_shadow:           Shadow::big_dark(),
		window_fill:             BG,
		window_stroke:           Stroke::new(1.0, Color32::from_gray(60)),
		menu_rounding:           Rounding::same(6.0),
		panel_fill:              BG,
		popup_shadow:            Shadow::small_dark(),
		resize_corner_size:      12.0,
		text_cursor_width:       2.0,
		text_cursor_preview:     false,
		clip_rect_margin:        3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
		button_frame:            true,
		collapsing_header_frame: false,
		indent_has_left_vline:   true,
		striped:                 false,
		slider_trailing_fill:    true,
	}
});

// Dark blue.
pub const ACCENT_COLOR_RGB: [u8; 3] = [30, 45, 85];
pub const ACCENT_COLOR: Color32 = Color32::from_rgb(
	ACCENT_COLOR_RGB[0],
	ACCENT_COLOR_RGB[1],
	ACCENT_COLOR_RGB[2],
);

// Pinkish red.
//pub const ACCENT_COLOR: Color32 = Color32::from_rgb(200, 100, 100);

//---------------------------------------------------------------------------------------------------- `egui` Spacing
pub static SPACING: Lazy<Spacing> = Lazy::new(|| {
	Spacing {
		scroll_bar_width: 12.5,
		..Default::default()
	}
});

//---------------------------------------------------------------------------------------------------- Search
/// How many bytes to allow in the search bar before truncating.
pub const SEARCH_MAX_LEN: usize = u8::MAX as usize;

//---------------------------------------------------------------------------------------------------- Disk
/// "gui", `GUI`'s sub-directory in the `Festival` project folder.
pub const GUI: &str = "gui";

/// Current major version of `GUI`'s `State`
pub const STATE_VERSION: u8 = 1;

/// Current major version of `GUI`'s `Settings`
pub const SETTINGS_VERSION: u8 = 1;

//---------------------------------------------------------------------------------------------------- Resolution
// 700.0 works on some `Album`'s in view tabs
// but `Album`'s with longer song titles makes
// the right side UI disappear.
pub const APP_WIDTH_MIN:          f32 = 870.0;
// This is also as low as the height can get
// before things get cut off.
pub const APP_HEIGHT_MIN:         f32 = 486.0;

pub const APP_WIDTH_DEFAULT:      f32 = 1000.0;
pub const APP_HEIGHT_DEFAULT:     f32 = 800.0;
// Default ratio should be 1.25
const _: () = {
	const RATIO: f32 = APP_WIDTH_DEFAULT / APP_HEIGHT_DEFAULT;
	const_assert!(RATIO == 1.25);
};

pub const APP_RESOLUTION_MIN:     [f32; 2] = [APP_WIDTH_MIN, APP_HEIGHT_MIN];
pub const APP_RESOLUTION_DEFAULT: [f32; 2] = [APP_WIDTH_DEFAULT, APP_HEIGHT_DEFAULT];
pub const ALBUM_ART_SIZE_MIN:     f32 = 100.0;
// Downscale the internal `shukusai` art
// just a little bit for a sharper image.
pub const ALBUM_ART_SIZE_MAX:     f32 = {
	const SIZE: f32 = shukusai::collection::ALBUM_ART_SIZE as f32 * 0.8;
	const_assert!(SIZE > ALBUM_ART_SIZE_MIN);
	SIZE
};
pub const ALBUM_ART_SIZE_DEFAULT: f32 = 227.0;
pub const ALBUMS_PER_ROW_MIN:      u8 = 1;
pub const ALBUMS_PER_ROW_MAX:      u8 = 20;
pub const ALBUMS_PER_ROW_DEFAULT:  u8 = 5;

//---------------------------------------------------------------------------------------------------- Playback controls
/// The width of the previous/pause/play/next buttons.
///
/// `77.0` lines up perfectly in fullscreen with the
/// variable width left panel on a 16:9 display.
pub const UI_CONTROL_WIDTH: f32 = 77.0;

//---------------------------------------------------------------------------------------------------- Queue tab
/// Fixed size of the `Album` art in the `Queue` tab.
pub const QUEUE_ALBUM_ART_SIZE: f32 = 80.0;

//---------------------------------------------------------------------------------------------------- Settings
pub const PREVIOUS_THRESHOLD_MIN: u32 = 0;
pub const PREVIOUS_THRESHOLD_MAX: u32 = 20;

#[cfg(target_os = "macos")]
// This needs to be slightly bigger on macOS.
pub const PIXELS_PER_POINT_DEFAULT: f32 = 2.0;
#[cfg(not(target_os = "macos"))]
pub const PIXELS_PER_POINT_DEFAULT: f32 = 1.5;

pub const PIXELS_PER_POINT_UNIT: f32 = 0.1;
pub const PIXELS_PER_POINT_MIN:  f32 = 0.1;
pub const PIXELS_PER_POINT_MAX:  f32 = 3.0;
// INVARIANT: must be the same as above.
// HACK: `const_format` can't take floats as input.
pub const PIXELS_PER_POINT_UNIT_STR: &str = "0.1";
pub const PIXELS_PER_POINT_MIN_STR:  &str = "0.1";
pub const PIXELS_PER_POINT_MAX_STR:  &str = "3.0";

//---------------------------------------------------------------------------------------------------- Fonts
pub const FONT_SOURCECODE_PRO: &[u8] = include_bytes!("../../assets/fonts/SourceCodePro-Regular.otf");
pub const FONT_SOURCECODE_CN:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansCN-Regular.otf");
pub const FONT_SOURCECODE_KR:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansKR-Regular.otf");
pub const FONT_SOURCECODE_JP:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansJP-Regular.otf");
pub const FONT_JULIAMONO:      &[u8] = include_bytes!("../../assets/fonts/JuliaMono-Regular.ttf");

pub const FONT_ARRAY: [(&str, &[u8]); 5] = [
	("SourceCode-Pro", FONT_SOURCECODE_PRO),
	("SourceCode-JP",  FONT_SOURCECODE_JP),
	("JuliaMono",      FONT_JULIAMONO),
	("SourceCode-KR",  FONT_SOURCECODE_KR),
	("SourceCode-CN",  FONT_SOURCECODE_CN),
	// This used to include `SourceCode-TW` and `SourceCode-HK`
	// but `festival.pm`'s server has a 32MB file upload limit...
	// I'm sorry HK and TW, you'll have to deal with
	// slightly different character radicals.
];

//---------------------------------------------------------------------------------------------------- Color
pub const RED:           Color32 = Color32::from_rgb(230, 50, 50);
pub const GREEN:         Color32 = Color32::from_rgb(80, 180, 80);
pub const YELLOW:        Color32 = Color32::from_rgb(180, 180, 80);
pub const BRIGHT_YELLOW: Color32 = Color32::from_rgb(250, 250, 100);
pub const BONE:          Color32 = Color32::from_rgb(190, 190, 190); // In between LIGHT_GRAY <-> GRAY
pub const WHITE:         Color32 = Color32::WHITE;
pub const LESS_WHITE:    Color32 = Color32::from_rgb(240, 240, 240);
pub const GRAY:          Color32 = Color32::GRAY;
pub const LIGHT_GRAY:    Color32 = Color32::LIGHT_GRAY;
pub const BLACK:         Color32 = Color32::BLACK;
pub const MEDIUM_GRAY:   Color32 = Color32::from_rgb(90, 90, 90);
pub const DARK_GRAY:     Color32 = Color32::from_rgb(18, 18, 18);
pub const BG:            Color32 = Color32::from_rgb(20, 20, 20);

//---------------------------------------------------------------------------------------------------- Custom Widget Colors
pub const SLIDER_CIRCLE_INACTIVE: egui::Stroke = Stroke{ width: 1.5, color: BLACK };
pub const SLIDER_CIRCLE_HOVERED:  egui::Stroke = Stroke{ width: 2.0, color: BLACK };
pub const SLIDER_CIRCLE_ACTIVE:   egui::Stroke = Stroke{ width: 2.5, color: WHITE };
