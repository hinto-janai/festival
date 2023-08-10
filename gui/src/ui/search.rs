//---------------------------------------------------------------------------------------------------- Use
use egui::{
	CentralPanel,ScrollArea,
	TextEdit,Label,RichText,
	SelectableLabel,Sense,
};
use benri::{
	send,
};
use shukusai::kernel::{
	FrontendToKernel,
};
use crate::constants::{
	SEARCH_MAX_LEN,GRAY,
};
use crate::text::{
	SEARCH_BAR,
	SEARCH_HELP,
	SEARCH_SORT_SONG,
	SEARCH_SORT_ALBUM,
	SEARCH_SORT_ARTIST,
};
use log::debug;
use egui_extras::{
	TableBuilder,Column,
};
use crate::data::SearchSort;
use readable::Unsigned;
use log::warn;

//---------------------------------------------------------------------------------------------------- Search
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_search(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, width: f32, height: f32) {
CentralPanel::default().show(ctx, |ui| {
	self.set_visuals(ui);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();
	let half   = height / 2.0;
	let hh     = half / 2.0;
	let hhh    = hh / 3.0;
	let s      = 35.0; // Spinner.

	ui.horizontal(|ui| {
		// Search bar.
		let width = ui.available_width();
		let id = egui::Id::new("Search TextEdit");
		let text_edit = TextEdit::singleline(&mut self.state.search_string)
			.char_limit(SEARCH_MAX_LEN)
			.id(id);
		ui.spacing_mut().text_edit_width = width;
		let response = ui.add_sized([width, 35.0], text_edit).on_hover_text(SEARCH_BAR);

		// Check if we came from a different
		// tab and need to lock focus.
		if self.search_jump {
			// This forces the text cursor to move forward 1 character.
			if let Some(mut state) = egui::widgets::text_edit::TextEditState::load(ctx, id) {
				let cursor = egui::widgets::text_edit::CCursorRange {
					primary: epaint::text::cursor::CCursor {
							index: 1,
							..Default::default()
						},
					..Default::default()
				};

				state.set_ccursor_range(Some(cursor));
			}

			ctx.memory_mut(|m| m.request_focus(id));
		}

		// Only update if user input has changed
		// or we jumped from a different tab.
		if response.changed() || self.search_jump {
			self.search_jump = false;

			if self.state.search_string.len() > SEARCH_MAX_LEN {
				debug!("GUI - Search string is longer than {SEARCH_MAX_LEN}, truncating");
				self.state.search_string.truncate(SEARCH_MAX_LEN);
			} else {
				send!(self.to_kernel, FrontendToKernel::Search((self.state.search_string.clone(), self.settings.search_kind)));
				self.searching = true;
			}
		}
	});

	// If search input is empty, reset result, show help.
	if self.state.search_string.is_empty() {
		if !self.state.search_result.is_empty() {
			self.state.search_result = Default::default();
			self.searching = false;
		}

		let label = Label::new(RichText::new(SEARCH_HELP).color(GRAY));
		ui.add_sized([width, hh], label);
		ui.add_sized([width, hhh], Label::new(RichText::new(&self.count_artist).color(GRAY)));
		ui.add_sized([width, hhh], Label::new(RichText::new(&self.count_album).color(GRAY)));
		ui.add_sized([width, hhh], Label::new(RichText::new(&self.count_song).color(GRAY)));

		return;
	}

	// Else, show results.

	ui.add_space(10.0);

	//-------------------------------------------------- SearchSort.
	ui.group(|ui| { ui.horizontal(|ui| {
		let width = (width / 3.0) - 20.0;

		{
			const SORT: SearchSort = SearchSort::Song;
			let label = SelectableLabel::new(self.settings.search_sort == SORT, SORT.human());
			if ui.add_sized([width, 30.0], label).on_hover_text(SEARCH_SORT_SONG).clicked() {
				self.settings.search_sort = SORT;
			}
		}

		ui.separator();

		{
			const SORT: SearchSort = SearchSort::Album;
			let label = SelectableLabel::new(self.settings.search_sort == SORT, SORT.human());
			if ui.add_sized([width, 30.0], label).on_hover_text(SEARCH_SORT_ALBUM).clicked() {
				self.settings.search_sort = SORT;
			}
		}

		ui.separator();

		{
			const SORT: SearchSort = SearchSort::Artist;
			let label = SelectableLabel::new(self.settings.search_sort == SORT, SORT.human());
			if ui.add_sized([width, 30.0], label).on_hover_text(SEARCH_SORT_ARTIST).clicked() {
				self.settings.search_sort = SORT;
			}
		}
	})});

	ui.add_space(10.0);

	const HEADER_HEIGHT: f32 = 80.0;

	//-------------------------------------------------- Song table.
	match self.settings.search_sort {
		SearchSort::Song => {
			// `.show_rows()` is slightly faster than
			// `.show_viewport()` but we need to know
			// exactly how many rows we need to paint.
			//
			// The below needs to account for the scrollbar height,
			// the title heights and must not overflow to the bottom bar.
			const ROW_HEIGHT:    f32 = 35.0;
			let height     = ui.available_height();
			let max_rows   = ((height - (HEADER_HEIGHT - 5.0)) / (ROW_HEIGHT - 1.0)) as usize;
			let row_range  = 0..max_rows;

			ScrollArea::horizontal()
				.id_source("SearchSong")
				.max_width(f32::INFINITY)
				.max_height(f32::INFINITY)
				.auto_shrink([false; 2])
				.show_rows(ui, ROW_HEIGHT, max_rows, |ui, row_range|
			{ ui.push_id("SearchSongInner", |ui| {
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
					.header(HEADER_HEIGHT, |mut header|
				{
					header.col(|ui| { ui.strong("Title"); });
					header.col(|ui| { ui.strong("Album"); });
					header.col(|ui| { ui.strong("Artist"); });
					header.col(|ui| { ui.strong("Release"); });
					header.col(|ui| { ui.strong("Runtime"); });
					header.col(|ui| { ui.strong("Track"); });
					header.col(|ui| { ui.strong("Disc"); });
					header.col(|ui| { ui.strong("Path"); });
				})
				.body(|mut body| {
					for key in self.state.search_result.songs.iter() {
						body.row(ROW_HEIGHT, |mut row| {
							let (artist, album, song) = self.collection.walk(key);

							row.col(|ui| {
								if ui.add(Label::new(&*song.title).sense(Sense::click())).clicked() {
									crate::play_song!(self, *key);
								}
							});

							row.col(|ui| {
								if ui.add(Label::new(&*album.title).sense(Sense::click())).clicked() {
									crate::album!(self, song.album);
								}
							});

							row.col(|ui| {
								crate::artist_label!(self, artist, album.artist, ui, Label::new(&*artist.name));
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
							});
						});
					}
				});
			})});
		},

	//-------------------------------------------------- Album table.
		SearchSort::Album => {
			ScrollArea::horizontal()
				.id_source("SearchSortAlbum")
				.max_width(f32::INFINITY)
				.max_height(f32::INFINITY)
				.auto_shrink([false; 2])
				.show_viewport(ui, |ui, _|
			{ ui.push_id("SearchSortAlbumInner", |ui| {
				// Sizing.
				let width  = ui.available_width();
				let height = ui.available_height();
				// c == Column sizing
				let c_width   = (width / 10.0) - 10.0;
				let c_title   = c_width * 4.0;
				let c_artist  = c_width * 3.0;
				let c_release = c_width * 2.0;

				TableBuilder::new(ui)
					.striped(true)
					.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
					.column(Column::initial(c_title).resizable(true).clip(true))
					.column(Column::initial(c_artist).resizable(true).clip(true))
					.column(Column::initial(c_release).resizable(true).clip(true))
					.column(Column::remainder().clip(true))
					.auto_shrink([false; 2])
					.max_scroll_height(height)
					.header(HEADER_HEIGHT, |mut header|
				{
					header.col(|ui| { ui.strong("Album"); });
					header.col(|ui| { ui.strong("Artist"); });
					header.col(|ui| { ui.strong("Release"); });
					header.col(|ui| { ui.strong("Runtime"); });
				})
				.body(|mut body| {
					for key in self.state.search_result.albums.iter() {
						body.row(130.0, |mut row| {
							let album  = &self.collection.albums[key];
							let (artist, _) = self.collection.artist_from_album(key);

							row.col(|ui| {
								crate::no_rounding!(ui);
								crate::album_button!(self, album, *key, ui, ctx, 120.0, "");
								ui.label(&*album.title);
							});

							row.col(|ui| {
								crate::artist_label!(self, artist, album.artist, ui, Label::new(&*artist.name));
							});

							row.col(|ui| { ui.label(album.release.as_str()); });
							row.col(|ui| { ui.label(album.runtime.as_str()); });
						});
					}
				});
			})});
		},

		SearchSort::Artist => {
			ScrollArea::horizontal()
				.id_source("SearchSortArtist")
				.max_width(f32::INFINITY)
				.max_height(f32::INFINITY)
				.auto_shrink([false; 2])
				.show_viewport(ui, |ui, _|
			{ ui.push_id("SearchSortArtistInner", |ui| {
				// Sizing.
				let width  = ui.available_width();
				let height = ui.available_height();
				// c == Column sizing
				let c_artist  = width / 4.0;
				let c_runtime = width / 8.0;

				TableBuilder::new(ui)
					.striped(true)
					.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
					.column(Column::initial(c_artist).resizable(true).clip(true))
					.column(Column::initial(c_runtime).resizable(true).clip(true))
					.column(Column::remainder().clip(true))
					.auto_shrink([false; 2])
					.max_scroll_height(height)
					.header(HEADER_HEIGHT, |mut header|
				{
					header.col(|ui| { ui.strong("Artist"); });
					header.col(|ui| { ui.strong("Runtime"); });
					header.col(|ui| { ui.strong("Albums"); });
				})
				.body(|mut body| {
					for key in self.state.search_result.artists.iter() {

						body.row(130.0, |mut row| {
							let artist = &self.collection.artists[key];

							row.col(|ui| {
								crate::artist_label!(self, artist, *key, ui, Label::new(&*artist.name));
							});

							row.col(|ui| { ui.label(artist.runtime.as_str()); });

							row.col(|ui| {
								crate::no_rounding!(ui);
								for key in artist.albums.iter() {
									let album = &self.collection.albums[key];

									crate::album_button!(self, album, *key, ui, ctx, 120.0, &*album.title);
								}
							});
						});
					}
				});
			})});
		},
	} // End of match.
});
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
