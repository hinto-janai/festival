# search

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Input a `string`, retrieve arrays of [`Artist`](/common-objects/artist.md)'s, [`Album`](/common-objects/album.md)'s, and [`Song`](/common-objects/song.md)'s, sorted by how similar their names/titles are to the input.

#### Inputs

| Field | Type                                                                     | Description |
|-------|--------------------------------------------------------------------------|-------------|
| input | string                                                                   | The string to match against, to use as input
| kind  | string, one of `all`, `sim60`, `sim70`, `sim80`, `top25`, `top5`, `top1` | See [`Search/Kind`](/json-rpc/search/index.md#Kind)

#### Outputs

| Field   | Type                      | Description |
|---------|---------------------------|-------------|
| artists | array of `Artist` objects | An array of `Artist` objects, sorted by most similar name first
| albums  | array of `Album` objects  | An array of `Album` objects, sorted by most similar title first
| songs   | array of `Song` objects   | An array of `Song` objects, sorted by most similar title first

#### Example Request
```bash
festival-cli search --input twice --kind sim70
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search","params":{"input":"twice","kind":"sim70"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "artists": [
      {
        "name": "TWICE",
        "key": 106,
        "runtime": 343,
        "albums": [
          598
        ],
        "songs": [
          5411
        ]
      },
    ],
    "albums": [
      {
        "title": "TIME",
        "key": 271,
        "artist": 42,
        "release": "2014-01-21",
        "runtime": 2904,
        "song_count": 3,
        "songs": [
          3058,
          3095,
          3121
        ],
        "discs": 0,
        "art": 1264656,
        "genre": null
      }
    ],
    "songs": [
      {
        "title": "TIME",
        "key": 5560,
        "album": 538,
        "runtime": 249,
        "sample_rate": 44100,
        "track": 5,
        "disc": 1,
        "mime": "audio/x-flac",
        "extension": "flac"
      }
    ]
  },
  "id": 0
}
```
