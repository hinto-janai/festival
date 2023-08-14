# queue_set_index

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Set the current `Song` to a queue index.

#### Inputs
| Field  | Type             | Description |
|--------|------------------|-------------|
| index  | unsigned integer | An index in the queue (1st `Song` is index `0`, 2nd `Song` is index `1`, etc). The current state of the "queue" can be viewed with [`state_audio`](../state/state_audio.md).

If the `index` is out-of-bounds (queue has 10 songs, index is `10` or greater), the queue will end.

#### Outputs
| Field         | Type    | Description |
|---------------|---------|-------------|
| out_of_bounds | boolean | If the provided index was equal to or greater than the queue length.

#### Example Request
Set the current `Song` to index `4`.
```bash
festival-cli queue_set_index --index 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_set_index","params":{"index":4}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "out_of_bounds": false
  },
  "id": 0
}
```
