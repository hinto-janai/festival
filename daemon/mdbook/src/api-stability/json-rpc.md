# JSON-RPC
[`JSON-RPC`](../json-rpc/json-rpc.md)-specific API stability notes.

### Ordering
The _ordering_ of [non-stable](marker.md) output/object fields should not be relied upon.

Old `v1.0.0` JSON-RPC example:
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "method_name",
  "param": {
    "param_field": "param_input"
  },
  "id": 0,
}

// Response
{
  "jsonrpc": "2.0",
  "result": {
    "field1": "Output field 1",
    "field2": "Output field 2"
  },
  "id": 0,
}
```

New `v1.x.x` JSON-RPC example:
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "method_name",
  "param": {
    "param_field": "param_input"
  },
  "id": 0,
}

// Response
{
  "jsonrpc": "2.0",
  "result": {
    "field3": "Output field 3",
    "field1": "Output field 1",
    "field4": "Output field 4",
    "field2": "Output field 2"
  },
  "id": 0,
}
```