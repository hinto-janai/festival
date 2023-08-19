# state_playing

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Retrieve playback status.

#### Inputs

`None`

#### Outputs

| Field   | Type    | Description |
|---------|---------|-------------|
| playing | boolean | Are we currently playing?


#### Example Request
```bash
festival-cli state_playing
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_playing"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "playing": true
  },
  "id": 0
}
```
