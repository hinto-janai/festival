# volume_up

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Raise the playback volume.

#### Inputs
| Field | Type             | Description |
|-------|------------------|-------------|
| up    | unsigned integer | The number to raise the volume by. Must be in-between `0..100`. If greater than `100`, it will set the volume to `100`. Volume cannot go higher than `100`.

#### Outputs
| Field    | Type             | Description |
|----------|------------------|-------------|
| previous | unsigned integer | What the volume was set to previously
| current  | unsigned integer | What the volume is now set at

#### Example Request
```bash
festival-cli volume_up --up 15
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"volume_up","params":{"up":15}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "previous": 0,
    "current": 15
  },
  "id": "festival-cli"
}
```
