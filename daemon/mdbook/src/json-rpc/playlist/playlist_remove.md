# playlist_remove

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Remove a [`Playlist`](/common-objects/playlist.md).

Does nothing if the `Playlist` did not exist.

#### Inputs
| Field    | Type   | Description |
|----------|--------|-------------|
| playlist | string | The name of the `Playlist` to remove

#### Outputs
| Field   | Type                                                    | Description |
|---------|---------------------------------------------------------|-------------|
| entries | optional (maybe-null) array of `Playlist Entry` objects | If the `Playlist` existed, its [`Playlist Entry`](/common-objects/playlist.md)'s are returned, else if it didn't exist, `null`

#### Example Request
```bash
festival-cli playlist_remove --playlist new
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_remove","params":{"playlist":"new"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "entries": [
      {
        "valid": {
          "key_artist": 65,
          "key_album": 237,
          "key_song": 2539,
          "artist": "Rex Orange County",
          "album": "RAINBOW",
          "song": "SUNFLOWER"
        }
      }
    ]
  },
  "id": 0
}
```
