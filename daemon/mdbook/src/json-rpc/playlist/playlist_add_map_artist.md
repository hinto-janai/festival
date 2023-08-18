# playlist_add_map_artist

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Add an [`Artist`](/common-objects/artist.md) to a [`Playlist`](/common-objects/playlist.md).

If the specified playlist does not already exist, it will be created.

This method errors if there was an `index` error.

#### Inputs
| Field    | Type                                        | Description |
|----------|---------------------------------------------|-------------|
| artist   | string                                      | `Artist` name
| playlist | string                                      | The name of the `Playlist`
| append   | string, one of `front`, `back` or `index`   | See [`Playlist/Append`](/json-rpc/playlist/playlist.md#append)
| index    | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used


#### Outputs
| Field   | Type    | Description |
|---------|---------|-------------|
| existed | boolean | If `playlist` already existed or not
| old_len | unsigned integer | The old length of `playlist`
| new_len | unsigned integer | The new length of `playlist`

#### Example Request 1
Add to back of the playlist "Hello".
```bash
festival-cli playlist_add_map_artist --playlist Hello --artist TWICE --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_add_map_artist","params":{"playlist":"Hello","artist":"TWICE","append":"back"}}'
```

#### Example Request 2
Append at playlist index 4.
```bash
festival-cli playlist_add_map_artist --playlist Hello --artist TWICE --append index --index 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_add_map_artist","params":{"playlist":"Hello","artist":"TWICE","append":"index","index":4}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "existed": true,
    "old_len": 10,
    "new_len": 187
  },
  "id": 0
}
```
