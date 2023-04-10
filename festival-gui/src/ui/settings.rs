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
	ALBUM_ART_MIN_SIZE,
	ALBUM_ART_MAX_SIZE,
};
use shukusai::sort::AlbumSort;
use shukusai::kernel::{
	FrontendToKernel,
};
use benri::{
	send,
	flip,
	atomic_load,
};
use std::sync::Arc;
use compact_str::format_compact;

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
			let width = (width / 2.0) - 22.0;
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
		ui.add_sized([width, text], label).on_hover_text("TODO");

		// Add folder (max 10).
		let collection_paths_len = self.settings.collection_paths.len();

		ui.scope(|ui| {
			ui.set_enabled(collection_paths_len < 10);

			if ui.add_sized([width - 10.0, text], Button::new("Add folder")).on_hover_text("Add a maximum of 10 folders").clicked() {
				if atomic_load!(self.rfd_open) {
					warn!("GUI - Add folder button pressed, but RFD is already open");
				} else {
					crate::data::spawn_rfd_thread(
						Arc::clone(&self.rfd_open),
						Arc::clone(&self.rfd_new),
					);
				}
			}
		});

		ui.add_space(10.0);

		// List folders (max 10)
		for i in 0..collection_paths_len {
			ui.horizontal(|ui| {
				let path  = format_compact!("{}", self.settings.collection_paths[i].display());
				let width = width / 20.0;

				// Delete button.
				if ui.add_sized([width, text], Button::new("-")).on_hover_text("Remove this folder").clicked() {
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

			if ui.add_sized([width - 10.0, text], Button::new("Reset Collection")).on_hover_text("Scan the folders listed and create a new Collection").clicked() {
				// TODO:
				// Send signal to `Kernel`.
				// Go into collection mode.
				send!(self.to_kernel, FrontendToKernel::NewCollection(self.settings.collection_paths.clone()));
				self.resetting_collection = true;
				return;
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
		ui.add_sized([width, text], label).on_hover_text("TODO");

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
