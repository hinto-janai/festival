# stop

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Clear the [queue](../queue/queue.md) and stop playback.

#### Inputs
`None`

#### Outputs
| Field | Type             | Description |
|-------|------------------|-------------|
| len   | unsigned integer | Amount of `Song`'s cleared from the queue

#### Example Request
```bash
festival-cli stop
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"stop"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 207
  },
  "id": 0
}
```
