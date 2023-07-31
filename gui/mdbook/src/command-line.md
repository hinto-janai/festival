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

      --path
          Print the PATH used by Festival
          
          All data saved by Festival is saved here.
          For more information, see:
          https://github.com/hinto-janai/festival/tree/main/gui#Disk

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
          
          This deletes the entire `GUI` Festival folder, which contains:
          - The `Collection`
          - `GUI` settings (sort methods, color, etc)
          - `GUI` state (currently selected album/artist, etc)
          - Audio state (currently playing song, queue, etc)
          - Cached images for the OS media controls
          
          The PATH deleted will be printed on success.

      --log-level <OFF|ERROR|INFO|WARN|DEBUG|TRACE>
          Set filter level for console logs
          
          [default: INFO]

  -v, --version
          Print version

  -h, --help
          Print help (see a summary with '-h')
```
