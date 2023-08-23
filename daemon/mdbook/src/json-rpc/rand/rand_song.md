# rand_song

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Access a random [`Song`](../../common-objects/song.md) in your [`Collection`](../../common-objects/collection.md).

#### Inputs

`None`

#### Outputs

| Field | Type          | Description |
|-------|---------------|-------------|
| song  | `Song` object | See [`Song`](../../common-objects/song.md)

#### Example Request
```bash
festival-cli rand_song
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"rand_song"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "song": {
      "title": "Home Alone",
      "key": 2825,
      "album": 269,
      "runtime": 182,
      "sample_rate": 48000,
      "track": 1,
      "disc": 1,
      "mime": "audio/x-flac",
      "extension": "flac"
    }
  },
  "id": 0
}
```
