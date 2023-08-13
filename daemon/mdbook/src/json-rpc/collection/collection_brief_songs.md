# collection_brief_songs
Retrieve an array of every [`Song`](../../common-objects/song.md) title in the current [`Collection`](../../common-objects/collection.md).

The returned array of `string`'s is in [`lexicographical`](https://en.wikipedia.org/wiki/Lexicographic_order) order.

#### Inputs
`None`

#### Outputs
| Field | Type                | Description |
|-------|---------------------|-------------|
| len   | unsigned integer    | How many `Song`'s there are
| songs | array of `string`'s | The title of every `Song` in the `Collection`

#### Example Request
```bash
festival-cli collection_brief_songs
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_brief_songs"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 3,
    "songs": [
      "aaa",
      "bbb",
      "ccc"
    ]
  },
  "id": 0
}
```
