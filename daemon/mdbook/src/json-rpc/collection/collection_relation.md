# collection_relation
Retrieve an array of every `Song` in the current [`Collection`](../../common-objects/collection.md), with its relational data.

This will return an empty array (`"result": []`) if the `Collection` is empty.

This is a subset of the [`collection_relation_full`](collection_relation_full.md) method.

See that method if you'd like to retrieve the `Song`'s PATH alongside this method.

#### Inputs
`None`

#### Outputs
The output is an un-named array containing:

| Field      | Type             | Description |
|------------|------------------|-------------|
| artist     | string           | The [`Artist`](../../common-objects/artist.md) name
| album      | string           | The [`Album`](../../common-objects/album.md) title
| song       | string           | The [`Song`](../../common-objects/song.md) title
| key_artist | unsigned integer | The `Artist` [key](../../common-objects/key.md)
| key_album  | unsigned integer | The `Album` [key](../../common-objects/key.md)
| key_song   | unsigned integer | The `Song` [key](../../common-objects/key.md)

#### Example Request
```bash
festival-cli collection_relation
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_relation"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": [
    {
      "artist": "artist_1",
      "album": "album_1",
      "song": "mp3",
      "key_artist": 0,
      "key_album": 0,
      "key_song": 0
    },
    {
      "artist": "artist_1",
      "album": "album_1",
      "song": "mp3",
      "key_artist": 0,
      "key_album": 0,
      "key_song": 1
    },
    {
      "artist": "artist_1",
      "album": "album_2",
      "song": "mp3",
      "key_artist": 0,
      "key_album": 1,
      "key_song": 2
    },
    {
      "artist": "artist_1",
      "album": "album_2",
      "song": "flac",
      "key_artist": 0,
      "key_album": 1,
      "key_song": 3
    },
    {
      "artist": "artist_2",
      "album": "album_3",
      "song": "m4a",
      "key_artist": 1,
      "key_album": 2,
      "key_song": 4
    },
    {
      "artist": "artist_2",
      "album": "album_3",
      "song": "song_6",
      "key_artist": 1,
      "key_album": 2,
      "key_song": 5
    },
    {
      "artist": "artist_3",
      "album": "album_4",
      "song": "mp3",
      "key_artist": 2,
      "key_album": 3,
      "key_song": 6
    }
  ],
  "id": 0
}
```
