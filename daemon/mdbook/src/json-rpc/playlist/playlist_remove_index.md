# playlist_remove_index

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Remove a single [`Playlist Entry`](../../common-objects/playlist.md) from a [`Playlist`](../../common-objects/playlist.md), using its index number.

This method errors if the `playlist` does not exist or if `index` is out-of-bounds.

#### Inputs
| Field    | Type             | Description |
|----------|------------------|-------------|
| playlist | string           | The name of the `Playlist`
| index    | unsigned integer | The index of the entry in the playlist

#### Outputs
| Field | Type                    | Description |
|-------|-------------------------|-------------|
| entry | `Playlist Entry` object | The `Playlist Entry` that was removed

#### Example Request
Remove the 1st entry in playlist "Hello"
```bash
festival-cli playlist_remove_index --playlist Hello --index 0 
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_remove_index","params":{"playlist":"Hello","index":0}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "entry": {
      "valid": {
        "key_artist": 65,
        "key_album": 237,
        "key_song": 2539,
        "artist": "Rex Orange County",
        "album": "RAINBOW",
        "song": "SUNFLOWER"
      }
    }
  },
  "id": 0
}
```
