# shuffle

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Shuffle the current [queue](../queue/queue.md), then set the current `Song` to the 1st `Song` in the queue.

#### Inputs
`None`

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli shuffle
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"shuffle"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```