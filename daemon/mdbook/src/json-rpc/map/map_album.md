# map_album

#### 🟡 Incomplete
This API may have [additions](../../api-stability/marker.md) in the future.

---

Input an [`Artist`](../../common-objects/artist.md) name and [`Album`](../../common-objects/album.md) title, retrieve an [`Album`](../../common-objects/album.md) object.

#### Inputs

| Field  | Type   | Description |
|--------|--------|-------------|
| artist | string | `Artist` name
| album  | string | `Album` title

#### Outputs

| Field | Type            | Description |
|-------|-----------------|-------------|
| album | `Album` object | See [`Album`](../../common-objects/album.md)

#### Example Request
```bash
festival-cli map_album --artist "Rex Orange County" --album RAINBOW
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"map_album","params":{"artist":"Rex Orange County","album":"RAINBOW"}}'
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
