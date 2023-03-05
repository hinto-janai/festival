//---------------------------------------------------------------------------------------------------- Resolution
pub(crate) const APP_MIN_WIDTH:  f32 = 1000.0;
pub(crate) const APP_MIN_HEIGHT: f32 = 800.0;
pub(crate) const APP_MIN_RESOLUTION: [f32; 2] = [APP_MIN_WIDTH, APP_MIN_HEIGHT];
pub(crate) const ALBUM_ART_DEFAULT_SIZE: f32 = APP_MIN_WIDTH / 4.0;

//---------------------------------------------------------------------------------------------------- Fonts
pub(crate) const FONT_SOURCECODE_PRO: &[u8] = include_bytes!("../../assets/fonts/SourceCodePro-Regular.otf");
pub(crate) const FONT_SOURCECODE_CN:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansCN-Regular.otf");
pub(crate) const FONT_SOURCECODE_HK:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansHK-Regular.otf");
pub(crate) const FONT_SOURCECODE_TW:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansTW-Regular.otf");
pub(crate) const FONT_SOURCECODE_KR:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansKR-Regular.otf");
pub(crate) const FONT_SOURCECODE_JP:  &[u8] = include_bytes!("../../assets/fonts/SourceHanSansJP-Regular.otf");
pub(crate) const FONT_JULIAMONO:      &[u8] = include_bytes!("../../assets/fonts/JuliaMono-Regular.ttf");

//---------------------------------------------------------------------------------------------------- Icon
pub(crate) const ICON: &[u8] = include_bytes!("../../assets/images/icon/512.png");

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
