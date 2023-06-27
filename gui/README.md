# Festival
`Festival`, the GUI client and the reference implementation.

* [Key/Mouse shortcuts](#KeyMouse-shortcuts)
* [Command Line](#Command-Line)
* [Disk](#Disk)

# Key/Mouse shortcuts
| Key/Mouse       | Action |
|-----------------|--------|
| `[A-Za-z0-9]`   | Jump to search tab
| CTRL+S          | Save Settings
| CTRL+Z          | Reset Settings
| CTRL+C          | Reset Collection
| CTRL+A          | Add Scan Directory
| CTRL+W          | Rotate Album Sort
| CTRL+E          | Rotate Artist Sort
| CTRL+R          | Rotate Song Sort
| CTRL+D          | Goto Last Tab
| Up              | Last Tab
| Down            | Next Tab
| Right           | Last Sub-Tab
| Left            | Last Sub-Tab
| Primary Mouse   | Set Artist, Album, Song
| Secondary Mouse | Append Artist, Album, Song to Queue
| Middle Mouse    | Open Album/Song Directory

*Note: `CTRL` is `Control` on macOS.*

`CTRL+Primary Mouse` will also do the same as action as `Middle Mouse`.

# Command Line
Festival has some command line flags, useful for adjusting playback.

Some examples:

Play the next song:
```bash
./festival --next
```

Seek backwards 25 seconds in the current song:
```bash
./festival --seek-backward 25
```

Skip 3 songs into the queue:
```bash
./festival --skip 3
```

Play the 10th song in the queue:
```bash
./festival --index 10
```

---

The full list of commands:
```
Usage: festival [OPTIONS]

Options:
      --play
          Start playback

      --pause
          Pause playback

      --toggle
          Toggle playback (play/pause)

      --next
          Skip to next track

      --previous
          Play previous track

      --stop
          Clear queue and stop playback

      --clear
          Clear queue but don't stop playback

      --shuffle
          Shuffle the current queue and reset to the first song

      --repeat-song
          Turn on single `Song` track repeat

      --repeat-queue
          Turn on queue repeat

      --repeat-off
          Turn off repeating

      --volume <VOLUME>
          Set the volume to `VOLUME` (0-100)

      --seek <SEEK>
          Seek to the absolute `SEEK` second in the current song

      --seek-forward <SEEK_FORWARD>
          Seek `SEEK_FORWARD` seconds forwards in the current song

      --seek-backward <SEEK_BACKWARD>
          Seek `SEEK_BACKWARD` seconds backwards in the current song

      --index <INDEX>
          Set the current song to the index `INDEX` in the queue.
          
          NOTE:
          The queue index starts from 1 (first song is `--index 1`).
          
          Providing an index that is out-of-bounds
          will end the queue (even if repeat is turned on).

      --skip <SKIP>
          Skip `SKIP` amount of songs
          
          If the last song in the queue is skipped over,
          and queue repeat is turned on, this will reset
          the current song to the 1st in the queue.

      --back <BACK>
          Go backwards in the queue by `BACK` amount of songs
          
          If `BACK` is greater than the amount of songs we can
          skip backwards, this will reset the current song to
          the 1st in the queue.

      --metadata
          Print JSON metadata about the current `Collection` on disk
          
          WARNING:
          This output is not meant to be relied on (yet).
          
          It it mostly for quick displaying and debugging
          purposes and may be changed at any time.
          
          This flag will attempt to parse the `Collection` that
          is currently on disk and extract the metadata from it.
          
          This also means the entire `Collection` will be read
          and deserialized from disk, which may be very expensive
          if you have a large `Collection`.

      --disable-watch
          Disable watching the filesystem for signals
          
          The way a newly launched Festival communicates to
          an already existing one (e.g, `festival --play`) is
          by creating a file in Festival's `signal` directory.
          
          `festival --FLAG` just creates a file in that directory,
          which an existing Festival will notice and do the appropriate task.
          
          Using `--disable-watch` will disable that part of the system so that
          filesystem signals won't work, e.g, `festival --play` will not work.

      --disable-media-controls
          Disable OS media controls
          
          Festival plugs into the native OS's media controls so that signals
          like `play/pause/stop` and/or keyboard controls can be processed.
          
          `--disable-media-controls` disables this.

      --delete
          Delete all Festival files that are currently on disk
          
          This includes:
          - The `Collection`
          - `GUI` settings (sort methods, color, etc)
          - `GUI` state (currently selected album/artist, etc)
          - Audio state (currently playing song, queue, etc)
          - Cached images (found in local OS cache folder)
          
          The PATH(s) deleted will be printed on success.

      --log-level <OFF|ERROR|INFO|WARN|DEBUG|TRACE>
          Set filter level for console logs
          
          [default: INFO]

  -v, --version
          Print version

  -h, --help
          Print help (see a summary with '-h')
```

# Disk
Festival saves all of its important files within the "OS data" directory.

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
   ├─ state/
   ├  ├─ audio.bin      # Audio state, e.g: elapsed time, current song.
   ├  ├─ collection.bin # The main music `Collection`, holds metadata and PATHs to audio files.
   ├  ├─ settings.bin   # `GUI`-specific settings, e.g: sorting methods, album size.
   ├  ├─ state.bin      # `GUI`-specific state, e.g: current tab, search input.
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
