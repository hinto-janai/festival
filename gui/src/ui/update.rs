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
	ScrollArea,Frame,ProgressBar,TextStyle,
	Color32,Vec2,Stroke,Rounding,RichText,
	TopBottomPanel,SidePanel,CentralPanel,
};
use egui::widgets::{
	Slider,Button,Spinner,TextEdit,
	SelectableLabel,Label,
};
use crate::data::{
	Tab,
	KeyPress,
	ALPHANUMERIC_KEY,
};
use disk::{Toml,Plain};
use log::{error,warn,info,debug,trace};
use shukusai::kernel::{
	FrontendToKernel,
	KernelToFrontend,
};
use shukusai::collection::{
	AlbumKey,
};
use benri::{
	log::*,
	sync::*,
	panic::*,
	flip,
};
use std::time::{
	Instant,
	Duration,
};
use rolock::RoLock;
use std::sync::Arc;
use crate::constants::{
	VISUALS,
	SLIDER_CIRCLE_INACTIVE,
	SLIDER_CIRCLE_HOVERED,
	SLIDER_CIRCLE_ACTIVE,
	BONE,
	BLACK,
};
use crate::text::{
	EMPTY_COLLECTION,
	COLLECTION_LOADING,
	COLLECTION_RESETTING,
	UI_PREVIOUS,
	UI_PLAY,
	UI_PAUSE,
	UI_FORWARDS,
	DECREMENT_ALBUM_SIZE,
	INCREMENT_ALBUM_SIZE,
	VOLUME_SLIDER,
};

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
		let to_kernel      = self.to_kernel.clone();
		let from_kernel    = self.from_kernel.clone();
		let kernel_state   = self.kernel_state.clone();
		let settings       = self.settings.clone();
		let state          = self.state; // `copy`-able
		let exit_countdown = self.exit_countdown.clone();

		// Spawn `exit` thread.
		std::thread::spawn(move || {
			Self::exit(to_kernel, from_kernel, state, settings, kernel_state, exit_countdown);
		});

		// Set the exit `Instant`.
		self.exit_instant = Instant::now();

		// Don't exit here.
		false
	}

	//-------------------------------------------------------------------------------- Main event loop.
	#[inline(always)]
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		atomic_store!(shukusai::frontend::egui::UPDATING, true);
		self.__update(ctx, frame);
		atomic_store!(shukusai::frontend::egui::UPDATING, false);
	}
}

