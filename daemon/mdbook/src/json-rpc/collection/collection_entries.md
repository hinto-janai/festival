# collection_entries

{{#include ../../marker/s}} v1.0.0`

---

Retrieve an array of [`Entry`](../../common-objects/entry.md)'s of every `Song` in the current [`Collection`](../../common-objects/collection.md).

This will return an empty array (`"result": []`) if the `Collection` is empty.

#### Inputs
`None`

#### Outputs
| Field   | Type                     | Description |
|---------|--------------------------|-------------|
| len     | unsigned integer         | How many `Entry`'s (`Song`'s) there are
| entries | array of `Entry` objects | Every `Song` in the `Collection` (in `Entry` form)

#### Example Request
```bash
festival-cli collection_entries
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_entries"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": [
    {
      "path": "/home/hinto/festival/assets/audio/song_1.mp3",
      "key_artist": 0,
      "key_album": 0,
      "key_song": 0,
      "artist": "artist_1",
      "album": "album_1",
      "song": "mp3"
    },
    {
      "path": "/home/hinto/festival/assets/audio/song_2.mp3",
      "key_artist": 0,
      "key_album": 0,
      "key_song": 1,
      "artist": "artist_1",
      "album": "album_1",
      "song": "mp3"
    },
    {
      "path": "/home/hinto/festival/assets/audio/song_3.mp3",
      "key_artist": 0,
      "key_album": 1,
      "key_song": 2,
      "artist": "artist_1",
      "album": "album_2",
      "song": "mp3"
    },
    {
      "path": "/home/hinto/festival/assets/audio/song_4.flac",
      "key_artist": 0,
      "key_album": 1,
      "key_song": 3,
      "artist": "artist_1",
      "album": "album_2",
      "song": "flac"
    },
    {
      "path": "/home/hinto/festival/assets/audio/song_5.m4a",
      "key_artist": 1,
      "key_album": 2,
      "key_song": 4,
      "artist": "artist_2",
      "album": "album_3",
      "song": "m4a"
    },
    {
      "path": "/home/hinto/festival/assets/audio/song_6.ogg",
      "key_artist": 1,
      "key_album": 2,
      "key_song": 5,
      "artist": "artist_2",
      "album": "album_3",
      "song": "song_6"
    },
    {
      "path": "/home/hinto/festival/assets/audio/song_7.mp3",
      "key_artist": 2,
      "key_album": 3,
      "key_song": 6,
      "artist": "artist_3",
      "album": "album_4",
      "song": "mp3"
    }
  ],
  "id": 0
}
```
