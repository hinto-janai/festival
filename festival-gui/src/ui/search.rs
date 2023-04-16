//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use egui::{
	CentralPanel,ScrollArea,TextStyle,
	TextEdit,Label,RichText,Spinner,
};
use benri::{
	send,
};
use shukusai::kernel::{
	FrontendToKernel,
};
use crate::constants::{
	BONE,
	GRAY,
	WHITE,
	GREEN,
	RED,
};

//---------------------------------------------------------------------------------------------------- Search
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_search(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
CentralPanel::default().show(ctx, |ui| {
	self.set_visuals(ui);


	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();
	let half   = height / 2.0;
	let hh     = half / 2.0;
	let hhh    = hh / 2.0;
	let s      = 35.0; // Spinner.

	ui.horizontal(|ui| {
		// If searching, show spinner.
		if self.searching {
			ui.add_sized([s, s], Spinner::new().size(s));
		} else if !self.search_result.is_empty() {
			ui.add_sized([s, s], Label::new(RichText::new("✔").color(GREEN)));
		} else {
			ui.add_sized([s, s], Label::new(RichText::new("➖").color(WHITE)));
		}

		// Search bar.
		let width = ui.available_width();
		let text_edit = TextEdit::singleline(&mut self.search_string);
		ui.spacing_mut().text_edit_width = width;
		let response = ui.add_sized([width, 35.0], text_edit);

		// Only update if user input has changed.
		if response.changed() {
			// Else, send.
			send!(self.to_kernel, FrontendToKernel::SearchSim(self.search_string.clone()));
			self.searching = true;
		}
	});

	// Return if the `Collection` is empty.
	if self.collection.empty {
		let label = Label::new(RichText::new("The Collection is empty. There is nothing to search.").color(GRAY));
		ui.add_sized([width, half], label);

		return;
	}

	// If search input is empty, reset result, show guide.
	if self.search_string.is_empty() {
		if !self.search_result.is_empty() {
			self.search_result.clear();
			self.searching = false;
		}

		let label = Label::new(RichText::new("🔍 Search for albums, artists, and songs.").color(GRAY));
		ui.add_sized([width, half], label);

		return;
	}

	// Else, show results.

	// Album.
	ui.separator();
	let label = Label::new(
		RichText::new("Albums")
		.color(BONE)
		.text_style(TextStyle::Heading)
	);
	ui.add_sized([width, hhh], label).on_hover_text("TODO");
	ScrollArea::horizontal().id_source("SearchAlbum").max_width(f32::INFINITY).max_height(100.0).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		ui.horizontal(|ui| {
			for album in &self.search_result.albums {
				self.collection.albums[album].art_or().show_size(ui, egui::vec2(100.0, 100.0));
			}
		});
	});

	// Artists.
	ui.separator();
	let label = Label::new(
		RichText::new("Artists")
		.color(BONE)
		.text_style(TextStyle::Heading)
	);
	ui.add_sized([width, hhh], label).on_hover_text("TODO");
	ScrollArea::both().id_source("SearchArtist").max_width(f32::INFINITY).max_height(110.0).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		for artist in &self.search_result.artists {
			ui.horizontal(|ui| {
				let artist = &self.collection.artists[artist];
				ui.add_sized([100.0, 100.0], Label::new(&artist.name));
				for album in &artist.albums {
					self.collection.albums[album].art_or().show_size(ui, egui::vec2(100.0, 100.0));
				}
			});
		}
	});

	// Songs.
	ui.separator();
	let label = Label::new(
		RichText::new("Songs")
		.color(BONE)
		.text_style(TextStyle::Heading)
	);
	ui.add_sized([width, hhh], label).on_hover_text("TODO");
	ScrollArea::vertical().id_source("SearchSong").max_width(f32::INFINITY).max_height(ui.available_width()).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
		for song in &self.search_result.songs {
			ui.horizontal(|ui| {
				let song = &self.collection.songs[song];
				ui.add_sized([100.0, 100.0], Label::new(&song.title));
//				for album in &artist.albums {
//					self.collection.albums[album].art_or().show_size(ui, egui::vec2(100.0, 100.0));
//				}
			});
		}
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
