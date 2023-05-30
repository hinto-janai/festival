//---------------------------------------------------------------------------------------------------- Use
use egui::{
	ScrollArea,Label,ComboBox,
	SelectableLabel,RichText,Sense,
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
		// FIXME:
		// The opened ComboBox must be big enough so
		// that the scrollbar does not appear.
		//
		// If the scrollbar appears, some logic makes the
		// width of the text conform the the column (which might be tiny).
		// In order to show the full width of text, this is used.
		//
//		ui.spacing_mut().combo_height = ui.available_height() / 2.0;
		// ^
		// |
		// -- This should have fixed the issue but....
		// https://github.com/emilk/egui/blob/7b76161a6a7e33a72e7331c1725758608c16ff30/crates/egui/src/containers/combo_box.rs#L341
		//
		// This line does not actually source the `combo_height` correctly.
		// It doesn't take from the parent `ui` and defaults to `200.0`.
		//
		// For now, use `ui.selectable_label()` instead of manually sizing.

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
			.header(80.0, |mut header|
		{
			use SongSort::*;

			// Title.
			header.col(|ui| {
				const SORT: [SongSort; 4] = [Lexi, LexiRev, Title, TitleRev];

				ComboBox::from_id_source("__song_sort_title")
					.selected_text(RichText::new("Title").color(WHITE))
					.width(ui.available_width() - 5.0)
					.show_ui(ui, |ui|
				{
					for i in SORT {
						if ui.selectable_label(self.settings.song_sort == i, i.as_str()).clicked() {
							self.settings.song_sort = i;
						}
					}
				});
			});

			// Album.
			header.col(|ui| {
				const SORT: [SongSort; 8] = [
					AlbumReleaseArtistLexi,
					AlbumReleaseArtistLexiRev,
					AlbumReleaseRevArtistLexi,
					AlbumReleaseRevArtistLexiRev,
					AlbumLexiArtistLexi,
					AlbumLexiArtistLexiRev,
					AlbumLexiRevArtistLexi,
					AlbumLexiRevArtistLexiRev,
				];

				ComboBox::from_id_source("__song_sort_album")
					.selected_text(RichText::new("Album").color(WHITE))
					.width(ui.available_width() - 5.0)
					.show_ui(ui, |ui|
				{
					for i in SORT {
						if ui.selectable_label(self.settings.song_sort == i, i.as_str()).clicked() {
							self.settings.song_sort = i;
						}
					}
				});
			});

			// Artist.
			header.col(|ui| {
				const SORT: [SongSort; 8] = [
					AlbumReleaseArtistLexi,
					AlbumReleaseArtistLexiRev,
					AlbumReleaseRevArtistLexi,
					AlbumReleaseRevArtistLexiRev,
					AlbumLexiArtistLexi,
					AlbumLexiArtistLexiRev,
					AlbumLexiRevArtistLexi,
					AlbumLexiRevArtistLexiRev,
				];

				ComboBox::from_id_source("__song_sort_artist")
					.selected_text(RichText::new("Artist").color(WHITE))
					.width(ui.available_width() - 5.0)
					.show_ui(ui, |ui|
				{
					for i in SORT {
						if ui.selectable_label(self.settings.song_sort == i, i.as_str()).clicked() {
							self.settings.song_sort = i;
						}
					}
				});
			});

			// Release.
			header.col(|ui| {
				const SORT: [SongSort; 2] = [Release, ReleaseRev];

				ComboBox::from_id_source("__song_sort_release")
					.selected_text(RichText::new("Release").color(WHITE))
					.width(ui.available_width() - 5.0)
					.show_ui(ui, |ui|
				{
					for i in SORT {
						if ui.selectable_label(self.settings.song_sort == i, i.as_str()).clicked() {
							self.settings.song_sort = i;
						}
					}
				});
			});

			// Runtime.
			header.col(|ui| {
				const SORT: [SongSort; 2] = [Runtime, RuntimeRev];

				ComboBox::from_id_source("__song_sort_runtime")
					.selected_text(RichText::new("Runtime").color(WHITE))
					.width(ui.available_width() - 5.0)
					.show_ui(ui, |ui|
				{
					for i in SORT {
						if ui.selectable_label(self.settings.song_sort == i, i.as_str()).clicked() {
							self.settings.song_sort = i;
						}
					}
				});
			});

			header.col(|ui| { ui.strong("Track"); });
			header.col(|ui| { ui.strong("Disc"); });
			header.col(|ui| { ui.strong("Path"); });
		})
		.body(|mut body| {
			// Song iterator.
			for key in self.collection.song_iter(self.settings.song_sort) {
				body.row(35.0, |mut row| {
					let (artist, album, song) = self.collection.walk(key);

					row.col(|ui| {
						if ui.add(Label::new(&song.title).sense(Sense::click())).clicked() {
							crate::song!(self, *key);
						}
					});

					row.col(|ui| {
						if ui.add(Label::new(&album.title).sense(Sense::click())).clicked() {
							crate::album!(self, song.album);
						}
					});

					row.col(|ui| {
						if ui.add(Label::new(&artist.name).sense(Sense::click())).clicked() {
							crate::artist!(self, album.artist);
						}
					});

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

						if ui.add(Label::new(&*song.path.to_string_lossy()).sense(Sense::click())).clicked() {
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
