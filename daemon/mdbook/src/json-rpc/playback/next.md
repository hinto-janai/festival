# next

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Skip to the next song in the [queue](../queue/queue.md).

Ends the queue (or repeats if a [`repeat`](../playback/repeat.md) mode is on) if at the last song.

Does nothing if the queue is empty.

#### Inputs
`None`

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli next
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"next"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
