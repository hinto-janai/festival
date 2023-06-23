//---------------------------------------------------------------------------------------------------- Use
use egui::{
	ScrollArea,TextStyle,ImageButton,
	TextEdit,Label,RichText,Spinner,
	SelectableLabel,Sense,
};
use crate::{
	constants::{
		BONE,MEDIUM_GRAY,GRAY,
	},
	text::SELECT_ARTIST,
	data::{
		ArtistSubTab,Tab,
	},
};
use readable::Unsigned;
use log::warn;
use readable::HeadTail;
use benri::send;
use shukusai::kernel::{
	FrontendToKernel,
};

//---------------------------------------------------------------------------------------------------- Artists
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_artists(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, width: f32, height: f32) {
	self.set_visuals(ui);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();

	//-------------------------------------------------- Artist sub-tab.
	ui.group(|ui| { ui.horizontal(|ui| {
		let width = (width / 2.0) - 20.0;

		{
			const TAB: ArtistSubTab = ArtistSubTab::All;
			let label = SelectableLabel::new(self.settings.artist_sub_tab == TAB, TAB.as_str());
			if ui.add_sized([width, 30.0], label).clicked() {
				self.settings.artist_sub_tab = TAB;
			}
		}

		ui.separator();

		{
			const TAB: ArtistSubTab = ArtistSubTab::View;
			let label = match self.state.artist {
				Some(key) => {
					let name = self.collection.artists[key].name.head_dot(18);
					SelectableLabel::new(self.settings.artist_sub_tab == TAB, name)
				},
				None => SelectableLabel::new(self.settings.artist_sub_tab == TAB, TAB.as_str())
			};

			if ui.add_sized([width, 30.0], label).clicked() {
				self.settings.artist_sub_tab = TAB;
			}
		}
	})});

	ui.add_space(10.0);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();

	//-------------------------------------------------- All artists
	match self.settings.artist_sub_tab {
	ArtistSubTab::All => {

	ScrollArea::vertical()
		.id_source("Artist")
		.max_width(width)
		.max_height(height)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		// For each `Artist`...
		for key in self.collection.artist_iter(self.settings.artist_sort) {
			let artist = &self.collection.artists[key];

			// `Artist` name.
			let label_name = Label::new(
				RichText::new(&artist.name)
				.text_style(TextStyle::Name("30".into()))
			);

			// `Artist` album count.
			let label_count = Label::new(
				RichText::new(Unsigned::from(artist.albums.len()).as_str())
				.color(MEDIUM_GRAY)
				.text_style(TextStyle::Name("25".into()))
			);

			// `Artist` runtime.
			let label_runtime = Label::new(
				RichText::new(artist.runtime.as_str())
				.color(MEDIUM_GRAY)
				.text_style(TextStyle::Name("25".into()))
			);

			ui.horizontal(|ui| {
				crate::artist_label!(self, artist, *key, ui, label_name);
				ui.add_space(20.0);
				ui.add(label_count);
				ui.add_space(20.0);
				ui.add(label_runtime);
			});

			ui.add_space(10.0);

			// Their `Album`'s.
			ScrollArea::horizontal()
				.id_source(key)
				.max_width(f32::INFINITY)
				.max_height(120.0)
				.auto_shrink([false; 2])
				.show_viewport(ui, |ui, _|
			{
				ui.horizontal(|ui| {
					crate::no_rounding!(ui);

					// Album
					for key in &artist.albums {
						let album = &self.collection.albums[key];

						crate::album_button!(self, album, *key, ui, ctx, 120.0, &album.title);
					}
				});
			});

			ui.add_space(10.0);
			ui.separator();
		}
	});

	},
	//-------------------------------------------------- View
	ArtistSubTab::View => {

	// Extract `ArtistKey`.
	let artist_key = match self.state.artist {
		Some(key) => key,

		// If no `AlbumKey` selected, show text.
		None => {
			let label = Label::new(RichText::new(SELECT_ARTIST).color(GRAY));
			ui.add_sized([width, height], label);

			return;
		}
	};

	let artist = &self.collection.artists[artist_key];

	// `Artist` name.
	let label_name = Label::new(
		RichText::new(&artist.name)
		.color(BONE)
		.text_style(TextStyle::Name("30".into()))
	);

	// `Artist` album count.
	let label_count = Label::new(
		RichText::new(Unsigned::from(artist.albums.len()).as_str())
		.color(MEDIUM_GRAY)
		.text_style(TextStyle::Name("25".into()))
	);

	// `Artist` runtime.
	let label_runtime = Label::new(
		RichText::new(artist.runtime.as_str())
		.color(MEDIUM_GRAY)
		.text_style(TextStyle::Name("25".into()))
	);

	// Albums.
	ScrollArea::vertical()
		.id_source("artist_view")
		.max_width(width)
		.max_height(height)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		// Artist info.
		ui.horizontal(|ui| {
			ui.add(label_name);
			ui.add_space(20.0);
			ui.add(label_count);
			ui.add_space(20.0);
			ui.add(label_runtime);
		});

		// How many char's before we need
		// to cut off the song title?
		// (scales based on pixels available).
		let head = (width / 26.5) as usize;

		// The total song offset of this `Artist`.
		let mut offset = 0;

		for album_key in artist.albums.iter() {
			ui.separator();
			ui.add_space(10.0);

			let album = &self.collection.albums[album_key];

			ui.horizontal(|ui| {
				ui.scope(|ui| {
					// Album.
					crate::no_rounding!(ui);
					crate::album_button!(self, album, *album_key, ui, ctx, self.settings.album_pixel_size, "");
				});

				ui.vertical(|ui| {
					// Info.
					let album_title = Label::new(RichText::new(&album.title).color(BONE));
					ui.add(album_title);
					ui.label(album.release.as_str());
					ui.label(album.runtime.as_str());
					ui.separator();

					// Song list.
					for key in album.songs.iter() {
						let song = &self.collection.songs[key];
						crate::song_button!(self, self.audio_state.song == Some(*key), album, song, *key, ui, offset, Some(artist_key), None, 35.0, 0.0);
						offset += 1;
					}
				});
			});

			ui.add_space(10.0);
		}
	});


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
