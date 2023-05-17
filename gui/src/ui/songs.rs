//---------------------------------------------------------------------------------------------------- Use
use egui::{
	ScrollArea,
};
use egui_extras::{
	StripBuilder,Size,
	TableBuilder,Column,
};
use readable::Unsigned;
use log::warn;
use crate::text::{
	OPEN_PARENT_FOLDER,
};

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
	let c_artist  = c_width * 1.5;
	let c_runtime = c_width * 1.5;
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
			.column(Column::initial(c_runtime).resizable(true).clip(true))
			.column(Column::initial(c_track).resizable(true).clip(true))
			.column(Column::initial(c_disc).resizable(true).clip(true))
			.column(Column::remainder().clip(true))
			.auto_shrink([false; 2])
			.max_scroll_height(height)
			.header(40.0, |mut header|
		{
			header.col(|ui| {
				ui.strong("Title");
			});
			header.col(|ui| {
				ui.strong("Album");
			});
			header.col(|ui| {
				ui.strong("Artist");
			});
			header.col(|ui| {
				ui.strong("Runtime");
			});
			header.col(|ui| {
				ui.strong("Track");
			});
			header.col(|ui| {
				ui.strong("Disc");
			});
			header.col(|ui| {
				ui.strong("Path");
			});
		})
		.body(|mut body| {
			// Song iterator.
			// TODO:
			// Iterate based off user selection.
			for key in self.collection.sort_song_album_release_artist_lexi.iter() {
				body.row(35.0, |mut row| {
					let song   = &self.collection.songs[key];
					let album  = &self.collection.albums[song.album];
					let artist = &self.collection.artists[album.artist];

					row.col(|ui| { ui.label(&song.title); });
					row.col(|ui| { ui.label(&album.title); });
					row.col(|ui| { ui.label(&artist.name); });
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
