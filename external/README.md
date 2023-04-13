# External libraries (with patches)

Some external libraries that with some custom patches for some features `Festival` needs.

| `egui` branch | Purpose |
|---------------|---------|
| `festival`    | The main patched `egui` that `festival-gui` uses
| `master`      | Up-to-date, un-modified upstream branch
| misc          | Small patches that are planned to be upstreamed

| Library       | Patch | Purpose | Merged upstream? |
|---------------|-------|---------|------------------|
| `egui`        | Add `trailing_fill()` bool toggle to `Slider` | Adds color to a slider similar to a `ProgressBar` | [Yes](https://github.com/emilk/egui/pull/2660)
| `egui`        | Add `thickness()` to `Slider` | Change thickness (not length/width) of a `Slider` | [No](https://github.com/emilk/egui/pull/2713)
| `lofty-rs`    | Add `into_data()` for `Picture` | Ownership of inner picture bytes | [Yes](https://github.com/Serial-ATA/lofty-rs/pull/173)
| `egui_extras` | Add `load_texture()` for `RetainedImage` | Preemptively turn image into texture without blocking GUI | [No](https://github.com/emilk/egui/pull/2891)
| `egui_extras` | Make `RetainedImage` fields public | Make it easier to manipulate with `RetainedImage` | No
| `epaint`      | Add `.try_write()` to internal `RwLock` | Prevent reader (GUI) starvation when image processing by using `try_write()` instead of `write()` | No
