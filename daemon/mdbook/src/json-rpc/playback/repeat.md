# repeat

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Set the playback repeat mode.

#### Inputs
| Field | Type                                  | Description |
|-------|---------------------------------------|-------------|
| mode  | string, one of `off`, `song`, `queue` | The repeat mode to set. `off` turns off repeating, `song` turns on [`Song`](../../common-objects/song.md) repeating, `queue` turns on [`queue`](../queue/queue.md) repeating.

#### Outputs
| Field    | Type                                  | Description |
|----------|---------------------------------------|-------------|
| previous | string, one of `off`, `song`, `queue` | What the repeat mode was set to previously
| current  | string, one of `off`, `song`, `queue` | What the repeat mode is now set at


#### Example Request
```bash
festival-cli repeat --mode off
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"repeat","params":{"mode":"off"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "previous": "off",
    "current": "song"
  },
  "id": 0
}
```
