## Collection
The main data structure that holds all the (meta)data about the user's music.

| File/Folder    | Purpose |
|----------------|---------|
| art.rs         | Wrapper around frontend-specific image formats for the `Album` art
| album.rs       | Struct for an `Album` (which include compilations)
| artist.rs      | Struct for an `Artist`
| song.rs        | Struct for a `Song`
| collection.rs  | The main `Collection` struct, holding all of the above
| sort.rs        | Sorting functions for the `Collection`
| key.rs         | `Key`; Wrappers around `usize` that act as the "key" to any given `Collection` index
| slice.rs       | `Slice`; Wrapper around `VecDeque<Key>` that acts as a `Queue/Playlist` of keys
| map.rs         | The "Map", a `HashMap` that holds `String` keys to the `Collection` instead of indicies

## Slice
`Queue` and `Playlist`

Both `Queue` and `Playlist` are practically the same thing:

- A slice of the `Collection`

They contain a bunch of `Key`'s that point
to "segments" of the `Collection` (it's a slice).

Both `Queue` and `Playlist` inner values are `VecDeque<Key>`.
