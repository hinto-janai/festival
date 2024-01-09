//---------------------------------------------------------------------------------------------------- Use
use readable::HeadTail;
use crate::data::Gui;
use egui::{
	ScrollArea,Frame,ProgressBar,TextStyle,RichText,
	TopBottomPanel,SidePanel,CentralPanel,
	Sense,
};
use egui::widgets::{
	Slider,Button,Spinner,TextEdit,
	SelectableLabel,Label,
};
use crate::data::{
	Tab,
	KeyPress,
	ALPHANUMERIC_KEY,
	EXIT_COUNTDOWN,
	SHOULD_EXIT,
};
use disk::Plain;
use log::{error,warn,info,debug};
use shukusai::{
	constants::FESTIVAL,
	kernel::{
		FrontendToKernel,
		KernelToFrontend,
	},
	state::{
		AUDIO_STATE,
		RESET_STATE,
		PLAYLISTS,
		Entry,
	},
	collection::KeyEnum,
};
use readable::{
	Unsigned,
	Runtime,
};
use benri::{
	log::*,
	sync::*,
	time::*,
	flip,
	debug_panic,
};
use std::time::Instant;
use crate::constants::{
	SLIDER_CIRCLE_INACTIVE,
	SLIDER_CIRCLE_HOVERED,
	SLIDER_CIRCLE_ACTIVE,
	UI_CONTROL_WIDTH,
	BONE,BLACK,
	YELLOW,GREEN,MEDIUM_GRAY,
	RUNTIME_WIDTH,
	SETTINGS_AUTO_SAVE_INTERVAL_SECS,
};
use crate::text::{
	HELP,MOD,DRAG_AND_DROP,
	EMPTY_COLLECTION,
	COLLECTION_LOADING,
	COLLECTION_RESETTING,
	UI_PREVIOUS,
	UI_PLAY,
	UI_PAUSE,
	UI_FORWARDS,
};
use strum::*;
use std::sync::Arc;

//---------------------------------------------------------------------------------------------------- `GUI`'s eframe impl.
impl eframe::App for Gui {
	//-------------------------------------------------------------------------------- On exit.
	fn on_close_event(&mut self) -> bool {
		// If already in the process of exiting, return,
		// else turn on `GUI`'s signal for exiting.
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
		let settings       = self.settings.clone();
		let state          = self.state.clone();

		// Spawn `exit` thread.
		std::thread::spawn(move || Self::exit(to_kernel, from_kernel, state, settings));

		// Set the exit `Instant`.
		self.exit_instant = Instant::now();

		// Don't exit here.
		false
	}

	//-------------------------------------------------------------------------------- Main event loop.
	#[inline(always)]
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		// If something has set this flag,
		// it means we should exit immediately.
		if atomic_load!(SHOULD_EXIT) {
			std::process::exit(0);
		}

		// If `souvlaki` sent an exit signal.
		if shukusai::state::media_controls_should_exit() {
			self.on_close_event();
		}

		// If `souvlaki` sent a `Raise` signal.
		if shukusai::state::media_controls_raise() {
			frame.request_user_attention(egui::output::UserAttentionType::Informational);
		}

		// Acquire a local copy of the `AUDIO_STATE`.
		let (queue_diff, audio_state_diff) = {
			let lock = AUDIO_STATE.read();
			let audio_state_diff = *lock != self.audio_state;
			let queue_diff = audio_state_diff && (lock.queue != self.audio_state.queue);
			lock.if_copy(&mut self.audio_state);
			(queue_diff, audio_state_diff)
		};
		// If the queue is different from ours,
		// recalculate the total `queue_time`.
		if self.kernel_returned && queue_diff &&
		/* needed to prevent panic */ !self.resetting_collection {
			self.queue_time = self.audio_state
				.queue
				.iter()
				.map(|k| self.collection.songs[k].runtime.inner() as u64)
				.sum::<u64>()
				.into();
		}
		// Auto-save state.
		if (self.kernel_returned && !self.resetting_collection) &&
			self.settings.auto_save && secs!(self.auto_save) > SETTINGS_AUTO_SAVE_INTERVAL_SECS
		{
			self.auto_save = now!();

			let settings  = self.diff_settings();
			let state     = self.diff_state();
			let playlists = self.diff_playlists();

			if !audio_state_diff && !settings && !state && !playlists {
				debug!("GUI - auto-save: no diffs, not saving");
			} else {
				use disk::Bincode2;
				if audio_state_diff {
					match self.audio_state.save_atomic() {
						Ok(_)  => ok!("GUI - Auto-saved AudioState"),
						Err(e) => fail!("GUI - AudioState auto-save: {e}"),
					}
				}

				if settings  { self.save_settings();  }
				if state     { self.save_state();     }
				if playlists { self.save_playlists(); }
			}
		}

