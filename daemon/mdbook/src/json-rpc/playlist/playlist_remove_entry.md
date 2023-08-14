# playlist_remove_entry

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Remove an [`Entry`](/common-objects/playlist.md) from a [`Playlist`](/common-objects/playlist.md).

#### Inputs
| Field    | Type             | Description |
|----------|------------------|-------------|
| playlist | string           | The name of the `Playlist`
| index    | unsigned integer | The index of the entry in the playlist

#### Outputs
This method errors if the `playlist` does not exist.

| Field | Type                                          | Description |
|-------|-----------------------------------------------|-------------|
| entry | optional (maybe-null) `Playlist Entry` object | If the `Entry` at `index` existed it is returned, else if it didn't exist, `null`

#### Example Request
Remove the 1st entry in playlist "Hello"
```bash
festival-cli playlist_remove_entry --playlist Hello --index 0 
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_remove_entry","params":{"playlist":"Hello","index":0}}'
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
