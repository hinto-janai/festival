# Disk
`festivald` saves all of its files within various "OS" directories, owned by the user running it.

You can list all the PATHs on your system with:
```bash
./festivald --path
```

And delete them with with:
```bash
./festivald --delete
```

`festivald` saves everything into `daemon/` because other Festival frontends also use the same `festival` project directory, e.g:
```
~/.local/share/festival/
  ├─ gui/
  ├─ daemon/
  ├─ cli/
```

## Cache
Where `festivald`'s cache data is saved (`REST` ZIP cache, etc).

| Platform | Value                                    | Example                                              |
|----------|------------------------------------------|------------------------------------------------------|
| Windows  | `{FOLDERID_LocalAppData}\Festival\cache` | `C:\Users\Alice\AppData\Local\Festival\daemon\cache` |
| macOS    | `$HOME/Library/Caches/Festival`          | `/Users/Alice/Library/Caches/Festival/daemon`        |
| Linux    | `$HOME/.cache/festival/daemon`           | `/home/alice/.cache/festival/daemon`                 |

The `Cache` sub-directories/files, and their purpose:
```plaintext
├─ daemon/
   │
   ├─ zip/ # Cached ZIP files
      │
      ├─ collection/ # Cached `Collection` ZIPs, created by the `/collection` REST endpoint
      │  │
      │  ├─ tmp/ # Temporary, un-finished ZIP files
      │
      ├─ playlist/ # Cached `Playlist` ZIPs, created by the `/playlist/*` REST endpoints
      │  │
      │  ├─ tmp/ # Temporary, un-finished ZIP files
      │
      ├─ artist/ # Cached `Artist` ZIPs, created by the `*_artist` REST endpoints
      │  │
      │  ├─ tmp/ # Temporary, un-finished ZIP files
      │
      ├─ album/ # # Cached `Artist` ZIPs, created by the `*_album` REST endpoints
      │  │
      │  ├─ tmp/ # Temporary, un-finished ZIP files
      │
      ├─ art/ # # Cached `Artist` ZIPs, created by the `art_*` REST endpoints
      │  │
      │  ├─ tmp/ # Temporary, un-finished ZIP files
```

## Config
Where `festivald`'s configuration files are saved.

| Platform | Value                                               | Example                                                    |
|----------|-----------------------------------------------------|------------------------------------------------------------|
| Windows  | `{FOLDERID_RoamingAppData}\Festival\config\daemon`  | `C:\Users\Alice\AppData\Roaming\Festival\config\daemon`    |
| macOS    | `$HOME/Library/Application Support/Festival/daemon` | `/Users/Alice/Library/Application Support/Festival/daemon` |
| Linux    | `$HOME/.config/festival/daemon`                     | `/home/alice/.config/festival/daemon`                      |

The `Config` sub-directories/files, and their purpose:
```plaintext
├─ daemon/
   │
   ├─ festivald.toml # Config file used by `festivald`
```

## Data
Where `festivald`'s main data is saved (the [`Collection`](common-objects/common-objects.md), audio state, etc).

| Platform | Value                                                                    | Example                                                 |
|----------|--------------------------------------------------------------------------|---------------------------------------------------------|
| Windows  | `{FOLDERID_RoamingAppData}\Festival\data\daemon`                         | `C:\Users\Alice\AppData\Roaming\Festival\data\daemon`      |
| macOS    | `$HOME/Library/Application Support/Festival/daemon`                      | `/Users/Alice/Library/Application Support/Festival/daemon` |
| Linux    | `$HOME/.local/share/festival/daemon`                                     | `/home/alice/.local/share/festival/daemon`                 |

The `Data` sub-directories/files, and their purpose:
```plaintext
├─ daemon/
   │
   ├─ state/
   │  ├─ audio.bin      # Audio state, e.g: elapsed time, current song.
   │  ├─ collection.bin # The main music `Collection`, holds metadata and PATHs to audio files.
   │  ├─ playlists.bin  # The `Playlists` database, holds all playlist data
   │
   ├─ txt/
   │  ├─ crash.txt  # Crash/panic data. Useful for bug reports.
   │  ├─ perf.json  # Collection creation performance timings, in JSON.
   │
   ├─ docs/ # The static documentations files served by `festivald`
   │
   ├─ image/ # Full-sized, un-changed `Album` art
   │
   ├─ signal/ # This is how `festivald` communicates with an existing one
              # (e.g `festivald signal --play`), via filesystem-based signals.
              # `festivald signal --play` quite literally just creates an empty
              # file inside this folder called `play`.
```
