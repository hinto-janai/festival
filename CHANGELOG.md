All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Types of changes:
- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` in case of vulnerabilities


---


# Festival GUI Unreleased


---


# `festivald` Unreleased


---


# `festival-cli` Unreleased


---


# Festival GUI v1.4.0 - 2024-01-09

## Added
- Show art on album title label hover ([#83](https://github.com/hinto-janai/festival/pull/83))
- `Auto Save` setting ([#84](https://github.com/hinto-janai/festival/pull/84))
- `Middle click` to copy Artist/Album/Song/Playlist text ([#87](https://github.com/hinto-janai/festival/pull/87))
- `CTRL + Middle click` to copy Artist/Album/Song PATHs ([#87](https://github.com/hinto-janai/festival/pull/87))


## Changed
- State (Tabs, specific `Album/Artist`) are now maintained across `Collection` resets if possible ([#85](https://github.com/hinto-janai/festival/pull/85))


---


# Festival GUI v1.3.3 - 2023-11-30
## Changed
* `FESTIVAL_FORCE_RESAMPLE` environment variable will force Festival to resample audio even if the sample rate is the same as the audio output device (Windows/macOS only)

## Fixed
* Resampled audio fixes on Windows ([#80](https://github.com/hinto-janai/festival/pull/80))


---


# Festival GUI v1.3.2 - 2023-11-26
## Fixed
* Crash on audio playback on Windows/macOS ([#76](https://github.com/hinto-janai/festival/pull/76))
* Resampled audio fixes ([#77](https://github.com/hinto-janai/festival/pull/77))


---


# Festival GUI v1.3.1 - 2023-11-15
## Changed
* Audio decoding/playback loop is tighter - more leeway during high CPU usage ([#73](https://github.com/hinto-janai/festival/pull/73))
* Audio thread is now real-time across all platforms ([#74](https://github.com/hinto-janai/festival/pull/74))
* Audio volume transformation is now applied after resampling ([#75](https://github.com/hinto-janai/festival/pull/75))

## Fixed
* Potential panic on audio hardware/server write failure (https://github.com/hinto-janai/festival/commit/2a5adf73fb051a9f6c0d2e75e229f58bcd53de2c)


---


# Festival GUI v1.3.0 - 2023-09-04
## Added
* Queue tab: random artist/album/song buttons ([#63](https://github.com/hinto-janai/festival/pull/63))
* Queue tab: total queue runtime ([#64](https://github.com/hinto-janai/festival/pull/64))

## Changed
* Moved repeat button to the queue tab ([#63](https://github.com/hinto-janai/festival/pull/63))

## Fixed
* Small album art being slightly cutoff on the sides ([#65](https://github.com/hinto-janai/festival/pull/65))


---


# `festivald` v1.0.0 - 2023-08-24
Festival [daemon](https://github.com/hinto-janai/festival/tree/main/daemon).

Serves `JSON-RPC` & `REST` APIs.

See [`festival-cli`](https://docs.festival.pm/cli) for a simple to use `JSON-RPC` client.

Full documentation available at: https://docs.festival.pm/daemon.

Public instance available at: `https://daemon.festival.pm`.


---


# `festival-cli` v1.0.0 - 2023-08-24
`JSON-RPC` client for [`festivald`](https://docs.festival.pm/daemon).

Full documentation available at: https://docs.festival.pm/cli.


---


# Festival GUI v1.2.0 - 2023-08-11
## Added
* Playlists ([#58](https://github.com/hinto-janai/festival/pull/58))
* Drag-and-drop support for `Collection` folders ([#51](https://github.com/hinto-janai/festival/pull/51))
* Audio state now recovers (as much as possible) across `Collection` resets ([#59](https://github.com/hinto-janai/festival/pull/59))
* Key-binding for adding songs/albums/artists to playlists - `CTRL + Primary Mouse`
* Documentation with `festival --docs`

## Changed
* Key-binding for `Open Album/Song Directory` is now `CTRL + Secondary Mouse`

## Fixed
* Audio thread now has high priority, less cutouts during high CPU usage ([#50](https://github.com/hinto-janai/festival/issues/50))


---


# Festival GUI v1.1.0 - 2023-07-12
Performance improvements. Expect 2x~ faster Collection resets (user time) and 5x~ faster overall time (including save).

## Changed
* Faster `JPG` album art decoding, 1.75x~ faster `Collection` reset ([#20](https://github.com/hinto-janai/festival/pull/20))
* `Collection` directories are now pre-emptively cached on startup and addition; initial reset speeds are faster ([#38](https://github.com/hinto-janai/festival/pull/38))
* Album art conversion now uses all available threads, 1.25x~ faster ([#20](https://github.com/hinto-janai/festival/pull/20))
* Post-`Collection` reset image encoding/save can now use multiple threads, 5x~ faster ([#37](https://github.com/hinto-janai/festival/pull/37))
* Search can now use multiple threads, 1.8x~ faster ([#37](https://github.com/hinto-janai/festival/pull/37))

## Fixed
* Crashes with songs that have odd date metadata, again (https://github.com/hinto-janai/readable/commit/02bdd467363e50627e68af56497eaeb13cdf632d)
* Runtime UI overflow when song is longer than an hour ([#44](https://github.com/hinto-janai/festival/pull/44))
* Over-saturated colors on Linux (KDE) ([#43](https://github.com/hinto-janai/festival/pull/43))


---


# Festival GUI v1.0.1 - 2023-07-02
## Added
* `Pixels Per Point` setting for manual UI pixel sizing (https://github.com/hinto-janai/festival/commit/f1fc011fac05e6daec9894477e52030745fea25b)

## Changed
* Lowered minimum resolution to `870x486` from `1000x800` (https://github.com/hinto-janai/festival/commit/075f1b8e48e2c733878ce3ee982e54ca4051fee9)

## Fixed
* Crashes with songs that have `MONTH-DAY` date metadata only ([#1](https://github.com/hinto-janai/readable/pull/1))
* Non-`jpg/png` album art image decoding issues (https://github.com/hinto-janai/festival/commit/f0e96b44e95555a08441e0b99983030bc528f490)


---


# Festival GUI v1.0.0 - 2023-06-28
The PGP key used to sign releases can be found at https://festival.pm/hinto or [`pgp/hinto-janai.asc`](https://github.com/hinto-janai/festival/blob/main/pgp/hinto-janai.asc).
