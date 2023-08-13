# collection_brief_albums

{{#include ../../marker/s}} v1.0.0`

---

Retrieve an array of every [`Album`](../../common-objects/album.md) title in the current [`Collection`](../../common-objects/collection.md).

The returned array of `string`'s is in [`lexicographical`](https://en.wikipedia.org/wiki/Lexicographic_order) order.

#### Inputs
`None`

#### Outputs
| Field  | Type                | Description |
|--------|---------------------|-------------|
| len    | unsigned integer    | How many `Album`'s there are
| albums | array of `string`'s | The title of every `Album` in the `Collection`

#### Example Request
```bash
festival-cli collection_brief_albums
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_brief_albums"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 3,
    "albums": [
      "aaa",
      "bbb",
      "ccc"
    ]
  },
  "id": 0
}
```
