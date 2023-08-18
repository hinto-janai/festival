# state_queue_key

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Retrieve state about the queue.

This returns the queue as [`Song`](/common-objects/song.md) [`key`](/common-objects/key.md)'s.

Returned `key`'s are in order of what will be played next.

#### Inputs

`None`

#### Outputs

| Field | Type                                     | Description |
|-------|------------------------------------------|-------------|
| len   | unsigned integer                         | Length of the queue
| keys  | array of `Song` keys (unsigned integers) | Array of the queue's `Song`'s as keys

#### Example Request
```bash
festival-cli state_queue_key
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_queue_key"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 5,
    "keys": [
      2896,
      2899,
      2904,
      2906,
      2911
    ]
  },
  "id": 0
}
```
