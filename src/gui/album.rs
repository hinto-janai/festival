//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use egui::{
	Rounding,Vec2,Color32,Stroke,
	ScrollArea,Frame,
	SelectableLabel,Label,
};

//----------------------------------------------------------------------------------------------------
//#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
impl super::Gui {
#[inline(always)]
pub(crate) fn show_tab_album(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	ui.vertical_centered(|ui| {
		ui.add_space(height/40.0);
		ui.set_max_width(height/3.0);
		Frame::window(&ctx.style()).rounding(Rounding::none()).inner_margin(1.0).show(ui, |ui| {
			let size = height / 2.5;
			self.img.vec[2].1.show_size(ui, Vec2::new(size, size));
		});

		ui.heading("祝祭");
		ui.heading("カネコアヤノ");
		ui.heading("2021");
		ui.heading("10/10");

	});

	ui.separator();
	ui.visuals_mut().selection.bg_fill = Color32::from_rgb(200, 100, 100);
	ui.visuals_mut().selection.stroke = Stroke { width: 5.0, color: Color32::from_rgb(255, 255, 255) };
	ScrollArea::vertical().max_width(f32::INFINITY).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
	for (i, time, name) in &self.list {
		let mut rect = ui.cursor();
		rect.max.y = rect.min.y + 35.0;
		if ui.put(rect, SelectableLabel::new(*i == self.s, "")).clicked() { self.s = *i; };
		rect.max.x = rect.min.x;
		ui.allocate_ui_at_rect(rect, |ui| {
			ui.horizontal_centered(|ui| {
				ui.add(Label::new(format!("{: >6}{: >10}      {}", i, time, name)).wrap(false));
			});
		});
	}
	});
}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
