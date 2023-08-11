# Disk
Festival saves all of its files within the "OS data" directory.

You can delete this folder to reset Festival's state/settings and the Collection, or more easily, just run:
```bash
./festival --delete
```

| Platform | Value                                                              | Example                                                 |
|----------|--------------------------------------------------------------------|---------------------------------------------------------|
| Windows  | `{FOLDERID_RoamingAppData}\Festival\data\gui`                      | `C:\Users\Hinto\AppData\Roaming\Festival\data\gui`      |
| macOS    | `$HOME/Library/Application Support/Festival/gui`                   | `/Users/Hinto/Library/Application Support/Festival/gui` |
| Linux    | `$XDG_DATA_HOME/festival/gui` or `$HOME/.local/share/festival/gui` | `/home/hinto/.local/share/festival/gui`                 |

---

Festival saves everything into `gui` because other frontends will use the same `festival` project directory, e.g:
```
~/.local/share/festival/
├─ gui/
├─ daemon/
├─ cli/
```

The sub-directories and files within `gui/` and their purpose:
```
├─ gui/
   ├
   ├─ state/
   ├  ├─ audio.bin      # Audio state, e.g: elapsed time, current song.
   ├  ├─ collection.bin # The main music `Collection`, holds metadata and PATHs to audio files.
   ├  ├─ playlists.bin  # The `Playlists` database, holds all info about all playlists.
   ├  ├─ settings.bin   # `GUI`-specific settings, e.g: sorting methods, album size.
   ├  ├─ state.bin      # `GUI`-specific state, e.g: current tab, search input.
   ├
   ├─ docs/  # Documentation files from `festival --docs`
   ├
   ├─ image/ # These are resized images of the album art.
   ├         # Festival itself does not need these (all images are
   ├         # within `collection.bin` itself) however, Festival hooks
   ├         # into the OS's native controls which needs a PATH to an
   ├         # image to display it. These are for that.
   ├
   ├─ signal/ # This is how Festival communicates with an existing one
   ├          # (e.g `festival --play`), via filesystem-based signals.
   ├          # `festival --play` quite literally just creates an empty
   ├          # file inside this folder called `play`.
   ├
   ├─ txt/
      ├─ crash.txt  # Crash/panic data. Useful for bug reports.
      ├─ perf.json  # Collection creation performance timings, in JSON.
```
