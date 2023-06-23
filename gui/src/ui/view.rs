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
use shukusai::{
	collection::AlbumKey,
	kernel::FrontendToKernel,
};
use crate::{
	constants::{
		BONE,GRAY,MEDIUM_GRAY,
		ALBUM_ART_SIZE_MIN,ALBUM_ART_SIZE_MAX,
	},
	text::{
		SELECT_ALBUM,
	},
};
use readable::HeadTail;
use log::warn;
use benri::send;

//---------------------------------------------------------------------------------------------------- Main central panel.
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_view(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, width: f32, height: f32) {
	self.set_visuals(ui);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();

	// Extract `AlbumKey`.
	let album_key = match self.state.album {
		Some(k) => k,

		// If no `AlbumKey` selected, show text.
		None => {
			let label = Label::new(RichText::new(SELECT_ALBUM).color(GRAY));
			ui.add_sized([width, height], label);

			return;
		}
	};

	let album = &self.collection.albums[album_key];

	ui.add_space(height/50.0);

	// `Album` art.
	ui.vertical_centered(|ui| {
		let size = (height / 2.5).clamp(ALBUM_ART_SIZE_MIN, ALBUM_ART_SIZE_MAX);
		let size = Vec2::new(size, size);
		ui.set_max_size(size);
		Frame::window(&ctx.style()).rounding(Rounding::none()).inner_margin(1.0).show(ui, |ui| {
			album.art_or().show_size(ui, size);
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
		let artist = &self.collection.artists[album.artist];
		crate::artist_label!(self, artist, album.artist, ui, Label::new(&artist.name));

		// `Album` release.
		ui.label(album.release.as_str());

		// `Album` runtime.
		ui.label(album.runtime.as_str());

		ui.add_space(8.0);
	});

	ui.separator();

	// `Song` list.
	ScrollArea::vertical()
		.id_source("view_song_list")
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		let mut last_disc = self.collection.songs[album.songs[0]].disc;
		if album.discs != 0 {
			ui.label(format!("Disc {}", last_disc.unwrap_or(0)));
			ui.separator();
		}

		for (offset, key) in album.songs.iter().enumerate() {
			let song = &self.collection.songs[key];

			// Add a separator if on a different disc.
			if album.discs != 0 {
				if song.disc != last_disc {
					ui.add_space(10.0);
					ui.label(format!("Disc {}", song.disc.unwrap_or(0)));
					ui.separator();
				}

				last_disc = song.disc;
			}

			crate::song_button!(self, self.audio_state.song == Some(*key), album, song, *key, ui, offset, None, None, 35.0, 0.0);
		}
	});
}}

//---------------------------------------------------------------------------------------------------- Right Panel
impl crate::data::Gui {
#[inline(always)]
pub(super) fn show_tab_view_right_panel(&mut self, album_key: Option<AlbumKey>, ctx: &egui::Context, width: f32, height: f32) {
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
		let (artist, artist_key) = self.collection.artist_from_album(album_key);
		let albums = artist.albums.iter();

		// How big the albums (on the right side) should be.
		let album_size = width / 1.4;

		// The scrollable area.
		ScrollArea::vertical()
			.id_source("view_right_panel")
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
					crate::album_button!(self, album, *key, ui, ctx, album_size, "");

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
