# add_queue_key_artist
Add an [`Artist`](../../common-objects/artist.md) to the queue with an `Artist` [key](../../common-objects/key.md).

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| key    | `Artist` key (unsigned integer)             | See [`Key`](key.md)
| append | `string`, one of `front`, `back` or `index` | In which way should we add to the queue? `front` means to the front of the queue. `back` means to the back. `index` means at an exact queue index. Queue index starts at `0`, so to mimic `front`, you would provide `0`.
| clear  | boolean                                     | Should the queue be cleared before adding?
| offset | unsigned integer                            | If this method is responsible for setting the current `Song`, should we start at an offset within the `Artist`? e.g, starting at the first `Song` would be offset `0`, starting at the 3rd `Song` would be offset `2`, etc.

#### `append`
If using `"append": index`, you must also specific the index like so:
```json
"append":{"index":0},
```
compared to `front` and `back`:
```json
"append":"front",
```
```json
"append":"back",
```

#### `offset`
If this method also happens to set the current `Song` (added to empty queue, added to front, etc), this field lets you _start_ at a particular `Song` offset.

The `Song`'s before the offset will still be added, but the _current_ `Song` set will be the one at the offset.

If the offset is out of bounds, it will start at the first `Song`.

The exact ordering of the [`Artist`](../../common-objects/artist.md)'s songs and what the offsets are relative to is the same as the [object's](../../common-objects/artist.md) internal ordering: [`Album`](../../common-objects/album.md) in release order, then [`Song`](../../common-objects/song.md) track order.

For example, given `"offset": 3`:
```plaintext
# Artist's songs.
index 0 | song_1 <---/ These are still added to the queue, but..
index 1 | song_2 <--/
index 2 | song_3 <-/
index 3 | song_4 <--- We will start playing from this `Song`.
index 4 | song_5
index 5 | song_6
```

#### Outputs
| Field         | Type    | Description |
|---------------|---------|-------------|
| out_of_bounds | boolean | If the provided `offset` was equal to or greater than the amount of `Songs` by the `Artist`

#### Example Request 1
```bash
# Add to back of the queue.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_key_artist","params":{"key":"123","append":"back","clear":false,"offset":0}}'
```

#### Example Request 2
```bash
# Append at queue index 4.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_key_artist","params":{"key":"123","append":{"index":4},"clear":false,"offset":0}}'
```

#### Example Request 3
```bash
# Clear the queue, add all the `Song`'s by this `Artist`, but start at the 5th `Song` (offset 4).
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_key_artist","params":{"key":"123","append":"front","clear":true,"offset":4}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "out_of_bounds": false
  },
  "id": 0
}
```
