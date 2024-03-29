# Search
Fuzzy similarity searches for [`Song`](../../common-objects/song.md)'s, [`Album`](../../common-objects/album.md)'s, [`Artist`](../../common-objects/artist.md)'s, and [`Entry`](../../common-objects/entry.md)'s.

In general: input a `string`, receive some objects that are similar to the input.

If you know the _exact_ `string` for a `Song`, `Album`, or `Artist`, consider using the [`Map`](../map/map.md) methods instead for a direct lookup.

If you know the _exact_ [`key`](../../common-objects/key.md), use the [`Key`](../key/key.md) methods instead.

### Kind
These are different "kinds" of searches you can do, affecting the result output.

Each `search` method requires one of these as input.

All of these return objects sorted from most-to-least similar.

| Kind    | Description |
|---------|-------------|
| `all`   | ALL objects will be returned
| `sim60` | Only objects `60%` similar
| `sim70` | Only objects `70%` similar
| `sim80` | Only objects `80%` similar
| `top25` | Only the top 25 results
| `top5`  | Only the top 5 results
| `top1`  | Only the top result

### Warning
The output array may return empty given no matches or an empty [`Collection`](../../common-objects/collection.md):
```json
{
  "jsonrpc": "2.0",
  "result": {
    "songs": []
  },
  "id": 0
}
```
