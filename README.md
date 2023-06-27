<div align="center">

# Festival (WIP)
<img src="assets/images/icon/512.png" width="10%"/>

![CI](https://github.com/hinto-janai/festival/actions/workflows/ci.yml/badge.svg)

Festival is a music player for local album collections.

https://github.com/hinto-janai/festival/assets/101352116/586e37e7-762d-4dc6-a9c4-9bdc45396961

</div>

## Frontends
All the frontends are built on top of the internals, [`shukusai`](https://github.com/hinto-janai/festival/tree/main/shukusai).

Currently, the most full frontend implementation is [`festival-gui`](https://github.com/hinto-janai/festival/tree/main/gui) (or just called `Festival`).

| Frontend                    | Description | Released |
|-----------------------------|-------------|----------|
| [`festival-gui`](https://github.com/hinto-janai/festival/tree/main/gui) | GUI ([`egui`](https://github.com/emilk/egui))                   | 🔴            |
| [`festivald`](https://github.com/hinto-janai/festival/tree/main/daemon) | Daemon ([`mpd`](https://github.com/MusicPlayerDaemon/MPD)-like) | 🔴            |
| [`festival-cli`](https://github.com/hinto-janai/festival/tree/main/cli) | CLI client                                                      | 🔴            |
| [`festival-web`](https://github.com/hinto-janai/festival/tree/main/web) | WASM client                                                     | 🔴            |

## Documentation
For a user guide on Festival, see [`gui/`](https://github.com/hinto-janai/festival/tree/main/gui).

For a broad overview of the internals, see [`shukusai/`](https://github.com/hinto-janai/festival/tree/main/shukusai).

For a comparison between Festival and other music players, see [`comparison/`](https://github.com/hinto-janai/festival/tree/main/comparison).

## Build
<details>
<summary>General Info</summary>

---

You need [`cargo`](https://www.rust-lang.org/learn/get-started).

You also need to clone the `submodules` that include patched libraries found in [`external/`](https://github.com/hinto-janai/festival/tree/main/external):
```bash
git clone --recursive https://github.com/hinto-janai/festival
```

The repo is a workspace, with `shukusai` and the `Frontend`'s all having a top-level directory, e.g:
- [`shukusai/`](https://github.com/hinto-janai/festival/tree/main/shukusai)
- [`gui/`](https://github.com/hinto-janai/festival/tree/main/gui)

Building at the root will build all binaries.

Currently, the only packages in the workspace are `shukusai` and `festival-gui`, which gets built as `festival[.exe]`.

---

</details>

<details>
<summary>Linux</summary>

---

The pre-compiled Linux binaries are built on Ubuntu 20.04, you'll need these packages to build:
```
sudo apt install libgtk-3-dev libasound2-dev libjack-dev libpulse-dev
```

To build:
```
cargo build --release
```

Optionally, to create an `AppImage` after building, at the repo root, run:
```bash
cd utils/
./mk_appimage.sh
```
This will create `Festival-v${VERSION}-x86_64.AppImage`.

This requires `appimagetool`. If not detected, it will `wget` the latest release to `/tmp` and use that instead.

---

</details>

<details>
<summary>macOS</summary>

---

To build:
```
cargo build --release
```
Optionally, to create a `Festival.app` after building, at the repo root, run:
```bash
cd utils/
./mk_app.sh
```
This will create `Festival.app`.

Optionally, to create a `.dmg` after that, run:
```bash
./mk_dmg.sh
```
This will create `Festival-v${VERSION}-macos-(x64|arm64).dmg`, `x64` or `arm64` depending on your `cargo` target.

---

</details>

<details>
<summary>Windows</summary>

---

To build:
```
cargo build --release
```

There is a [`build.rs`](https://github.com/hinto-janai/festival/blob/main/gui/build.rs) file in `gui/` solely for Windows-specific things:

1. It sets the icon in `File Explorer`
2. It statically links `VCRUNTIME140.dll` (the binary will not be portable without this)

---

</details>

## License
Festival is licensed under the [MIT License](https://github.com/hinto-janai/festival/blob/main/LICENSE).

[Symphonia](https://github.com/pdeljanov/Symphonia), the audio decoding/demuxing/metadata library used by Festival is licensed under MPL-2.0.

## FAQ
</details>

<details>
<summary>Playlists</summary>

---

Festival does not support playlists at the moment.

---

</details>

<details>
<summary>Compilations</summary>

---

Festival does not directly support compilations (a single album, but with various artists) at the moment.

It will still load the album, but it will be spread out for each different artist.

---

</details>

<details>
<summary>Missing music</summary>

---

Your audio files must have proper metadata for Festival to detect it.

The required tags are:
- Artist
- Album

If the song title tag does not exist, the filename will be used instead.

For more details on metadata related errors, start Festival in a console:
```bash
./festival
```
and look for yellow `W` (Warn) log messages during a `Collection` reset.

---

</details>

<details>
<summary>Missing album art</summary>

---

If your audio file has embedded album art, Festival will use it.

If no embedded album art metadata is found, Festival will:
- Search in the same directory as the file for an image file
- Search in the file's parent directory for an image file

If an image file is not found, a default `?` album art will be used.

The supported image file formats are:
- `JPG/JPEG`
- `PNG`
- `BMP`
- `ICO`
- `TIFF`
- `WebP`
- `AVIF`

---

</details>

<details>
<summary>Metadata editing</summary>

---

Festival is only a music player, not a metadata editor.

Some metadata editors you could use:

- [`Kid3`](https://kid3.kde.org)
- [`mp3tag`](https://www.mp3tag.de/en)
- [`puddletag`](https://docs.puddletag.net)
- [`MusicBrainz Picard`](https://picard.musicbrainz.org)

---

</details>

<details>

<summary>Supported audio codecs</summary>

---

The supported audio codecs are:
- `AAC`
- `ADPCM`
- `ALAC`
- `FLAC`
- `MP3/MP2/MP1/MPA/MPEG`
- `Ogg/Vorbis`
- `Opus`
- `WAV`
- `AIFF`
- `WavPack`

---

</details>

<details>
<summary>Supported metadata formats</summary>

---

| Format                | Status    |
|-----------------------|-----------|
| ID3v1                 | Great     |
| ID3v2                 | Great     |
| ISO/MP4               | Great     |
| RIFF                  | Great     |
| Vorbis comment (FLAC) | Perfect   |
| Vorbis comment (OGG)  | Perfect   |

---

</details>
