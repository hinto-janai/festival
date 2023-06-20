//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use log::{info,error,warn,trace,debug};
use egui::{
	TextStyle,
	ScrollArea,ComboBox,
	Label,RichText,Slider,
	SelectableLabel,Button,
};
use crate::constants::{
	BONE,
	ALBUM_ART_SIZE_MIN,
	ALBUM_ART_SIZE_MAX,
	ALBUMS_PER_ROW_MIN,
	ALBUMS_PER_ROW_MAX,
	SLIDER_CIRCLE_INACTIVE,
	SLIDER_CIRCLE_HOVERED,
	SLIDER_CIRCLE_ACTIVE,
	PREVIOUS_THRESHOLD_MIN,
	PREVIOUS_THRESHOLD_MAX,
};
use crate::data::{
	AlbumSizing,
	SearchSort,
	WindowTitle,
};
use shukusai::{
	sort::{
		ArtistSort,AlbumSort,SongSort,
	},
	search::SearchKind,
	kernel::FrontendToKernel,
	collection::Collection,
};
use benri::{
	send,
	flip,
	atomic_load,
	atomic_store,
	ok_debug,
};
use std::sync::Arc;
use crate::text::*;
use disk::Bincode2;

//---------------------------------------------------------------------------------------------------- Settings
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_settings(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	// Reset width.
	let width = ui.available_width();

	// Set sizes.
	let text = height / 25.0;

	//-------------------------------------------------- Main ScrollArea.
	ScrollArea::vertical()
		.id_source("Settings")
		.always_show_scroll(true)
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2])
		.show_viewport(ui, |ui, _|
	{
		//-------------------------------------------------- Reset/Save
		let reset = Button::new(
			RichText::new("Reset")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		let save = Button::new(
			RichText::new("Save")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);

		ui.add_space(15.0);
		ui.horizontal_top(|ui| {
		ui.group(|ui| {
			let width = (width / 2.0) - 18.0;

			ui.set_enabled(self.diff_settings());

			if ui.add_sized([width, text], reset).on_hover_text(RESET).clicked() {
				self.reset_settings();
			}

			if ui.add_sized([width, text], save).on_hover_text(SAVE).clicked() {
				if let Err(e) = self.save_settings() {
					crate::toast_err!(self, "Settings save failed: {e}");
				}
			}
		})});

		ui.add_space(20.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Collection paths.

		// Heading.
		let label = Label::new(
			RichText::new("Collection")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(COLLECTION);

		// Add folder (max 10).
		let collection_paths_len = self.settings.collection_paths.len();

		ui.scope(|ui| {
			ui.set_enabled(collection_paths_len < 10);

			if ui.add_sized([width - 15.0, text], Button::new("Add folder")).on_hover_text(ADD_FOLDER).clicked() {
				self.add_folder();
			}
		});

		ui.add_space(10.0);

		// List folders (max 10)
		for i in 0..collection_paths_len {
			ui.horizontal(|ui| {
				let path  = format!("{}", self.settings.collection_paths[i].display());
				let width = width / 20.0;

				// Delete button.
				if ui.add_sized([width, text], Button::new("-")).on_hover_text(REMOVE_FOLDER).clicked() {
					self.deleted_paths.push(i);
				}

				// Show PATH.
				ui.label(path.as_str()).on_hover_text(path.as_str());
			});
		}

		// Delete folders.
		// The PATHs cannot be deleted above
		// because it will invalidate the next
		// index and cause a panic, so the results
		// are stored in `deleted_path`, which are used here.
		if self.deleted_paths.len() > 0 {
			for i in &self.deleted_paths {
				self.settings.collection_paths.remove(*i);
			}
			self.deleted_paths.clear();
		}

		ui.add_space(10.0);

		// Reset collection.
		ui.scope(|ui| {
			// Make button color red.
			let mut visuals = ui.visuals_mut();
			visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(120, 40, 40);
			visuals.widgets.hovered.weak_bg_fill  = egui::Color32::from_rgb(180, 40, 40);
			visuals.widgets.active.weak_bg_fill   = egui::Color32::from_rgb(140, 40, 40);

			if ui.add_sized([width - 15.0, text], Button::new("Reset Collection"))
				.on_hover_text(RESET_COLLECTION)
				.clicked()
			{
				self.reset_collection();
			}
		});

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

        //-------------------------------------------------- Artist Sort Order.
		// Heading.
		let label = Label::new(
			RichText::new("Artist Sort Order")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(ARTIST_SORT_ORDER);

		// FIXME:
		// Trying to center `ComboBox` uncovers all sorts
		// of `egui` bugs, so instead, just make it max width.
		ui.spacing_mut().combo_width = width - 15.0;
		ui.spacing_mut().icon_width = height / 15.0;

		// ComboBox.
		ui.add_space(10.0);
		ComboBox::from_id_source("settings_artist_sort_order")
			.selected_text(RichText::new(self.settings.artist_sort.as_str()).color(BONE))
			.show_ui(ui, |ui|
		{
			// Album Sort methods.
			for i in ArtistSort::iter() {
				ui.selectable_value(&mut self.settings.artist_sort, *i, i.as_str());
			}
		});

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Album Sort Order.
		// Heading.
		let label = Label::new(
			RichText::new("Album Sort Order")
				.color(BONE)
				.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(ALBUM_SORT_ORDER);

		// ComboBox.
		ui.add_space(10.0);
		ComboBox::from_id_source("settings_album_sort_order")
			.selected_text(RichText::new(self.settings.album_sort.as_str()).color(BONE))
			.show_ui(ui, |ui|
		{
			// Album Sort methods.
			for i in AlbumSort::iter() {
				ui.selectable_value(&mut self.settings.album_sort, *i, i.as_str());
			}
		});

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Song Sort Order.
		// Heading.
		let label = Label::new(
			RichText::new("Song Sort Order")
				.color(BONE)
				.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(SONG_SORT_ORDER);

		// ComboBox.
		ui.add_space(10.0);
		ComboBox::from_id_source("settings_song_sort_order")
			.selected_text(RichText::new(self.settings.song_sort.as_str()).color(BONE))
			.show_ui(ui, |ui|
		{
			// Song Sort methods.
			for i in SongSort::iter() {
				ui.selectable_value(&mut self.settings.song_sort, *i, i.as_str());
			}
		});

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Search Kind.
		// Heading.
		let label = Label::new(
			RichText::new("Search Kind")
				.color(BONE)
				.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(SEARCH_KIND);

		// ComboBox.
		ui.add_space(10.0);
		ComboBox::from_id_source("settings_search_kind")
			.selected_text(RichText::new(self.settings.search_kind.as_str()).color(BONE))
			.show_ui(ui, |ui|
		{
			for i in SearchKind::iter() {
				ui.selectable_value(&mut self.settings.search_kind, *i, i.as_str());
			}
		});

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Window title.
		// Heading.
		let label = Label::new(
			RichText::new("Window Title")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(WINDOW_TITLE);

		ui.add_space(10.0);

		// ComboBox.
		ui.add_space(10.0);
		ComboBox::from_id_source("settings_window_title")
			.selected_text(RichText::new(self.settings.window_title.as_str()).color(BONE))
			.show_ui(ui, |ui|
		{
			for i in WindowTitle::iter() {
				ui.selectable_value(&mut self.settings.window_title, *i, i.as_str());
			}
		});

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Album Art Size.
		// Heading.
		let label = Label::new(
			RichText::new("Album Art Size")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(ALBUM_ART_SIZE);

		// SelectableLabel.
		ui.add_space(10.0);
		ui.group(|ui| { ui.horizontal(|ui| {
			let width = (width / 2.0) - 25.0;
			if ui.add_sized([width, text], SelectableLabel::new(self.settings.album_sizing == AlbumSizing::Pixel, format!("[{}] Pixels", self.settings.album_pixel_size)))
				.on_hover_text(STATIC_PIXEL_SIZE).clicked()
			{
				self.settings.album_sizing = AlbumSizing::Pixel;
			}
			ui.separator();
			if ui.add_sized([width, text], SelectableLabel::new(self.settings.album_sizing == AlbumSizing::Row,  format!("[{}] Albums Per Row", self.settings.albums_per_row)))
				.on_hover_text(ALBUM_PER_ROW).clicked()
			{
				self.settings.album_sizing = AlbumSizing::Row;
			}
		})});

		// Slider.
		// FIXME:
		// Same issue as above. Slider centering is pain.
		ui.spacing_mut().slider_width = width - 15.0;
		ui.add_space(10.0);
		let (slider, hover) = match self.settings.album_sizing {
			AlbumSizing::Pixel => {
				let size = self.settings.album_pixel_size;
				(Slider::new(&mut self.settings.album_pixel_size, ALBUM_ART_SIZE_MIN..=ALBUM_ART_SIZE_MAX), format!("{0}x{0} album art pixel size", size))
			},
			AlbumSizing::Row => {
				let size = self.settings.albums_per_row;
				(Slider::new(&mut self.settings.albums_per_row, ALBUMS_PER_ROW_MIN..=ALBUMS_PER_ROW_MAX), format!("{} albums per row", size))
			},
		};

		ui.scope(|ui| {
			{
				let v = &mut ui.visuals_mut().widgets;
				v.inactive.fg_stroke = SLIDER_CIRCLE_INACTIVE;
				v.hovered.fg_stroke  = SLIDER_CIRCLE_HOVERED;
				v.active.fg_stroke   = SLIDER_CIRCLE_ACTIVE;
			}
			let slider = slider
				.step_by(1.0)
				.thickness(text)
				.fixed_decimals(0)
				.show_value(false)
				.trailing_fill(false);
			ui.add_sized([width, text], slider).on_hover_text(hover);
		});

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Previous Leeway.
		// Heading.
		let label = Label::new(
			RichText::new(format!("Previous Threshold ({})", self.settings.previous_threshold))
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(PREVIOUS_THRESHOLD);

		let old_threshold = self.settings.previous_threshold;
		ui.scope(|ui| {
			{
				let v = &mut ui.visuals_mut().widgets;
				v.inactive.fg_stroke = SLIDER_CIRCLE_INACTIVE;
				v.hovered.fg_stroke  = SLIDER_CIRCLE_HOVERED;
				v.active.fg_stroke   = SLIDER_CIRCLE_ACTIVE;
			}
			let slider = Slider::new(&mut self.settings.previous_threshold, PREVIOUS_THRESHOLD_MIN..=PREVIOUS_THRESHOLD_MAX);
			let slider = slider
				.step_by(1.0)
				.thickness(text)
				.show_value(false)
				.trailing_fill(false);
			ui.add_sized([width, text], slider);
		});
		if old_threshold != self.settings.previous_threshold {
			atomic_store!(shukusai::audio::PREVIOUS_THRESHOLD, self.settings.previous_threshold);
		}

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Empty playback.
		// Heading.
		let label = Label::new(
			RichText::new("Empty Queue Auto-Play")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(EMPTY_AUTOPLAY);

		// SelectableLabel.
		ui.add_space(10.0);
		ui.group(|ui| { ui.horizontal(|ui| {
			let width = (width / 2.0) - 25.0;
			if ui.add_sized([width, text], SelectableLabel::new(self.settings.empty_autoplay,  "Yes")).clicked() { flip!(self.settings.empty_autoplay); }
			ui.separator();
			if ui.add_sized([width, text], SelectableLabel::new(!self.settings.empty_autoplay, "No")).clicked()  { flip!(self.settings.empty_autoplay); }
		})});

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Restore state.
		// Heading.
		let label = Label::new(
			RichText::new("Restore State On Startup")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(RESTORE_STATE);

		// SelectableLabel.
		ui.add_space(10.0);
		ui.group(|ui| { ui.horizontal(|ui| {
			let width = (width / 2.0) - 25.0;
			if ui.add_sized([width, text], SelectableLabel::new(self.settings.restore_state,  "Yes")).clicked() { flip!(self.settings.restore_state); }
			ui.separator();
			if ui.add_sized([width, text], SelectableLabel::new(!self.settings.restore_state, "No")).clicked()  { flip!(self.settings.restore_state); }
		})});

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Accent Color.
		// Heading.
		let label = Label::new(
			RichText::new("Accent Color")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(ACCENT_COLOR);

		// Color picker.
		ui.spacing_mut().interact_size = egui::vec2(width - 15.0, text);
		egui::widgets::color_picker::color_edit_button_srgba(
			ui,
			&mut self.settings.accent_color,
			egui::widgets::color_picker::Alpha::Opaque,
		);

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Stats.
		// Heading.
		let label = Label::new(
			RichText::new("Stats")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(STATS);

		// Stats.
		ui.add_sized([width, text], Label::new(&self.count_artist));
		ui.add_sized([width, text], Label::new(&self.count_album));
		ui.add_sized([width, text], Label::new(&self.count_song));

		ui.add_space(40.0);
		ui.separator();
		ui.add_space(40.0);

		//-------------------------------------------------- Help.
		let label = Label::new(
			RichText::new("Help")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label);

		ui.add_sized([width, text], Label::new(HELP));
		ui.add_space(40.0);
//		ui.separator();
//		ui.add_space(40.0);
	});
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
