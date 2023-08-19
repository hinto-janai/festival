# volume

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Set the playback volume.

#### Inputs
| Field     | Type             | Description |
|-----------|------------------|-------------|
| volume    | unsigned integer | The volume % to set. Must be in-between `0..100`. If greater than `100`, it will set the volume to `100`.

#### Outputs
| Field    | Type             | Description |
|----------|------------------|-------------|
| previous | unsigned integer | What the volume was set to previously
| current  | unsigned integer | What the volume is now set at

#### Example Request
```bash
festival-cli volume --volume 15
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"volume","params":{"volume":15}}'
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
