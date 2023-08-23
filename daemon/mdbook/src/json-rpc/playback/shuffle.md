# shuffle

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

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
