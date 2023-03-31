# GUI
A slightly patched version of `egui` located at `/external/` is used for `GUI`.

Since a `Collection` might contain songs of various languages, multiple fonts are needed, or they will show up as unknown square blobs. Thankfully, `egui` allows embedding arbitrary fonts. The extra fonts are located at: `/assets/fonts/`.

The default size of `GUI` is `1000x800`.

## Files
| File           | Purpose |
|----------------|---------|
| `main.rs`      | Barebones `main()` that spawns `Kernel` and turns into `GUI`
| `cli.rs`       | CLI handling
| `constants.rs` | General constants
| `text.rs`      | Constant `&'static str`'s for `GUI` text

## Folders
| Folder         | Purpose |
|----------------|---------|
| `data/`        | Main data definitions and initialization
| `ui/`          | The actual `egui` code & main `eframe::App::update()` loop
