# state_queue_song

#### 🟡 Incomplete
This API may have [additions](../../api-stability/marker.md) in the future.

---

Retrieve state about the queue.

This returns the queue as full [`Song`](../../common-objects/song.md) objects.

Returned `Song`'s are in order of what will be played next.

#### Inputs

`None`

#### Outputs

| Field | Type                    | Description |
|-------|-------------------------|-------------|
| len   | unsigned integer        | Length of the queue
| songs | array of `Song` objects | Array of the queue's `Song`'s

#### Example Request
```bash
festival-cli state_queue_song
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_queue_song"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "songs": [
      {
        "title": "SUNFLOWER",
        "key": 2539,
        "album": 237,
        "runtime": 252,
        "sample_rate": 44100,
        "track": 1,
        "disc": null,
        "mime": "audio/mpeg",
        "extension": "mp3"
      },
      {
        "title": "BEST FRIEND",
        "key": 2517,
        "album": 237,
        "runtime": 262,
        "sample_rate": 44100,
        "track": 2,
        "disc": null,
        "mime": "audio/mpeg",
        "extension": "mp3"
      }
    ]
  },
  "id": 0
}
```
