# collection_full_albums

#### 🟡 Incomplete
This API may have [additions](../../api-stability/marker.md) in the future.

---

Retrieve an array of every [`Album`](../../common-objects/album.md) object in the current [`Collection`](../../common-objects/collection.md).

The returned array is in incrementing key order, as in:
```
Album 0,
Album 1,
Album 2,

[... etc ...]
```

#### Inputs
`None`

#### Outputs
| Field  | Type                     | Description |
|--------|--------------------------|-------------|
| len    | unsigned integer         | How many `Album`'s there are
| albums | array of `Album` objects | Every `Album` in the `Collection`

#### Example Request
```bash
festival-cli collection_full_albums
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_full_albums"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "albums": [
      {
        "title": "Album Title",
        "key": 0,
        "artist": 0,
        "release": "2019",
        "runtime": 1385,
        "song_count": 4,
        "songs": [
          0,
          1,
          7,
          11
        ],
        "discs": 0,
        "art": 525016,
        "genre": null
      },
      {
        "title": "Album Title 2",
        "key": 1,
        "artist": 0,
        "release": "2019",
        "runtime": 3605,
        "song_count": 4,
        "songs": [
          12,
          16,
          22,
          23
        ],
        "discs": 0,
        "art": 628931,
        "genre": null
      }
    ]
  },
  "id": 0
}
```
