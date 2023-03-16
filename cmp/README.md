# Comparison
Some comparisons between `Festival` and other album centered music players.

The graphs are created with the code in this directory. Big thanks to [Rucknium](https://github.com/rucknium) for teaching me `R`.

* [Comparison](#Comparison)
	- [Behavior](#Behavior)
	- [Playlists](#Playlists)
	- [Sorting](#Sorting)
	- [Features](#Features)
* [Testbench](#Testbench)
* [Data](#Data)
* [Tests](#Tests)
	- [New Collection (from scratch)](#New-Collection-From-Scratch)
	- [New Collection (cached)](#New-Collection-Cached)
	- [Error](#Error)
	- [Boot](#Boot)
* [Closing Notes](#Closing Notes)

## Comparison
| Music Player | Version                 | OS                    | Open Source?   | Playlist handling | Sorting methods | Features vs Minimal |
|--------------|-------------------------|-----------------------|----------------|-------------------|-----------------|---------------------|
| [Festival](https://github.com/hinto-janai/festival)       | `1.0.0`                 | Windows, macOS, Linux | 🟢             | 🔴                | 🟢              | Minimal
| [Lollypop](https://gitlab.gnome.org/World/lollypop)       | `1.4.37` (2023-01-03)   | Linux (GTK)           | 🟢             | 🟡                | 🟡              | Both
| [GNOME Music](https://gitlab.gnome.org/GNOME/gnome-music) | `1.42` (2022-04-25)     | Linux (GTK)           | 🟢             | 🟡                | 🔴              | Minimal
| [MusicBee](https://www.getmusicbee.com)                   | `3.5.8447` (2023-02-19) | Windows               | 🔴             | 🟢                | 🟢              | Features
| [iTunes](https://www.apple.com/itunes)                    | `12.12.7` (2023-12-15)  | Windows               | 🔴             | 🟢                | 🟢              | Features

Similar behavior:

- All programs had similar peak memory usage when creating a new collection, around `1-2GB`.

Invariants:

- Some music players use multiple threads, some do not
- Some music players are Windows-only, some are Linux-only
- The resolution album art gets processed/saved as varies

#### Playlists
Playlist handling: What happens when you delete the underlying file of a song in an existing playlist? Or worse, reset the whole collection?

| Music Player                 | Behavior |
|------------------------------|----------|
| `Festival`                   | Playlists are linked with the `Collection` itself, if you reset the `Collection`, all your playlists are also reset
| `Lollypop` and `GNOME Music` | Silently adds/removes songs from playlists if the underlying file gets added/deleted
| `MusicBee` and `iTunes`      | Continues displaying the playlist with the correct metadata, shows error when attempting to play missing song

#### Sorting
"Sorting methods" refers to how many options the music player provides to sort the music, either by artist, album, song, or a cross-sort combining them, e.g: album covers by artist name and album release:
```
a_artist
    |_ 2020_album
    |_ 2021_album

b_artist
	|_ 1980_album
	|_ 2000_album

[...]
```

#### Features
"Features vs Minimal" refers to if the music player is packed with (useful) features or if it's just a minimal music player.

Features could be things like:

- Searching
- Re-encoding
- Metadata editing
- etc.

## Testbench
The tests were conducted on the following PC:

| Component | Spec |
|-----------|------|
| CPU       | AMD Ryzen 5950x (32 threads)
| RAM       | 2x32GB DDR4 3800MHz
| SSD       | 500GB Samsung Evo 850 (540MB/s sequential reads)
| OS        | `Windows 10` and `Arch Linux (2023-03-01)`

Some things to keep in mind:

- The `NTFS` filesystem was used for both `Windows` and `Linux` tests (advantage for `iTunes`, `MusicBee`)
- `FLAC` files were converted to `ALAC/AAC/MP3` for iTunes (advantage for `iTunes`)

## Data
The tests were tested on a music collection consisting of:

- 135 Artists
- 500 Albums
- 7000 Songs
- 70% `FLAC`
- 15% `MP3/MPEG`
- 14% `OGG/WAV/AAC`
- 1% `ALAC/M4A`

Totaling to `170GB` of disk space.

The average album art size is around `1MB`, with a resolution of around `1400x1400`.

## Tests
### New Collection (from scratch)
How long does it take to create a new collection and render all the album art _for the very first time?_

<img src="assets/images/cmp/scratch.png" width="60%"/>

| Music Player  | Seconds (less is better) |
|---------------|--------------------------|
| Festival      | 5
| MusicBee      | 24
| GNOME Music   | 32
| Lollypop      | 60
| iTunes        | 97

### New Collection (cached)
Both the system's cache and the application's cache itself (`~/.cache`) can drastically speed things up.

After resetting/removing the first collection, how long does it take to create it _again_ and render the album art?

<img src="assets/images/cmp/cache.png" width="60%"/>

| Music Player  | Seconds (less is better) |
|---------------|--------------------------|
| Festival      | 0.25
| MusicBee      | 12
| Lollypop      | 15
| GNOME Music   | 32
| iTunes        | 97

Notes:

- `Festival` doesn't cache files, but it benefits a lot from system cache
- Both `Lollypop` and `MusicBee` cache files and also benefit from system cache
- Both `GNOME Music` and `iTunes` don't benefit at all even though they cache files

### Error
After creating a collection, how many albums are _not_ processed correctly? Missing metadata, missing album art, etc.

<img src="assets/images/cmp/error.png" width="60%"/>

| Music Player  | Errors (less is better) |
|---------------|-------------------------|
| Festival      | 0
| Lollypop      | 0
| GNOME Music   | 28
| iTunes        | 30
| MusicBee      | 87

### Boot
When opening the music player, how long does it take to render _all_ the album art?

<img src="assets/images/cmp/boot.png" width="60%"/>

| Music Player  | Seconds (less is better) |
|---------------|--------------------------|
| Festival      | 0.15
| Lollypop      | 0.80
| MusicBee      | 1.50
| iTunes        | 10
| GNOME Music   | 32

## Closing Notes
Some other big differences that aren't shown in graphs here:

- GUI bugs (`GNOME Music` really should be deprecated in favor of `Lollypop`)
- Album art size (`Festival` uses `600x600`, others use `300x300`)
- Access times (`SQL` query vs `Vec` index)

On the other hand, the playlist implementation in `Festival` is quite bad, since I don't use playlists at all. In order to achieve the speeds that it does, `Festival` sacrifices everything in terms of dynamic collection management, since after the initial creation of the `Collection`, it is static _forever_. This doesn't make implementing dynamic data impossible, but annoying due to index invalidation.

`Festival` ignores some metadata tags as well, again, because I don't personally use them (they would also take additional processing time and disk space). Some maybe important ones:

- Genre
- Lyrics
- Composer/Conductor/Ensemble
- Movement
- Location
- Grouping
- Comment
- Label
- MusicBrainz_*

I modeled `Festival` after [`Lollypop`](https://gitlab.gnome.org/World/lollypop), which I still believe is by far the best album-centered music player I've used because it is:

- Correct (no metadata errors)
- Relatively bug-free
- Relatively minimal
- Relatively fast
- Pretty

There are some bugs (resetting collection _always_ needs a restart since it does not load properly), some extra features I'll never use (internet related stuff), and some features I wish it had (album art bigger than `300x300`), but nothing else really comes close to it.

As this project grew in scope, I realized separating `Festival`'s internals from the GUI would result in a _fast, correct,_ and _re-usable_ base for multiple different frontends. This is the only reason I made `Festival` when `Lollypop` already existed. Also, I wanted to practice Rust.
