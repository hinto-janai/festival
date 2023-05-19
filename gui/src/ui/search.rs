//---------------------------------------------------------------------------------------------------- Use
use egui::{
	CentralPanel,ScrollArea,TextStyle,
	TextEdit,Label,RichText,Spinner,
	SelectableLabel,Sense,
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
	SEARCH_MAX_LEN
};
use crate::text::{
	SEARCH_MAX,
	SEARCH_BAR,
	SEARCH_HELP,
	SEARCH_EMPTY_COLLECTION,
	SEARCH_SORT_SONG,
	SEARCH_SORT_ALBUM,
	SEARCH_SORT_ARTIST,
};
use log::debug;
use egui_extras::{
	StripBuilder,Size,
	TableBuilder,Column,
};
use crate::data::{
	Tab,
	SearchSort,
};
use readable::Unsigned;
use crate::text::{
	OPEN_PARENT_FOLDER,
};
use log::warn;

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
	let hhh    = hh / 3.0;
	let s      = 35.0; // Spinner.

	ui.horizontal(|ui| {
		// If searching, show spinner.
		// 2022-05-18:
		// The search is so fast it actually
		// looks kinda buggy. So, maybe don't.
//		if self.searching {
//			ui.add_sized([s, s], Spinner::new().size(s));
//		} else if self.state.search_string.len() >= SEARCH_MAX_LEN {
//			ui.add_sized([s, s], Label::new(RichText::new("❌").color(RED))).on_hover_text(SEARCH_MAX);
//		} else if !self.state.search_result.is_empty() {
//			ui.add_sized([s, s], Label::new(RichText::new("✔").color(GREEN)));
//		} else {
//			ui.add_sized([s, s], Label::new(RichText::new("➖").color(WHITE)));
//		}

		// Search bar.
		let width = ui.available_width();
		let id = egui::Id::new("Search TextEdit");
		let text_edit = TextEdit::singleline(&mut self.state.search_string).id(id);
		ui.spacing_mut().text_edit_width = width;
		let response = ui.add_sized([width, 35.0], text_edit).on_hover_text(SEARCH_BAR);

		// Check if we came from a different
		// tab and need to lock focus.
		if self.search_jump {
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
				send!(self.to_kernel, FrontendToKernel::SearchSim(self.state.search_string.clone()));
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
			let label = SelectableLabel::new(self.settings.search_sort == SORT, SORT.as_str());
			if ui.add_sized([width, 30.0], label).on_hover_text(SEARCH_SORT_SONG).clicked() {
				self.settings.search_sort = SORT;
			}
		}

		ui.separator();

		{
			const SORT: SearchSort = SearchSort::Album;
			let label = SelectableLabel::new(self.settings.search_sort == SORT, SORT.as_str());
			if ui.add_sized([width, 30.0], label).on_hover_text(SEARCH_SORT_ALBUM).clicked() {
				self.settings.search_sort = SORT;
			}
		}

		ui.separator();

		{
			const SORT: SearchSort = SearchSort::Artist;
			let label = SelectableLabel::new(self.settings.search_sort == SORT, SORT.as_str());
			if ui.add_sized([width, 30.0], label).on_hover_text(SEARCH_SORT_ARTIST).clicked() {
				self.settings.search_sort = SORT;
			}
		}
	})});

	ui.add_space(10.0);

	//-------------------------------------------------- Song table.
	match self.settings.search_sort {
		SearchSort::Song => {
			ScrollArea::horizontal()
				.id_source("SearchSong")
				.max_width(f32::INFINITY)
				.max_height(f32::INFINITY)
				.auto_shrink([false; 2])
				.show_viewport(ui, |ui, _|
			{
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
					.header(40.0, |mut header|
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
						body.row(35.0, |mut row| {
							let (artist, album, song) = self.collection.walk(key);

							row.col(|ui| { ui.label(&song.title); });

							row.col(|ui| {
								if ui.add(Label::new(&album.title).sense(Sense::click())).clicked() {
									crate::album!(self, song.album);
								}
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
			});
		},

	//-------------------------------------------------- Album table.
		SearchSort::Album => {
			ScrollArea::horizontal()
				.id_source("SearchSortAlbum")
				.max_width(f32::INFINITY)
				.max_height(f32::INFINITY)
				.auto_shrink([false; 2])
				.show_viewport(ui, |ui, _|
			{
				// Sizing.
				let width  = ui.available_width();
				let height = ui.available_height();
				// c == Column sizing
				let c_width   = (width / 10.0) - 10.0;
				let c_title   = c_width * 4.0;
				let c_artist  = c_width * 3.0;
				let c_release = c_width * 2.0;

				crate::no_rounding!(ui);

				TableBuilder::new(ui)
					.striped(true)
					.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
					.column(Column::initial(c_title).resizable(true).clip(true))
					.column(Column::initial(c_artist).resizable(true).clip(true))
					.column(Column::initial(c_release).resizable(true).clip(true))
					.column(Column::remainder().clip(true))
					.auto_shrink([false; 2])
					.max_scroll_height(height)
					.header(80.0, |mut header|
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
							let artist = self.collection.artist_from_album(key);

							row.col(|ui| {
								crate::album_button!(self, album, key, ui, ctx, 120.0);
								ui.label(&album.title);
							});

							row.col(|ui| { ui.label(&artist.name); });
							row.col(|ui| { ui.label(album.release.as_str()); });
							row.col(|ui| { ui.label(album.runtime.as_str()); });
						});
					}
				});
			});
		},

		SearchSort::Artist => {
			ScrollArea::horizontal()
				.id_source("SearchSortArtist")
				.max_width(f32::INFINITY)
				.max_height(f32::INFINITY)
				.auto_shrink([false; 2])
				.show_viewport(ui, |ui, _|
			{
				// Sizing.
				let width  = ui.available_width();
				let height = ui.available_height();
				// c == Column sizing
				let c_artist  = width / 5.0;
				let c_runtime = width / 8.0;

				crate::no_rounding!(ui);

				TableBuilder::new(ui)
					.striped(true)
					.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
					.column(Column::initial(c_artist).resizable(true).clip(true))
					.column(Column::initial(c_runtime).resizable(true).clip(true))
					.column(Column::remainder().clip(true))
					.auto_shrink([false; 2])
					.max_scroll_height(height)
					.header(80.0, |mut header|
				{
					header.col(|ui| { ui.strong("Artist"); });
					header.col(|ui| { ui.strong("Runtime"); });
					header.col(|ui| { ui.strong("Albums"); });
				})
				.body(|mut body| {
					for key in self.state.search_result.artists.iter() {

						body.row(130.0, |mut row| {
							let artist = &self.collection.artists[key];

							row.col(|ui| { ui.label(&artist.name); });
							row.col(|ui| { ui.label(artist.runtime.as_str()); });

							row.col(|ui| {
								for key in artist.albums.iter() {
									let album = &self.collection.albums[key];

									crate::album_button!(self, album, key, ui, ctx, 120.0, album.title);
								}
							});
						});
					}
				});
			});
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
