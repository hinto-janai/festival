# current_album

#### 🟡 Incomplete
This API may have [additions](../../api-stability/marker.md) in the future.

---

Access the [`Album`](../../common-objects/album.md) of the currently set [`Song`](../../common-objects/song.md).

#### Inputs

`None`

#### Outputs

| Field | Type           | Description |
|-------|----------------|-------------|
| album | `Album` object | See [`Album`](../../common-objects/album.md)

#### Example Request
```bash
festival-cli current_album
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"current_album"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "album": {
      "title": "RAINBOW",
      "key": 237,
      "artist": 65,
      "release": "????-??-??",
      "runtime": 1090,
      "song_count": 6,
      "songs": [
        2594,
        2540,
        2600,
        2496,
        2557,
        2500
      ],
      "discs": 0,
      "art": 7753,
      "genre": null
    }
  },
  "id": 0
}
```
