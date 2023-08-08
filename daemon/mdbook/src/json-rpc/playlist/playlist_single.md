# playlist_single
Retreive a single [`Playlist`](playlist.md).

This method errors if the specified playlist does not exist.

#### Inputs
| Field    | Type   | Description |
|----------|--------|-------------|
| playlist | string | The name of the `Playlist`

#### Outputs
| Field     | Type                                    | Description |
|-----------|-----------------------------------------|-------------|
| playlist  | string                                  | The name of the `Playlist`
| all_valid | boolean                                 | If all the `Entry`'s are valid
| len       | unsigned integer                        | How many `Entry`'s there are in this `Playlist`
| valid     | unsigned integer                        | How many `Entry`'s are `valid`
| invalid   | unsigned integer                        | How many `Entry'`s are `invalid`
| entries   | array of [`Entry`](playlist.md) objects | The `Entry`'s themselves

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_single","params":{"playlist":"Hello"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "playlist": "Hello",
    "all_valid": false,
    "len": 2,
    "valid": 1,
    "invalid": 1,
    "entries": [
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
  },
  "id": 0
}
```
