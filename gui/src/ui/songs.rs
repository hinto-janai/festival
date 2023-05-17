//---------------------------------------------------------------------------------------------------- Use
use egui::{
	ScrollArea,
};
use egui_extras::{
	StripBuilder,Size,
	TableBuilder,Column,
};

//---------------------------------------------------------------------------------------------------- Songs
impl crate::data::Gui {
#[inline(always)]
pub fn show_tab_songs(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	self.set_visuals(ui);

	// Sizing.
	let width  = ui.available_width();
	let height = ui.available_height();

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
			.column(Column::auto().resizable(true).clip(true))
			.column(Column::auto().resizable(true).clip(true))
			.column(Column::auto().resizable(true).clip(true))
			.column(Column::auto().resizable(true).clip(true))
			.column(Column::remainder().clip(true))
			.auto_shrink([false; 2])
			.max_scroll_height(height)
			.header(20.0, |mut header|
		{
			header.col(|ui| {
				ui.strong("#");
			});
			header.col(|ui| {
				ui.strong("Title");
			});
			header.col(|ui| {
				ui.strong("Track");
			});
			header.col(|ui| {
				ui.strong("Album");
			});
			header.col(|ui| {
				ui.strong("Runtime");
			});
		})
		.body(|mut body| {
			body.rows(20.0, 100, |row_index, mut row| {
				row.col(|ui| {
					ui.label(row_index.to_string());
				});
				row.col(|ui| {
					ui.label(row_index.to_string());
				});
				row.col(|ui| {
					ui.label(row_index.to_string());
				});
				row.col(|ui| {
					ui.label(row_index.to_string());
				});
				row.col(|ui| {
					ui.label(row_index.to_string());
				});
			});
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
