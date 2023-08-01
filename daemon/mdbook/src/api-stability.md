# API Stability
Some notes on what parts of `festivald`'s API can/cannot be relied upon.

In general, things may be added, but never removed from the public API.

These rules will apply until a potential `v2.0.0` breaking release.

## Config
- [`config`](config.md) names are stable (`max_connections` will always be named `max_connections`)
- Their expected inputs are stable (`max_connections` will always want an unsigned integer)
- Their expected behavior is stable (`max_connections` will always control the max amount of connections)
- Default config values _may_ be changed (default `port` value may not always be `18425`)

## Command Line
- [`--flags`](command-line/command-line.md) and sub-command names are stable (`festivald data --path` will always be `festivald data --path`)
- Their expected inputs/outputs are stable

## Disk
All locations and filenames of all files written to disk by `festivald` are stable.

| Type                       | Example |
|----------------------------|---------|
| [`Cache`](disk.md#cache)   | `Artist` ZIP cache is always at `~/.cache/festival/daemon/zip/artist`
| [`Config`](disk.md#config) | `Config` file is always at `~/.config/festival/daemon/festivald.toml`
| [`Data`](disk.md#data)     | `Collection` file is always at `~/.local/share/festival/daemon/state/collection.bin`

## JSON-RPC
- Method names ([`new_collection`](json-rpc/collection/new_collection.md)) and their input types are stable
- Parameter names ([`volume`](json-rpc/playback-control/volume.md)) are stable
- More fields _may_ be added in the future, previous fields will remain
- Although old fields will remain, the _ordering_ of fields should _NOT_ be relied upon
- These rules also apply to the [`Common Objects`](common-objects/common-objects.md)

Old `v1.0.0` JSON-RPC example:
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "Introduced in v1.0.0",
  "param": {
    "param_field": "Introduced in v1.0.0",
    "param_field2": "Introduced in v1.0.0"
  },
  "id": 0,
}

// Response
{
  "jsonrpc": "2.0",
  "result": {
    "result_field": "Introduced in v1.0.0",
    "result_field2": "Introduced in v1.0.0"
  },
  "id": 0,
}
```

New `v1.x.x`+ JSON-RPC example:
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "Introduced in v1.0.0",
  "param": {
    "param_field3": "Introduced in v1.x.x",
    "param_field2": "Introduced in v1.0.0",
    "param_field": "Introduced in v1.0.0"
  },
  "id": 0,
}

// Response
{
  "jsonrpc": "2.0",
  "result": {
    "result_field": "Introduced in v1.0.0",
    "result_field3": "Introduced in v1.x.x",
    "result_field2": "Introduced in v1.0.0"
  },
  "id": 0,
}
```

## REST
- Endpoint names ([`/key_artist`](rest/key/artist.md)) and their input types are stable
- Endpoint outputs are stable (the above endpoint will _always_ output a ZIP file)
- More endpoints could be added in the future, previous endpoints will remain
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
