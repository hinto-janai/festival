# queue_set_index

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Set the current `Song` to a queue index.

If the `index` is out-of-bounds (queue has 10 songs, index is `10` or greater), this method will do nothing.

#### Inputs
| Field  | Type             | Description |
|--------|------------------|-------------|
| index  | unsigned integer | An index in the queue (1st `Song` is index `0`, 2nd `Song` is index `1`, etc). The current state of the "queue" can be viewed with [`state_audio`](../state/state_audio.md).

#### Outputs
| Field         | Type             | Description |
|---------------|------------------|-------------|
| out_of_bounds | boolean          | If the provided index was equal to or greater than the queue length.
| index         | unsigned integer | The provided `index`
| queue_len     | unsigned integer | The queue length

#### Example Request
Set the current `Song` to index `4`.
```bash
festival-cli queue_set_index --index 123
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_set_index","params":{"index":123}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "out_of_bounds": true,
    "index": 123,
    "queue_len": 0
  },
  "id": 0
}
```
