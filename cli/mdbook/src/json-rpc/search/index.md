# Search
Fuzzy similarity searches for [`Song`](../../common-objects/song.md)'s, [`Album`](../../common-objects/album.md)'s, and [`Artist`](../../common-objects/artist.md)'s.

In general: input a `string`, receive some [`Artist`](../../common-objects/artist.md)/[`Album`](../../common-objects/album.md)/[`Song`](../../common-objects/song.md) objects that are similar to the input.

If you know the _exact_ `string` for a `Song`, `Album`, or `Artist`, consider using the [`Map`](../map/map.md) methods instead.

## Warning
The output array may return empty given no matches or an empty `Collection`:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "songs": []
  },
  "id": 0
}
```
