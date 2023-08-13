# playlist_single

{{#include ../../marker/s}} v1.0.0`

---

Retrieve a single [`Playlist`](playlist.md).

This method errors if the specified playlist does not exist.

#### Inputs
| Field    | Type   | Description |
|----------|--------|-------------|
| playlist | string | The name of the `Playlist`

#### Outputs
| Field       | Type                                             | Description |
|-------------|--------------------------------------------------|-------------|
| playlist    | string                                           | The name of the `Playlist`
| all_valid   | boolean                                          | If all the `Playlist Entry`'s are valid
| entry_len   | unsigned integer                                 | How many `Playlist Entry`'s there are in this `Playlist`
| valid_len   | unsigned integer                                 | How many `Playlist Entry`'s are `valid`
| invalid_len | unsigned integer                                 | How many `Playlist Entry'`s are `invalid`
| entries     | array of [`Playlist Entry`](playlist.md) objects | The `Playlsit Entry`'s themselves

#### Example Request
```bash
festival-cli playlist_single --playlist Hello 
```
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
    "entry_len": 2,
    "valid_len": 1,
    "invalid_len": 1,
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