		// HACK:
		// Unconditionally copy this.
		// See below `Volume` slider for more info on why this is a hack.
		//
		// We need to do this because the user might have
		// `festival --volume x`, where we now should update to `x`.
		self.state.volume = atomic_load!(shukusai::state::VOLUME);

		// Audio leeway.
		// HACK:
		// This leeway needs to be big to account for slower
		// computers, but it means there's a noticeable delay
		// when `GUI` starts up; the sliders will be out-of-sync.
		//
		// To workaround this, unconditionally copy if we
		// very recently started up. This is a hack because
		// now this `Instant` has to be calculated every
		// frame when it'll only matter in the startup.
		if secs!(shukusai::logger::INIT_INSTANT) < 1 || secs_f32!(self.audio_leeway) > 0.5 {
			self.state.repeat  = self.audio_state.repeat;
			self.audio_seek    = self.audio_state.elapsed.inner() as u64;
		}

		// Set resize leeway.
		let rect = ctx.available_rect();
		if self.rect != rect {
			self.rect = rect;
			self.resize_leeway = now!();
		}

		// HACK:
		// The real "update" function is surrounded by these timer
		// sets for better read/write locking behavior with CCD.
		atomic_store!(shukusai::frontend::gui::GUI_UPDATING, true);
		self.update(ctx, frame);
		atomic_store!(shukusai::frontend::gui::GUI_UPDATING, false);
	}
}

