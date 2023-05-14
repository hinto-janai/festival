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
use crate::data::{
	AlbumSizing,
	Tab,
};
use crate::slice::Head;
use crate::constants::{
	LESS_WHITE,
};
use shukusai::collection::{
	AlbumKey,
};
use std::slice::Iter;
use std::iter::Peekable;

//---------------------------------------------------------------------------------------------------- Constants
// How many `char`'s before we cut it off with `...`?
const ALBUM_TITLE_LIMIT: usize = 30;

//----------------------------------------------------------------------------------------------------
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_albums(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	// Make each `Album` separated by `10.0x10.0` pixels.
	ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

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

	let width = ui.available_width();
	match self.settings.album_sizing {
		AlbumSizing::Pixel => self.show_album_sizing_pixel(ui, ctx, frame, width, height),
		AlbumSizing::Row   => self.show_album_sizing_row(ui, ctx, frame, width, height),
	};
}

#[inline(always)]
//---------------------------------------------------------------------------------------------------- If `AlbumSizing::Pixel`
fn show_album_sizing_pixel(
	&mut self,
	ui: &mut egui::Ui,
	ctx: &egui::Context,
	frame: &mut eframe::Frame,
	width: f32,
	height: f32,
//	iter: Peekable<Iter<'_, AlbumKey>>,
) {
	// The iterator over sorted `Album`'s.
	let mut iter = self.collection
		.album_sort(self.settings.sort_order)
		.iter()
		.peekable();

	// Get pixel size.
	let pixel = self.settings.album_pixel_size;
	// How many `Album`'s can fit in one row?
	let album_width = (width / pixel).trunc();
	// Remainder (space between `Album`'s).
	let remainder = width - ((pixel * album_width) - album_width);
	let remainder = remainder / album_width;
	ui.spacing_mut().item_spacing.x += remainder;
	let first_album_padding = ui.spacing().item_spacing.x / 2.5;
	// Account for separation space and padding.
	let pixel = pixel - 15.0;
	// How many rows?
	let rows = (self.collection.count_album.inner() as f32 / album_width).ceil() as usize;

	let album_width = album_width as usize;

	ScrollArea::vertical().id_source("AlbumPixel").max_width(f32::INFINITY).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		// Sizing.
		let label_width = (pixel / 11.5) as usize;
		let padding     = pixel / 100.0;

		// Start the row.
		for row in 0..rows {
			// Paint `Album`'s horizontally.
			ui.horizontal(|ui| {
				ui.add_space(first_album_padding);

				// Paint as many `Album`'s that can fit.
				for i in 0..album_width {
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

								// `0.0` will cause the text to expand
								// the `ui` to however much space it needs.
								// Album title.
								ui.add_sized([pixel, 0.0], Label::new(RichText::new(album.title.head_dot(ALBUM_TITLE_LIMIT).as_str()).color(LESS_WHITE))).on_hover_text(&album.title);
								// Artist name.
								let artist = &self.collection.artist_from_album(*key);
								ui.add_sized([pixel, 0.0], Label::new(artist.name.head_dot(label_width).as_str())).on_hover_text(&artist.name);
								ui.add_space(padding);
							});
						},

						// We're at the end, no more `Album`'s left.
						None => break,
					};
				}
				ui.add_space(ui.available_width());
			});
		}
	});
}

#[inline(always)]
//---------------------------------------------------------------------------------------------------- If `AlbumSizing::Row`
fn show_album_sizing_row(
	&mut self,
	ui: &mut egui::Ui,
	ctx: &egui::Context,
	frame: &mut eframe::Frame,
	width: f32,
	height: f32,
//	iter: Peekable<Iter<'_, AlbumKey>>,
) {
	// The iterator over sorted `Album`'s.
	let mut iter = self.collection
		.album_sort(self.settings.sort_order)
		.iter()
		.peekable();

	// Get spacing.
	// Get per row.
	let album_width = self.settings.albums_per_row;
	// How many pixels per `Album`'s in one row? (rounded down)
	let per_f32 = album_width as f32;
	let pixel = width / per_f32;
	// Account for separation space and padding.
	let pixel = pixel - 15.0;
	// How many rows?
	let rows = (self.collection.count_album.inner() as f32 / per_f32).ceil() as usize;

	ScrollArea::vertical().id_source("AlbumRow").max_width(f32::INFINITY).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		// Sizing.
		let label_width = (pixel / 11.5) as usize;
		let padding     = pixel / 100.0;

		// Start the row.
		for row in 0..rows {
			// Paint `Album`'s horizontally.
			ui.horizontal(|ui| {
				// Paint as many `Album`'s that can fit.
				for i in 0..album_width {
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

								// `0.0` will cause the text to expand
								// the `ui` to however much space it needs.
								// Album title.
								ui.add_sized([pixel, 0.0], Label::new(RichText::new(album.title.head_dot(ALBUM_TITLE_LIMIT).as_str()).color(LESS_WHITE)));
								// Artist name.
								let artist = &self.collection.artist_from_album(*key);
								ui.add_sized([pixel, 0.0], Label::new(artist.name.head_dot(label_width).as_str()));
								ui.add_space(padding);
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