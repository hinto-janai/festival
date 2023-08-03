# collection_song_paths
Retrieve an array of _all_ the `Song` PATHs in the current [`Collection`](../../common-objects/collection.md).

#### Inputs
`None`

#### Outputs
| Field | Type             | Description |
|-------|------------------|-------------|
| len   | unsigned integer | The total count of PATHs (length of the array)
| paths | string (PATH)    | The absolute `Song` PATHs, on `festivald`'s local filesystem

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_brief"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 3,
    "paths": [
      "/home/hinto/Music/song_1.flac",
      "/home/hinto/Music/song_2.mp3",
      "/home/hinto/Music/song_3.ogg"
    ]
  },
  "id": 0
}
```
