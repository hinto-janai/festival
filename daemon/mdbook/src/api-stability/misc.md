# Misc
Miscellaneous parts of `festivald` that _can_ or _cannot_ be relied on.

## `festival-cli`
`festivald` and `festival-cli`'s versions are tied together to represent their compatibility.

`festivald` will be able to respond to any [`stable`](/api-stability/marker.md) `festival-cli` request as long as `festivald`'s:
- Major version is the same
- Minor version is the same or greater

For example, `festivald v1.2.x` is compatible with:
- `festival-cli v1.2.x`
- `festival-cli v1.1.x`
- `festival-cli v1.0.x`

but not necessarily with `festival-cli v1.3.x` and beyond.

Note that `festivald` will still be able to communicate with newer/older `festival-cli`'s, however:
- If a new `festival-cli` is requesting a method unknown to an old `festivald`, or
- If a new `festivald` has additional output unknown to an old `festival-cli` (use of [`non-stable`](/api-stability/marker.md) API)

there will be communication issues.

## Config
- [`config`](/config.md) names are stable (`max_connections` will always be named `max_connections`)
- Their expected inputs are stable (`max_connections` will always want an unsigned integer)
- Their expected behavior is stable (`max_connections` will always control the max amount of connections)
- Default config values _may_ be changed (default `port` value may not always be `18425`)
- Additional fields _may_ be added in the future

## Command Line
- [`--flags`](/command-line/command-line.md) and sub-command names are stable (`festivald --path` will always be `festivald --path`)
- Their expected inputs/outputs are stable
- Additional flags and/or sub-commands _may_ be added in the future

## Disk
All locations and filenames of all files written to disk by `festivald` are stable.

| Type                        | Example |
|-----------------------------|---------|
| [`Cache`](/disk.md#cache)   | `Artist` ZIP cache is always at `~/.cache/festival/daemon/zip/artist`
| [`Config`](/disk.md#config) | `Config` file is always at `~/.config/festival/daemon/festivald.toml`
| [`Data`](/disk.md#data)     | `Collection` file is always at `~/.local/share/festival/daemon/state/collection.bin`

## Documentation
Do not rely on the exact name or position of something within this documentation.

For example, this section [`API Stability`](/api-stability/api-stability.md) may be renamed to `API Reliability` and/or moved to a different chapter.

Documentation may change at any given moment to provide the most accurate information.

## Errors 
Do not rely on the _details_ of the `JSON-RPC` & `REST` API errors.

You can rely that an input that leads to OK/ERROR will always be the same.

## Logs
Do not rely on the log output of `festivald`.

## Protocol
- HTTP(s) will always be the transport
- Version [`2.0`](https://jsonrpc.org/specification) of `JSON-RPC` will always be used
- `JSON-RPC` will always require `POST HTTP` requests
- `REST` and documentation will always require `GET HTTP` requests
