# state_repeat

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Retrieve the currently set [`repeat`](/json-rpc/playback/repeat.md) mode.

#### Inputs

`None`

#### Outputs

| Field | Type                                  | Description |
|-------|---------------------------------------|-------------|
| mode  | string, one of `off`, `song`, `queue` | The currently set repeat mode. `off` mean repeat is off, `song` means [`Song`](/common-objects/song.md) repeating, `queue` means [`queue`](/json-rpc/queue/queue.md) repeating.

#### Example Request
```bash
festival-cli state_repeat
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_repeat"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "mode": "off"
  },
  "id": 0
}
```
