# collection_full_songs

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Retrieve an array of every [`Song`](/common-objects/song.md) object in the current [`Collection`](/common-objects/collection.md).

The returned array is in incrementing [key](/common-objects/key.md) order, as in:
```
Song 0,
Song 1,
Song 2,

[... etc ...]
```

#### Inputs
`None`

#### Outputs
| Field | Type                    | Description |
|-------|-------------------------|-------------|
| len   | unsigned integer        | How many `Song`'s there are
| songs | array of `Song` objects | Every `Song` in the `Collection`

#### Example Request
```bash
festival-cli collection_full_songs
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_full_songs"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "songs": [
      {
        "title": "Song Title 1",
        "key": 0,
        "album": 0,
        "runtime": 371,
        "sample_rate": 96000,
        "track": 1,
        "disc": 1,
        "mime": "audio/x-flac",
        "extension": "flac"
      },
      {
        "title": "Song Title 2",
        "key": 1,
        "album": 0,
        "runtime": 348,
        "sample_rate": 96000,
        "track": 2,
        "disc": 1,
        "mime": "audio/x-flac",
        "extension": "flac"
      }
    ]
  },
  "id": 0
}
```
