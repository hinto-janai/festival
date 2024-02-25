//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{
	BONE,GRAY,YELLOW,GREEN,MEDIUM_GRAY,
	QUEUE_ALBUM_ART_SIZE,WHITE,
};
use crate::text::{
	UI_QUEUE_CLEAR,UI_QUEUE_SHUFFLE,UI_MINUS,
	QUEUE_CLEAR,QUEUE_SHUFFLE,SELECT_QUEUE,
	UI_QUEUE_SHUFFLE_ARTIST,QUEUE_SHUFFLE_ARTIST,
	UI_QUEUE_SHUFFLE_ALBUM,QUEUE_SHUFFLE_ALBUM,
	UI_QUEUE_SHUFFLE_SONG,QUEUE_SHUFFLE_SONG,
	QUEUE_LENGTH,QUEUE_RUNTIME,
	UI_REPEAT_SONG,UI_REPEAT,REPEAT_SONG,REPEAT_QUEUE,REPEAT_OFF,
	REPEAT_QUEUE_PAUSE,
};
use shukusai::kernel::{
	FrontendToKernel,
};
use egui::{
	ScrollArea,Label,RichText,
	Sense,TextStyle,Button,
};
use benri::{
	send,
	now,
};

//---------------------------------------------------------------------------------------------------- Queue
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_queue(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, width: f32, height: f32) {
	self.set_visuals(ui);

	//-------------------------------------------------- Queue.
	ScrollArea::vertical()
		.id_source("Queue")
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		// Sizing.
		let width  = ui.available_width();
		let height = ui.available_height();
		const SIZE:  f32 = 35.0;
		const SIZE2: f32 = SIZE * 2.0;

		ui.horizontal(|ui| {
			let width = (width / 4.0) - 5.0;

			// Queue length `[xx/yy]`
			let len = self.audio_state.queue.len();
			let index = if len == 0 { 0 } else { self.audio_state.queue_idx.unwrap_or(0) + 1 };
			let text = Label::new(
				RichText::new(format!("[{index}/{len}]"))
					.color(BONE)
					.text_style(TextStyle::Name("25".into()))
			);
			ui.add_sized([width, SIZE2], text).on_hover_text(QUEUE_LENGTH);

			// Total queue human time `3 minutes, 2 seconds`.
			let text = Label::new(
				RichText::new(self.queue_time.as_str())
					.color(BONE)
					.text_style(TextStyle::Monospace)
			);
			ui.add_sized([ui.available_width(), SIZE2], text).on_hover_text(QUEUE_RUNTIME);
		});

		ui.horizontal(|ui| {
			let width = (width / 6.0) - 7.5;

			// Stop.
			let button = Button::new(RichText::new(UI_QUEUE_CLEAR).size(SIZE));
			if ui.add_sized([width, SIZE2], button).on_hover_text(QUEUE_CLEAR).clicked() {
				crate::clear_stop!(self);
			}

			// Shuffle.
			let button = Button::new(RichText::new(UI_QUEUE_SHUFFLE).size(SIZE));
			if ui.add_sized([width, SIZE2], button).on_hover_text(QUEUE_SHUFFLE).clicked() {
				send!(self.to_kernel, FrontendToKernel::Shuffle);
			}

			// Repeat.
			{
				use shukusai::audio::Repeat;
				let (icon, text, color) = match self.state.repeat {
					Repeat::Song  => (UI_REPEAT_SONG, REPEAT_SONG, YELLOW),
					Repeat::Queue => (UI_REPEAT, REPEAT_QUEUE, GREEN),
					Repeat::QueuePause => (UI_REPEAT, REPEAT_QUEUE_PAUSE, WHITE),
					Repeat::Off   => (UI_REPEAT, REPEAT_OFF, MEDIUM_GRAY),
				};
				let button = Button::new(
					RichText::new(icon)
						.size(30.0)
						.color(color)
				);
				if ui.add_sized([width, SIZE2], button).on_hover_text(text).clicked() {
					self.audio_leeway = now!();
					let next = self.state.repeat.next();
					send!(self.to_kernel, FrontendToKernel::Repeat(next));
					self.state.repeat = next;
				}
			}

			// INVARIANT:
			// Below `*_rand` macros unwrap on the rand functions which
			// return `Option` since the `Collection` might be empty.
			// These UIs must be greyed out if it is empty.
			ui.scope(|ui| {
				ui.set_enabled(!self.collection.empty);

				let button = Button::new(RichText::new(UI_QUEUE_SHUFFLE_ARTIST));
				let resp = ui.add_sized([width, SIZE2], button).on_hover_text(QUEUE_SHUFFLE_ARTIST);
				crate::artist_rand!(self, ui, resp);

				let button = Button::new(RichText::new(UI_QUEUE_SHUFFLE_ALBUM));
				let resp = ui.add_sized([width, SIZE2], button).on_hover_text(QUEUE_SHUFFLE_ALBUM);
				crate::album_rand!(self, ui, resp);

				let button = Button::new(RichText::new(UI_QUEUE_SHUFFLE_SONG));
				let resp = ui.add_sized([width, SIZE2], button).on_hover_text(QUEUE_SHUFFLE_SONG);
				crate::song_rand!(self, ui, resp);
			});
		});

		ui.add_space(5.0);
		ui.separator();

		//-------------------------------------------------- Empty queue.
		// INVARIANT:
		// We're returning early if queue is empty.
		// Make sure the code below knows this.
		if self.audio_state.queue.is_empty() {
			let label = Label::new(RichText::new(SELECT_QUEUE).color(GRAY));
			ui.add_sized([width, height/1.35], label);
			return;
		}

		//-------------------------------------------------- Start painting the artists/albums/songs.
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
					ui.add_space(60.0);
				}

				// Artist info.
				let artist_name = Label::new(
					RichText::new(&*artist.name)
					.text_style(TextStyle::Name("30".into()))
				);
				crate::artist_label!(self, artist, album.artist, ui, artist_name);
				current_artist = Some(artist);
				ui.add_space(5.0);
			}

			if !same_album {
				ui.separator();
				ui.horizontal(|ui| {
					// Remove button.
					let button = Button::new(RichText::new(UI_MINUS).size(SIZE));
					if ui.add_sized([SIZE, QUEUE_ALBUM_ART_SIZE], button).clicked() {
						// HACK:
						// Iterate until we find a `Song` that doesn't
						// belong to the same `Album`.
						//
						// This could end bad if there's an `Album` with _many_ `Song`'s.
						// Considering this is in the `GUI` update
						// loop, even worse... buuuuut who is going to
						// have an `Album` with 10,000s of `Song`'s... right?
						let mut end = index;
						let mut hit = false;
						let len = self.audio_state.queue.len();
						for key in self.audio_state.queue.range(index..) {
							if self.collection.songs[key].album != song.album {
								let end = if end == 0 { 1 } else { end };
								crate::remove_queue_range!(self, index..end);
								hit = true;
								break;
							}
							end += 1;
						}

						if !hit {
							let end = if end == 0 { 1 }  else { end };
							crate::remove_queue_range!(self, index..end);
						}
					}
					crate::no_rounding!(ui);
					crate::album_button!(self, album, song.album, ui, ctx, QUEUE_ALBUM_ART_SIZE, "");

					ui.vertical(|ui| {
						// Info.
						let album_title = Label::new(RichText::new(&*album.title).color(BONE));
						ui.add(album_title);
						ui.label(album.release.as_str());
						ui.label(album.runtime.as_str());
					});
				});

				current_album = Some(album);
			}

			//-------------------------------------------------- Song.
			ui.horizontal(|ui| {
				// Remove button.
				if ui.add_sized([SIZE, SIZE], Button::new(UI_MINUS)).clicked() {
					crate::remove_queue_range!(self, index..index+1);
				}

				// HACK:
				// If we remove an index but are still playing the `Song`,
				// the colored label indicating which one we're on will be wrong,
				// so it has to the the same index _and_ the same song.
				let same =
					self.audio_state.queue_idx == Some(index) &&
					self.audio_state.song      == Some(*key);

				crate::song_button!(self, same, album, song, *key, ui, 0, None, Some(index), SIZE, ui.available_width());
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
