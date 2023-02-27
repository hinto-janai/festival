//---------------------------------------------------------------------------------------------------- Resolution
pub const APP_MIN_WIDTH:  f32 = 1000.0;
pub const APP_MIN_HEIGHT: f32 = 800.0;
pub const APP_MIN_RESOLUTION: [f32; 2] = [APP_MIN_WIDTH, APP_MIN_HEIGHT];
pub const ALBUM_ART_DEFAULT_SIZE: f32 = APP_MIN_WIDTH / 4.0;

//---------------------------------------------------------------------------------------------------- Fonts
pub const FONT_SOURCECODE_PRO: &[u8] = include_bytes!("../../fonts/SourceCodePro-Regular.otf");
pub const FONT_SOURCECODE_CN:  &[u8] = include_bytes!("../../fonts/SourceHanSansCN-Regular.otf");
pub const FONT_SOURCECODE_HK:  &[u8] = include_bytes!("../../fonts/SourceHanSansHK-Regular.otf");
pub const FONT_SOURCECODE_TW:  &[u8] = include_bytes!("../../fonts/SourceHanSansTW-Regular.otf");
pub const FONT_SOURCECODE_KR:  &[u8] = include_bytes!("../../fonts/SourceHanSansKR-Regular.otf");
pub const FONT_SOURCECODE_JP:  &[u8] = include_bytes!("../../fonts/SourceHanSansJP-Regular.otf");
pub const FONT_JULIAMONO:      &[u8] = include_bytes!("../../fonts/JuliaMono-Regular.ttf");

//---------------------------------------------------------------------------------------------------- Icon
pub const ICON: &[u8] = include_bytes!("../../images/icon/512.png");

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
