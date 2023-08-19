# Queue
Methods for adding/removing [`Songs`](../../common-objects/song.md) to/from the queue.


### Append
These are different way you can append to the `queue`.

All `queue` methods involving appending requires one of these as input.

| Kind    | Description |
|---------|-------------|
| `front` | Append `Song`(s) to the front of the queue
| `back`  | Append `Song`(s) to the back of the queue
| `index` | Append `Song`(s) at a specific `index` in the queue. Queue index starts at `0`, so to mimic `front`, you would provide `0`.

If the `index` append is used, the separate `index` field must be non-`null` and provide an index.

The way this works is that the provided `Song`(s)'s will be inserted into the queue, starting from that index, for example:
```rust
// Our queue.
[0] Song A // <- Currently Playing.
[1] Song A
[2] Song A

// The songs we'd like to add at `index 2`
Song B
Song B
Song B

// The queue after appending.
[0] Song A // <- Currently Playing.
[1] Song A
[2] Song B // <- our songs were inserted
[3] Song B //    into the queue, starting
[4] Song B //    from index 2.
[5] Song A
```

### `offset`
All `queue` methods involving appending multiple `Song`'s has an optional input: `offset`.

If the method happens to set the current `Song` (added to empty queue, added to front, etc), this field lets you _start_ at a particular `Song` offset in the `Artist/Album/Playlist`.

The `Song`'s before the offset will still be added, but the _current_ `Song` set will be the one at the offset.

The exact ordering of the [`Artist`](../../common-objects/artist.md)'s songs and what the offsets are relative to is the same as the [object's](../../common-objects/artist.md) internal ordering: [`Album`](../../common-objects/album.md) in release order, then [`Song`](../../common-objects/song.md) track order.

Ordering for an [`Album`](../../common-objects/album.md)'s songs is by `Track + Disc order`.

Ordering for [`Playlist`](../../common-objects/playlist.md) is just their regular array order.

For example, given `"offset": 3`:
```rust
// Artist's songs.
[0] song_1 // <----/ These are still added to the queue, but..
[1] song_2 // <---/
[2] song_3 // <--/
[3] song_4 // <-/ We will start playing from this `Song`.
[4] song_5
[5] song_6
```
