# search_song
Input a `string`, retrieve an array of [`Song`](../../common-objects/song.md)'s, sorted by how similar their titles are to the input.

#### Inputs

| Field | Type                                           | Description |
|-------|------------------------------------------------|-------------|
| input | string                                         | The string to match against, to use as input
| kind  | string, one of `all`, `sim70`, `top25`, `top1` | This dictates how many objects back you will receive. `all` means ALL `Song`'s will be returned. `sim70` means only `Song`'s that are `70%` similar will be returned. `top25` means only the top 25 results will be returned. `top1` means only the top result will be returned.

#### Outputs

| Field | Type                    | Description |
|-------|-------------------------|-------------|
| songs | array of `Song` objects | An array of `Song` objects, sorted by most similar title first

#### Example Request
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
        "extension": "flac",
        "path": "/home/hinto/Music/TIME.flac"
      }
    ]
  },
  "id": 0
}
```
