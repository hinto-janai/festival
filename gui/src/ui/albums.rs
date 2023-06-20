//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use egui::{
	Rounding,Vec2,Color32,Stroke,
	ScrollArea,Frame,RichText,
	SelectableLabel,Label,Button,
	ImageButton,TextStyle,Sense,
};
use crate::data::{
	AlbumSizing,
	Tab,
};
use readable::HeadTail;
use crate::constants::{
	LESS_WHITE,BONE,
};
use crate::text::{
	EMPTY_COLLECTION,
};
use shukusai::collection::{
	AlbumKey,
};
use std::slice::Iter;
use std::iter::Peekable;
use log::warn;

//---------------------------------------------------------------------------------------------------- Constants
// How many `char`'s before we cut it off with `...`?
const ALBUM_TITLE_LIMIT: usize = 30;

//----------------------------------------------------------------------------------------------------
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_albums(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, width: f32, height: f32) {
	// Album Spacing "Algorithm"
	//
	// If `AlbumSizing::Pixel` is selected, the below code will dynamically
	// fit as many albums that can fit into the available width given the
	// static pixel size.
	//
	// When stretching the GUI's windows left <-> right, new albums will
	// be added/removed with a nice even amount of space between them all.
	//
	// Some things to care for:
	//   - Space added by separators
	//   - Space added by padding
	//   - Space added by scrollbar
	//   - Evenly spreading the remainder space
	//
	// I say "algorithm" because the way the below code was
	// created was by me spending a day incrementing/decrementing
	// the variables until it looked nice.

	// Make each `Album` separated by `10.0 x 10.0` pixels.
	ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

	// Small font.
	ui.style_mut().override_text_style = Some(TextStyle::Name("15".into()));

	let width = ui.available_width();

	match self.settings.album_sizing {
		AlbumSizing::Pixel => {
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
			let pixel = pixel - 16.0;
			// How many rows?
			let rows = (self.collection.count_album.inner() as f32 / album_width).ceil() as usize;

			let album_width = album_width as usize;

			self.paint_albums(ui, ctx, width, height, pixel, rows, album_width, first_album_padding);
		},
		AlbumSizing::Row   => {
			// Get per row count.
			let album_width = self.settings.albums_per_row as usize;
			// How many pixels per `Album`'s in one row? (rounded down)
			let per_f32 = album_width as f32;
			let pixel = width / per_f32;
			// Account for separation space and padding.
			let pixel = pixel - 16.0;
			// How many rows?
			let rows = (self.collection.count_album.inner() as f32 / per_f32).ceil() as usize;

			self.paint_albums(ui, ctx, width, height, pixel, rows, album_width, 0.0);
		},
	};
}

#[inline(always)]
//---------------------------------------------------------------------------------------------------- Paints either `Pixel` or `Row`.
fn paint_albums(
	&mut self,
	ui: &mut egui::Ui,
	ctx: &egui::Context,
	width: f32,
	height: f32,
	pixel: f32,
	rows: usize,
	album_width: usize,
	first_album_padding: f32,
) {
	// The iterator over sorted `Album`'s.
	let mut iter = self.collection
		.album_iter(self.settings.album_sort)
		.peekable();

	ScrollArea::vertical()
		.id_source("Albums")
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		// Sizing.
		let label_width = (pixel / 11.5) as usize;
		let padding     = pixel / 100.0;

		crate::no_rounding!(ui);

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
								crate::album_button!(self, album, *key, ui, ctx, pixel, "");

								// `0.0` will cause the text to expand
								// the `ui` to however much space it needs.
								// Album title.
								ui.add_sized([pixel, 0.0], Label::new(RichText::new(album.title.head_dot(ALBUM_TITLE_LIMIT).as_str()).color(LESS_WHITE))).on_hover_text(&album.title);

								// Artist name.
								let (artist, _) = &self.collection.artist_from_album(key);
								let artist_name = Label::new(artist.name.head_dot(label_width).as_str()).sense(Sense::click());
								// We don't use `crate::artist_label!()` here
								// because we need a custom `ui.add_sized()`
								let resp = ui.add_sized([pixel, 0.0], artist_name).on_hover_text(&artist.name);
								if resp.clicked() {
									crate::artist!(self, album.artist);
								} else if resp.secondary_clicked() {
									crate::add_artist!(self, artist, album.artist);
								}

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
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
