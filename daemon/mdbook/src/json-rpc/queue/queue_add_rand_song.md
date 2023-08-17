# queue_add_rand_song

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Add a random [`Song`](/common-objects/song.md) to the queue.

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| append | `string`, one of `front`, `back` or `index` | See [`Queue/Append`](/json-rpc/queue/queue.md#append)
| clear  | boolean                                     | Should the queue be cleared before adding?
| play   | boolean                                     | Should we start playing?
| index  | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used

#### Outputs
| Field | Type                                          | Description |
|-------|-----------------------------------------------|-------------|
| song  | [`Song`](/common-objects/song.md) object | The `Song` that was added to the queue

#### Example Request 1
Add to back of the queue.
```bash
festival-cli queue_add_rand_song --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_song","params":{"append":"back","clear":false}'
```

#### Example Request 2
Append at queue index 4.
```bash
festival-cli queue_add_rand_song --append index --index 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_song","params":{"append":"index","index":4,"clear":false}'
```

#### Example Request 3
Clear the queue, add `Song` 123.
```bash
festival-cli queue_add_rand_song --append front --clear --offset 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_song","params":{"append":"front","clear":true}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "song": {
      "title": "SUNFLOWER",
      "key": 2594,
      "album": 237,
      "runtime": 252,
      "sample_rate": 44100,
      "track": 1,
      "disc": null,
      "mime": "audio/mpeg",
      "extension": "mp3"
    }
  },
  "id": 0
}
```
