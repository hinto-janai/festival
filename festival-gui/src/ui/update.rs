//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::data::Gui;
use egui::{
	ScrollArea,Frame,ProgressBar,
	Color32,Vec2,Stroke,Rounding,RichText,
	TopBottomPanel,SidePanel,CentralPanel,
};
use egui::widgets::{
	Slider,Button,Spinner,
	SelectableLabel,Label,
};
use crate::data::{
	Tab,
};
use disk::Toml;
use log::{error,warn,info,debug,trace};
use shukusai::kernel::{
	FrontendToKernel,
	KernelToFrontend,
};
use shukusai::key::{
	AlbumKey,
};
use benri::{
	log::*,
	sync::*,
	panic::*,
};
use std::time::{
	Instant,
	Duration,
};
use rolock::RoLock;
use std::sync::Arc;
use crate::constants::{
	VISUALS,
};
use crate::text::*;

//---------------------------------------------------------------------------------------------------- `GUI`'s eframe impl.
impl eframe::App for Gui {
	//-------------------------------------------------------------------------------- On exit.
	#[inline(always)]
	fn on_close_event(&mut self) -> bool {
		// If already exiting, return, else
		// turn on `GUI`'s signal for exiting.
		match self.exiting {
			true  => {
				warn!("GUI - Already in process of exiting, ignoring exit signal");
				return false;
			},
			false => {
				info!("GUI - Exiting...");
				self.exiting = true;
			},
		};

		// Clone things to send to exit thread.
		let to_kernel    = self.to_kernel.clone();
		let from_kernel  = self.from_kernel.clone();
		let kernel_state = self.kernel_state.clone();
		let settings     = self.settings.clone();
		let state        = self.state; // `copy`-able

		// Spawn `exit` thread.
		std::thread::spawn(move || {
			Self::exit(to_kernel, from_kernel, state, settings, kernel_state);
		});

		// Set the exit `Instant`.
		self.exit_instant = Instant::now();

		// Don't exit here.
		false
	}

	//-------------------------------------------------------------------------------- Main event loop.
	#[inline(always)]
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		// Check for `Kernel` messages.
		if let Ok(msg) = self.from_kernel.try_recv() {
			use KernelToFrontend::*;
			match msg {
				NewCollection(collection) => {
					ok_debug!("GUI: New Collection");
					self.collection = collection;
					self.resetting_collection = false;
					self.cache_collection();
				},
				NewKernelState(k)      => self.kernel_state = k,
				NewResetState(r)      => self.reset_state = r,
				Failed((old_collection, error_string)) => println!("failed"),
				SearchSim(keychain) => {
					self.search_result = keychain;
					self.searching     = false;
				},
				_ => todo!(),
			}
		}

		// Set global available width/height.
		let rect   = ctx.available_rect();
		let width  = rect.width();
		let height = rect.height();

		// If exiting, show fullscreen spinner.
		if self.exiting {
			// To prevent showing a flash of the spinner
			// when exiting really quickly, rack up enough
			// time (100ms) before showing the spinner.
			if self.exit_instant.elapsed().as_secs_f32() > 0.1 {
				self.show_exit_spinner(ctx, frame, width, height);
				return;
			}
		}

		// If resetting the `Collection`,
		// show fullscreen spinner with info.
		if self.resetting_collection {
			self.show_resetting_collection(ctx, frame, width, height);
			return;
		}

		// Copy `Kernel`'s `AudioState`.
		if self.state.audio.playing {
			self.copy_kernel_audio();
		}

//		// Determine if there is a diff in `Settings`'s.
		let diff_settings = self.diff_settings();

		// Set global UI [Style/Visual]'s

		// Check if `RFD` thread added some PATHs.
		match lock!(self.rfd_new).take() {
			Some(p) => {
				let mut exists = false;
				for path in &self.settings.collection_paths {
					if p == *path {
						exists = true;
					}
				}
				match exists {
					true  => info!("GUI - PATH exists, not adding: {}", p.display()),
					false => self.settings.collection_paths.push(p),
				}
			},
			None    => (),
		}

		// Size definitions of the major UI panels.
		let bottom_panel_height = height / 15.0;
		let side_panel_width    = width / 8.0;
		let side_panel_height   = height - (bottom_panel_height*2.0);

		// Bottom Panel
		self.show_bottom(ctx, frame, width, bottom_panel_height);

		// Left Panel
		self.show_left(ctx, frame, side_panel_width, side_panel_height);

		// If `Tab::View`, show right panel.
		if self.state.tab == Tab::View {
			self.show_tab_view_right_panel(self.state.album, ctx, frame, side_panel_width, height);
		}

		// Central Panel
		self.show_central(ctx, frame, width, height, side_panel_width, side_panel_height);
	}
}

//---------------------------------------------------------------------------------------------------- Bottom Panel
impl Gui {
#[inline(always)]
fn show_bottom(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	TopBottomPanel::bottom("bottom").resizable(false).show(ctx, |ui| {
		self.set_visuals(ui);
		ui.set_height(height);

		// Base unit for sizing UI.
		let unit = width / 15.0;

		ui.horizontal(|ui| {
			// Media control buttons
			ui.group(|ui| {
				ui.add_sized([unit, height], Button::new("⏪"));
				ui.add_sized([unit*1.5, height], Button::new("▶"));
				ui.add_sized([unit, height], Button::new("⏩"));
			});

			// Song time elapsed
			ui.add_sized([unit, height], Label::new("1:33 / 3:22"));

			// Slider (playback)
			ui.spacing_mut().slider_width = ui.available_width();
			let h = height / 5.0;
			ui.add_sized(
				[ui.available_width(), height],
				// TODO:
				// Send signal on slider mutation by user.
				Slider::new(&mut self.state.audio.current_elapsed, 0.0..=100.0)
					.smallest_positive(1.0)
					.show_value(false)
					.thickness(h*2.0)
					.circle_size(h)
			);
		});
	});
}}

