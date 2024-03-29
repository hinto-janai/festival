# search_song

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Input a `string`, retrieve an array of [`Song`](../../common-objects/song.md)'s, sorted by how similar their titles are to the input.

#### Inputs

| Field | Type                                                                     | Description |
|-------|--------------------------------------------------------------------------|-------------|
| input | string                                                                   | The string to match against, to use as input
| kind  | string, one of `all`, `sim60`, `sim70`, `sim80`, `top25`, `top5`, `top1` | See [`Search/Kind`](../search/index.md#Kind)

#### Outputs

| Field | Type                    | Description |
|-------|-------------------------|-------------|
| songs | array of `Song` objects | An array of `Song` objects, sorted by most similar title first

#### Example Request
```bash
festival-cli search_song --input time --kind sim70
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search_song","params":{"input":"time","kind":"sim70"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "songs": [
      {
        "title": "TIME",
        "key": 5412,
        "album": 528,
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
