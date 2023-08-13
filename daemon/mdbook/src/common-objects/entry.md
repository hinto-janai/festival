# Entry

| Field      | Type                            | Description |
|------------|---------------------------------|-------------|
| path       | string (PATH)                   | The PATH of this `Song` on the filesystem `festivald` is running on
| key_artist | `Artist` key (unsigned integer) | The `Artist` key
| key_album  | `Album` key (unsigned integer)  | The `Album` key
| key_song   | `Song` key (unsigned integer)   | This `Song`'s key
| artist     | string                          | The `Artist` name
| album      | string                          | The `Album` title
| song       | string                          | This `Song`'s title

#### Example
```json
{
  "path": "/home/hinto/Music/song.mp3",
  "key_artist": 0,
  "key_album": 0,
  "key_song": 0,
  "artist": "Artist Name",
  "album": "Album Title",
  "song": "Song Title"
}
```