impl Gui {
	#[inline(always)]
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		// Check for `Kernel` messages.
		// Only if we're not exiting, to prevent stealing
		// the message intended for the exit thread.
		if !self.exiting {
			if let Ok(msg) = self.from_kernel.try_recv() {
				use KernelToFrontend::*;
				match msg {
					NewCollection(collection) => self.new_collection(collection),
					Failed((old_collection, err)) => {
						warn!("GUI - New Collection error: {err}");
						crate::toast_err!(self, format!("New Collection error: {err}"));
						self.new_collection(old_collection);
					},
					SearchResp(keychain) => {
						self.state.search_result = keychain;
						self.searching = false;
					},
					DeviceError(err) => {
						warn!("GUI - Audio device error: {err}");
						crate::toast_err!(self, format!("{err}"));
					},
					PlayError(err) => {
						warn!("GUI - Playback error: {err}");
						crate::toast_err!(self, format!("{err}"));
					},
					SeekError(err) => {
						warn!("GUI - Seek error: {err}");
						crate::toast_err!(self, format!("{err}"));
					},
					PathError((key, err)) => {
						let song = &self.collection.songs[key];
						warn!("GUI - Audio file error: {err} | Song: {song:#?}");
						crate::toast_err!(self, format!("Audio file error: {err} | Song: {}", song.title));
					},

					// These should never be received here.
					DropCollection => debug_panic!("incorrect gui recv(): `DropCollection`"),
					Exit(_)        => debug_panic!("incorrect gui recv(): `Exit`"),
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
				self.show_exit_spinner(ctx, width, height);
				return;
			}
			ctx.request_repaint();
		}

		// If `Kernel` hasn't finished startup yet,
		// show fullscreen spinner with info.
		if !self.kernel_returned {
			self.show_collection_spinner(ctx, width, height, COLLECTION_LOADING);
			return;
		}

		// If resetting the `Collection`,
		// show fullscreen spinner with info.
		if self.resetting_collection {
			self.show_collection_spinner(ctx, width, height, COLLECTION_RESETTING);
			return;
		}

		if self.last_song != self.audio_state.song {
			// Set window title.
			if let (Some(key), Some(index)) = (self.audio_state.song, self.audio_state.queue_idx) {
				let (artist, album, song) = self.collection.walk(key);
				frame.set_window_title(&self.settings.window_title.format(
					index + 1,
					self.audio_state.queue.len(),
					&song.runtime,
					&artist.name,
					&album.title,
					&song.title,
				));
			} else {
				frame.set_window_title(FESTIVAL);
			}

			// Set bottom UI runtime text width.
			self.runtime_width = match self.audio_state.runtime.as_str().len() {
				4|5|3|2|1|0 => RUNTIME_WIDTH,
				7|6         => RUNTIME_WIDTH + 65.0,
				_           => RUNTIME_WIDTH + 85.0,
			};
		}

		// Copied.
		self.last_song = self.audio_state.song;

		// Show `egui_notify` toasts.
		self.toasts.show(ctx);

		// Update local `modifiers` state.
		self.modifiers = ctx.input(|input| input.modifiers);

		// Handle raw input.
		if !ctx.wants_keyboard_input() && secs_f32!(self.resize_leeway) > 0.5 {
			use egui::{PointerButton,Modifiers,Key};

			//-------------------------------- Check for key presses.
			ctx.input_mut(|input| {
				// Last tab.
				if input.pointer.button_clicked(PointerButton::Extra1)  || // FIXME:
					input.pointer.button_clicked(PointerButton::Extra2) || // These two don't work with my mouse.
					input.consume_key(Modifiers::COMMAND, Key::D)
				{
					if let Some(tab) = self.state.last_tab {
						crate::tab!(self, tab);
					}
				}

				// Check for `Up/Down` (Tab switch)
				if input.consume_key(Modifiers::NONE, Key::ArrowDown) {
					crate::tab!(self, self.state.tab.next());
				} else if input.consume_key(Modifiers::NONE, Key::ArrowUp) {
					crate::tab!(self, self.state.tab.previous());
				// Check for `Left/Right`
				} else if input.consume_key(Modifiers::NONE, Key::ArrowRight) {
					match self.state.tab {
						Tab::View   => {
							if let Some(key) = self.state.album {
								self.state.album = Some(self.collection.next_album(key));
							}
						}
						Tab::Albums    => self.increment_art_size(),
						Tab::Artists   => self.state.artist_sub_tab = self.state.artist_sub_tab.next(),
						Tab::Playlists => self.state.playlist_sub_tab = self.state.playlist_sub_tab.next(),
						Tab::Search    => self.settings.search_sort = self.settings.search_sort.next(),
						_ => (),
					}
				} else if input.consume_key(Modifiers::NONE, Key::ArrowLeft) {
					match self.state.tab {
						Tab::View   => {
							if let Some(key) = self.state.album {
								self.state.album = Some(self.collection.previous_album(key));
							}
						}
						Tab::Albums => self.decrement_art_size(),
						Tab::Artists => self.state.artist_sub_tab = self.state.artist_sub_tab.previous(),
						Tab::Playlists => self.state.playlist_sub_tab = self.state.playlist_sub_tab.previous(),
						Tab::Search => self.settings.search_sort = self.settings.search_sort.previous(),
						_ => (),
					}
				// Check for `F11` (Fullscreen)
				} else if input.consume_key(Modifiers::NONE, Key::F11) {
					frame.set_fullscreen(!frame.info().window_info.fullscreen);
				// Check for `Ctrl+C` (Reset Collection)
				} else if input.consume_key(Modifiers::COMMAND, Key::C) {
					self.reset_collection();
				// Check for `Ctrl+S` (Save Settings)
				} else if input.consume_key(Modifiers::COMMAND, Key::S) {
					let settings  = self.diff_settings();
					let playlists = self.diff_playlists();

					if settings && playlists {
						match (self.save_settings(), self.save_playlists()) {
							(Ok(_), Ok(_)) => crate::toast!(self, "Saved settings & playlists"),
							(Err(e), _)    => crate::toast_err!(self, "Settings save failed: {e}"),
							(_, Err(e))    => crate::toast_err!(self, "Playlists save failed: {e}"),
						}
					} else if settings {
						match self.save_settings() {
							Ok(_)  => crate::toast!(self, "Saved settings"),
							Err(e) => crate::toast_err!(self, "Settings save failed: {e}"),
						}
					} else if playlists {
						match self.save_playlists() {
							Ok(_)  => crate::toast!(self, "Saved playlists"),
							Err(e) => crate::toast_err!(self, "Playlists save failed: {e}"),
						}
					} else {
						crate::toast!(self, "No changes to save");
					}
				// Check for `Ctrl+Z` (Reset Settings)
				} else if input.consume_key(Modifiers::COMMAND, Key::Z) {
					let settings  = self.diff_settings();
					let playlists = self.diff_playlists();
					if settings && playlists {
						self.reset_settings();
						self.reset_playlists();
						crate::toast!(self, "Reset settings & playlists");
					} else if settings {
						self.reset_settings();
						crate::toast!(self, "Reset settings");
					} else if playlists {
						self.reset_playlists();
						crate::toast!(self, "Reset playlists");
					} else {
						crate::toast!(self, "No changes to undo");
					}
				// Check for `Ctrl+A` (Add Folder)
				} else if input.consume_key(Modifiers::COMMAND, Key::A) {
					self.add_folder();
				// Check for `Ctrl+W` (Next Album Order)
				} else if input.consume_key(Modifiers::COMMAND, Key::W) {
					crate::tab!(self, Tab::Albums);
					let next = self.settings.album_sort.next();
					crate::toast!(self, next.human());
					self.settings.album_sort = next;
				// Check for `Ctrl+E` (Next Artist Order)
				} else if input.consume_key(Modifiers::COMMAND, Key::E) {
					self.state.artist_sub_tab = crate::data::ArtistSubTab::All;
					crate::tab!(self, Tab::Artists);
					let next = self.settings.artist_sort.next();
					crate::toast!(self, next.human());
					self.settings.artist_sort = next;
				// Check for `Ctrl+R` (Next Song Order)
				} else if input.consume_key(Modifiers::COMMAND, Key::R) {
					crate::tab!(self, Tab::Songs);
					let next = self.settings.song_sort.next();
					crate::toast!(self, next.human());
					self.settings.song_sort = next;
				// Check for `Ctrl+Shift+P` (force `panic!()`)
				} else if
					input.modifiers.matches(Modifiers::COMMAND.plus(Modifiers::SHIFT))
					&&
					input.key_pressed(Key::P)
				{
					info!("GUI - {MOD}+SHIFT+P was pressed, forcefully panic!()'ing...!");
					panic!("GUI - {MOD}+SHIFT+P force panic");
				// Check for `Ctrl+Shift+D` (toggle debug screen)
				} else if
					input.modifiers.matches(Modifiers::COMMAND.plus(Modifiers::SHIFT))
					&&
					input.key_pressed(Key::D)
				{
					flip!(self.debug_screen);
					// Refresh stats if true.
					if self.debug_screen {
						self.update_debug_info();
					}
				// Check for [ESC]
				} else if input.consume_key(Modifiers::NONE, Key::Escape) {
					self.debug_screen        = false;
					self.playlist_add_screen = None;
				// Check for [A-Za-z0-9] (Search)
				} else {
					for key in ALPHANUMERIC_KEY {
						if input.consume_key(Modifiers::NONE, key) {
							crate::search!(self, key, false);
						} else if input.consume_key(Modifiers::SHIFT, key) {
							crate::search!(self, key, true);
						}
					}
				}

				//-------------------------------- Check for drag+dropped files.
				// FIXME: We only handle 1 PATH per frame.
				if let Some(file) = input.raw.dropped_files.get_mut(0) {
					if let Some(path) = file.path.take() {
						if path.is_absolute() {
							let mut add = || {
								info!("GUI - Adding dropped PATH: {}", path.display());
								if self.collection.empty {
									self.settings.collection_paths.clear();
									self.settings.collection_paths.push(path.clone());
									self.reset_collection();
								} else {
									// HACK: this isn't RFD but it's used to reuse the existing path code.
									lock!(self.rfd_new).replace(path.to_path_buf());
									crate::toast!(self, format!("Added [{}] to Collection folders", path.display()));
								}
							};

							if path.is_dir() {
								add();
							} else if path.is_file() {
								if let Some(path) = path.parent() {
									add();
								}
							}
						}
					}

					input.raw.dropped_files.clear();
				}
			});
		}

		// We must show the spinner here again because after `CTRL+R`,
		// a few frames of the "Empty Collection" will flash.
		if self.resetting_collection {
			self.show_collection_spinner(ctx, width, height, COLLECTION_RESETTING);
			return;
		}

		// Determine if there is a diff in `Settings`'s.
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
				false => {
					self.settings.collection_paths.push(p);
					send!(self.to_kernel, FrontendToKernel::CachePath(self.settings.collection_paths.clone()));
				}
			}
		}

		// Show full-screen debug screen if `true`.
		if self.debug_screen {
			self.show_debug_screen(ctx, width, height);
			return;
		}

		// Show full-screen playlist screen if `true`.
		if let Some(key) = self.playlist_add_screen {
			self.show_playlist_add_screen(ctx, width, height, key);
			return;
		}

		// Size definitions of the major UI panels.
		let bottom_panel_height = (height / 15.0).clamp(50.0, 60.0);
		let side_panel_width    = (width / 8.0).clamp(125.0, 250.0);
		let side_panel_height   = height - (bottom_panel_height*2.0);

		// Bottom Panel
		self.show_bottom(ctx, width, bottom_panel_height);

		// Left Panel
		self.show_left(ctx, side_panel_width, side_panel_height);

		// If `Tab::View`, show right panel.
		if self.state.tab == Tab::View {
			self.show_tab_view_right_panel(self.state.album, ctx, side_panel_width, height);
		}

		// Central Panel
		self.show_central(ctx, width, height, side_panel_width, side_panel_height);
	}
}

