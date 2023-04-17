//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use egui::{
	Rounding,Vec2,Color32,Stroke,
	ScrollArea,Frame,RichText,
	SelectableLabel,Label,Button,
	ImageButton,
};
use crate::data::AlbumSizing;
use crate::ui::Tab;

//----------------------------------------------------------------------------------------------------
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_albums(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	// Make each `Album` separated by `10.0x10.0` pixels.
	ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

	// The iterator over sorted `Album`'s.
	let mut iter = self.collection
		.album_sort(self.settings.sort_order)
		.iter()
		.peekable();

	{
		// Reduce rounding corners.
		let widgets = &mut ui.visuals_mut().widgets;
		widgets.hovered.rounding  = egui::Rounding::none();
		widgets.inactive.rounding = egui::Rounding::none();
		widgets.active.rounding   = egui::Rounding::none();
		// Reduced padding.
		ui.spacing_mut().button_padding.x -= 2.0;
	}

	//---------------------------------------------------------------------------------------------------- If `AlbumSizing::Pixel`
	if let AlbumSizing::Pixel = self.settings.album_sizing {

	ScrollArea::vertical().id_source("AlbumPixel").max_width(f32::INFINITY).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		// Get width.
		let width = ui.available_width();

		// Get pixel size.
		let pixel = self.settings.album_pixel_size;

		// How many `Album`'s can fit in one row?
		let album_width = width / pixel;
		// Account for separation space and padding.
		let pixel = pixel - 13.0;

		// How many rows?
		let rows = (self.collection.count_album.inner() as f32 / album_width).ceil() as usize;

		// Start the row.
		for row in 0..rows {
			// Paint `Album`'s horizontally.
			ui.horizontal(|ui| {
				// Paint as many `Album`'s that can fit.
				for _ in 0..album_width as usize {
					match iter.next() {
						Some(key) => self.collection.albums[key].art_or().show_size(ui, egui::vec2(pixel, pixel)),

						// We're at the end, no more `Album`'s left.
						None => break,
					};
				}
			});
		}
	});
	return;
	}

	//---------------------------------------------------------------------------------------------------- If `AlbumSizing::Row`
	if let AlbumSizing::Row = self.settings.album_sizing {

	ScrollArea::vertical().id_source("AlbumRow").max_width(f32::INFINITY).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		// Get width.
		let width = ui.available_width();

		// Get per row.
		let per = self.settings.albums_per_row;

		// How many pixels per `Album`'s in one row? (rounded down)
		let per_f32 = per as f32;
		let per_width = width / per_f32;
		// Account for separation space and padding.
		let per_width = per_width - 13.0;

		// How many rows?
		let rows = (self.collection.count_album.inner() as f32 / per_f32).ceil() as usize;

		// Start the row.
		for row in 0..rows {
			// Paint `Album`'s horizontally.
			ui.horizontal(|ui| {
				// Paint as many `Album`'s that can fit.
				for _ in 0..per {
					match iter.next() {
						Some(key) => {
							// ImageButton.
							let img_button = ImageButton::new(self.collection.albums[key].texture_id(ctx), egui::vec2(per_width, per_width));

							if ui.add(img_button).clicked() {
								// Can't do this due to `&` + `&mut` rules.
//								self.set_album_tab_view(*key);
								self.state.album = Some(*key);
								self.state.tab   = Tab::View;
							}
						},

						// We're at the end, no more `Album`'s left.
						None => break,
					};
				}
			});
		}
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
