# playlist_remove

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Remove a [`Playlist`](/common-objects/playlist.md).

This method errors if `playlist` does not exist.

#### Inputs
| Field    | Type   | Description |
|----------|--------|-------------|
| playlist | string | The name of the `Playlist` to remove

#### Outputs
| Field   | Type                                                    | Description |
|---------|---------------------------------------------------------|-------------|
| len     | unsigned integer                  | The amount of [`Playlist Entry`](/common-objects/playlist.md)'s this removed `Playlist` had
| entries | array of `Playlist Entry` objects | The [`Playlist Entry`](/common-objects/playlist.md)'s of the remove `Playlist`

#### Example Request
```bash
festival-cli playlist_remove --playlist Playlist 1
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_remove","params":{"playlist":"Playlist 1"}}'
```

#### Example Response 1
The playlist existed, it was empty, and was removed:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 0,
    "entries": []
  },
  "id": 0
}
```

#### Example Response 2
The playlist existed, it contained this 1 `Playlist Entry`, and was removed:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 1,
    "entries": [
      {
        "valid": {
          "key_artist": 67,
          "key_album": 238,
          "key_song": 2588,
          "artist": "Rex Orange County",
          "album": "Apricot Princess",
          "song": "Waiting Room"
        }
      }
    ]
  },
  "id": 0
}
```
