# playlist_all
Retreive _all_ [`Playlist`](playlist.md)'s.

Playlists are sorted by their name's [lexicographical order](https://en.wikipedia.org/wiki/Lexicographic_order).

#### Inputs
`None`

#### Outputs
| Field         | Type                                     | Description |
|---------------|------------------------------------------|-------------|
| all_valid     | boolean                                  | If every [`Entry`](playlist.md) is valid
| playlist_len  | unsigned integer                         | How many `Playlist`'s there are
| entry_len     | unsigned integer                         | How many total `Entry`'s there are
| valid         | unsigned integer                         | How many `Entry`'s are `valid`
| invalid       | unsigned integer                         | How many `Entry'`s are `invalid`
| playlists     | map of [`Playlist`](playlist.md) objects | The map's field keys are `string`'s, the playlist names themselves

#### Example Request
```bash
festival-cli playlist_all
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_all"}'
```

#### Example Response 1
```json
{
  "jsonrpc": "2.0",
  "result": {
    "all_valid": false,
    "playlist_len": 2,
    "entry_len": 3,
    "valid": 2,
    "invalid": 1,
    "playlists": { // <--- Note the '{' not '[' - this is a MAP not an ARRAY
      "hello": [ // <--- This Playlist's name is "hello"
        {
          "valid": {
            "key_artist": 46,
            "key_album": 168,
            "key_song": 1762,
            "artist": "Artist Name",
            "album": "Album Title",
            "song": "Song Title"
          }
        }
      ],
      "hmm": [ // <--- This Playlist's name is "hmm"
        {
          "invalid": {
            "artist": "Artist Name",
            "album": "Album Title",
            "song": "Song Title"
          }
        },
        {
          "valid": {
            "key_artist": 46,
            "key_album": 168,
            "key_song": 1762,
            "artist": "Artist Name",
            "album": "Album Title",
            "song": "Song Title"
          }
        }
      ]
    }
  },
  "id": 0
}
```

#### Example Response 2
If there are no playlists at all:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "all_valid": true,
    "playlist_len": 0,
    "entry_len": 0,
    "valid": 0,
    "invalid": 0,
    "playlists": {} // Empty MAP, not array `[]`
  },
  "id": 0
}
```
