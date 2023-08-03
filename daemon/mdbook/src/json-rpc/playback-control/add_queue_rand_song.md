# add_queue_key_song
Add a random [`Song`](../../common-objects/song.md) to the queue.

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| append | `string`, one of `front`, `back` or `index` | In which way should we add to the queue? `front` means to the front of the queue. `back` means to the back. `index` means at an exact queue index. Queue index starts at `0`, so to mimic `front`, you would provide `0`.
| clear  | boolean                                     | Should the queue be cleared before adding?

#### `append`
If using `"append": index`, you must also specific the index like so:
```json
"append":{"index":0},
```
compared to `front` and `back`:
```json
"append":"front",
```
```json
"append":"back",
```

#### Outputs

`null` if everything went okay.

#### Example Request 1
```bash
# Add to back of the queue.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_rand_song","params":{"append":"back","clear":false}'
```

#### Example Request 2
```bash
# Append at queue index 4.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_rand_song","params":{"append":{"index":4},"clear":false}'
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
      "extension": "mp3",
      "path": "/home/hinto/Music/SUNFLOWER.mp3"
    }
  },
  "id": 0
}
```
