# daemon_no_auth_rest

#### 🔴 Unstable
This API's output may be [changed](../../api-stability/marker.md) in the future.

---

Retrieve all [`REST` resources](../../rest/rest.md) this `festivald` allows [without authorization](../../authorization/rest.md), set in the [`no_auth_rest`](../../config.md) option

The reason why this method is `🔴 Unstable` is because it will output _all_ `REST` resources, even `🔴 Unstable` ones, which may not exist in the future.

Ordering of the resource names [should not be relied upon](../../api-stability/rest.md).

#### Inputs
`None`

#### Outputs
| Field | Type                | Description |
|-------|---------------------|-------------|
| len   | unsigned integer    | Total amount of methods
| rest  | array of `string`'s | The names of all the `REST` resources this `festivald` allows without authorization

#### Example Request
```bash
festival-cli daemon_no_auth_rest
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"daemon_no_auth_rest"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "rest": [
      "song",
      "art"
    ]
  },
  "id": 0
}
```
