# Command Line
`festivald` takes in `--flags` for many purposes.

Arguments passed to `festivald` will always take priority over the same [configuration](/config.md) options read from disk.

`festivald` also has a `signal` sub-command. For example, to send a signal to a `festivald` running on the same local machine, you can run:
```bash
./festivald signal --play
```
All these "signal"-related `--flags` live in the [`signal`](signal.md) sub-command.

## Examples
Here are some command-line usage examples.

#### Start `festivald` on `http://localhost:18425` (by default)
```bash
./festivald
```

#### Print the PATH used by `festivald`
```bash
./festivald --path
```

#### Delete all data created/used by `festivald`
```bash
./festivald --delete
```

#### Set log level, disable everything except `JSON-RPC`
```bash
./festivald --log-level DEBUG --disable-watch --disable-media-controls --disable-rest --disable-docs
```
