//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use super::gui::Gui;
use egui::{
	ScrollArea,Frame,
	Color32,Vec2,Stroke,Rounding,RichText,
	TopBottomPanel,SidePanel,CentralPanel,
};
use egui::widgets::{
	Slider,Button,SelectableLabel,Label,
};
use super::{
	tab::Tab,
	tab,
};
use disk::Bincode;
use log::{error,warn,info,debug,trace};
use shukusai::{
	FrontendToKernel,
	KernelToFrontend,
	mass_panic,
	send,
	recv,
	ok,
};
use std::time::Duration;

//---------------------------------------------------------------------------------------------------- Main GUI event loop.
impl eframe::App for Gui {
	#[inline(always)]
	fn on_close_event(&mut self) -> bool {
		// Tell `Kernel` to save stuff.
		send!(self.to_kernel, FrontendToKernel::Exit);

		// Save `State` if diff.
		if self.diff_state() {
			if let Err(e) = self.settings.write() {
				error!("GUI | Could not save `Settings`: {}", e)
			}
		}

		// Save `Settings` if diff.
		if self.diff_settings() {
			if let Err(e) = self.state.write() {
				error!("GUI | Could not save `State`: {}", e)
			}
		}

		// Check if `Kernel` succeeded.
		// Loop through 3 messages just in-case
		// there were others in the channel queue.
		//
		// This waits a max `900ms` before
		// continuing without the response.
		let mut n = 0;
		loop {
			if let Ok(KernelToFrontend::ExitResponse(r)) = self.from_kernel.recv_timeout(Duration::from_millis(300)) {
				match r {
					Ok(_)  => ok!("GUI | Kernel save"),
					Err(e) => error!("GUI | Kernel save failed: {}", e),
				}
				break
			} else if n > 3 {
				error!("GUI | Could not determine Kernel's exit result, continuing exit");
			} else {
				n += 1;
			}
		}

		// Exit.
		std::process::exit(0);
		true
	}

	#[inline(always)]
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		// Determine if there is a diff in settings.
//		let diff = self.settings != self.og;

		// Set global UI [Style/Visual]s
//		if diff {
//			ctx.set_visuals();
//		}

		// Set global available width/height.
		let rect   = ctx.available_rect();
		let width  = rect.width();
		let height = rect.height();

		// Size definitions of the major UI panels.
		let bottom_panel_height = height / 15.0;
		let side_panel_width    = width / 8.0;
		let side_panel_height   = height - (bottom_panel_height*2.0);

		// Bottom Panel
		Self::show_bottom(self, ctx, frame, width, bottom_panel_height);

		// Left Panel
		Self::show_left(self, ctx, frame, side_panel_width, side_panel_height);

		// Right Panel
		Self::show_right(self, ctx, frame, side_panel_width, side_panel_height);

		// Central Panel
		Self::show_central(self, ctx, frame, width, height);
	}
}

//---------------------------------------------------------------------------------------------------- Bottom Panel
impl Gui {
#[inline(always)]
fn show_bottom(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
// TODO
//	TopBottomPanel::bottom("bottom").resizable(false).show(ctx, |ui| {
//		ui.set_height(height);
//
//		// Base unit for sizing UI.
//		let unit = width / 15.0;
//
//		ui.horizontal(|ui| {
//			// Media control buttons
//			ui.group(|ui| {
//				ui.add_sized([unit, height], Button::new("⏪"));
//				ui.add_sized([unit*1.5, height], Button::new("▶"));
//				ui.add_sized([unit, height], Button::new("⏩"));
//			});
//
//			// Song time elapsed
//			ui.add_sized([unit, height], Label::new("1:33 / 3:22"));
//
//			// Slider (playback)
//			ui.spacing_mut().slider_width = ui.available_width();
//			ui.add_sized(
//				[ui.available_width(), height],
//				Slider::new(&mut self.v, 0.0..=100.0).smallest_positive(1.0).show_value(false)
//			);
//		});
//	});
}}

