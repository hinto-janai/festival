# queue_add_rand_album

{{#include ../../marker/i}}

---

Add a random [`Album`](../../common-objects/album.md) to the queue.

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| append | `string`, one of `front`, `back` or `index` | In which way should we add to the queue? `front` means to the front of the queue. `back` means to the back. `index` means at an exact queue index. Queue index starts at `0`, so to mimic `front`, you would provide `0`.
| clear  | boolean                                     | Should the queue be cleared before adding?
| index  | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used
| offset | optional (maybe_null) unsigned integer      | If this method is responsible for setting the current `Song`, should we start at an offset within the `Album`? e.g, starting at the first `Song` would be offset `0`, starting at the 3rd `Song` would be offset `2`, etc.

#### `offset`
If this method also happens to set the current `Song` (added to empty queue, added to front, etc), this field lets you _start_ at a particular `Song` offset.

The `Song`'s before the offset will still be added, but the _current_ `Song` set will be the one at the offset.

If the offset is out of bounds, it will start at the first `Song`.

The exact ordering of the [`Album`](../../common-objects/album.md)'s songs and what the offsets are relative to is the same as the internal ordering: [`Song`](../../common-objects/song.md) track order.

For example, given `"offset": 3`:
```plaintext
# Album's songs.
index 0 | song_1 <---/ These are still added to the queue, but..
index 1 | song_2 <--/
index 2 | song_3 <-/
index 3 | song_4 <--- We will start playing from this `Song`.
index 4 | song_5
index 5 | song_6
```

#### Outputs
| Field         | Type                                            | Description |
|---------------|-------------------------------------------------|-------------|
| album         | [`Album`](../../common-objects/album.md) object | The `Album` that was added to the queue

#### Example Request 1
Add to back of the queue.
```bash
festival-cli queue_add_rand_album --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_album","params":{"append":"back","clear":false}}'
```

#### Example Request 2
Append at queue index 4.
```bash
festival-cli queue_add_rand_album --append index --index 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_album","params":{"append":"index","index":4,"clear":false}}'
```

#### Example Request 3
Clear the queue, add all the `Song`'s in this `Album`, but start at the 5th `Song` (offset 4).
```bash
festival-cli queue_add_rand_album --append front --clear --offset 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_album","params":{"append":"front","clear":true,"offset":4}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "album": {
      "title": "RAINBOW",
      "key": 237,
      "artist": 65,
      "release": "????-??-??",
      "runtime": 1090,
      "song_count": 6,
      "songs": [
        2594,
        2540,
        2600,
        2496,
        2557,
        2500
      ],
      "discs": 0,
      "art": 7753,
      "genre": null
    }
  },
  "id": 0
}
```