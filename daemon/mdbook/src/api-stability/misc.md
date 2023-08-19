# Misc
Miscellaneous parts of `festivald` that _can_ or _cannot_ be relied on.

### `festival-cli`
`festivald` and `festival-cli`'s versions are tied together to represent their compatibility.

`festivald` will be able to respond to any [`🟢 Stable`](marker.md) `festival-cli` request as long as `festivald`'s:
- Major version is the same
- Minor version is the same or greater

For example, `festivald v1.2.x` is compatible with:
- `festival-cli v1.2.x`
- `festival-cli v1.1.x`
- `festival-cli v1.0.x`

but not necessarily with `festival-cli v1.3.x` and beyond.

Note that `festivald` will still be able to communicate with newer/older `festival-cli`'s, however:
- If a new `festival-cli` is requesting a method unknown to an old `festivald`, or
- If a new `festivald` has additional output unknown to an old `festival-cli` (use of [`non-stable`](marker.md) API)

there will be communication issues.

### Config
- [`config`](../config.md) names are [`🟢 Stable`](marker.md) (`max_connections` will always be named `max_connections`)
- Their expected inputs are [`🟢 Stable`](marker.md) (`max_connections` will always want an unsigned integer)
- Their expected behavior is [`🟢 Stable`](marker.md) (`max_connections` will always control the max amount of connections)
- Default config values _may_ be changed (default `port` value may not always be `18425`)
- Additional fields _may_ be added in the future

Referencing any `JSON-RPC` methods and/or `REST` resources that are [`🔴 Unstable`](marker.md) in options like `no_auth_rpc` & `no_auth_rest` will also make your configuration [`🔴 Unstable`](marker.md).

### Command Line
- [`--flags`](../command-line/command-line.md) and sub-command names are [`🟢 Stable`](marker.md) (`festivald --path` will always be `festivald --path`)
- Their expected inputs/outputs are [`🟢 Stable`](marker.md)
- Additional flags and/or sub-commands _may_ be added in the future

### Disk
All locations and filenames of all files written to disk by `festivald` are [`🟢 Stable`](marker.md).

| Type                        | Example |
|-----------------------------|---------|
| [`Cache`](../disk.md#cache)   | `Artist` ZIP cache is always at `~/.cache/festival/daemon/zip/artist`
| [`Config`](../disk.md#config) | `Config` file is always at `~/.config/festival/daemon/festivald.toml`
| [`Data`](../disk.md#data)     | `Collection` file is always at `~/.local/share/festival/daemon/state/collection.bin`

### Documentation
[`🔴 Unstable`](marker.md).

Do not rely on the exact name or position of something within this documentation.

For example, this section [`API Stability`](api-stability.md) may be renamed to `API Reliability` and/or moved to a different chapter.

Documentation may change at any given moment to provide the most accurate information.

### Errors 
[`🔴 Unstable`](marker.md).

Do not rely on the _details_ of the `JSON-RPC` & `REST` API errors.

You can rely that an input that leads to OK/ERROR will always be the same.

### Logs
[`🔴 Unstable`](marker.md).

Do not rely on the log output of `festivald`.

### Protocol
- Transports other than HTTP(s) may be supported in the future
- Version [`2.0`](https://jsonrpc.org/specification) of `JSON-RPC` will always be used
- `JSON-RPC` will always require `HTTP POST` requests
- `REST` and documentation will always require `HTTP GET` requests
