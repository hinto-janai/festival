# back

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Go backwards a variable amount of [`Song`](/common-objects/song.md)'s in the [queue](/json-rpc/queue/queue.md).

#### Inputs
| Field | Type             | Description |
|-------|------------------|-------------|
| back  | unsigned integer | How many `Song`'s to go backwards. This will _not_ wrap around if we hit the 1st `Song` in the queue.

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli back --back 10
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"back","params":{"back":10}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
