# UI
The actual `egui` code & main `eframe::App::update()` loop.

Upon exiting `GUI`, `eframe::App::on_close_event()` is called.

Since we have to:
- Save data
- (Maybe) wait for events to finish

instead of executing this code in the `GUI` thread and freezing the `update()` event loop,
another thread is spawned with `Gui::exit()`. This thread does all the communicating with `Kernel`
and the `Collection`.  While this is happening, `GUI` will loop forever, rendering an `egui::Spinner`.

## Files
| File          | Purpose |
|---------------|---------|
| `album.rs`    | `Album` tab
| `artist.rs`   | `Artist` tab
| `search.rs`   | `Search` tab
| `settings.rs` | `Settings` tab
| `tab.rs`      | Tab enum definition
| `exit.rs`     | What to do when the GUI is closed
| `update.rs`   | The main `eframe::App::update()` loop
| `queue.rs`    | `Queue` tab
