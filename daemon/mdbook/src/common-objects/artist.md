# Artist

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

The [keys](/common-objects/key.md) inside `albums` is sorted by `Release date`.

The keys inside `songs` is sorted by `Track + Disc order`.

| Field   | Type                                      | Description |
|---------|-------------------------------------------|-------------|
| name    | string                                    | The `Artist`'s name
| key     | `Artist` key (unsigned integer)           | The `Artist` key associated with this `Artist`
| runtime | unsigned integer                          | The total runtime of all songs owned by this `Artist` in seconds
| albums  | array of [`Album`](/common-objects/album.md) keys (unsigned integers) | Keys to all `Album`'s owned by this `Artist`, in release order
| songs   | array of [`Song`](/common-objects/song.md) keys (unsigned integers)  | Keys to all `Songs`'s owned by this `Artist`, in `Album` release order, then `Song` track order

#### Example
```json
{
  "name": "Artist Name",
  "key": 65,
  "runtime": 7583,
  "albums": [
    255,
    263
  ],
  "songs": [
    2829,
    2832,
    2835,
    2841
  ]
}
```
