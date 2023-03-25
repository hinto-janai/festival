# Sort
Various sorting methods for the `Collection`

These `enum`'s just represent `Collection` fields and are used for convenience:
```rust
// These two both return the same data.
// The enum can be useful when programming frontend stuff.

collection.album_sort(AlbumSort::ReleaseArtistLexi);

collection.sort_album_release_artist_lexi;
```

| File/Folder    | Purpose |
|----------------|---------|
| sort.rs        | Sorting methods for the `Collection`