//---------------------------------------------------------------------------------------------------- Bottom Panel
impl Gui {
#[inline(always)]
fn show_bottom(&mut self, ctx: &egui::Context, width: f32, height: f32) {
	TopBottomPanel::bottom("bottom").resizable(false).show(ctx, |ui| {
		self.set_visuals(ui);
		ui.set_height(height);

		// Base unit for sizing UI.
		let unit = width / 15.0;

		ui.horizontal(|ui| {
			// HACK:
			// There's a little bit a height space left after
			// adding these buttons, we want to expand the buttons
			// that it fills the full height.
			//
			// Add a fixed extra height to the button padding and button `height`.
			const EXTRA_HEIGHT: f32 = 8.0;
			ui.spacing_mut().button_padding.y += EXTRA_HEIGHT;

			// Media control buttons
			ui.group(|ui| {
				let height = height + EXTRA_HEIGHT;

				const SIZE: f32 = 33.0;

				if ui.add_sized([UI_CONTROL_WIDTH, height], Button::new(RichText::new(UI_PREVIOUS).size(SIZE))).clicked() {
					send!(self.to_kernel, FrontendToKernel::Previous(Some(self.settings.previous_threshold)));
					send!(self.to_kernel, FrontendToKernel::Play);
				}

				let play_pause = if self.audio_state.playing {
					UI_PAUSE
				} else {
					UI_PLAY
				};

				if ui.add_sized([UI_CONTROL_WIDTH, height], Button::new(RichText::new(play_pause).size(SIZE))).clicked() {
					send!(self.to_kernel, FrontendToKernel::Toggle);
				}

				if ui.add_sized([UI_CONTROL_WIDTH, height], Button::new(RichText::new(UI_FORWARDS).size(SIZE))).clicked() {
					send!(self.to_kernel, FrontendToKernel::Next);
					send!(self.to_kernel, FrontendToKernel::Play);
				}
			});

			// Return if we don't have a `SongKey`.
			let key = match self.audio_state.song {
				Some(k) => k,
				_ => return,
			};
			let (artist, album, song) = self.collection.walk(key);

			// Album button.
			ui.group(|ui| {
				// Album button.
				crate::no_rounding!(ui);
				crate::album_button!(self, album, song.album, ui, ctx, height, "");

				ui.vertical(|ui| {
					// How many char's before we need
					// to cut off the song title?
					// (scales based on pixels available).
					let head = (unit / 5.0) as usize;

					let text_style = TextStyle::Name("12.5".into());

					//-------------------------------------------------- `Song` title.
					let song_head  = song.title.head_dot(head);
					let chopped    = &*song.title != song_head;
					let song_title = Label::new(
						RichText::new(song_head)
							.text_style(text_style.clone())
							.color(BONE)
					);
					// Show the full title on hover
					// if we chopped it with head.
					if chopped {
						ui.add(song_title).on_hover_text(&*song.title);
					} else {
						ui.add(song_title);
					}

					//-------------------------------------------------- `Album` name.
					let album_head = album.title.head_dot(head);
					let chopped    = &*album.title != album_head;
					let album_name = Label::new(
						RichText::new(album_head)
							.text_style(text_style.clone())
					);
					if chopped {
						ui.add(album_name).on_hover_text(&*album.title);
					} else {
						ui.add(album_name);
					}

					//-------------------------------------------------- `Artist` name.
					let artist_head = artist.name.head_dot(head);
					let chopped     = &*artist.name != artist_head;
					let artist_name = Label::new(
						RichText::new(artist_head)
							.text_style(text_style)
					);
					if chopped {
						if ui.add(artist_name.sense(Sense::click())).on_hover_text(&*artist.name).clicked() {
							crate::artist!(self, album.artist);
						}
					} else {
						if ui.add(artist_name.sense(Sense::click())).clicked() {
							crate::artist!(self, album.artist);
						}
					}
				});
			});

			// Leave space for the runtime at the end.
			let width = ui.available_width() - self.runtime_width;

			// Slider (playback)
			ui.spacing_mut().slider_width = width;
			{
				let v = &mut ui.visuals_mut().widgets;
				v.inactive.fg_stroke = SLIDER_CIRCLE_INACTIVE;
				v.hovered.fg_stroke  = SLIDER_CIRCLE_HOVERED;
				v.active.fg_stroke   = SLIDER_CIRCLE_ACTIVE;
			}

			let h = height / 5.0;

			// Runtime/seek slider.
			let resp = ui.add_sized(
				[width, height],
				Slider::new(&mut self.audio_seek, 0..=self.audio_state.runtime.inner() as u64)
					.smallest_positive(1.0)
					.show_value(false)
					.thickness(h*2.0)
					.circle_size(h)
			);

			// Only send signal if the slider was dragged + released.
			if resp.drag_released() {
				// If we dragged to the very last second, just skip.
				if self.audio_seek == self.audio_state.runtime.inner() as u64 {
					debug!("GUI - Seeked to last second, sending Next");
					send!(self.to_kernel, FrontendToKernel::Next);
				} else {
					self.audio_leeway = now!();
					send!(self.to_kernel, FrontendToKernel::Seek((shukusai::audio::Seek::Absolute, self.audio_seek)));
				}
			}

			let time = format!("{} / {}", readable::Runtime::from(self.audio_seek), self.audio_state.runtime);
			ui.add_sized([ui.available_width(), height], Label::new(time));

		});
	});
}}

