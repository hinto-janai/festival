# playlist_clone

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Clone an existing [`Playlist`](/common-objects/playlist.md) and all it's [`Entry`](/common-objects/playlist.md)'s into a new one.

#### Inputs
If `to` already exists, it will be overwritten.

| Field | Type   | Description |
|-------|--------|-------------|
| from  | string | The name of the `Playlist` to clone FROM
| to    | string | The name of the new `Playlist` to clone TO

#### Outputs
This method errors if `from` does not exist.

| Field   | Type                                                    | Description |
|---------|---------------------------------------------------------|-------------|
| entries | optional (maybe-null) array of `Playlist Entry` objects | If the `to` already existed, its [`Playlist Entry`](/common-objects/playlist.md)'s are returned, else if it didn't exist, `null`

#### Example Request
```bash
festival-cli playlist_clone --from original --to clone
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_remove","params":{"from":"original","to":"clone"}}'
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
