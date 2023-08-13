# collection_brief_artists

{{#include ../../marker/s}} v1.0.0`

---

Retrieve an array of every [`Artist`](../../common-objects/artist.md) name in the current [`Collection`](../../common-objects/collection.md).

The returned array of `string`'s is in [`lexicographical`](https://en.wikipedia.org/wiki/Lexicographic_order) order.

#### Inputs
`None`

#### Outputs
| Field   | Type                | Description |
|---------|---------------------|-------------|
| len     | unsigned integer    | How many `Artist`'s there are
| artists | array of `string`'s | The title of every `Artist` in the `Collection`

#### Example Request
```bash
festival-cli collection_brief_artists
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_brief_artists"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 3,
    "artists": [
      "aaa",
      "bbb",
      "ccc"
    ]
  },
  "id": 0
}
```
