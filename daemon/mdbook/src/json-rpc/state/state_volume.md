# state_volume

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Retrieve the current volume level.

#### Inputs

`None`

#### Outputs

| Field     | Type                                                | Description |
|-----------|-----------------------------------------------------|-------------|
| volume    | unsigned integer in between `0..100`                | The current volume level


#### Example Request
```bash
festival-cli state_volume
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_volume"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "volume": 25
  },
  "id": 0
}
```
