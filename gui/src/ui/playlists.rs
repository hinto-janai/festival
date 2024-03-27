//---------------------------------------------------------------------------------------------------- Use
use crate::{
    constants::{BONE, GRAY, GREEN, MEDIUM_GRAY, PLAYLIST_NAME_MAX_LEN, RED, YELLOW},
    data::PlaylistSubTab,
    text::{
        PLAYLIST_COPY, PLAYLIST_COUNT, PLAYLIST_CREATE, PLAYLIST_DELETE, PLAYLIST_EDIT,
        PLAYLIST_EDIT_SAVE, PLAYLIST_EMPTY, PLAYLIST_ENTRY_DELETE, PLAYLIST_ENTRY_DOWN,
        PLAYLIST_ENTRY_UP, PLAYLIST_EXISTS, PLAYLIST_INVALID, PLAYLIST_TEXT, PLAYLIST_TEXT_EMPTY,
        PLAYLIST_TOTAL_RUNTIME, PLAYLIST_TOTAL_SONG, SELECT_PLAYLIST, UI_DOWN, UI_MINUS, UI_PLUS,
        UI_UP,
    },
};
use egui::{Button, Label, RichText, ScrollArea, SelectableLabel, Sense, TextEdit, TextStyle};
use egui_extras::{Column, TableBuilder};
use log::warn;
use readable::HeadTail;
use readable::{Runtime, Unsigned};
use shukusai::state::Entry;
use std::sync::Arc;

