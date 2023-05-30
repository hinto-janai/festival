//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{
	BONE,MEDIUM_GRAY,
	QUEUE_ALBUM_ART_SIZE,
};
use shukusai::collection::{
	Song,Album
};
use shukusai::kernel::{
	FrontendToKernel,
	AUDIO_STATE,
};
use egui::{
	ScrollArea,Label,RichText,SelectableLabel,
	Sense,TextStyle,
};
use benri::send;
use readable::HeadTail;

//---------------------------------------------------------------------------------------------------- Queue
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_queue(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	self.set_visuals(ui);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();

	//-------------------------------------------------- Queue.
	ScrollArea::vertical()
		.id_source("Queue")
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		let mut current_artist = None;
		let mut current_album  = None;

		// TODO:
		// Copy audio state somewhere else
		// in a more global manner.
		let mut iter = AUDIO_STATE
			.read()
			.queue
			.clone()
			.into_iter()
			.peekable();

		loop {
			let key = match iter.next() {
				Some(k) => k,
				None => break,
			};

			let (artist, album, song) = self.collection.walk(key);

			//-------------------------------------------------- Artist.
			if current_artist != Some(artist) {
				// Artist info.
				let artist_name = Label::new(
					RichText::new(&artist.name)
					.text_style(TextStyle::Name("30".into()))
				);
				if ui.add(artist_name.sense(Sense::click())).clicked() {
					crate::artist!(self, album.artist);
				}
				ui.separator();
				ui.add_space(10.0);
				current_artist = Some(artist);

				// FIXME:
				// This code is duplicated below for new albums.
				ui.horizontal(|ui| {
					crate::no_rounding!(ui);
					crate::album_button!(self, album, song.album, ui, ctx, QUEUE_ALBUM_ART_SIZE);

					ui.vertical(|ui| {
						// Info.
						let album_title = Label::new(RichText::new(&album.title).color(BONE));
						ui.add(album_title);
						ui.label(album.release.as_str());
						ui.label(album.runtime.as_str());
					});
				});

				ui.add_space(10.0);
				ui.separator();
				current_album = Some(album)
			//-------------------------------------------------- Album.
			} else if current_album != Some(album) {
				// FIXME: see above.
				ui.horizontal(|ui| {
					crate::no_rounding!(ui);
					crate::album_button!(self, album, song.album, ui, ctx, QUEUE_ALBUM_ART_SIZE);

					ui.vertical(|ui| {
						// Info.
						let album_title = Label::new(RichText::new(&album.title).color(BONE));
						ui.add(album_title);
						ui.label(album.release.as_str());
					});
				});

				ui.add_space(10.0);
				ui.separator();
				current_album = Some(album)
			}

			//-------------------------------------------------- Song.
			// How many char's before we need
			// to cut off the song title?
			// (scales based on pixels available).
			let head = (width / 18.0) as usize;

			// As long as the `Artist/Album` are
			// the same, continue painting songs.
			while let Some(peek_key) = iter.peek() {
				let (artist, album, song) = self.collection.walk(*peek_key);
				if current_artist != Some(artist) {
					ui.add_space(30.0);
					break;
				} else if current_album != Some(album) {
					ui.add_space(10.0);
					ui.separator();
					ui.add_space(10.0);
					break;
				} else {
					iter.next();
				}

				let mut rect = ui.cursor();
				rect.max.y = rect.min.y + 35.0;
				if ui.put(rect, SelectableLabel::new(false, "")).clicked() {
					// TODO: Implement song key state.

					send!(self.to_kernel, FrontendToKernel::AddQueueSongTailFront(key));
					send!(self.to_kernel, FrontendToKernel::Play);
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
		}
	});
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
