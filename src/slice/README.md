# Slice
`Queue` and `Playlist`

Both `Queue` and `Playlist` are practically the same thing:

- A slice of the `Collection`

They contain a bunch of `Key`'s that point
to "segments" of the `Collection` (it's a slice).

Both `Queue` and `Playlist` inner values are `VecDeque<Key>`.

| File/Folder    | Purpose |
|----------------|---------|
| slice.rs       | `Slice`; Wrapper around `VecDeque<Key>` that acts as a `Queue/Playlist` of keys