//---------------------------------------------------------------------------------------------------- Left Panel
impl Gui {
#[inline(always)]
fn show_left(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
// TODO
//	SidePanel::left("left").resizable(false).show(ctx, |ui| {
//		ui.set_width(width);
//		ui.set_height(height);
//
//		// Size definitions of the elements within the left panel.
//		let half_height = height / 2.0;
//		let tab_height  = half_height / 7.0;
//		let tab_width   = width / 1.2;
//
//		// Main UI
//		ui.vertical_centered_justified(|ui| {
//
//			// Display [SelectableLabel] for each [Tab].
//			if ui.add_sized([tab_width, tab_height], SelectableLabel::new(self.tab == Tab::Albums, tab::ALBUMS)).clicked()       { self.tab = Tab::Albums; }
//			ui.separator();
//			if ui.add_sized([tab_width, tab_height], SelectableLabel::new(self.tab == Tab::Artists, tab::ARTISTS)).clicked()     { self.tab = Tab::Artists; }
//			ui.separator();
//			if ui.add_sized([tab_width, tab_height], SelectableLabel::new(self.tab == Tab::Songs, tab::SONGS)).clicked()         { self.tab = Tab::Songs; }
//			ui.separator();
//			if ui.add_sized([tab_width, tab_height], SelectableLabel::new(self.tab == Tab::Queue, tab::QUEUE)).clicked()         { self.tab = Tab::Queue; }
//			ui.separator();
//			if ui.add_sized([tab_width, tab_height], SelectableLabel::new(self.tab == Tab::Playlists, tab::PLAYLISTS)).clicked() { self.tab = Tab::Playlists; }
//			ui.separator();
//			if ui.add_sized([tab_width, tab_height], SelectableLabel::new(self.tab == Tab::Search, tab::SEARCH)).clicked()       { self.tab = Tab::Search; }
//			ui.separator();
//			if ui.add_sized([tab_width, tab_height], SelectableLabel::new(self.tab == Tab::Settings, tab::SETTINGS)).clicked()   { self.tab = Tab::Settings; }
//			ui.separator();
//
//			// Volume slider
//			let slider_height = ui.available_height() - 20.0;
//
//			ui.add_space(10.0);
//
//			ui.spacing_mut().slider_width = slider_height;
//			ui.visuals_mut().selection.bg_fill = Color32::from_rgb(200, 100, 100);
//
//			ui.horizontal(|ui| {
//				let unit = width / 10.0;
//				ui.add_space(unit*4.0);
//				ui.add(Slider::new(&mut self.v, 0.0..=100.0).smallest_positive(1.0).show_value(false).vertical().thickness(unit*2.0).circle_size(unit));
//			});
//		});
//	});
}}

//---------------------------------------------------------------------------------------------------- Right Panel
impl Gui {
#[inline(always)]
fn show_right(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
//	TODO
//	SidePanel::right("right").resizable(false).show(ctx, |ui| {
//		ui.set_width(width);
//
//		// How big the albums (on the right side) should be.
//		let ALBUM_SIZE = width / 1.4;
//
//		ScrollArea::vertical().max_width(width).max_height(f32::INFINITY).auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
//			ui.vertical_centered(|ui| {
//			for (name, img) in &self.img.vec {
//				ui.add_space(5.0);
//				ui.scope(|ui| {
//					ui.set_width(ALBUM_SIZE);
//	 				Frame::window(&ctx.style()).rounding(Rounding::none()).inner_margin(1.0).show(ui, |ui| {
//						let mut rect = ui.cursor();
//						rect.max.x += 2.0;
//						rect.max.y = rect.min.y + ALBUM_SIZE;
//						if ui.put(rect, Button::new("").rounding(Rounding::none())).clicked() { self.name = *name };
//						rect.max.x = rect.min.x;
//						ui.allocate_ui_at_rect(rect, |ui| {
//							ui.horizontal_centered(|ui| {
//								img.show_size(ui, Vec2::new(ALBUM_SIZE, ALBUM_SIZE));
//							});
//						});
//					});
//				});
//				if *name == self.name {
//					ui.add(Label::new(RichText::new(name.to_string()).color(Color32::LIGHT_BLUE)));
//				} else {
//					ui.label(name.to_string());
//				}
//				ui.add_space(5.0);
//			}
//			});
//		});
//	});
}}

//---------------------------------------------------------------------------------------------------- Central Panel
impl Gui {
#[inline(always)]
fn show_central(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	CentralPanel::default().show(ctx, |ui| {
		match self.state.tab {
			Tab::Albums    => Self::show_tab_album(self, ui, ctx, frame, width, height),
			_ => (),
		}
	});
}}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
