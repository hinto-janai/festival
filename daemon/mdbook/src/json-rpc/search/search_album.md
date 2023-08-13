# search_album

{{#include ../../marker/i}}

---

Input a `string`, retrieve an array of [`Album`](../../common-objects/album.md)'s, sorted by how similar their titles are to the input.

#### Inputs

| Field | Type                                           | Description |
|-------|------------------------------------------------|-------------|
| input | string                                         | The string to match against, to use as input
| kind  | string, one of `all`, `sim70`, `top25`, `top1` | This dictates how many objects back you will receive. `all` means ALL `Album`'s will be returned. `sim70` means only `Album`'s that are `70%` similar will be returned. `top25` means only the top 25 results will be returned. `top1` means only the top result will be returned.

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
