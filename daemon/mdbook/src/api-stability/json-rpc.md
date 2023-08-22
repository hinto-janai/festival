# JSON-RPC
[`JSON-RPC`](../json-rpc/json-rpc.md)-specific API stability edge cases.

### Naming
All method names, parameter names, and field names are in `lower_case_snake_case`.

This is [`🟢 Stable`](marker.md).

### Ordering
The ordering of [non-stable](marker.md) output/object fields is [`🔴 Unstable`](marker.md).

The ordering of [`🟢 Stable`](marker.md) output/object fields is [`🟢 Stable`](marker.md).

Old `v1.0.0` JSON-RPC example:
```json
// 🟡 Incomplete Response
{
  "jsonrpc": "2.0",
  "result": {
    "field1": "Output field 1",
    "field2": "Output field 2"
  },
  "id": 0,
}

// 🟢 Stable Response
{
  "jsonrpc": "2.0",
  "result": {
    "field1": "Output field 1", // Will always be 1, then 2.
    "field2": "Output field 2"
  },
  "id": 0,
}
```

New `v1.1.0` JSON-RPC example:
```json
// 🟡 Incomplete Response
{
  "jsonrpc": "2.0",
  "result": {
    "field3": "Output field 3", // Ordering may
    "field1": "Output field 1", // shift around
    "field4": "Output field 4", // on incomplete
    "field2": "Output field 2"  // methods.
  },
  "id": 0,
}

// 🟢 Stable Response
{
  "jsonrpc": "2.0",
  "result": {
    "field1": "Output field 1", // Still 1, 2.
    "field2": "Output field 2"
  },
  "id": 0,
}
```
