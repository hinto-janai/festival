# API Stability
Some notes on what parts of `festivald`'s API can/cannot be relied upon.

In general:
- Things may be added, but never removed
- Inputs will never change, but their outputs _may_ contain additions in the future

These rules will apply until a potential `v2.0.0` breaking release.

## `festival-cli`
`festivald` and `festival-cli`'s versions are tied together to represent their compatibility.

`festivald` will be able to (fully) respond to `festival-cli` as long as `festivald`'s:
- Major version is the same
- Minor version is the same

For example, `festivald v1.2.x` is fully compatible with:
- `festival-cli v1.2.x`

but not necessarily with:
- `festival-cli v1.0.x`
- `festival-cli v1.1.x`
- `festival-cli v1.3.x` and beyond

Note that `festivald` will still be able to communicate with newer/older `festival-cli`'s, however:
- If a new `festival-cli` is requesting a method unknown to an old `festivald`, or
- If a new `festivald` has additional output unknown to an old `festival-cli`

there will be communication issues.

## Config
- [`config`](config.md) names are stable (`max_connections` will always be named `max_connections`)
- Their expected inputs are stable (`max_connections` will always want an unsigned integer)
- Their expected behavior is stable (`max_connections` will always control the max amount of connections)
- Default config values _may_ be changed (default `port` value may not always be `18425`)
- Additional fields _may_ be added in the future

## Command Line
- [`--flags`](command-line/command-line.md) and sub-command names are stable (`festivald --path` will always be `festivald --path`)
- Their expected inputs/outputs are stable
- Additional flags and/or sub-commands _may_ be added in the future

## Disk
All locations and filenames of all files written to disk by `festivald` are stable.

| Type                       | Example |
|----------------------------|---------|
| [`Cache`](disk.md#cache)   | `Artist` ZIP cache is always at `~/.cache/festival/daemon/zip/artist`
| [`Config`](disk.md#config) | `Config` file is always at `~/.config/festival/daemon/festivald.toml`
| [`Data`](disk.md#data)     | `Collection` file is always at `~/.local/share/festival/daemon/state/collection.bin`

## JSON-RPC
- Method names ([`collection_new`](json-rpc/collection/collection_new.md)) are stable
- Additional methods _may_ be added in the future
- Parameter names, types, and count are stable (new parameter fields to an _existing_ method will never be added)
- Additional response fields to an _existing_ method _may_ be added in the future
- Although old response fields will remain, the _ordering_ of response fields should _NOT_ be relied upon
- These rules also apply to the [`Common Objects`](common-objects/common-objects.md)

Old `v1.0.0` JSON-RPC example:
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "This will never change",
  "param": {
    "param_field": "This will never change"
  },
  "id": 0,
}

// Response
{
  "jsonrpc": "2.0",
  "result": {
    "result_field": "Introduced in v1.0.0"
  },
  "id": 0,
}
```

New `v1.x.x` JSON-RPC example:
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "This will never change",
  "param": {
    "param_field": "This will never change"
  },
  "id": 0,
}

// Response
{
  "jsonrpc": "2.0",
  "result": {
    "result_field3": "Introduced in v1.3.x",
    "result_field": "Introduced in v1.0.0",
    "result_field2": "Introduced in v1.2.x"
  },
  "id": 0,
}
```

## REST
- Endpoint names ([`/key_artist`](rest/key/artist.md)) and their input types are stable
- Endpoint outputs are stable (the above endpoint will _always_ output a ZIP file)
- Additional endpoints _may_ be added in the future
- Output filenames are stable (`/key_artist` will always output a ZIP file with the name `${ARTIST_NAME}.zip`)
- The file hierarchy within ZIPs are stable

This below example will be the same for the rest of `festivald v1.x.x`:
```plaintext
# Input
https://localhost:18425/map/My Artist

# Output
My Artist.zip

# Extracted
My Artist/
    ├─ Album Name 1/
    │    ├─ Album Name 1.jpg
    │    ├─ Song Name 1.mp3
    │
    │─ Album Name 2/
    │    ├─ Album Name 2.png
    │    ├─ Song Name 2.mp3
```

## Errors 
Do not rely on the _details_ of the `JSON-RPC` & `REST` API errors.

You can rely that an input that leads to OK/ERROR will always be the same.

## Logs
Do not rely on the log output of `festivald`.

## Documentation
Do not rely on the exact name or position of something within this documentation.

For example, this section [`API Stability`](./api-stability.md) may be renamed to `API Reliability` and/or moved to a different chapter.

Documentation may change at any given moment to provide the most accurate information.

## Protocol
- HTTP(s) will always be the transport
- Version [`2.0`](https://jsonrpc.org/specification) of `JSON-RPC` will always be used
- `JSON-RPC` will always require `POST HTTP` requests
- `REST` and documentation will always require `GET HTTP` requests
