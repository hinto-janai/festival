# playlist_add_map_artist
Add an [`Artist`](../../common-objects/artist.md) to a [`Playlist`](playlist.md).

If the specified playlist does not already exist, it will be created.

#### Inputs
| Field    | Type                                        | Description |
|----------|---------------------------------------------|-------------|
| playlist | string                                      | The name of the `Playlist`
| artist   | string                                      | `Artist` name
| append   | string, one of `front`, `back` or `index`   | In which way should we add to the playlist? `front` means to the front of the playlist. `back` means to the back. `index` means at an exact playlist index. Playlist index starts at `0`, so to mimic `front`, you would provide `0`.
| index    | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used


#### Outputs
This method errors if there was a `index` error.

| Field   | Type    | Description |
|---------|---------|-------------|
| existed | boolean | If `playlist` already existed or not

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
    "existed": true
  },
  "id": 0
}
```
