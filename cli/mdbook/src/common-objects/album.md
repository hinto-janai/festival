# Album

| Field      | Type                                      | Description |
|------------|-------------------------------------------|-------------|
| title      | string                                    | The title of this `Album`
| key        | `Album` key (unsigned integer)            | The `Album` key associated with this `Album`
| artist     | `Artist` key (unsigned integer)           | The `Artist` key of the `Artist` that owns this `Album`
| release    | string                                    | Release date of this `Album` in `YYYY-MM-DD`/`YYYY-MM`/`YYYY` format, `????-??-??` if unknown
| runtime    | unsigned integer                          | The total runtime of this `Album` in seconds
| song_count | unsigned integer                          | How many `Song`'s are in this `Album`
| songs      | array of `Song` keys (unsigned integers)  | Keys to all of the `Song`'s in this `Album`, in track order
| discs      | unsigned integer                          | Count of how many "discs" are in this `Album`, most will be `0`
| art        | optional (maybe null) unsigned integer    | Size of this `Album`'s art in bytes, `null` if not found
| genre      | optional (maybe null) string              | Genre of this `Album`, `null` if not found

#### Example
```json
{
  "title": "Album Title",
  "key": 100,
  "artist": 16,
  "release": "2011-07-13",
  "runtime": 2942,
  "song_count": 3,
  "songs": [
    972,
    1024,
    1051,
  ],
  "discs": 0,
  "art": 306410,
  "genre": null
}
```
