# key_other_entries

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Input a [`Song` key](/common-objects/key.dm), retrieve all the other [`Song`](/common-objects/song.md)'s in the same [`Album`](/common-objects/album.md) in [`Entry`](/common-objects/entry.md) form.

The `Entry`'s are sorted by `Track + Disc order`.

#### Inputs

| Field | Type                                         | Description |
|-------|----------------------------------------------|-------------|
| key   | `Song` key (unsigned integer)                | See [`Key`](/common-objects/key.md)

#### Outputs

| Field   | Type                     | Description |
|---------|--------------------------|-------------|
| len     | unsigned integer         | How many `Entry`'s there are
| entries | array of `Entry` objects | See [`Entry`](/common-objects/entry.md)

#### Example Request
```bash
festival-cli key_other_entries --key 2444
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_other_entries","params":{"key":2444}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "entries": [
      {
        "path": "/home/hinto/Music/Rex Orange County/RAINBOW/SUNFLOWER.mp3",
        "key_artist": 62,
        "key_album": 222,
        "key_song": 2444,
        "artist": "Rex Orange County",
        "album": "RAINBOW",
        "song": "SUNFLOWER"
      },
      {
        "path": "/home/hinto/Music/Rex Orange County/RAINBOW/BEST FRIEND.mp3",
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
