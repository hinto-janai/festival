# queue_remove_range

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Remove a range of queue indices.

If either `start` or `end` is out-of-bounds, this method will do nothing.

#### Inputs
| Field  | Type             | Description |
|--------|------------------|-------------|
| start  | unsigned integer | The beginning index to start removing from
| end    | unsigned integer | The index to stop at
| skip   | boolean          | Should we skip to the next song if the range includes the current one? `false` will leave playback as is, even if the current song is wiped from the queue.

#### `start` and `end`
This method will start removing from the `start` index up UNTIL the `end` index.

It is a NON-inclusive range, i.e: it is `0..4`, not `0..=4`.

For example, given `"start": 0` and `"end": 4`:
```plaintext
# The queue.
index 0 | song_1 <--- We start removing from here.
index 1 | song_2    |
index 2 | song_3    |
index 3 | song_4 <--- To here.
index 4 | song_5 <--- This song is not removed.
index 5 | song_6
```

#### Outputs
| Field         | Type             | Description |
|---------------|------------------|-------------|
| out_of_bounds | boolean          | If either `start` or `end` was out-of-bounds
| start         | unsigned integer | The provided `start`
| end           | unsigned integer | The provided `end`
| queue_len     | unsigned integer | The queue length

#### Example Request 1
Remove the 1st `Song` in the queue.
```bash
festival-cli queue_remove_range --start 0 --end 1 --skip
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_remove_range","params":{"start":0,"end":1,"skip":true}}'
```

#### Example Request 2
Remove 2, 3, 4 from the queue.
```bash
festival-cli queue_remove_range --start 2 --end 5 --skip
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_remove_range","params":{"start":2,"end":5,"skip":true}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "out_of_bounds": false,
    "start": 2,
    "end": 5,
    "queue_len": 5
  },
  "id": 0
}
```
