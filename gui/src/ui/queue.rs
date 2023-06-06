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
};
use egui::{
	ScrollArea,Label,RichText,SelectableLabel,
	Sense,TextStyle,Button,
};
use benri::send;
use readable::HeadTail;

//---------------------------------------------------------------------------------------------------- Queue
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_queue(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	self.set_visuals(ui);

	//-------------------------------------------------- Queue.
	ScrollArea::both()
		.id_source("Queue")
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		// Sizing.
		let width  = ui.available_width();
		let height = ui.available_height();
		const REMOVE_SONG_SIZE: f32 = 35.0;
		const REMOVE_SIZE:      f32 = REMOVE_SONG_SIZE * 2.0;

		if ui.add_sized([width - 10.0, REMOVE_SIZE], Button::new("Clear queue and stop playback")).clicked() {
			crate::clear_stop!(self);
		}

		ui.add_space(10.0);

		let mut current_artist = None;
		let mut current_album  = None;

		for (index, key) in self.audio_state.queue.iter().enumerate() {
			let (artist, album, song) = self.collection.walk(key);

			let same_artist = current_artist == Some(artist);
			let same_album  = current_album == Some(album);

			//-------------------------------------------------- Artist.
			if !same_artist {
				// Only add space if we've added previous `Artist`'s before.
				if current_artist.is_some() {
					ui.add_space(30.0);
				}

				// Artist info.
				let artist_name = Label::new(
					RichText::new(&artist.name)
					.text_style(TextStyle::Name("30".into()))
				);
				ui.horizontal(|ui| {
					// Remove button.
					let button = Button::new(RichText::new("-").size(REMOVE_SONG_SIZE));
					if ui.add_sized([REMOVE_SIZE, REMOVE_SIZE], button).clicked() {
						// HACK:
						// Iterate until we find a `Song` that doesn't
						// belong to the same `Album`.
						//
						// This could end bad if we have a long queue,
						// and considering this is in the `GUI` update
						// loop, even worse... buuuuut who is going to
						// have a queue with 1000s of elements... right?
						let mut end = index;
						let mut hit = false;
						let len = self.audio_state.queue.len();
						for key in self.audio_state.queue.range(index..) {
							if self.collection.songs[key].album != song.album {
								let end = if end == 0 { 1 } else { end };
								send!(self.to_kernel, FrontendToKernel::RemoveQueueRange(index..end));
								hit = true;
								break;
							}
							end += 1;
						}

						if !hit {
							let end = if end == 0 { 1 }  else { end };
							send!(self.to_kernel, FrontendToKernel::RemoveQueueRange(index..end));
						}
					}
					if ui.add(artist_name.sense(Sense::click())).clicked() {
						crate::artist!(self, album.artist);
					}
				});
				current_artist = Some(artist);

				// FIXME:
				// This code is duplicated below for new albums.
				ui.add_space(10.0);
				ui.separator();
				ui.add_space(10.0);

				ui.horizontal(|ui| {
					crate::no_rounding!(ui);
					crate::album_button!(self, album, song.album, ui, ctx, QUEUE_ALBUM_ART_SIZE, "");

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
				current_album = Some(album);
			//-------------------------------------------------- Album.
			} else if !same_album {
				// FIXME: see above.
				ui.add_space(10.0);
				ui.separator();
				ui.add_space(10.0);

				ui.horizontal(|ui| {
					crate::no_rounding!(ui);
					crate::album_button!(self, album, song.album, ui, ctx, QUEUE_ALBUM_ART_SIZE, "");

					ui.vertical(|ui| {
						// Info.
						let album_title = Label::new(RichText::new(&album.title).color(BONE));
						ui.add(album_title);
						ui.label(album.release.as_str());
					});
				});

				ui.add_space(10.0);
				ui.separator();
				current_album = Some(album);
			}

			//-------------------------------------------------- Song.
			ui.horizontal(|ui| {
				// Remove button.
				if ui.add_sized([REMOVE_SONG_SIZE, REMOVE_SONG_SIZE,], Button::new("-")).clicked() {
					send!(self.to_kernel, FrontendToKernel::RemoveQueueRange(index..index+1));
				}

				let mut rect = ui.cursor();
				rect.max.y = rect.min.y + REMOVE_SONG_SIZE;
				rect.max.x = rect.min.x + ui.available_width();

				// HACK:
				// If we remove an index but are still playing the `Song`,
				// the colored label indicating which one we're on will be wrong,
				// so it has to the the same index _and_ the same song.
				let same =
					self.audio_state.queue_idx == Some(index) &&
					self.audio_state.song      == Some(*key);

				let resp = ui.put(rect, SelectableLabel::new(same, ""));
				if resp.clicked() {
					crate::play_queue_index!(self, index);
				} else if resp.middle_clicked() {
					crate::open!(self, album);
				} else if resp.secondary_clicked() {
					crate::add_song!(self, song.title, *key);
				}


				ui.allocate_ui_at_rect(rect, |ui| {
					ui.horizontal_centered(|ui| {
						ui.add(Label::new(format!("{: >3}    {: >8}    {}", song.track.unwrap_or(0), &song.runtime, &song.title)));
					});
				});
			});
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
