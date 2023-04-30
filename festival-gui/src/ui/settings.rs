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
};
use crate::data::{
	AlbumSizing,
};
use shukusai::sort::AlbumSort;
use shukusai::kernel::{
	FrontendToKernel,
};
use shukusai::collection::Collection;
use benri::{
	send,
	flip,
	atomic_load,
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

	// Reset/Save.
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
		let width = (width / 2.0) - 10.0;

		ui.set_enabled(self.diff_settings());

		if ui.add_sized([width, text], reset).on_hover_text(RESET).clicked() {
			self.reset_settings();
		}

		if ui.add_sized([width, text], save).on_hover_text(SAVE).clicked() {
			self.save_settings();
		}
	})});

	ui.add_space(15.0);
	ui.separator();

	//-------------------------------------------------- Main ScrollArea.
	let scroll_area = ScrollArea::vertical()
		.id_source("Settings")
		.always_show_scroll(true)
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2]);

	scroll_area.show_viewport(ui, |ui, _| {
		//-------------------------------------------------- Album Sort Order.
		ui.add_space(45.0);
		// Heading.
		let label = Label::new(
			RichText::new("Album Sort Order")
			.color(BONE)
			.text_style(TextStyle::Heading)
		);
		ui.add_sized([width, text], label).on_hover_text(ALBUM_SORT_ORDER);

		// ComboBox.
		// FIXME:
		// Trying to center `ComboBox` uncovers all sorts
		// of `egui` bugs, so instead, just make it max width.
		ui.spacing_mut().combo_width = width - 15.0;
		ui.spacing_mut().icon_width = height / 15.0;
		ui.add_space(10.0);
		ComboBox::from_id_source("sort_order").selected_text(RichText::new(self.settings.sort_order.as_str()).color(BONE)).show_ui(ui, |ui| {
			// Album Sort methods.
			for i in AlbumSort::iter() {
				ui.selectable_value(&mut self.settings.sort_order, *i, i.as_str());
			}
		});

		ui.add_space(60.0);
		ui.separator();
		ui.add_space(60.0);

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

		ui.add_space(60.0);
		ui.separator();
		ui.add_space(60.0);

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
			if ui.add_sized([width, text], SelectableLabel::new(!self.settings.restore_state, "No")).clicked()  { flip!(self.settings.restore_state); }
			ui.separator();
			if ui.add_sized([width, text], SelectableLabel::new(self.settings.restore_state,  "Yes")).clicked() { flip!(self.settings.restore_state); }
		})});

		ui.add_space(60.0);
		ui.separator();
		ui.add_space(60.0);

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

		ui.add_space(60.0);
		ui.separator();
		ui.add_space(60.0);

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
			ui.set_enabled(collection_paths_len > 0 && !self.resetting_collection);

			if ui.add_sized([width - 15.0, text], Button::new("Reset Collection")).on_hover_text(RESET_COLLECTION).clicked() {
				self.reset_collection();
			}
		});

		ui.add_space(60.0);
		ui.separator();
		ui.add_space(60.0);

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
	});
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