//---------------------------------------------------------------------------------------------------- Left Panel
impl Gui {
#[inline(always)]
fn show_left(&mut self, ctx: &egui::Context, width: f32, height: f32) {
	SidePanel::left("left").resizable(false).show(ctx, |ui| {
		self.set_visuals(ui);
		ui.set_width(width);
		ui.set_height(height);

		// Size definitions of the elements within the left panel.
		let half_height = height / 2.0;
		let tab_height  = half_height / 9.0;
		let tab_width   = width / 1.2;

		// Main UI
		ui.vertical_centered_justified(|ui| {

			// Display `SelectableLabel` for each `Tab`.
			ui.add_space(2.5);
			for tab in Tab::iter() {
				if ui.add_sized([tab_width, tab_height], SelectableLabel::new(self.state.tab == tab, tab.human())).clicked() {
					crate::tab!(self, tab);
				}
				ui.separator();
			}

			// Volume slider
			//
			// This is subtracted by a magic number to allow
			// for space for the number label below it.
			let slider_height = ui.available_height() - 42.0;

			ui.add_space(2.5);

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

				// Volume slider.
				let resp = ui.add(Slider::new(&mut self.state.volume, 0..=100)
					.smallest_positive(1.0)
					.show_value(false)
					.vertical()
					.thickness(unit*2.0)
					.circle_size(unit)
				);

				// Send signal if the slider was/is being dragged.
				if resp.dragged() {
					// HACK:
					// THIS IS VERY VERY VERY TERRIBLE, REALLY BAD CODE.
					//
					// This used to be a `send!([ ... volume ... ])` but
					// that actually ended up causing a DoS on `Audio` since
					// we would be sending an insane amount of messages.
					//
					// To combat this specifically, this exact slider, `shukusai`
					// now uses a global, mutable `AtomicU8` as the global "Volume"
					// which `Audio` will use instead of `AUDIO_STATE`.
					//
					// Please please please please rewrite `Audio`.
					atomic_store!(shukusai::state::VOLUME, self.state.volume);
				// If scrolled up/down, send signal.
				} else if resp.hovered() {
					ctx.input_mut(|input| {
						for event in input.events.iter() {
							if let egui::Event::Scroll(vec2) = event {
								if vec2.y.is_sign_positive() {
									self.add_volume(1);
								} else if vec2.y.is_sign_negative() {
									self.sub_volume(1);
								}
								break;
							}
						}
					});
				}
			});

			// Volume %.
			ui.label(readable::itoa!(self.state.volume));
		});
	});
}}

