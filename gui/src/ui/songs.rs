//---------------------------------------------------------------------------------------------------- Use
use egui::{
	ScrollArea,Label,ComboBox,
	SelectableLabel,RichText,
};
use egui_extras::{
	StripBuilder,Size,
	TableBuilder,Column,
};
use readable::Unsigned;
use log::warn;
use crate::constants::{
	BONE,WHITE,GREEN,
};
use crate::text::{
	OPEN_PARENT_FOLDER,
};
use shukusai::sort::{
	SongSort,
};
use crate::data::Tab;

//---------------------------------------------------------------------------------------------------- Songs
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_songs(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	self.set_visuals(ui);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();
	// c == Column sizing
	let c_width   = (width / 10.0) - 10.0; // Account for separators, let `Path` peek a little.
	let c_title   = c_width * 2.5;
	let c_album   = c_width * 2.5;
	let c_artist  = c_width;
	let c_release = c_width;
	let c_runtime = c_width;
	let c_track   = c_width;
	let c_disc    = c_width;

	// Show Table.
	ScrollArea::horizontal()
		.id_source("Songs")
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		// Create Table.
		TableBuilder::new(ui)
			.striped(true)
			.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
			.column(Column::initial(c_title).resizable(true).clip(true))
			.column(Column::initial(c_album).resizable(true).clip(true))
			.column(Column::initial(c_artist).resizable(true).clip(true))
			.column(Column::initial(c_release).resizable(true).clip(true))
			.column(Column::initial(c_runtime).resizable(true).clip(true))
			.column(Column::initial(c_track).resizable(true).clip(true))
			.column(Column::initial(c_disc).resizable(true).clip(true))
			.column(Column::remainder().clip(true))
			.auto_shrink([false; 2])
			.max_scroll_height(height)
			.header(60.0, |mut header|
		{
			let w = width / 1.1;
			let h = 30.0;

			use SongSort::*;

			// Title.
			header.col(|ui| {
				const SORT: [SongSort; 4] = [Lexi, LexiRev, Title, TitleRev];

				ui.spacing_mut().combo_width = ui.available_width() - 5.0;
				ComboBox::from_id_source("__song_sort_title")
					.selected_text(RichText::new("Title").color(WHITE))
					.show_ui(ui, |ui|
				{
					for i in SORT {
						let label = SelectableLabel::new(self.settings.song_sort == i, i.as_str());
						if ui.add_sized([w, h], label).clicked() { self.settings.song_sort = i; }
					}
				});
			});

			// Album.
			header.col(|ui| {
				const SORT: [SongSort; 4] = [
					AlbumReleaseArtistLexi,
					AlbumReleaseRevArtistLexi,
					AlbumLexiArtistLexi,
					AlbumLexiRevArtistLexi,
				];

				ui.spacing_mut().combo_width = ui.available_width() - 5.0;
				ComboBox::from_id_source("__song_sort_album")
					.selected_text(RichText::new("Album").color(WHITE))
					.show_ui(ui, |ui|
				{
					for i in SORT {
						let label = SelectableLabel::new(self.settings.song_sort == i, i.as_str());
						if ui.add_sized([w, h], label).clicked() { self.settings.song_sort = i; }
					}
				});
			});

			// Artist.
			header.col(|ui| {
				const SORT: [SongSort; 6] = [
					AlbumReleaseArtistLexi,
					AlbumReleaseArtistLexiRev,
					AlbumLexiArtistLexi,
					AlbumLexiArtistLexiRev,
					AlbumLexiRevArtistLexi,
					AlbumLexiRevArtistLexiRev,
				];

				ui.spacing_mut().combo_width = ui.available_width() - 5.0;
				ComboBox::from_id_source("__song_sort_artist")
					.selected_text(RichText::new("Artist").color(WHITE))
					.show_ui(ui, |ui|
				{
					for i in SORT {
						let label = SelectableLabel::new(self.settings.song_sort == i, i.as_str());
						if ui.add_sized([w, h], label).clicked() { self.settings.song_sort = i; }
					}
				});
			});

			// Release.
			header.col(|ui| {
				const SORT: [SongSort; 2] = [Release, ReleaseRev];

				ui.spacing_mut().combo_width = ui.available_width() - 5.0;
				ComboBox::from_id_source("__song_sort_release")
					.selected_text(RichText::new("Release").color(WHITE))
					.show_ui(ui, |ui|
				{
					for i in SORT {
						let label = SelectableLabel::new(self.settings.song_sort == i, i.as_str());
						if ui.add_sized([w, h], label).clicked() { self.settings.song_sort = i; }
					}
				});
			});

			// Runtime.
			header.col(|ui| {
				const SORT: [SongSort; 2] = [Runtime, RuntimeRev];

				ui.spacing_mut().combo_width = ui.available_width() - 5.0;
				ComboBox::from_id_source("__song_sort_runtime")
					.selected_text(RichText::new("Runtime").color(WHITE))
					.show_ui(ui, |ui|
				{
					for i in SORT {
						let label = SelectableLabel::new(self.settings.song_sort == i, i.as_str());
						if ui.add_sized([w, h], label).clicked() { self.settings.song_sort = i; }
					}
				});
			});

			// Track.
			header.col(|ui| {
				ui.strong("Track");
			});

			// Disc.
			header.col(|ui| {
				ui.strong("Disc");
			});

			// Path.
			header.col(|ui| {
				ui.strong("Path");
			});
		})
		.body(|mut body| {
			// Song iterator.
			for key in self.collection.song_iter(self.settings.song_sort) {
				body.row(35.0, |mut row| {
					let (artist, album, song) = self.collection.walk(key);

					row.col(|ui| { ui.label(&song.title); });

					row.col(|ui| {
						if ui.button(" View ").clicked() {
							self.state.album = Some(song.album);
							self.state.tab   = Tab::View;
						}

						ui.label(&album.title);
					});

					row.col(|ui| { ui.label(&artist.name); });
					row.col(|ui| { ui.label(album.release.as_str()); });
					row.col(|ui| { ui.label(song.runtime.as_str()); });

					match song.track {
						Some(t) => row.col(|ui| { ui.label(Unsigned::from(t).as_str()); }),
						None    => row.col(|ui| { ui.label("???"); }),
					};
					match song.disc {
						Some(d) => row.col(|ui| { ui.label(Unsigned::from(d).as_str()); }),
						None    => row.col(|ui| { ui.label("???"); }),
					};

					row.col(|ui| {
						ui.add_space(5.0);

						if ui.button(" 🗁 Open ").on_hover_text(OPEN_PARENT_FOLDER).clicked() {
							match &song.path.parent() {
								Some(p) => {
									if let Err(e) = open::that(p) {
										warn!("GUI - Could not open path: {e}");
									}
								}
								None => warn!("GUI - Could not get parent path: {}", song.path.display()),
							}
						}

						ui.label(&*song.path.to_string_lossy());
					});
				});
			}
		});
	});
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
