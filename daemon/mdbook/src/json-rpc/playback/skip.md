# skip

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Skip forwards a variable amount of `Song`'s in the current queue.

#### Inputs
| Field | Type             | Description |
|-------|------------------|-------------|
| skip  | unsigned integer | How many `Song`'s to skip. If greater than the rest of the `Song`'s in the queue, the queue will end (else a [`repeat`](/json-rpc/playback/repeat_queue.md) mode is on).


#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli skip --skip 3
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"skip","param":{"skip":3}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