//---------------------------------------------------------------------------------------------------- Central Panel
impl Gui {
#[inline(always)]
fn show_central(&mut self, ctx: &egui::Context, width: f32, height: f32, side_panel_width: f32, side_panel_height: f32) {
	CentralPanel::default().show(ctx, |ui| {
		self.set_visuals(ui);

		// Handle empty `Collection`.
		if self.collection.empty && self.state.tab != Tab::Settings {
			self.show_empty_collection(ui, ctx, width, height);
			return;
		}

		match self.state.tab {
			Tab::View      => self.show_tab_view(ui, ctx, width, height),
			Tab::Albums    => self.show_tab_albums(ui, ctx, width, height),
			Tab::Artists   => self.show_tab_artists(ui, ctx, width, height),
			Tab::Songs     => self.show_tab_songs(ui, ctx, width, height),
			Tab::Queue     => self.show_tab_queue(ui, ctx, width, height),
			Tab::Playlists => self.show_tab_playlists(ui, ctx, width, height),
			Tab::Search    => self.show_tab_search(ui, ctx, width, height),
			Tab::Settings  => self.show_tab_settings(ui, ctx, width, height),
		}
	});
}}

//---------------------------------------------------------------------------------------------------- Empty Collection
// Shows a button that resets the `Collection` and some help text.
// Used for tabs when the `Collection` is empty.
impl Gui {
#[inline(always)]
fn show_empty_collection(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, width: f32, height: f32) {
	// Handle empty or no `Collection`.
	let button = Button::new(RichText::new("🔃 Empty Collection. Click to scan.").color(BONE));
	let width  = width / 1.5;
	let height = height / 10.0;

	ui.vertical_centered(|ui| {
		ui.add_space(height / 2.0);

		if ui.add_sized([width, height], button).on_hover_text(EMPTY_COLLECTION).clicked() {
			self.reset_collection();
		}

		ui.add_space(ui.available_height() / 3.0);

		ui.label(DRAG_AND_DROP);
	});
}}

