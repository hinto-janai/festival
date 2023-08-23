# queue_add_rand_entry

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Add a random [`Song`](../../common-objects/song.md) to the queue, receive it back in [`Entry`](../../common-objects/entry.md) form.

This is the same as [`queue_add_rand_song`](../queue/queue_add_rand_song.md) but returns an `Entry`.

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| append | `string`, one of `front`, `back` or `index` | See [`Queue/Append`](../queue/queue.md#append)
| clear  | boolean                                     | Should the queue be cleared before adding?
| play   | boolean                                     | Should we start playing?
| index  | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used

#### Outputs
| Field | Type                                       | Description |
|-------|--------------------------------------------|-------------|
| entry | [`Entry`](../../common-objects/entry.md) object | The `Song` that was added to the queue, in `Entry` form

#### Example Request 1
Add to back of the queue.
```bash
festival-cli queue_add_rand_entry --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_entry","params":{"append":"back","clear":false,"play":false}}'
```

#### Example Request 2
Insert at queue index 4.
```bash
festival-cli queue_add_rand_entry --append index --index 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_entry","params":{"append":"index","clear":false,"play":false,"index":4}}'
```

#### Example Request 3
Clear the queue, add to front of queue.
```bash
festival-cli queue_add_rand_entry --append front --clear --play
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_entry","params":{"append":"front","clear":true,"play":true}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "entry": {
      "path": "/home/hinto/Music/Rex Orange County/RAINBOW/SUNFLOWER.mp3",
      "key_artist": 69,
      "key_album": 254,
      "key_song": 2738,
      "artist": "Rex Orange County",
      "album": "RAINBOW",
      "song": "SUNFLOWER"
    }
  },
  "id": 0
}
```
