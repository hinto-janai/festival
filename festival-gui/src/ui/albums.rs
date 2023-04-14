//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use egui::{
	Rounding,Vec2,Color32,Stroke,
	ScrollArea,Frame,
	SelectableLabel,Label,
};

//----------------------------------------------------------------------------------------------------
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_albums(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	ScrollArea::vertical().max_width(f32::INFINITY).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		let size = self.settings.album_art_size;

		let columns  = ui.available_width() / size;
		let rows     = (self.collection.albums.len() as f32 / columns as f32).ceil() as usize;
		let mut iter = self.collection.album_sort(self.settings.sort_order).iter().peekable();

		for row in 0..rows {
			ui.horizontal(|ui| {
				for _ in 0..columns as usize {
					match iter.next() {
						Some(key) => self.collection.albums[key].art_or().show_size(ui, Vec2::new(size, size)),
						None => break,
					};
				}
			});
		}

	});
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
