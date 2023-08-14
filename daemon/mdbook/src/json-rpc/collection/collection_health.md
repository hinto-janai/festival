# collection_health

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Retrieve the health of the current [`Collection`](/common-objects/collection.md).

This method tells you how many [`Song`](/common-objects/song.md)'s referenced by the current `Collection` have a _missing_ underlying file, i.e, there is no file at the PATH the `Collection` points to for a particular `Song`.

Missing `Song`'s will be returned in [`Entry`](/common-objects/entry.md) form.

#### Inputs
`None`

#### Outputs

| Field       | Type                     | Description |
|-------------|--------------------------|-------------|
| all_ok      | boolean                  | If the underlying file for every single `Song` exists, this is `true`, else if even 1 is missing, it is `false`
| song_len    | unsigned integer         | The total count of `Song`'s in the `Collection`
| missing_len | unsigned integer         | The total count of `Song`'s with missing underlying files
| missing     | array of `Entry` objects | An array of each `Song` that is missing, in [`Entry`](/common-objects/entry.md) object form

#### Example Request
```bash
festival-cli collection_health
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_health"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "all_ok": false,
    "song_len": 24,
    "missing_len": 2,
    "missing": [
      {
        "path": "/home/hinto/Music/song.flac",
        "key_artist": 0,
        "key_album": 0,
        "key_song": 0,
        "artist": "Artist Name",
        "album": "Album Title",
        "song": "Song Title"
      },
      {
        "path": "/home/hinto/Music/song2.flac",
        "key_artist": 1,
        "key_album": 1,
        "key_song": 1,
        "artist": "Artist Name 2",
        "album": "Album Title 2",
        "song": "Song Title 2"
      }
    ]
  },
  "id": 0
}
```
