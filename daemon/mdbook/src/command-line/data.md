# Sub-command: `data`
Various utility flags related to `festivald`'s data.

```plaintext
Various utility flags related to `festivald` data

Usage: festivald data OPTION [ARG]

Options:
      --docs
          Open documentation locally in browser
          
          This opens `festivald'`s documentation in a web
          browser, and does not start `festivald` itself.

      --path
          Print the PATHs used by Festival
          
          All data saved by Festival is saved in these directories.
          For more information, see: <https://docs.festival.pm/daemon/disk.html>

      --state-collection-full
          Print JSON metadata about the current `Collection` on disk
          
          This output is the exact same as the `state_collection_full` JSON-RPC call.

      --delete
          Delete all Festival files that are currently on disk
          
          This deletes all `daemon` Festival folder, which contains:
          - The `Collection`
          - `daemon` configuration (`festivald.toml`)
          - Audio state (currently playing song, queue, etc)
          - Cached images for the OS media controls
          
          The PATH deleted will be printed on success.

  -h, --help
          Print help (see a summary with '-h')
```
