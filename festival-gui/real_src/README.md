# GUI
A slightly patched version of `egui` located at `external/` is used for `GUI`.

Since a `Collection` might contain songs of various languages, multiple fonts are needed, or they will show up as unknown square blobs. Thankfully, `egui` allows embedding arbitrary fonts. The extra fonts are located at: `assets/fonts`.

The default size of `GUI` is `1000x800`.

## Files
| File           | Purpose |
|----------------|---------|
| `main.rs`      | Barebones `main()` that spawns `Kernel` and turns into `GUI`
| `gui.rs`       | Main data & loop for the GUI
| `constants.rs` | General constants for the GUI
| `tab.rs`       | (Left) Tab buttons
| `album.rs`     | Album tab 
| `artist.rs`    | Artist tab
| `queue.rs`     | Queue tab
| `search.rs`    | Search tab
| `settings.rs`  | Settings tab
