//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use egui::{
	Rounding,Vec2,Color32,Stroke,
	ScrollArea,Frame,RichText,ImageButton,
	SelectableLabel,Label,Button,SidePanel,
};
use shukusai::collection::{
	AlbumKey,
};
use crate::constants::{
	GRAY,
};

//---------------------------------------------------------------------------------------------------- Main central panel.
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_view(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	// Extract `AlbumKey`.
	let album_key = match self.state.album {
		Some(k) => k,

		// If no `AlbumKey` selected, show text.
		None => {
			let label = Label::new(RichText::new("🗋 Select an album in the [Album] tab").color(GRAY));
			ui.add_sized(ui.available_size(), label);

			return;
		}
	};

	let album = &self.collection.albums[album_key];

	ui.add_space(height/50.0);

	ui.vertical_centered(|ui| {
		ui.set_max_width(height/3.0);
		Frame::window(&ctx.style()).rounding(Rounding::none()).inner_margin(1.0).show(ui, |ui| {
			let size = height / 2.5;
			album.art_or().show_size(ui, Vec2::new(size, size));
		});
	});
	ui.vertical_centered(|ui| {
		ui.set_max_width(ui.available_width());
		ui.add_space(8.0);
		ui.heading(&album.title);
		ui.label(&self.collection.artists[album.artist].name);
		ui.label(album.release.as_str());
		ui.add_space(8.0);
	});

	ui.separator();
	ui.visuals_mut().selection.bg_fill = Color32::from_rgb(200, 100, 100);
	ui.visuals_mut().selection.stroke = Stroke { width: 5.0, color: Color32::from_rgb(255, 255, 255) };
	ScrollArea::vertical().max_width(f32::INFINITY).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
	for key in &self.collection.artists[album.artist].albums {
		let mut rect = ui.cursor();
		rect.max.y = rect.min.y + 35.0;
		if ui.put(rect, SelectableLabel::new(*key == self.state.album.unwrap(), "")).clicked() {
			self.state.album = Some(*key);
		}
		rect.max.x = rect.min.x;
		ui.allocate_ui_at_rect(rect, |ui| {
			ui.horizontal_centered(|ui| {
//				ui.add(Label::new(format!("{: >6}{: >10}      {}", i, time, name)).wrap(false));
			});
		});
	}
	});
}}

//---------------------------------------------------------------------------------------------------- Right Panel
impl crate::data::Gui {
#[inline(always)]
pub(super) fn show_tab_view_right_panel(&mut self, album_key: Option<AlbumKey>, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	let album_key = match album_key {
		Some(k) => k,
		None    => return,
	};

	SidePanel::right("right").resizable(false).show(ctx, |ui| {
		self.set_visuals(ui);
		ui.set_width(width);

		// The scrollable section to the RIGHT side of an album.
		//
		// We only show this if the user has an album selected.
		// We must:
		// - Find the artist of this album
		// - Iterate over all the albums of that artist
		// - Make the album we're on pop out
		let artist = self.collection.artist_from_album(album_key);
		let albums = artist.albums.iter();

		// How big the albums (on the right side) should be.
		let album_size = width / 1.4;

		// The scrollable area.
		ScrollArea::vertical().max_width(width).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
			ui.vertical_centered(|ui| {

			ui.add_space(5.0);

			{
				// Reduce rounding corners.
				let widgets = &mut ui.visuals_mut().widgets;
				widgets.hovered.rounding  = egui::Rounding::none();
				widgets.inactive.rounding = egui::Rounding::none();
				widgets.active.rounding   = egui::Rounding::none();
				// Reduced padding.
				ui.spacing_mut().button_padding.x -= 2.0;
			}

			// For each album...
			for album in albums {
				// Get the actual `Album`.
				let key   = album;
				let album = &self.collection.albums[key];

				// Draw the art with the title.
				let img_button = ImageButton::new(self.collection.albums[key].texture_id(ctx), egui::vec2(album_size, album_size));

				if ui.add(img_button).clicked() {
					self.state.album = Some(*key);
				}

				// If this is the album we're on, make it pop.
				if *key == album_key {
					ui.add(Label::new(RichText::new(&album.title).color(Color32::LIGHT_BLUE)));
				} else {
					ui.label(&album.title);
				}
				ui.add_space(5.0);
			}

			});
		});
	});
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
