| File/Folder    | Purpose |
|----------------|---------|
| album.rs       | Struct for an `Album` (which include compilations)
| artist.rs      | Struct for an `Artist`
| song.rs        | Struct for a `Song`
| collection.rs  | The main `Collection` struct, holding all of the above
| sort.rs        | Sorting functions for the `Collection`
| key.rs         | `CollectionKey`; Wrapper around `(usize, usize, usize)` that act as the "key" to any given `Collection` index
| slice.rs       | `CollectionSlice`; Wrapper around `VecDeque<CollectionKey>` that acts as a `Queue/Playlist` of keys
