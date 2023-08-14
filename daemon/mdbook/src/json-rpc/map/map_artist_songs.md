# map_artist_songs

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Input an [`Artist`](/common-objects/artist.md), retrieve all their [`Song`](/common-objects/song.md)'s.

The `Song`'s are sorted by `Album release date`, then `Track + Disc order`.

#### Inputs

| Field  | Type   | Description |
|--------|--------|-------------|
| artist | string | `Artist` name

#### Outputs

| Field | Type                    | Description |
|-------|-------------------------|-------------|
| len   | unsigned integer        | How many `Song`'s there are
| songs | array of `Song` objects | See [`Song`](/common-objects/song.md)

#### Example Request
```bash
festival-cli map_artist_songs --artist "Rex Orange County"
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"map_artist_songs","params":{"artist":"Rex Orange County"}}'
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
        "map": 2444,
        "album": 222,
        "runtime": 252,
        "sample_rate": 44100,
        "track": 1,
        "disc": null,
        "mime": "audio/mpeg",
        "extension": "mp3"
      },
      {
        "title": "BEST FRIEND",
        "map": 2398,
        "album": 222,
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
