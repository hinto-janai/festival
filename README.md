<div align="center">

# Festival
<img src="assets/images/icon/512.png" width="10%"/>

![CI](https://github.com/hinto-janai/festival/actions/workflows/ci.yml/badge.svg)

Festival is a music player for local album collections.

https://github.com/hinto-janai/festival/assets/101352116/586e37e7-762d-4dc6-a9c4-9bdc45396961

</div>

## Documentation
See documentation at https://docs.festival.pm/gui.

## Comparison
For a comparison between Festival and other music players, see [`comparison/`](https://github.com/hinto-janai/festival/tree/main/comparison/README.md).

## Build
<details>
<summary>General Info</summary>

---

You need [`cargo`](https://www.rust-lang.org/learn/get-started) and at least `rustc 1.70`.

You also need to clone the `submodules` that include patched libraries found in [`external/`](https://github.com/hinto-janai/festival/tree/main/external):
```bash
git clone --recursive https://github.com/hinto-janai/festival
```

The built binary is found in `target/release/festival[.exe]` by default.

---

</details>

<details>
<summary>Linux</summary>

---

The pre-compiled Linux binaries are built on Ubuntu 20.04, you'll need these packages to build:
```bash
sudo apt install build-essential pkg-config libdbus-1-dev libpulse-dev libgtk-3-dev
```

To build the latest _stable_ release:
```bash
git checkout gui-v1.4.0
cargo build --release
```

---

</details>

<details>
<summary>macOS</summary>

---

To build the latest _stable_ release:
```bash
git checkout gui-v1.4.0
cargo build --release
```

---

</details>

<details>
<summary>Windows</summary>

---

To build the latest _stable_ release:
```bash
git checkout gui-v1.4.0
cargo build --release
```

There is a [`build.rs`](https://github.com/hinto-janai/festival/blob/main/gui/build.rs) file in `gui/` solely for Windows-specific things:

1. It sets the icon in `File Explorer`
2. It sets some miscellaneous metadata
3. It statically links `VCRUNTIME140.dll` (the binary will not be portable without this)


---

</details>

## License
Festival is licensed under the [MIT License](https://github.com/hinto-janai/festival/blob/main/LICENSE).

However, its dependency tree includes many other licenses.

## FAQ
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

---

</details>

<details>
<summary>Missing date</summary>

---

Festival will look for a date metadata tag generally resembling the `YYYY-MM-DD` format.

Some examples of dates that will work:
- `2022-12-31` (YYYY-MM-DD)
- `2022` (YYYY)
- `31-12-2022` (DD-MM-YYYY)
- `12-31-2022` (MM-DD-YYYY)
- `2022/12/31` (YYYY-MM-DD but with a different separator)
- `20221231` (YYYY-MM-DD but with no separator)
- `2022-1-1` (YYYY-MM-DD)
- `2022-01-01` (YYYY-MM-DD)

As long as the year exists, the date will be parsed correctly. This means `MM-DD` metadata will be not parsed, so:
- `12-31` (MM-DD)
- `31-12` (DD-MM)

will not work. These will show up as `????-??-??` in Festival.

To fix your music metadata, see below for metadata editors.

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
