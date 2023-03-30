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
	Label,RichText,
};
use crate::constants::{
	BONE,
};

//---------------------------------------------------------------------------------------------------- Settings
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_settings(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	// Some space, to lower the scrollbar.
	ui.add_space(7.0);

	// Set sizes.
	let text = height / 25.0;

	let scroll_area = ScrollArea::vertical()
		.always_show_scroll(true)
		.max_width(f32::INFINITY)
		.max_height(f32::INFINITY)
		.auto_shrink([false; 2]);

	scroll_area.show_viewport(ui, |ui, _| {
		ui.vertical_centered(|ui| {
			//-------------------------------------------------- Sort order.
			// Heading.
			let label = Label::new(
				RichText::new("Sort Order")
				.underline()
				.color(BONE)
				.text_style(TextStyle::Heading)
			);
			ui.add_sized([width, text], label).on_hover_text("TODO");

			// ComboBox.
			// TODO:
			// Fix alignment.
			ui.spacing_mut().slider_width = width / 4.0;
			ui.spacing_mut().icon_width = height / 20.0;
			ComboBox::from_id_source("sort_order")
				.selected_text(RichText::new(self.settings.sort_order.as_str()).color(BONE))
				.show_ui(ui, |ui| {
				}
			);

			//-------------------------------------------------- Album art size.
			// Heading.

			//-------------------------------------------------- Restore state.
			// Heading.

			//-------------------------------------------------- Accent color.
			// Heading.

			//-------------------------------------------------- Collection paths.
			// Heading.

			//-------------------------------------------------- Stats.
			// Heading.
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
