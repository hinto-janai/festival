//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use egui::{
	Rounding,Vec2,Color32,Stroke,
	ScrollArea,Frame,RichText,
	SelectableLabel,Label,Button,
	ImageButton,TextStyle,
};
use crate::data::AlbumSizing;
use crate::ui::Tab;
use crate::slice::Head;
use crate::constants::{
	BONE,
};

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
		// Small font.
		ui.style_mut().override_text_style = Some(TextStyle::Name("Medium".into()));
	}

	// Get spacing.
	let (
		width,
		pixel,
		rows,
		album_width,
	) = {
		let width = ui.available_width();
		//---------------------------------------------------------------------------------------------------- If `AlbumSizing::Pixel`
		if let AlbumSizing::Pixel = self.settings.album_sizing {
			// Get pixel size.
			let pixel = self.settings.album_pixel_size;
			// How many `Album`'s can fit in one row?
			let album_width = width / pixel;
			// Account for separation space and padding.
			let pixel = pixel - 13.0;
			// How many rows?
			let rows = (self.collection.count_album.inner() as f32 / album_width).ceil() as usize;
			(width, pixel, rows, album_width as usize)
		//---------------------------------------------------------------------------------------------------- If `AlbumSizing::Row`
		} else {
			// Get per row.
			let per = self.settings.albums_per_row;
			// How many pixels per `Album`'s in one row? (rounded down)
			let per_f32 = per as f32;
			let pixel = width / per_f32;
			// Account for separation space and padding.
			let pixel = pixel - 13.0;
			// How many rows?
			let rows = (self.collection.count_album.inner() as f32 / per_f32).ceil() as usize;
			(width, pixel, rows, per as usize)
		}
	};

	ScrollArea::vertical().id_source("AlbumPixel").max_width(f32::INFINITY).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		// Sizing.
		let label_width   = (pixel / 10.0) as usize;
		let label_height  = pixel / 10.0;
		let label_padding = label_height / 3.0;

		// Start the row.
		for row in 0..rows {
			// Paint `Album`'s horizontally.
			ui.horizontal(|ui| {
				// Paint as many `Album`'s that can fit.
				for _ in 0..album_width {
					match iter.next() {
						Some(key) => {
							ui.vertical(|ui| {
								let album = &self.collection.albums[key];

								// ImageButton.
								let img_button = ImageButton::new(album.texture_id(ctx), egui::vec2(pixel, pixel));

								if ui.add(img_button).clicked() {
									self.state.album = Some(*key);
									self.state.tab   = Tab::View;
								}

								// Album title.
								ui.add_sized([pixel, label_height], Label::new(RichText::new(album.title.head_dot(10).as_str()).color(BONE)));
								// Artist name.
								let artist = &self.collection.artist_from_album(*key);
								ui.add_sized([pixel, label_padding], Label::new(artist.name.head_dot(label_width).as_str()));
								ui.add_space(label_padding);
							});
						},

						// We're at the end, no more `Album`'s left.
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