//---------------------------------------------------------------------------------------------------- Artists
impl crate::data::Gui {
    #[inline(always)]
    pub fn show_tab_playlists(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        width: f32,
        height: f32,
    ) {
        self.set_visuals(ui);

        // Sizing.
        let width = ui.available_width();
        let height = ui.available_height();

        //-------------------------------------------------- Artist sub-tab.
        ui.group(|ui| {
            ui.horizontal(|ui| {
                let width = (width / 2.0) - 20.0;

                {
                    const TAB: PlaylistSubTab = PlaylistSubTab::All;
                    let label =
                        SelectableLabel::new(self.state.playlist_sub_tab == TAB, TAB.human());
                    if ui.add_sized([width, 30.0], label).clicked() {
                        self.state.playlist_sub_tab = TAB;
                    }
                }

                ui.separator();

                {
                    const TAB: PlaylistSubTab = PlaylistSubTab::View;
                    let label = match &self.state.playlist {
                        Some(name) => {
                            let name = name.head_dot(18);
                            SelectableLabel::new(self.state.playlist_sub_tab == TAB, name)
                        }
                        None => {
                            SelectableLabel::new(self.state.playlist_sub_tab == TAB, TAB.human())
                        }
                    };

                    if ui.add_sized([width, 30.0], label).clicked() {
                        self.state.playlist_sub_tab = TAB;
                    }
                }
            })
        });

        ui.add_space(10.0);

        // Sizing.
        let width = ui.available_width();
        let height = ui.available_height();

        //-------------------------------------------------- Acquire playlist lock.
        let mut playlists = shukusai::state::PLAYLISTS.write();

        //-------------------------------------------------- All artists
        match self.state.playlist_sub_tab {
            PlaylistSubTab::All => {
                ScrollArea::both()
                    .id_source("Playlists")
                    .max_width(width)
                    .max_height(height)
                    .auto_shrink([false; 2])
                    .show_viewport(ui, |ui, _| {
                        const SIZE: f32 = 35.0;
                        const SIZE2: f32 = 50.0;

                        //-------------------------------------------------- Playlist add/remove text edit.
                        ui.horizontal(|ui| {
                            ui.group(|ui| {
                                if self.state.playlist_string.is_empty() {
                                    ui.scope(|ui| {
                                        ui.set_enabled(false);
                                        let button = Button::new(RichText::new(UI_PLUS).size(SIZE));
                                        ui.add_sized([SIZE2, SIZE2], button)
                                            .on_disabled_hover_text(PLAYLIST_TEXT_EMPTY);
                                    });
                                } else {
                                    ui.scope(|ui| {
                                        ui.set_enabled(
                                            !playlists
                                                .contains_key(self.state.playlist_string.as_str()),
                                        );

                                        // Add button.
                                        let button = Button::new(RichText::new(UI_PLUS).size(SIZE));
                                        if ui
                                            .add_sized([SIZE2, SIZE2], button)
                                            .on_hover_text(PLAYLIST_CREATE)
                                            .on_disabled_hover_text(PLAYLIST_EXISTS)
                                            .clicked()
                                        {
                                            let string =
                                                std::mem::take(&mut self.state.playlist_string);
                                            playlists.playlist_new(&string);
                                        }
                                    });
                                }

                                // Playlist count
                                let text = Label::new(
                                    RichText::new(format!("[{}]", playlists.len()))
                                        .color(BONE)
                                        .text_style(TextStyle::Name("30".into())),
                                );
                                ui.add_sized([SIZE2, SIZE2], text)
                                    .on_hover_text(PLAYLIST_COUNT);

                                // Text edit.
                                let width = ui.available_width();
                                let text_edit =
                                    TextEdit::singleline(&mut self.state.playlist_string)
                                        .char_limit(PLAYLIST_NAME_MAX_LEN)
                                        .hint_text("Enter playlist name...");
                                ui.spacing_mut().text_edit_width = width;
                                let resp = ui
                                    .add_sized([width, SIZE], text_edit)
                                    .on_hover_text(PLAYLIST_TEXT);

                                // Check `[enter]` and add.
                                if resp.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                    && !self.state.playlist_string.is_empty()
                                {
                                    if !playlists.contains_key(self.state.playlist_string.as_str())
                                    {
                                        let string =
                                            std::mem::take(&mut self.state.playlist_string);
                                        playlists.playlist_new(&string);
                                    }
                                }
                            })
                        });

                        ui.add_space(10.0);

                        // For each `Playlist`...
                        for (playlist_name, playlist) in playlists.iter() {
                            // `Playlist` name.
                            let label_name = Label::new(
                                RichText::new(&**playlist_name)
                                    .text_style(TextStyle::Name("30".into())),
                            );

                            // `Playlist` entry count.
                            let label_count = Label::new(
                                RichText::new(
                                    Unsigned::from(shukusai::state::Playlists::valid_len(
                                        &playlist,
                                    ))
                                    .as_str(),
                                )
                                .color(MEDIUM_GRAY)
                                .text_style(TextStyle::Name("25".into())),
                            );

                            // `Playlist` runtime.
                            let runtime: usize = playlist
                                .iter()
                                .map(|v| match v {
                                    Entry::Valid { key_song, .. } => {
                                        self.collection.songs[key_song].runtime.usize()
                                    }
                                    _ => 0,
                                })
                                .sum();
                            let label_runtime = Label::new(
                                RichText::new(Runtime::from(runtime).as_str())
                                    .color(MEDIUM_GRAY)
                                    .text_style(TextStyle::Name("25".into())),
                            );

                            let playlist_name_is_being_edited = self
                                .state
                                .playlist_edit
                                .as_ref()
                                .is_some_and(|x| x == playlist_name);

                            ui.horizontal(|ui| {
                                ui.group(|ui| {
                                    let button = Button::new(RichText::new(UI_MINUS).size(SIZE));
                                    if ui
                                        .add_sized([SIZE2, SIZE2], button)
                                        .on_hover_text(PLAYLIST_DELETE)
                                        .clicked()
                                    {
                                        self.playlist_remove = Some(Arc::clone(&playlist_name));
                                    }

                                    if playlist_name_is_being_edited {
                                        ui.scope(|ui| {
                                            let ok_to_save = !self
                                                .state
                                                .playlist_edit_string
                                                .is_empty()
                                                && (self.state.playlist_edit.as_ref().is_some_and(
                                                    |s| &**s == self.state.playlist_edit_string,
                                                ) || playlists
                                                    .get(self.state.playlist_edit_string.as_str())
                                                    .is_none());

                                            ui.set_enabled(ok_to_save);

                                            let button =
                                                Button::new(RichText::new("🗋").size(SIZE - 5.0));
                                            let hover =
                                                if self.state.playlist_edit_string.is_empty() {
                                                    PLAYLIST_EMPTY
                                                } else {
                                                    PLAYLIST_EXISTS
                                                };
                                            let resp = ui
                                                .add_sized([SIZE2, SIZE2], button)
                                                .on_hover_text(PLAYLIST_EDIT_SAVE)
                                                .on_disabled_hover_text(hover);
                                            if resp.clicked()
                                                || (self.playlist_name_edit_enter && ok_to_save)
                                            {
                                                self.playlist_name_edit_enter = false;
                                                self.state.playlist_edit = None;
                                                let arc_str: Arc<str> = std::mem::take(
                                                    &mut self.state.playlist_edit_string,
                                                )
                                                .into();
                                                self.playlist_from =
                                                    Some(Arc::clone(&playlist_name));
                                                self.playlist_to = Some(arc_str);
                                            }
                                        });
                                    } else {
                                        let button =
                                            Button::new(RichText::new("Ａ").size(SIZE - 5.0));
                                        if ui
                                            .add_sized([SIZE2, SIZE2], button)
                                            .on_hover_text(PLAYLIST_EDIT)
                                            .clicked()
                                        {
                                            self.state.playlist_edit =
                                                Some(Arc::clone(&playlist_name));
                                            self.state.playlist_edit_string =
                                                playlist_name.to_string();
                                        }
                                    }

                                    let button = Button::new(RichText::new("🗐").size(SIZE - 5.0));
                                    if ui
                                        .add_sized([SIZE2, SIZE2], button)
                                        .on_hover_text(PLAYLIST_COPY)
                                        .clicked()
                                    {
                                        self.playlist_clone = Some(Arc::clone(playlist_name));
                                    }

                                    ui.add_space(15.0);

                                    if playlist_name_is_being_edited {
                                        // Text edit.
                                        let width = ui.available_width() - 10.0;
                                        let text_edit = TextEdit::singleline(
                                            &mut self.state.playlist_edit_string,
                                        )
                                        .char_limit(PLAYLIST_NAME_MAX_LEN);
                                        ui.spacing_mut().text_edit_width = width;
                                        let resp = ui
                                            .add_sized([width, SIZE], text_edit)
                                            .on_hover_text(PLAYLIST_TEXT);
                                        self.playlist_name_edit_enter = resp.lost_focus()
                                            && ui.input(|i| i.key_pressed(egui::Key::Enter));
                                    } else {
                                        crate::playlist_label!(self, playlist_name, ui, label_name);
                                        ui.add_space(20.0);
                                        ui.add(label_count).on_hover_text(PLAYLIST_TOTAL_SONG);
                                        ui.add_space(20.0);
                                        ui.add(label_runtime).on_hover_text(PLAYLIST_TOTAL_RUNTIME);
                                    }

                                    ui.add_space(ui.available_width());
                                })
                            });
                            ui.add_space(10.0);
                        }

                        // Clone playlist if set.
                        match self.playlist_clone.take() {
                            None => (),
                            Some(name) => {
                                // Clone old.
                                let maybe_playlist = playlists.get(&name).map(|v| v.clone());

                                // Create new.
                                if let Some(vec) = maybe_playlist {
                                    // Prevent overwriting existing playlists.
                                    let mut copy = "(Copy)".to_string();
                                    let new_name = loop {
                                        let new_name = format!("{name} {copy}");
                                        if playlists.get(new_name.as_str()).is_none() {
                                            break new_name;
                                        }
                                        copy += " (Copy)";
                                    };

                                    playlists.insert(new_name.into(), vec);
                                }
                            }
                        }

                        // Remove playlist if set.
                        match self.playlist_remove.take() {
                            None => (),
                            Some(p) => {
                                playlists.remove(&p);
                            }
                        }

                        // Swap keys if we renamed them above.
                        match (&mut self.playlist_from, &mut self.playlist_to) {
                            (Some(from), Some(to)) => {
                                if let Some(value) = playlists.remove(&**from) {
                                    playlists.insert(Arc::clone(to), value);
                                }

                                self.playlist_from = None;
                                self.playlist_to = None;
                            }
                            _ => (),
                        }
                    });
            }

            //-------------------------------------------------- View
            PlaylistSubTab::View => {
                let mut return_on_none = || {
                    let label = Label::new(RichText::new(SELECT_PLAYLIST).color(GRAY));
                    ui.add_sized([width, height], label);
                };

                let Some(arc_str) = &self.state.playlist else {
                    return_on_none();
                    return;
                };
                let arc_str = std::sync::Arc::clone(&arc_str);

                let Some(playlist) = playlists.get(&arc_str) else {
                    return_on_none();
                    return;
                };

                // `Playlist` name.
                let label_name =
                    Label::new(RichText::new(&*arc_str).text_style(TextStyle::Name("30".into())));

                // `Playlist` entry count.
                let label_count = Label::new(
                    RichText::new(
                        Unsigned::from(shukusai::state::Playlists::valid_len(&playlist)).as_str(),
                    )
                    .color(MEDIUM_GRAY)
                    .text_style(TextStyle::Name("25".into())),
                );

                // `Playlist` runtime.
                let runtime: usize = playlist
                    .iter()
                    .map(|v| match v {
                        Entry::Valid { key_song, .. } => {
                            self.collection.songs[key_song].runtime.usize()
                        }
                        _ => 0,
                    })
                    .sum();
                let label_runtime = Label::new(
                    RichText::new(Runtime::from(runtime).as_str())
                        .color(MEDIUM_GRAY)
                        .text_style(TextStyle::Name("25".into())),
                );

                // `.show_rows()` is slightly faster than
                // `.show_viewport()` but we need to know
                // exactly how many rows we need to paint.
                //
                // The below needs to account for the scrollbar height,
                // the title heights and must not overflow to the bottom bar.
                const HEADER_HEIGHT: f32 = 80.0;
                const ROW_HEIGHT: f32 = 35.0;
                let height = ui.available_height();
                let max_rows = ((height - (HEADER_HEIGHT - 5.0)) / ROW_HEIGHT) as usize;
                let row_range = 0..max_rows;

                ScrollArea::horizontal()
                    .id_source("PlaylistView")
                    .max_width(f32::INFINITY)
                    .max_height(f32::INFINITY)
                    .auto_shrink([false; 2])
                    .show_rows(ui, ROW_HEIGHT, max_rows, |ui, row_range| {
                        ui.horizontal(|ui| {
                            crate::playlist_label!(self, arc_str, ui, label_name);
                            ui.add_space(20.0);
                            ui.add(label_count).on_hover_text(PLAYLIST_TOTAL_SONG);
                            ui.add_space(20.0);
                            ui.add(label_runtime).on_hover_text(PLAYLIST_TOTAL_RUNTIME);
                        });

                        ui.add_space(10.0);
                        ui.separator();

                        ui.push_id("PlaylistViewInner", |ui| {
                            // Sizing.
                            let width = ui.available_width();
                            let height = ui.available_height();
                            const SIZE: f32 = 35.0;
                            // c == Column sizing
                            let c_width = width / 10.0;
                            let c_buttons = (SIZE * 3.0) + 20.0;
                            let c_runtime = c_width;
                            let c_title = c_width * 4.0;
                            let c_album = c_width * 2.0;
                            let c_artist = c_width * 2.0;

                            TableBuilder::new(ui)
                                .striped(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(Column::initial(c_buttons).resizable(true).clip(false))
                                .column(Column::initial(c_title).resizable(true).clip(true))
                                .column(Column::initial(c_runtime).resizable(true).clip(true))
                                .column(Column::initial(c_album).resizable(true).clip(true))
                                //				.column(Column::initial(c_artist).resizable(true).clip(true))
                                .column(Column::remainder().clip(true))
                                .auto_shrink([false; 2])
                                .max_scroll_height(height)
                                .header(HEADER_HEIGHT, |mut header| {
                                    header.col(|ui| {
                                        ui.strong("");
                                    });
                                    header.col(|ui| {
                                        ui.strong("Song");
                                    });
                                    header.col(|ui| {
                                        ui.strong("Runtime");
                                    });
                                    header.col(|ui| {
                                        ui.strong("Album");
                                    });
                                    header.col(|ui| {
                                        ui.strong("Artist");
                                    });
                                })
                                .body(|mut body| {
                                    for (offset, entry) in playlist.iter().enumerate() {
                                        match entry {
                                            Entry::Valid { key_song, .. } => {
                                                let key = key_song;
                                                body.row(ROW_HEIGHT, |mut row| {
                                                    let (artist, album, song) =
                                                        self.collection.walk(key);

                                                    row.col(|ui| {
                                                        // Buttons.
                                                        if ui
                                                            .add_sized(
                                                                [SIZE, SIZE],
                                                                Button::new(UI_MINUS),
                                                            )
                                                            .on_hover_text(PLAYLIST_ENTRY_DELETE)
                                                            .clicked()
                                                        {
                                                            self.playlist_remove_entry = Some((
                                                                Arc::clone(&arc_str),
                                                                offset,
                                                            ));
                                                        } else if ui
                                                            .add_sized(
                                                                [SIZE, SIZE],
                                                                Button::new(UI_DOWN),
                                                            )
                                                            .on_hover_text(PLAYLIST_ENTRY_DOWN)
                                                            .clicked()
                                                        {
                                                            let to = offset + 1;
                                                            if to != playlist.len() {
                                                                self.playlist_swap_entry = Some((
                                                                    Arc::clone(&arc_str),
                                                                    offset,
                                                                    to,
                                                                ));
                                                            }
                                                        } else if ui
                                                            .add_sized(
                                                                [SIZE, SIZE],
                                                                Button::new(UI_UP),
                                                            )
                                                            .on_hover_text(PLAYLIST_ENTRY_UP)
                                                            .clicked()
                                                        {
                                                            self.playlist_swap_entry = Some((
                                                                Arc::clone(&arc_str),
                                                                offset,
                                                                offset.saturating_sub(1),
                                                            ));
                                                        }
                                                    });

                                                    row.col(|ui| {
                                                        let resp = ui.add(
                                                            Label::new(&*song.title)
                                                                .sense(Sense::click()),
                                                        );
                                                        if resp.clicked() {
                                                            crate::play_playlist_offset!(
                                                                self,
                                                                Arc::clone(&arc_str),
                                                                offset
                                                            );
                                                        } else if resp.secondary_clicked() {
                                                            crate::add_song!(
                                                                self,
                                                                &*song.title,
                                                                *key
                                                            );
                                                        }
                                                    });

                                                    row.col(|ui| {
                                                        ui.label(song.runtime.as_str());
                                                    });

                                                    row.col(|ui| {
                                                        crate::album_label!(
                                                            self,
                                                            album,
                                                            song.album,
                                                            ui,
                                                            Label::new(&*album.title)
                                                        );
                                                    });

                                                    row.col(|ui| {
                                                        crate::artist_label!(
                                                            self,
                                                            artist,
                                                            album.artist,
                                                            ui,
                                                            Label::new(&*artist.name)
                                                        );
                                                    });
                                                });
                                            }
                                            Entry::Invalid {
                                                artist,
                                                album,
                                                song,
                                            } => {
                                                body.row(ROW_HEIGHT, |mut row| {
                                                    row.col(|ui| {
                                                        // Buttons.
                                                        if ui
                                                            .add_sized(
                                                                [SIZE, SIZE],
                                                                Button::new(UI_MINUS),
                                                            )
                                                            .on_hover_text(PLAYLIST_ENTRY_DELETE)
                                                            .clicked()
                                                        {
                                                            self.playlist_remove_entry = Some((
                                                                Arc::clone(&arc_str),
                                                                offset,
                                                            ));
                                                        } else if ui
                                                            .add_sized(
                                                                [SIZE, SIZE],
                                                                Button::new(UI_DOWN),
                                                            )
                                                            .on_hover_text(PLAYLIST_ENTRY_DOWN)
                                                            .clicked()
                                                        {
                                                            let to = offset + 1;
                                                            if to != playlist.len() {
                                                                self.playlist_swap_entry = Some((
                                                                    Arc::clone(&arc_str),
                                                                    offset,
                                                                    to,
                                                                ));
                                                            }
                                                        } else if ui
                                                            .add_sized(
                                                                [SIZE, SIZE],
                                                                Button::new(UI_UP),
                                                            )
                                                            .on_hover_text(PLAYLIST_ENTRY_UP)
                                                            .clicked()
                                                        {
                                                            self.playlist_swap_entry = Some((
                                                                Arc::clone(&arc_str),
                                                                offset,
                                                                offset.saturating_sub(1),
                                                            ));
                                                        }
                                                    });

                                                    row.col(|ui| {
                                                        ui.add(Label::new(
                                                            RichText::new(&**song).color(YELLOW),
                                                        ))
                                                        .on_hover_text(PLAYLIST_INVALID);
                                                    });

                                                    row.col(|ui| {
                                                        ui.add(Label::new(
                                                            RichText::new("??:??").color(YELLOW),
                                                        ))
                                                        .on_hover_text(PLAYLIST_INVALID);
                                                    });

                                                    row.col(|ui| {
                                                        ui.add(Label::new(
                                                            RichText::new(&**album).color(YELLOW),
                                                        ))
                                                        .on_hover_text(PLAYLIST_INVALID);
                                                    });

                                                    row.col(|ui| {
                                                        ui.add(Label::new(
                                                            RichText::new(&**artist).color(YELLOW),
                                                        ))
                                                        .on_hover_text(PLAYLIST_INVALID);
                                                    });
                                                });
                                            }
                                        }
                                    }
                                });
                        });
                    });

                // Remove playlist entry set above.
                if let Some((playlist_name, index)) = self.playlist_remove_entry.take() {
                    if let Some(playlist) = playlists.get_mut(&playlist_name) {
                        playlist.remove(index);
                    }
                }

                // Swap playlist entry positions set above.
                if let Some((playlist_name, from, to)) = self.playlist_swap_entry.take() {
                    if let Some(playlist) = playlists.get_mut(&playlist_name) {
                        playlist.swap(from, to);
                    }
                }
            }
        } // end of match.
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