impl Gui {
	#[inline(always)]
	fn __update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		// Check for `Kernel` messages.
		// Only if we're not exiting, to prevent stealing
		// the message intended for the exit thread.
		if !self.exiting {
			if let Ok(msg) = self.from_kernel.try_recv() {
				use KernelToFrontend::*;
				match msg {
					NewCollection(collection) => {
						ok_debug!("GUI - New Collection");
						self.collection = collection;
						self.resetting_collection = false;
						self.kernel_returned = true;
						self.cache_collection();
					},
					Failed((old_collection, error_string)) => {
						println!("failed");
						self.kernel_returned = true;
					},
					NewKernelState(k)      => self.kernel_state = k,
					NewResetState(r)      => self.reset_state = r,
					SearchSim(keychain) => {
						self.search_result = keychain;
						self.searching     = false;
					},
					_ => todo!(),
				}
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
			ctx.request_repaint();
		}

		// If `Kernel` hasn't finished startup yet,
		// show fullscreen spinner with info.
		if !self.kernel_returned {
			self.show_collection_spinner(ctx, frame, width, height, COLLECTION_LOADING);
			return;
		}

		// If resetting the `Collection`,
		// show fullscreen spinner with info.
		if self.resetting_collection {
			self.show_collection_spinner(ctx, frame, width, height, COLLECTION_RESETTING);
			return;
		}

		// Copy `Kernel`'s `AudioState`.
		if self.state.audio.playing {
			self.copy_kernel_audio();
		}

		// Check for key presses.
		if !ctx.wants_keyboard_input() {
			ctx.input_mut(|input| {
				// Last tab.
				if input.pointer.button_clicked(egui::PointerButton::Extra1) ||  // FIXME:
					input.pointer.button_clicked(egui::PointerButton::Extra2) || // These two don't work with my mouse.
					input.consume_key(egui::Modifiers::CTRL, egui::Key::D)
				{
					if let Some(tab) = self.last_tab {
						crate::tab!(self, tab);
					}
				}

				// Check for `Up/Down` (Tab switch)
				if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
					crate::tab!(self, self.state.tab.next());
				} else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
					crate::tab!(self, self.state.tab.previous());
				// Check for `Left/Right` (Volume)
				} else if self.state.tab == Tab::Albums && input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
					self.increment_art_size()
				} else if self.state.tab == Tab::Albums && input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
					self.decrement_art_size()
				// Check for `F11` (Fullscreen)
				} else if input.consume_key(egui::Modifiers::NONE, egui::Key::F11) {
					frame.set_fullscreen(!frame.info().window_info.fullscreen);
				// Check for `Ctrl+R` (Reset Collection)
				} else if input.consume_key(egui::Modifiers::CTRL, egui::Key::R) {
					self.reset_collection();
				// Check for `Ctrl+S` (Save Settings)
				} else if input.consume_key(egui::Modifiers::CTRL, egui::Key::S) {
					self.save_settings();
				// Check for `Ctrl+Z` (Reset Settings)
				} else if input.consume_key(egui::Modifiers::CTRL, egui::Key::Z) {
					self.reset_settings();
				// Check for `Ctrl+A` (Add Folder)
				} else if input.consume_key(egui::Modifiers::CTRL, egui::Key::A) {
					self.add_folder();
				// Check for `Ctrl+Q` (Next Artist Order)
				} else if input.consume_key(egui::Modifiers::CTRL, egui::Key::Q) {
					crate::tab!(self, Tab::Artists);
					self.settings.artist_sort = self.settings.artist_sort.next();
				// Check for `Ctrl+W` (Next Album Order)
				} else if input.consume_key(egui::Modifiers::CTRL, egui::Key::W) {
					crate::tab!(self, Tab::Albums);
					self.settings.album_sort = self.settings.album_sort.next();
				// Check for `Ctrl+E` (Next Song Order)
				} else if input.consume_key(egui::Modifiers::CTRL, egui::Key::E) {
					crate::tab!(self, Tab::Songs);
					self.settings.song_sort = self.settings.song_sort.next();
				// Check for `Ctrl+Shift+P` (force `panic!()`)
				} else if
					input.modifiers.matches(egui::Modifiers::CTRL.plus(egui::Modifiers::SHIFT))
					&&
					input.key_pressed(egui::Key::P)
				{
					info!("GUI - CTRL+SHIFT+P was pressed, forcefully panic!()'ing...!");
					panic!("GUI - CTRL+SHIFT+P force panic");
				// Check for `Ctrl+Shift+D` (toggle debug screen)
				} else if
					input.modifiers.matches(egui::Modifiers::CTRL.plus(egui::Modifiers::SHIFT))
					&&
					input.key_pressed(egui::Key::D)
				{
					flip!(self.debug_screen);
					// Refresh stats if true.
					if self.debug_screen {
						self.update_debug_info();
					}
				// Check for [A-Za-z] (Search)
				} else {
					for key in ALPHANUMERIC_KEY {
						if input.consume_key(egui::Modifiers::NONE, key) {
							crate::tab!(self, Tab::Search);
							self.search_string = KeyPress::from_egui_key(&key).to_string();
							self.search_jump   = true;
							break
						}
					}
				}
			});
		}

