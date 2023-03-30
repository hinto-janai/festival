//---------------------------------------------------------------------------------------------------- Version
/// Current major version of `State`
pub const STATE_VERSION: u8 = 1;

/// Current major version of `Settings`
pub const SETTINGS_VERSION: u8 = 1;

//---------------------------------------------------------------------------------------------------- Resolution
pub const APP_MIN_WIDTH:  f32 = 1000.0;
pub const APP_MIN_HEIGHT: f32 = 800.0;
pub const APP_MIN_RESOLUTION: [f32; 2] = [APP_MIN_WIDTH, APP_MIN_HEIGHT];
pub const ALBUM_ART_MIN_SIZE: f32 = 50.0;
pub const ALBUM_ART_MAX_SIZE: f32 = 600.0;
pub const ALBUM_ART_DEFAULT_SIZE: f32 = 300.0;

//---------------------------------------------------------------------------------------------------- Fonts
pub const FONT_SOURCECODE_PRO: &[u8] = include_bytes!("../../assets/fonts/SourceCodePro-Regular.otf");
pub const FONT_SOURCECODE_CN:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansCN-Regular.otf");
pub const FONT_SOURCECODE_HK:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansHK-Regular.otf");
pub const FONT_SOURCECODE_TW:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansTW-Regular.otf");
pub const FONT_SOURCECODE_KR:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansKR-Regular.otf");
pub const FONT_SOURCECODE_JP:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansJP-Regular.otf");
pub const FONT_JULIAMONO:      &[u8] = include_bytes!("../../assets/fonts/JuliaMono-Regular.ttf");

//---------------------------------------------------------------------------------------------------- Icon
pub const ICON: &[u8] = include_bytes!("../../assets/images/icon/512.png");

//---------------------------------------------------------------------------------------------------- Color
pub const RED:           egui::Color32 = egui::Color32::from_rgb(230, 50, 50);
pub const GREEN:         egui::Color32 = egui::Color32::from_rgb(100, 230, 100);
pub const YELLOW:        egui::Color32 = egui::Color32::from_rgb(230, 230, 100);
pub const BRIGHT_YELLOW: egui::Color32 = egui::Color32::from_rgb(250, 250, 100);
pub const BONE:          egui::Color32 = egui::Color32::from_rgb(190, 190, 190); // In between LIGHT_GRAY <-> GRAY
pub const WHITE:         egui::Color32 = egui::Color32::WHITE;
pub const GRAY:          egui::Color32 = egui::Color32::GRAY;
pub const LIGHT_GRAY:    egui::Color32 = egui::Color32::LIGHT_GRAY;
pub const BLACK:         egui::Color32 = egui::Color32::BLACK;
pub const DARK_GRAY:     egui::Color32 = egui::Color32::from_rgb(18, 18, 18);

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
