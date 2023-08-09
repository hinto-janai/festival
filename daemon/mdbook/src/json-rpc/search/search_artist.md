# search_artist
Input a `string`, retrieve an array of [`Artist`](../../common-objects/artist.md)'s, sorted by how similar their names are to the input.

#### Inputs

| Field | Type                                           | Description |
|-------|------------------------------------------------|-------------|
| input | string                                         | The string to match against, to use as input
| kind  | string, one of `all`, `sim70`, `top25`, `top1` | This dictates how many objects back you will receive. `all` means ALL `Artist`'s will be returned. `sim70` means only `Artist`'s that are `70%` similar will be returned. `top25` means only the top 25 results will be returned. `top1` means only the top result will be returned.

#### Outputs

| Field   | Type                      | Description |
|---------|---------------------------|-------------|
| artists | array of `Artist` objects | An array of `Artist` objects, sorted by most similar name first

#### Example Request
```bash
festival-cli search_artist --input twice --kind sim70
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search_artist","params":{"input":"twice","kind":"sim70"}}'
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
      }
    ]
  },
  "id": 0
}
```
