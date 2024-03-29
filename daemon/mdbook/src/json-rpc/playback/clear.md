# clear

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Clear the [queue](../queue/queue.md).

#### Inputs
| Field    | Type    | Description |
|----------|---------|-------------|
| playback | boolean | If there is a [`Song`](../../common-objects/song.md) currently set and playing, should we continue playback?

#### Outputs
| Field | Type             | Description |
|-------|------------------|-------------|
| len   | unsigned integer | Amount of `Song`'s cleared from the queue

#### Example Request
```bash
festival-cli clear
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"clear","params":{"playback":false}}'
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
