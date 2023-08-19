# daemon_no_auth_rpc

#### 🔴 Unstable
This API may be [changed](../../api-stability/marker.md) in the future.

---

Retrieve all [`JSON-RPC` methods](../json-rpc.md) this `festivald` allows [without authorization](../../authorization/json-rpc.md), set in the [`no_auth_rpc`](../../config.md) option

The reason why this method is `🔴 Unstable` is because it will output _all_ methods, even `🔴 Unstable` ones, which may not exist in the future.

Ordering of the method names [should not be relied upon](../../api-stability/json-rpc.md).

#### Inputs
`None`

#### Outputs
| Field | Type                | Description |
|-------|---------------------|-------------|
| len   | unsigned integer    | Total amount of methods
| rpc   | array of `string`'s | The names of all the methods this `festivald` allows without authorization

#### Example Request
```bash
festival-cli daemon_no_auth_rpc
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"daemon_no_auth_rpc"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 3,
    "rpc": [
      "collection_brief_artists",
      "key_artist_entries",
      "search_album"
    ]
  },
  "id": 0
}
```