//		// Determine if there is a diff in `Settings`'s.
		let diff_settings = self.diff_settings();

		// Set global UI [Style/Visual]'s

		// Check if `RFD` thread added some PATHs.
		if let Some(p) = lock!(self.rfd_new).take() {
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
		}

		// Show debug screen if `true`.
		if self.debug_screen {
			self.show_debug_screen(ctx, frame, width, height);
			return;
		}

		// Size definitions of the major UI panels.
		let bottom_panel_height = (height / 15.0).clamp(50.0, 60.0);
		let side_panel_width    = (width / 8.0).clamp(125.0, 250.0);
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
				ui.add_sized([unit, height], Button::new(RichText::new(UI_PREVIOUS).size(35.0)));
				ui.add_sized([unit*1.5, height], Button::new(RichText::new(UI_PLAY).size(20.0)));
				ui.add_sized([unit, height], Button::new(RichText::new(UI_FORWARDS).size(35.0)));
			});

			// Song time elapsed
			ui.add_sized([unit, height], Label::new("1:33 / 3:22"));

			// Slider (playback)
			ui.spacing_mut().slider_width = ui.available_width();
			{
				let v = &mut ui.visuals_mut().widgets;
				v.inactive.fg_stroke = SLIDER_CIRCLE_INACTIVE;
				v.hovered.fg_stroke  = SLIDER_CIRCLE_HOVERED;
				v.active.fg_stroke   = SLIDER_CIRCLE_ACTIVE;
			}
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
					crate::tab!(self, *tab);
				}
				ui.separator();
			}

			// Album art size (only on `Albums` tab, only in `DEBUG` build).
//			#[cfg(debug_assertions)]
//			{
//				if self.state.tab == Tab::Albums {
//					ui.horizontal(|ui| { ui.group(|ui| {
//						let width  = (ui.available_width() / 2.0) - 10.0;
//						let height = tab_height / 2.0;
//						ui.scope(|ui| {
//							ui.set_enabled(!self.album_size_is_min());
//							if ui.add_sized([width, tab_height], Button::new(RichText::new("-").size(30.0))).on_hover_text(DECREMENT_ALBUM_SIZE).clicked() {
//								self.decrement_art_size();
//							}
//						});
//						ui.separator();
//						ui.scope(|ui| {
//							ui.set_enabled(!self.album_size_is_max());
//							if ui.add_sized([width, tab_height], Button::new(RichText::new("+").size(25.0))).on_hover_text(INCREMENT_ALBUM_SIZE).clicked() {
//								self.increment_art_size();
//							}
//						});
//					})});
//				}
//			}

			// Volume slider
			let slider_height = ui.available_height() - 20.0;

			ui.add_space(10.0);

			ui.spacing_mut().slider_width = slider_height;

			ui.horizontal(|ui| {
				let unit = width / 10.0;
				ui.add_space(unit*4.0);
				{
					let v = &mut ui.visuals_mut().widgets;
					v.inactive.fg_stroke = SLIDER_CIRCLE_INACTIVE;
					v.hovered.fg_stroke  = SLIDER_CIRCLE_HOVERED;
					v.active.fg_stroke   = SLIDER_CIRCLE_ACTIVE;
				}
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

		// Handle empty `Collection`.
		if self.collection.empty && self.state.tab != Tab::Settings {
			self.show_empty_collection(ui, ctx, frame, width, height);
			return;
		}

		match self.state.tab {
			Tab::View      => self.show_tab_view(ui, ctx, frame, width, height),
			Tab::Albums    => self.show_tab_albums(ui, ctx, frame, width, height),
			Tab::Artists   => self.show_tab_artists(ui, ctx, frame, width, height),
			Tab::Songs     => self.show_tab_songs(ui, ctx, frame, width, height),
			Tab::Queue     => self.show_tab_queue(ui, ctx, frame, width, height),
			// SOMEDAY: Make `shukusai` playlists suck less.
//			Tab::Playlists => (),
			Tab::Search    => self.show_tab_search(ui, ctx, frame, width, height),
			Tab::Settings  => self.show_tab_settings(ui, ctx, frame, width, height),
		}
	});
}}

