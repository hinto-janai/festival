//---------------------------------------------------------------------------------------------------- Use
use egui::{
	ScrollArea,TextStyle,ImageButton,
	TextEdit,Label,RichText,Spinner,
};
use crate::constants::{
	BONE,MEDIUM_GRAY,
};
use crate::data::{
	Tab,
};
use readable::Unsigned;
use log::warn;

//---------------------------------------------------------------------------------------------------- Artists
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_artists(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	self.set_visuals(ui);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();

	{
		// Reduce rounding corners.
		let widgets = &mut ui.visuals_mut().widgets;
		widgets.hovered.rounding  = egui::Rounding::none();
		widgets.inactive.rounding = egui::Rounding::none();
		widgets.active.rounding   = egui::Rounding::none();
		// Reduced padding.
		ui.spacing_mut().button_padding.x -= 2.0;
	}

	// Outer scroll.
	ScrollArea::vertical().id_source("Artist").max_width(f32::INFINITY).max_height(height).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		// For each `Artist`...
		for key in self.collection.artist_iter(self.settings.artist_sort) {
			let artist = &self.collection.artists[key];

			// `Artist` name.
			let label_name = Label::new(
				RichText::new(&artist.name)
				.color(BONE)
				.text_style(TextStyle::Name("30".into()))
			);

			// `Artist` album count.
			let label_count = Label::new(
				RichText::new(Unsigned::from(artist.albums.len()).as_str())
				.color(MEDIUM_GRAY)
				.text_style(TextStyle::Name("30".into()))
			);

			ui.horizontal(|ui| {
				ui.add(label_name);
				ui.add_space(25.0);
				ui.add(label_count);
			});

			ui.add_space(10.0);

			// Their `Album`'s.
			ScrollArea::horizontal().id_source(key).max_width(f32::INFINITY).max_height(120.0).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
				ui.horizontal(|ui| {
					// Album
					for key in &artist.albums {
						let album = &self.collection.albums[key];

						crate::album_button!(self, album, key, ui, ctx, 120.0);
					}
				});
			});

			ui.add_space(10.0);
			ui.separator();
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
