# playlist_new

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Create a new empty [`Playlist`](/common-objects/playlist.md), overwriting an existing one.

#### Inputs
| Field    | Type   | Description |
|----------|--------|-------------|
| playlist | string | The name of the new `Playlist`

#### Outputs
| Field   | Type                                                    | Description |
|---------|---------------------------------------------------------|-------------|
| entries | optional (maybe-null) array of `Playlist Entry` objects | If the `Playlist` existed (and thus, overwritten), its [`Playlist Entry`](/common-objects/playlist.md)'s are returned, else if it didn't exist, `null`

#### Example Request
```bash
festival-cli playlist_new --playlist my_new_playlist
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_new","params":{"playlist":"my_new_playlist"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": null
  },
  "id": 0
}
```