//---------------------------------------------------------------------------------------------------- Empty Collection
// Shows a button that resets the `Collection`.
// Used for tabs when the `Collection` is empty.
impl Gui {
#[inline(always)]
fn show_empty_collection(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	// Handle empty or no `Collection`.
	let button = Button::new(RichText::new("🔃 Empty Collection. Click to scan.").color(BONE));
	let width  = width / 1.5;
	let height = height / 10.0;

	ui.vertical_centered(|ui| {
		ui.add_space(height * 4.0);

		if ui.add_sized([width, height], button).on_hover_text(EMPTY_COLLECTION).clicked() {
			self.reset_collection();
		}
	});
}}

//---------------------------------------------------------------------------------------------------- Spinner (Exit)
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

			let text = RichText::new(format!("--- Saving ---\nForce exiting in {}...", atomic_load!(self.exit_countdown)))
				.size(hh / 4.0)
				.color(BONE);

			ui.add_sized([width, half], Label::new(text));
			ui.add_sized([width, hh], Spinner::new().size(hh / 2.0));
		});
	});
}}

//---------------------------------------------------------------------------------------------------- Spinner (Collection)
// This is a fullscreen spinner.
// Used when waiting on `Kernel` to hand over the new `Collection`.
//
// Activated when either resetting the `Collection` or at GUI startup.
impl Gui {
#[inline(always)]
fn show_collection_spinner(
	&mut self,
	ctx: &egui::Context,
	frame: &mut eframe::Frame,
	width: f32,
	height: f32,
	text: &'static str,
) {
	CentralPanel::default().show(ctx, |ui| {
		self.set_visuals(ui);
		ui.vertical_centered(|ui| {
			let half = height / 2.0;

			// Header.
			let text = RichText::new(text)
				.heading()
				.color(BONE);
			ui.add_sized([width, half], Label::new(text));

			let height = half / 6.0;

			// Spinner.
			ui.add_sized([width, height], Spinner::new().size(height));
			// Percent.
			ui.add_sized([width, height], Label::new(lockr!(self.reset_state).percent.as_str()));
			// Phase.
			ui.add_sized([width, height], Label::new(lockr!(self.reset_state).phase.as_str()));
			// Specific.
			ui.add_sized([width, height], Label::new(&lockr!(self.reset_state).specific));
			// ProgressBar.
			ui.add_sized([width / 1.1, height], ProgressBar::new(lockr!(self.reset_state).percent.inner() as f32 / 100.0));

		});
	});
}}

//---------------------------------------------------------------------------------------------------- Debug scren
// This is a fullscreen debug screen, showing
// a bunch of useful runtime information.
// Toggled with `CTRL+SHIFT+D`.
impl Gui {
#[inline(always)]
fn show_debug_screen(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, width: f32, height: f32) {
	CentralPanel::default().show(ctx, |ui| {
		self.set_visuals(ui);
		ui.vertical_centered(|ui| {
			let header = height / 25.0;

			let text = RichText::new("--- Debug Info ---\nCTRL+SHIFT+D to toggle")
				.size(header)
				.color(BONE);

			// Header.
			ui.add_sized([width, header], Label::new(text));

			// Save button.
			ui.add_space(header);
			if ui.add_sized([width, header], Button::new("Save to disk")).clicked() {
				match self.debug_info.save_atomic() {
					Ok(_)  => {
						let path = match crate::data::DebugInfo::absolute_path() {
							Ok(p) => p.display().to_string(),
							_ => String::from("???"),
						};
						ok!("GUI - DebugInfo @ {path}");
					},
					Err(e) => error!("GUI - DebugInfo save to disk error: {e}"),
				}
			}
			ui.add_space(header);

			// Debug Info.
			Frame::none().fill(BLACK).show(ui, |ui| {
				let width = ui.available_width();
				let height = ui.available_height();
				ScrollArea::vertical()
					.id_source("debug_info")
					.max_width(width)
					.max_height(height)
					.auto_shrink([false; 2])
					.show_viewport(ui, |ui, _|
				{
					ui.add_sized([width-20.0, height], TextEdit::multiline(&mut self.debug_info.as_str()));
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
