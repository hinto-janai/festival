//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use egui::{
	TextStyle,
	ScrollArea,ComboBox,
	Label,RichText,Slider,
	SelectableLabel,Button,
};
use crate::constants::{
	BONE,
	ALBUM_ART_MIN_SIZE,
	ALBUM_ART_MAX_SIZE,
};
use shukusai::sort::AlbumSort;
use shukusai::{
	flip,
};

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

		ui.set_enabled(!self.diff_settings());

		if ui.add_sized([width, text], reset).on_hover_text("TODO").clicked() {
			self.reset_settings();
		}

		if ui.add_sized([width, text], save).on_hover_text("TODO").clicked() {
			self.set_settings();
		}
	})});

	ui.add_space(15.0);
	ui.separator();

	//-------------------------------------------------- Main ScrollArea.
	let scroll_area = ScrollArea::vertical()
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
		ui.add_sized([width, text], label).on_hover_text("TODO");

		// ComboBox.
		// FIXME:
		// Trying to center `ComboBox` uncovers all sorts
		// of `egui` bugs, so instead, just make it max width.
		ui.spacing_mut().combo_width = width - 10.0;
		ui.spacing_mut().icon_width = height / 15.0;
		ui.add_space(10.0);
		ComboBox::from_id_source("sort_order").selected_text(RichText::new(self.settings.sort_order.as_str()).color(BONE)).show_ui(ui, |ui| {
			// Album Sort methods.
			ui.selectable_value(&mut self.settings.sort_order, AlbumSort::ReleaseArtistLexi, "Artists lexicographically, albums in release order");
			ui.selectable_value(&mut self.settings.sort_order, AlbumSort::LexiArtistLexi,    "Artists lexicographically, albums lexicographically");
			ui.selectable_value(&mut self.settings.sort_order, AlbumSort::Lexi,              "Albums lexicographically");
			ui.selectable_value(&mut self.settings.sort_order, AlbumSort::Release,           "Albums in release order");
			ui.selectable_value(&mut self.settings.sort_order, AlbumSort::Runtime,           "Albums shortest to longest");
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
		ui.add_sized([width, text], label).on_hover_text("TODO");

		// Slider.
		// FIXME:
		// Same issue as above. Slider centering is pain.
		ui.spacing_mut().slider_width = width - 75.0;
		ui.add_space(10.0);
		let slider = Slider::new(&mut self.settings.album_art_size, ALBUM_ART_MIN_SIZE..=ALBUM_ART_MAX_SIZE)
			.step_by(1.0)
			.thickness(text)
			.fixed_decimals(0)
			.trailing_fill(false);
		ui.add_sized([width, text], slider).on_hover_text("TODO");

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
		ui.add_sized([width, text], label).on_hover_text("TODO");

		// SelectableLabel.
		ui.add_space(10.0);
		ui.group(|ui| { ui.horizontal(|ui| {
			let width = (width / 2.0) - 20.0;
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
		ui.add_sized([width, text], label).on_hover_text("TODO");

		// Color picker.
		ui.spacing_mut().interact_size = egui::Vec2::new(width - 10.0, text);
		egui::widgets::color_picker::color_edit_button_srgb(ui, &mut self.settings.accent_color);

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
		ui.add_sized([width, text], label).on_hover_text("TODO");

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
		ui.add_sized([width, text], label).on_hover_text("TODO");

	});
}}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
