# current_song

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Access the currently set [`Song`](/common-objects/song.md).

#### Inputs

`None`

#### Outputs

| Field | Type          | Description |
|-------|---------------|-------------|
| song  | `Song` object | See [`Song`](/common-objects/song.md)

#### Example Request
```bash
festival-cli current_song
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"current_song"}'
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
