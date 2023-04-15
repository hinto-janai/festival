//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use egui::{
	Rounding,Vec2,Color32,Stroke,
	ScrollArea,Frame,
	SelectableLabel,Label,
};
use crate::data::AlbumSizing;

//----------------------------------------------------------------------------------------------------
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_albums(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {

	//---------------------------------------------------------------------------------------------------- If `AlbumSizing::Pixel`
	if let AlbumSizing::Pixel = self.settings.album_sizing {
		return;
	}

	//---------------------------------------------------------------------------------------------------- If `AlbumSizing::Row`
	if let AlbumSizing::Row = self.settings.album_sizing {

	ScrollArea::vertical().max_width(f32::INFINITY).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		// Get width.
		let width = ui.available_width();

		// Get per row.
		let per = self.settings.albums_per_row;

		// How many pixels per `Album`'s in one row? (rounded down)
		let per_f32 = per as f32;
		let row_width = width / per_f32;
//		// Account for space between `Album`'s.
		let row_width = row_width - per_f32;
		// How many rows?
		let rows = (self.collection.albums.len() as f32 / per_f32).ceil() as usize;

		// The iterator over sorted `Album`'s.
		let mut iter = self.collection
			.album_sort(self.settings.sort_order)
			.iter()
			.peekable();

		// Start the row.
//		ui.vertical_centered(|ui| {
		for row in 0..rows {
			// Paint `Album`'s horizontally.
			ui.horizontal(|ui| {
				// Paint as many `Album`'s that can fit.
				for _ in 0..per {
					match iter.next() {
						Some(key) => self.collection.albums[key].art_or().show_size(ui, Vec2::new(row_width, row_width)),

						// We're at the end, no more `Album`'s left.
						None => break,
					};
				}
			});
		}
//		});
	});
	}
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
