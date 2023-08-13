# collection_full_artists
Retrieve an array of every [`Artist`](../../common-objects/artist.md) object in the current [`Collection`](../../common-objects/collection.md).

The returned array is in incrementing key order, as in:
```
Artist 0,
Artist 1,
Artist 2,

[... etc ...]
```

#### Inputs
`None`

#### Outputs
| Field   | Type                      | Description |
|---------|---------------------------|-------------|
| len     | unsigned integer          | How many `Artist`'s there are
| artists | array of `Artist` objects | Every `Artist` in the `Collection`

#### Example Request
```bash
festival-cli collection_full_artists
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_full_artists"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 1,
    "artists": [
      {
        "name": "Artist Name",
        "key": 0,
        "runtime": 3561,
        "albums": [
          0,
          1
        ],
        "songs": [
          0,
          1,
          5,
          20,
          22,
          23
        ]
      }
    ]
  },
  "id": 0
}
```
