# search_album

#### 🟡 Incomplete
This API may have [additions](../../api-stability/marker.md) in the future.

---

Input a `string`, retrieve an array of [`Album`](../../common-objects/album.md)'s, sorted by how similar their titles are to the input.

#### Inputs

| Field | Type                                                                     | Description |
|-------|--------------------------------------------------------------------------|-------------|
| input | string                                                                   | The string to match against, to use as input
| kind  | string, one of `all`, `sim60`, `sim70`, `sim80`, `top25`, `top5`, `top1` | See [`Search/Kind`](../search/index.md#Kind)

#### Outputs

| Field  | Type                     | Description |
|--------|--------------------------|-------------|
| albums | array of `Album` objects | An array of `Album` objects, sorted by most similar title first

#### Example Request
```bash
festival-cli search_album --input time --kind sim70
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search_album","params":{"input":"time","kind":"sim70"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
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
    ]
  },
  "id": 0
}
```
