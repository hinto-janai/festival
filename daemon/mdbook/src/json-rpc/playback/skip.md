# skip

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Skip forward a variable amount of [`Song`](../../common-objects/song.md)'s in the [queue](../queue/queue.md).

#### Inputs
| Field | Type             | Description |
|-------|------------------|-------------|
| skip  | unsigned integer | How many `Song`'s to skip. If greater than the rest of the `Song`'s in the queue, the queue will end (unless a [`repeat`](../playback/repeat.md) mode is on).


#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli skip --skip 3
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"skip","params":{"skip":3}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
