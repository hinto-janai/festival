# Watch
`Watch` watches the filesystem for "signal" files, created by commands like `./festival --play`.

These are located in the subdirectory `signal`.

You can manually send signals to `Festival` by creating a file within the `signal` subdirectory with any of these filenames:

- `play`
- `stop`
- `next`
- `last`
- `shuffle`
- `repeat`

They'll immediately get deleted, and `Festival` will act on the signal.

| Platform | Value                                                              | Example                                                     |
|----------|--------------------------------------------------------------------|-------------------------------------------------------------|
| Linux    | `$XDG_DATA_HOME/festival` or `$HOME/.local/share/festival/signal/` | `/home/alice/.local/share/festival/signal/`                 |
| macOS    | `$HOME/Library/Application Support/Festival/signal/`               | `/Users/Alice/Library/Application Support/Festival/signal/` |
| Windows  | `{FOLDERID_LocalAppData}\Festival\signal\`                         | `C:\Users\Alice\AppData\Local\Festival\signal\`             |

## Files
| File           | Purpose |
|----------------|---------|
| watch.rs       | Main `Watch` loop
| msg.rs         | Types of messages `Watch` can send to `Kernel`

`Watch` is the only thread that sends messages to `Kernel`, but not the other way around.