//---------------------------------------------------------------------------------------------------- Spinner (Exit)
// This is a fullscreen spinner.
// Used when exiting and waiting for everything to save.
impl Gui {
#[inline(always)]
fn show_exit_spinner(&mut self, ctx: &egui::Context, width: f32, height: f32) {
	CentralPanel::default().show(ctx, |ui| {
		self.set_visuals(ui);
		ui.vertical_centered(|ui| {
			let half = height / 2.0;
			let hh   = half / 2.0;

			let text = RichText::new(format!("--- Saving ---\n\nForce exiting in {}", atomic_load!(EXIT_COUNTDOWN)))
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
	width: f32,
	height: f32,
	text: &'static str,
) {
	// Update local copy of `RESET_STATE`.
	if let Ok(reset_state) = RESET_STATE.try_read() {
		self.reset_state = reset_state.clone();
	}

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
			if self.reset_state.phase == shukusai::state::Phase::Finalize {
				ui.add_space(height + 2.5);
			} else {
				ui.add_sized([width, height], Spinner::new().size(height));
			}
			// Percent.
			ui.add_sized([width, height], Label::new(self.reset_state.percent.as_str()));
			// Phase.
			ui.add_sized([width, height], Label::new(self.reset_state.phase.human()));
			// Specific.
			ui.add_sized([width, height], Label::new(&*self.reset_state.specific));
			// ProgressBar.
			ui.add_sized([width / 1.1, height], ProgressBar::new(self.reset_state.percent.inner() as f32 / 100.0));

		});
	});
}}

