//---------------------------------------------------------------------------------------------------- Use
use egui::{
	ScrollArea,TextStyle,Label,RichText,
	SelectableLabel,Sense,Button,TextEdit,
};
use crate::{
	constants::{
		BONE,MEDIUM_GRAY,GRAY,
		PLAYLIST_NAME_MAX_LEN,
	},
	text::SELECT_ARTIST,
	data::PlaylistSubTab,
};
use readable::{
	Unsigned,
	Runtime,
};
use log::warn;
use readable::HeadTail;
use shukusai::state::{
	Entry,
};
use egui_extras::{
	Column,TableBuilder,
};

//---------------------------------------------------------------------------------------------------- Artists
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_playlists(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, width: f32, height: f32) {
	self.set_visuals(ui);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();

	//-------------------------------------------------- Artist sub-tab.
	ui.group(|ui| { ui.horizontal(|ui| {
		let width = (width / 2.0) - 20.0;

		{
			const TAB: PlaylistSubTab = PlaylistSubTab::All;
			let label = SelectableLabel::new(self.settings.playlist_sub_tab == TAB, TAB.human());
			if ui.add_sized([width, 30.0], label).clicked() {
				self.settings.playlist_sub_tab = TAB;
			}
		}

		ui.separator();

		{
			const TAB: PlaylistSubTab = PlaylistSubTab::View;
			let label = match self.state.artist {
				Some(key) => {
					let name = self.collection.artists[key].name.head_dot(18);
					SelectableLabel::new(self.settings.playlist_sub_tab == TAB, name)
				},
				None => SelectableLabel::new(self.settings.playlist_sub_tab == TAB, TAB.human())
			};

			if ui.add_sized([width, 30.0], label).clicked() {
				self.settings.playlist_sub_tab = TAB;
			}
		}
	})});

	ui.add_space(10.0);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();

	//-------------------------------------------------- Acquire playlist lock.
	let mut playlists = shukusai::state::PLAYLISTS.write();

	//-------------------------------------------------- All artists
	match self.settings.playlist_sub_tab {
	PlaylistSubTab::All => {
		ScrollArea::vertical()
			.id_source("Playlists")
			.max_width(width)
			.max_height(height)
			.auto_shrink([false; 2])
			.show_viewport(ui, |ui, _|
		{
			//-------------------------------------------------- Playlist add/remove text edit.
			ui.horizontal(|ui| { ui.group(|ui| {
				const SIZE: f32 = 35.0;
				const SIZE2: f32 = 50.0;

				// Add button.
				let button = Button::new(RichText::new("+").size(SIZE));
				if ui.add_sized([SIZE2, SIZE2], button).clicked() {
					// add playlist
				}

				// Remove button.
				let button = Button::new(RichText::new("-").size(SIZE));
				if ui.add_sized([SIZE2, SIZE2], button).clicked() {
					// remove playlist
				}

				// Text edit.
				let width = ui.available_width();
				let text_edit = TextEdit::singleline(&mut self.state.playlist_string)
					.char_limit(PLAYLIST_NAME_MAX_LEN);
				ui.spacing_mut().text_edit_width = width;
				ui.add_sized([width, SIZE], text_edit).on_hover_text("TODO");
			})});

			ui.add_space(10.0);

			// For each `Playlist`...
			for (playlist_name, playlist) in playlists.iter() {
				ui.separator();
				ui.add_space(10.0);

				// `Playlist` name.
				let label_name = Label::new(
					RichText::new(&**playlist_name)
					.text_style(TextStyle::Name("30".into()))
				);

				// `Playlist` entry count.
				let label_count = Label::new(
					RichText::new(Unsigned::from(playlist.len()).as_str())
					.color(MEDIUM_GRAY)
					.text_style(TextStyle::Name("25".into()))
				);

				// `Playlist` runtime.
				let runtime: usize = playlist
					.iter()
					.map(|v| {
						match v {
							Entry::Valid { key_song, .. } => self.collection.songs[key_song].runtime.usize(),
							_ => 0,
						}
					})
					.sum();
				let label_runtime = Label::new(
					RichText::new(Runtime::from(runtime).as_str())
					.color(MEDIUM_GRAY)
					.text_style(TextStyle::Name("25".into()))
				);

				ui.horizontal(|ui| {
					crate::playlist_label!(self, playlist_name, ui, label_name);
					ui.add_space(20.0);
					ui.add(label_count);
					ui.add_space(20.0);
					ui.add(label_runtime);
				});

				ui.add_space(10.0);
				ui.separator();
			}
		});
	},

	//-------------------------------------------------- View
	PlaylistSubTab::View => {
		let Some(arc_str) = &self.state.playlist else {
			return;
		};
		let arc_str = std::sync::Arc::clone(&arc_str);

		let Some(playlist) = playlists.get(&arc_str) else {
			return;
		};

		// `Playlist` name.
		let label_name = Label::new(
			RichText::new(&*arc_str)
			.text_style(TextStyle::Name("30".into()))
		);

		// `Playlist` entry count.
		let label_count = Label::new(
			RichText::new(Unsigned::from(playlist.len()).as_str())
			.color(MEDIUM_GRAY)
			.text_style(TextStyle::Name("25".into()))
		);

		// `Playlist` runtime.
		let runtime: usize = playlist
			.iter()
			.map(|v| {
				match v {
					Entry::Valid { key_song, .. } => self.collection.songs[key_song].runtime.usize(),
					_ => 0,
				}
			})
			.sum();
		let label_runtime = Label::new(
			RichText::new(Runtime::from(runtime).as_str())
			.color(MEDIUM_GRAY)
			.text_style(TextStyle::Name("25".into()))
		);

		ui.horizontal(|ui| {
			crate::playlist_label!(self, arc_str, ui, label_name);
			ui.add_space(20.0);
			ui.add(label_count);
			ui.add_space(20.0);
			ui.add(label_runtime);
		});

		ui.add_space(10.0);
		ui.separator();

//			for (index, entry) in playlist.iter().enumerate() {
//				let Entry::Valid { key_song, .. } = entry else {
//					return;
//				};
//
//				let (artist, album, song) = self.collection.walk(key_song);
//
//				crate::song_button!(self, false, album, song, *key_song, ui, 0, None, Some(index), 35.0, ui.available_width());
//			}

		// `.show_rows()` is slightly faster than
		// `.show_viewport()` but we need to know
		// exactly how many rows we need to paint.
		//
		// The below needs to account for the scrollbar height,
		// the title heights and must not overflow to the bottom bar.
		const HEADER_HEIGHT: f32 = 80.0;
		const ROW_HEIGHT:    f32 = 35.0;
		let height     = ui.available_height();
		let max_rows   = ((height - (HEADER_HEIGHT - 5.0)) / ROW_HEIGHT) as usize;
		let row_range  = 0..max_rows;

		ScrollArea::horizontal()
			.id_source("PlaylistView")
			.max_width(f32::INFINITY)
			.max_height(f32::INFINITY)
			.auto_shrink([false; 2])
			.show_rows(ui, ROW_HEIGHT, max_rows, |ui, row_range|
		{ ui.push_id("PlaylistViewInner", |ui| {
			// Sizing.
			let width  = ui.available_width();
			let height = ui.available_height();
			// c == Column sizing
			let c_width   = width / 10.0;
			let c_runtime = c_width * 1.1;
			let c_title   = c_width * 5.0;
			let c_album   = c_width * 2.0;
			let c_artist  = c_width * 2.0;

			TableBuilder::new(ui)
				.striped(true)
				.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
				.column(Column::initial(c_runtime).resizable(true).clip(true))
				.column(Column::initial(c_title).resizable(true).clip(true))
				.column(Column::initial(c_album).resizable(true).clip(true))
//				.column(Column::initial(c_artist).resizable(true).clip(true))
				.column(Column::remainder().clip(true))
				.auto_shrink([false; 2])
				.max_scroll_height(height)
				.header(HEADER_HEIGHT, |mut header|
			{
				header.col(|ui| { ui.strong("Runtime"); });
				header.col(|ui| { ui.strong("Song"); });
				header.col(|ui| { ui.strong("Album"); });
				header.col(|ui| { ui.strong("Artist"); });
			})
			.body(|mut body| {
				for entry in playlist.iter() {
					let Entry::Valid { key_song, .. } = entry else {
						continue;
					};
					let key = key_song;

					body.row(ROW_HEIGHT, |mut row| {
						let (artist, album, song) = self.collection.walk(key);

						row.col(|ui| { ui.label(song.runtime.as_str()); });

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
					});
				}
			});
		})});

	},
	} // end of match.
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
