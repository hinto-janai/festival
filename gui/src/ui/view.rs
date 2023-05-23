//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use egui::{
	Rounding,Vec2,Color32,Stroke,TextStyle,
	ScrollArea,Frame,RichText,ImageButton,
	SelectableLabel,Label,Button,SidePanel,
	Sense,
};
use shukusai::collection::{
	AlbumKey,
};
use crate::constants::{
	BONE,GRAY,MEDIUM_GRAY,
};
use readable::HeadTail;
use log::warn;

//---------------------------------------------------------------------------------------------------- Main central panel.
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_view(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	self.set_visuals(ui);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();

	// Extract `AlbumKey`.
	let album_key = match self.state.album {
		Some(k) => k,

		// If no `AlbumKey` selected, show text.
		None => {
			let label = Label::new(RichText::new("🗋 Select an album in the [Album] tab").color(GRAY));
			ui.add_sized([width, height], label);

			return;
		}
	};

	let album = &self.collection.albums[album_key];

	ui.add_space(height/50.0);

	// `Album` art.
	ui.vertical_centered(|ui| {
		ui.set_max_width(height/3.0);
		Frame::window(&ctx.style()).rounding(Rounding::none()).inner_margin(1.0).show(ui, |ui| {
			// += `0.02` allows 7 songs to perfectly fit
			// without a scrollbar on the min resolution.
			let size = height / 2.52;
			album.art_or().show_size(ui, Vec2::new(size, size));
		});
	});

	// `Artist/Album` info.
	ui.vertical_centered(|ui| {
		ui.set_max_width(width);
		ui.add_space(8.0);

		// `Album` title.
		let label = Label::new(
			RichText::new(&album.title)
				.color(BONE)
				.text_style(TextStyle::Name("25".into()))
		);
		ui.add(label);

		// `Artist` name.
		let artist_name = Label::new(&self.collection.artists[album.artist].name);
		if ui.add(artist_name.sense(Sense::click())).clicked() {
			crate::artist!(self, album.artist);
		}

		// `Album` release.
		ui.label(album.release.as_str());

		// `Album` runtime.
		ui.label(album.runtime.as_str());

		ui.add_space(8.0);
	});

	ui.separator();

	// `Song` list.
	ScrollArea::vertical()
		.id_source(album_key)
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		// How many char's before we need
		// to cut off the song title?
		// (scales based on pixels available).
		let head = (width / 22.0) as usize;

		let mut last_disc = self.collection.songs[album.songs[0]].disc;
		if album.discs > 1 {
			ui.label(format!("Disc {}", last_disc.unwrap_or(0)));
			ui.separator();
		}

		for key in album.songs.iter() {
			let song = &self.collection.songs[key];

			// Add a separator if on a different disc.
			if album.discs > 1 {
				if song.disc != last_disc {
					ui.add_space(10.0);
					ui.label(format!("Disc {}", last_disc.unwrap_or(0)));
					ui.separator();
				}

				last_disc = song.disc;
			}

			let mut rect = ui.cursor();
			rect.max.y = rect.min.y + 35.0;
			if ui.put(rect, SelectableLabel::new(false, "")).clicked() {
			// TODO: Implement song key state.
//			if ui.put(rect, SelectableLabel::new(self.state.audio.current_key.song() == Some(key), "")).clicked() {
//				self.state.audio.current_key = Some(key);
			}
			rect.max.x = rect.min.x;

			ui.allocate_ui_at_rect(rect, |ui| {
				ui.horizontal_centered(|ui| {

					// Show the full title on hover
					// if we chopped it with head.
					let head = song.title.head_dot(head);
					if song.title == head {
						ui.add(Label::new(format!("{: >3}    {: >8}    {}", song.track.unwrap_or(0), &song.runtime, &song.title)));
					} else {
						ui.add(Label::new(format!("{: >3}    {: >8}    {}", song.track.unwrap_or(0), &song.runtime, &head))).on_hover_text(&song.title);
					}
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
		ScrollArea::vertical()
			.max_width(width)
			.max_height(f32::INFINITY)
			.auto_shrink([false; 2])
			.show_viewport(ui, |ui, _|
		{
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
				for key in albums {
					// Get the actual `Album`.
					let album = &self.collection.albums[key];

					// Album button.
					crate::album_button!(self, album, key, ui, ctx, album_size);

					// If this is the album we're on, make it pop.
					if key == album_key {
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