//---------------------------------------------------------------------------------------------------- Playlist add
// This is a fullscreen screen for adding a (Artist|Album|Song)key to a playlist.
impl Gui {
#[inline(always)]
fn show_playlist_add_screen(&mut self, ctx: &egui::Context, width: f32, height: f32, key: KeyEnum) {
	CentralPanel::default().show(ctx, |ui| {
		self.set_visuals(ui);
		ui.vertical_centered(|ui| {
			let header = height / 25.0;

			// Exit button.
			ui.add_space(header);
			if ui.add_sized([width / 1.5, header], Button::new("Exit (or press ESC)")).clicked() {
				self.playlist_add_screen = None;
			}
			ui.add_space(header);

			ui.separator();
			ui.add_space(header);

			let (object, name) = match key {
				KeyEnum::Artist(k) => ("Artist", &self.collection.artists[k].name),
				KeyEnum::Album(k) => ("Album", &self.collection.albums[k].title),
				KeyEnum::Song(k) => ("Song", &self.collection.songs[k].title),
			};

			let text = RichText::new(format!("[{name}]"))
				.size(header)
				.color(BONE);

			// Header.
			ui.label(format!("Add {object}"));
			ui.add_sized([width, header], Label::new(text));
			ui.label("to playlist...");

			ui.add_space(header);

			if PLAYLISTS.read().is_empty() {
				let button = Button::new(
					RichText::new("Create a new playlist")
					.text_style(TextStyle::Name("30".into()))
				);

				if ui.add_sized([width / 2.0, 35.0], button).clicked() {
					self.playlist_add_screen = None;
					self.state.playlist_sub_tab = crate::data::PlaylistSubTab::All;
					crate::tab!(self, Tab::Playlists);
				}
			}

			ScrollArea::both()
				.id_source("PlaylistsAddScreen")
				.max_width(width)
				.max_height(height)
				.auto_shrink([false; 2])
				.show_viewport(ui, |ui, _|
			{
				let width = ui.available_width() - 10.0;

				for playlist_name in PLAYLISTS.read().keys() {
					// `Playlist` name.
					let button = Button::new(
						RichText::new(&**playlist_name)
						.text_style(TextStyle::Name("30".into()))
					);

					if ui.add_sized([width, 35.0], button).clicked() {
						self.playlist_add_screen_result = Some((Arc::clone(playlist_name), key));
					}

					ui.add_space(10.0);
				}
			});

			if let Some((playlist_name, key)) = self.playlist_add_screen_result.take() {
				if let Some(playlist) = PLAYLISTS.write().get_mut(&playlist_name) {
					match key {
						KeyEnum::Artist(k) => Entry::valid_from_artist(k, &self.collection).into_iter().for_each(|e| playlist.push_back(e)),
						KeyEnum::Album(k) => Entry::valid_from_album(k, &self.collection).into_iter().for_each(|e| playlist.push_back(e)),
						KeyEnum::Song(k) => playlist.push_back(Entry::valid_from_song(k, &self.collection)),
					}

					crate::toast!(self, format!("Added {object} [{name}] to playlist [{playlist_name}]"));
				}

				self.playlist_add_screen = None;
			}
		});
	});
}}

//---------------------------------------------------------------------------------------------------- Debug screen
// This is a fullscreen debug screen, showing
// a bunch of useful runtime information.
// Toggled with `CTRL+SHIFT+D`.
impl Gui {
#[inline(always)]
fn show_debug_screen(&mut self, ctx: &egui::Context, width: f32, height: f32) {
	CentralPanel::default().show(ctx, |ui| {
		self.set_visuals(ui);
		ui.vertical_centered(|ui| {
			let header = height / 25.0;

			let text = RichText::new(const_format::formatcp!("--- Debug Info ---\n{MOD}+SHIFT+D to toggle"))
				.size(header)
				.color(BONE);

			// Header.
			ui.add_sized([width, header], Label::new(text));

			// Save button.
			ui.add_space(header);
			if ui.add_sized([width / 1.5, header], Button::new("Save to disk")).clicked() {
				match self.debug_info.save_atomic() {
					Ok(md)  => ok!("GUI - {md}"),
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
