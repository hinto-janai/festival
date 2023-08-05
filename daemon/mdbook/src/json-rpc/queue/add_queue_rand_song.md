# add_queue_key_song
Add a random [`Song`](../../common-objects/song.md) to the queue.

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| append | `string`, one of `front`, `back` or `index` | In which way should we add to the queue? `front` means to the front of the queue. `back` means to the back. `index` means at an exact queue index. Queue index starts at `0`, so to mimic `front`, you would provide `0`.
| index  | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used
| clear  | boolean                                     | Should the queue be cleared before adding?

#### Outputs
| Field         | Type    | Description |
|---------------|---------|-------------|
| out_of_bounds | boolean | If the `index` append was chosen and the index was out of bounds

#### Example Request 1
```bash
# Add to back of the queue.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_rand_song","params":{"append":"back","clear":false}'
```

#### Example Request 2
```bash
# Append at queue index 4.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_rand_song","params":{"append":"index","index":4,"clear":false}'
```

#### Example Request 3
```bash
# Clear the queue, add `Song` 123.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_rand_song","params":{"append":"front","clear":true}'
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
