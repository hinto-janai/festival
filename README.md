<img src="assets/images/icon/512.png" align="left" width="50%"/>

# Festival
Music player for Windows/macOS/Linux.

![Build](https://github.com/hinto-janai/festival/actions/workflows/build.yml/badge.svg)

![Tests](https://github.com/hinto-janai/festival/actions/workflows/tests.yml/badge.svg)

<br clear="left"/>

* [Comparison](#Comparison)
* [Documentation](#Documentation)
* [Build](#Build)
* [License](#License)

---

## Comparison
For context on these graphs (there's always trade-offs), see the [`cmp/` folder's README.](https://github.com/hinto-janai/festival/cmp)

Input data:

- 135 Artists
- 500 Albums
- 7000 Songs
- `170GB` total disk space

<img src="assets/images/cmp/scratch.png" align="left" width="50%"/>
<img src="assets/images/cmp/cache.png" align="right" width="50%"/>

<img src="assets/images/cmp/error.png" align="left" width="50%"/>
<img src="assets/images/cmp/boot.png" align="right" width="50%"/>

## Documentation
For a broad overview of `Festival`'s internals, see the [`src/` folder's README.](https://github.com/hinto-janai/festival/src)

For more specifics, see the README's of the subdirectories inside `src/`.

## Build
### General Info
You need [`cargo`](https://www.rust-lang.org/learn/get-started).

There are `30` unit tests, you may want to run:
```
cargo test
```
before attempting a full build.

---

### Linux
The pre-compiled Linux binaries are built on Debian 11, you'll need these packages to build:
```
sudo apt install build-essential cmake libgtk-3-dev
```

After that, run:
```
cargo build --release
```

---

### macOS
On macOS, if you want the binary to have an icon, you must install [`cargo-bundle`](https://github.com/burtonageo/cargo-bundle).

After that, run:
```
cargo bundle --release
```
This bundles `Festival` into a `Festival.app`, the way it comes in the pre-built tars for macOS.

---

### Windows
```
cargo build --release
```

There is a `build.rs` file in the repo solely for Windows-specific things:

1. It sets the icon in `File Explorer`
2. It statically links `VCRUNTIME140.dll` (the binary will not be portable without this)

## License
`Festival` is licensed under the [MIT License](https://github.com/hinto-janai/festival/LICENSE).

All of the libraries `Festival` (directly) uses:

| Library | Purpose | License |
|---------|---------|---------|
| [anyhow](https://github.com/dtolnay/anyhow)                        | Error handling           | `MIT` & `Apache-2.0`
| [bincode](https://github.com/bincode-org/bincode)                  | Binary file en/decoder   | `MIT` 
| [chrono](https://github.com/chronotope/chrono)                     | Time formatting          | `MIT` & `Apache-2.0`
| [clap](https://github.com/clap-rs/clap)                            | CLI arguments            | `MIT` & `Apache-2.0`
| [crossbeam_channel](https://github.com/crossbeam-rs/crossbeam)     | Thread message passing   | `MIT` & `Apache-2.0`
| [directories](https://github.com/soc/directories-rs)               | Native user directories  | `MIT` & `Apache-2.0`
| [egui](https://github.com/emilk/egui)                              | GUI                      | `MIT` & `Apache-2.0`
| [egui_extras](https://github.com/emilk/egui/crates/egui_extras)    | GUI                      | `MIT` & `Apache-2.0`
| [eframe](https://github.com/emilk/egui/crates/eframe)              | GUI                      | `MIT` & `Apache-2.0`
| [egui-notify](https://github.com/ItsEthra/egui-notify)             | GUI                      | `MIT`
| [env_logger](https://github.com/rust-cli/env_logger)               | Logging                  | `MIT` & `Apache-2.0`
| [fast_image_resize](https://github.com/cykooz/fast_image_resize)   | Image processing         | `MIT` & `Apache-2.0`
| [image](https://github.com/image-rs/image)                         | Image processing         | `MIT`
| [infer](https://github.com/bojand/infer)                           | File MIME detection      | `MIT`
| [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) | Lazy static macro        | `MIT` & `Apache-2.0`
| [lofty](https://github.com/Serial-ATA/lofty-rs)                    | Audio metadata parsing   | `MIT` & `Apache-2.0`
| [log](https://github.com/rust-lang/log)                            | Logging                  | `MIT` & `Apache-2.0`
| [notify](https://github.com/notify-rs/notify)                      | Filesystem watching      | `Artistic License 2.0` & `CC Zero 1.0`
| [mime_guess](https://github.com/abonander/mime_guess)              | File MIME detection      | `MIT`
| [num-format](https://docs.rs/num-format)                           | Number formatting        | `MIT` & `Apache-2.0`
| [rand](https://github.com/rust-random/rand)                        | RNG                      | `MIT` & `Apache-2.0`
| [rfd](https://github.com/PolyMeilex/rfd)                           | Native file dialog       | `MIT`
| [serde](https://github.com/serde-rs/serde)                         | (De)serialization        | `MIT` & `Apache-2.0`
| [serde_bytes](https://github.com/serde-rs/bytes)                   | (De)serialization        | `MIT` & `Apache-2.0`
| [souvlaki](https://github.com/Sinono3/souvlaki)                    | Native media controls    | `MIT`
| [strsim](https://github.com/dguo/strsim-rs)                        | String similarity        | `MIT`
| [strum](https://github.com/Peternator7/strum)                      | Enum iteration           | `MIT`
| [Symphonia](https://github.com/pdeljanov/Symphonia)                | Audio demuxing, decoding | `MPL-2.0`
| [toml_edit](https://github.com/ordian/toml_edit)                   | TOML parsing             | `MIT` & `Apache-2.0` 
| [walkdir](https://github.com/BurntSushi/walkdir)                   | Recursive PATH walking   | `MIT` & `Unlicense` 
