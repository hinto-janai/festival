# Playlist

#### 🟢 Stable
This API is [stable](../api-stability/marker.md) since `festivald v1.0.0`.

---

`Playlist`'s are objects where
- The key _is_ the playlist name (`string`)
- The value is an array of [`Entry`](entry.md)-like objects, `Playlist Entries`

| Field          | Type                        | Description |
|----------------|-----------------------------|-------------|
| $PLAYLIST_NAME | array of `Playlist Entry`'s | The name of the playlist as the key, and the `Song` entries in an array

## Playlist Entry
The `Entry` object in the context of playlists is similar to [`Entry`](entry.md), except it does not have the `path` field.

The `Playlist Entry` takes 2 forms: `valid` & `invalid`:

| `Playlist Entry` form | Meaning |
|-----------------------|---------|
| `valid`               | This is a valid `Song`; it exists within the current `Collection`
| `invalid`             | This `Song` does _NOT_ exist in the current `Collection`, although it did in a previous one (it may have been lost in-between `Collection` resets)

These objects will appear in playlist-related methods, like [`playlist_full`](../json-rpc/playlist/playlist_full.md) & [`playlist_single`](../json-rpc/playlist/playlist_single.md).

Example:
```json
{
  "hello": [ // <--- the playlist name is "hello"
    // This is an "valid" entry.
    // The underlying Song exists in the current Collection.
    {
      "valid": {
        "key_artist": 46,
        "key_album": 168,
        "key_song": 1756,
        "artist": "Artist Name",
        "album": "Album Title",
        "song": "Song Title"
      }
    },
    // This is an "invalid" entry.
    // The underlying Song doesn't exist in Collection anymore.
    {
      "invalid": {
        "artist": "Artist Name",
        "album": "Album Title",
        "song": "Song Title"
      }
    }
  ]
}
```

### `valid`
The data definition for `valid` playlist entry types.

Exact same as [`Entry`](entry.md) but does not have `path`.

| Field      | Type                          | Description |
|------------|-------------------------------|-------------|
| key_artist | Artist key (unsigned integer) | This entry's [`Artist key`](key.md)
| key_album  | Album key (unsigned integer)  | This entry's [`Album key`](key.md)
| key_song   | Song key (unsigned integer)   | This entry's [`Song key`](key.md)
| artist     | string                        | The [`Artist` name](artist.md)
| album      | string                        | The [`Album` title](album.md)
| song       | string                        | The [`Song` title](song.md)

Example:
```json
{
  "key_artist": 46,
  "key_album": 168,
  "key_song": 1762,
  "artist": "Artist Name",
  "album": "Album Title",
  "song": "Song Title"
}
```

### `invalid`
The data definition for `invalid` playlist entry types is the same as `valid`, but with no keys.

| Field  | Type                          | Description |
|--------|-------------------------------|-------------|
| artist | string                        | The [`Artist` name](artist.md)
| album  | string                        | The [`Album` title](album.md)
| song   | string                        | The [`Song` title](song.md)

Example:
```json
{
  "artist": "Artist Name",
  "album": "Album Title",
  "song": "Song Title"
}
```

## Validation
Upon a [`Collection`](collection.md) reset, it is not guaranteed that the same [`Song`](song.md)'s will even exist at all.

This is a problem since existing `Playlist`'s may now be referencing `Song`'s that don't exist anymore.

The way `festivald` handles this is _not_ to delete them, but "mark" those entries as `invalid`.

These `invalid` entries will continue to exist indefinitely.

Upon every future `Collection` reset, if an `invalid` entry is able to recover (a matching `Song` file is found with the correct metadata), it will turn back into a `valid` entry automatically.