//---------------------------------------------------------------------------------------------------- Left Panel
impl Gui {
#[inline(always)]
fn show_left(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	SidePanel::left("left").resizable(false).show(ctx, |ui| {
		self.set_visuals(ui);
		ui.set_width(width);
		ui.set_height(height);

		// Size definitions of the elements within the left panel.
		let half_height = height / 2.0;
		let tab_height  = half_height / 8.0;
		let tab_width   = width / 1.2;

		// Main UI
		ui.vertical_centered_justified(|ui| {

			// Display `SelectableLabel` for each `Tab`.
			ui.add_space(2.5);
			for tab in Tab::iter() {
				if ui.add_sized([tab_width, tab_height], SelectableLabel::new(self.state.tab == *tab, tab.as_str())).clicked() {
					self.state.tab = *tab;
				}
				ui.separator();
			}

			// Album art size (only on `Albums` tab, only in `DEBUG` build).
			#[cfg(debug_assertions)]
			{
				if self.state.tab == Tab::Albums {
					ui.horizontal(|ui| { ui.group(|ui| {
						let width  = (ui.available_width() / 2.0) - 10.0;
						let height = tab_height / 2.0;
						ui.scope(|ui| {
							ui.set_enabled(!self.album_size_is_min());
							if ui.add_sized([width, tab_height], Button::new("-")).on_hover_text(DECREMENT_ALBUM_SIZE).clicked() {
								self.decrement_art_size();
							}
						});
						ui.separator();
						ui.scope(|ui| {
							ui.set_enabled(!self.album_size_is_max());
							if ui.add_sized([width, tab_height], Button::new("+")).on_hover_text(INCREMENT_ALBUM_SIZE).clicked() {
								self.increment_art_size();
							}
						});
					})});
				}
			}

			// Volume slider
			let slider_height = ui.available_height() - 20.0;

			ui.add_space(10.0);

			ui.spacing_mut().slider_width = slider_height;

			ui.horizontal(|ui| {
				let unit = width / 10.0;
				ui.add_space(unit*4.0);
				// TODO:
				// Send signal on slider mutation by user.
				ui.add(Slider::new(&mut self.state.audio.volume.inner(), 0..=100)
					.smallest_positive(1.0)
					.show_value(false)
					.vertical()
					.thickness(unit*2.0)
					.circle_size(unit)
				).on_hover_text(VOLUME_SLIDER);
			});
		});
	});
}}

//---------------------------------------------------------------------------------------------------- Central Panel
impl Gui {
#[inline(always)]
fn show_central(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32, side_panel_width: f32, side_panel_height: f32) {
	CentralPanel::default().show(ctx, |ui| {
		self.set_visuals(ui);
		match self.state.tab {
			Tab::View      => self.show_tab_view(ui, ctx, frame, width, height),
			Tab::Albums    => self.show_tab_albums(ui, ctx, frame, width, height),
			Tab::Artists   => self.show_tab_artists(ui, ctx, frame, width, height),
			Tab::Songs     => self.show_tab_songs(ui, ctx, frame, width, height),
			Tab::Queue     => self.show_tab_queue(ui, ctx, frame, width, height),
			// TODO: Make `shukusai` playlists suck less.
//			Tab::Playlists => (),
			Tab::Search    => self.show_tab_search(ui, ctx, frame, width, height),
			Tab::Settings  => self.show_tab_settings(ui, ctx, frame, width, height),
		}
	});
}}


//---------------------------------------------------------------------------------------------------- Exit spinner
// This is a fullscreen spinner.
// Used when exiting and waiting for everything to save.
impl Gui {
#[inline(always)]
fn show_exit_spinner(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	CentralPanel::default().show(ctx, |ui| {
		self.set_visuals(ui);
		ui.vertical_centered(|ui| {
			let half = height / 2.0;
			let hh   = half / 2.0;

			let text = RichText::new("Saving...")
				.size(hh / 4.0)
				.color(crate::constants::BONE);

			ui.add_sized([width, half], Label::new(text));
			ui.add_sized([width, hh], Spinner::new().size(hh));
		});
	});
}}

//---------------------------------------------------------------------------------------------------- Reset Collection
// This is a fullscreen spinner.
// Used when waiting on `Kernel` to hand over the new `Collection`.
impl Gui {
#[inline(always)]
fn show_resetting_collection(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	CentralPanel::default().show(ctx, |ui| {
		self.set_visuals(ui);
		ui.vertical_centered(|ui| {
			let half = height / 2.0;

			// Header.
			let text = RichText::new("Resetting the Collection...")
				.heading()
				.color(crate::constants::BONE);
			ui.add_sized([width, half], Label::new(text));

			let height = half / 6.0;

			// Spinner.
			ui.add_sized([width, height], Spinner::new().size(height));
			// Percent.
			ui.add_sized([width, height], Label::new(lock_read!(self.reset_state).percent.as_str()));
			// Phase.
			ui.add_sized([width, height], Label::new(&lock_read!(self.reset_state).phase));
			// Specific.
			ui.add_sized([width, height], Label::new(&lock_read!(self.reset_state).specific));
			// ProgressBar.
			ui.add_sized([width / 1.1, height], ProgressBar::new(lock_read!(self.reset_state).percent.inner() as f32 / 100.0));

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
