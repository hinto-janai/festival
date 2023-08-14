# map_artist_entries

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Input an [`Artist`](/common-objects/artist.md), retrieve all their [`Song`](/common-objects/song.md)'s in [`Entry`](/common-objects/entry.md) form.

The `Entry`'s are sorted by `Album release date`, then `Track + Disc order`.

#### Inputs

| Field  | Type   | Description |
|--------|--------|-------------|
| artist | string | `Artist` name

#### Outputs

| Field   | Type                     | Description |
|---------|--------------------------|-------------|
| len     | unsigned integer         | How many `Entry`'s there are
| entries | array of `Entry` objects | See [`Entry`](/common-objects/entry.md)

#### Example Request
```bash
festival-cli map_artist_entries --artist "Rex Orange County"
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"map_artist_entries","params":{"artist":"Rex Orange County"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "entries": [
      {
        "path": "/home/hinto/Rex Orange County/RAINBOW/SUNFLOWER.mp3",
        "key_artist": 62,
        "key_album": 222,
        "key_song": 2444,
        "artist": "Rex Orange County",
        "album": "RAINBOW",
        "song": "SUNFLOWER"
      },
      {
        "path": "/home/hinto/Rex Orange County/RAINBOW/BEST FRIEND.mp3",
        "key_artist": 62,
        "key_album": 222,
        "key_song": 2398,
        "artist": "Rex Orange County",
        "album": "RAINBOW",
        "song": "BEST FRIEND"
      }
    ]
  },
  "id": 0
}
```
