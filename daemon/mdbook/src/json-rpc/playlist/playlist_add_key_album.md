# playlist_add_key_album

{{#include ../../marker/s}} v1.0.0`

---

Add an [`Album`](../../common-objects/album.md) to a [`Playlist`](playlist.md) with a [`Key`](../../common-objects/key.md).

If the specified playlist does not already exist, it will be created.

#### Inputs
| Field    | Type                                        | Description |
|----------|---------------------------------------------|-------------|
| playlist | string                                      | The name of the `Playlist`
| key      | Album key (unsigned integer)                | `Album` key
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
festival-cli playlist_add_key_album --playlist Hello --key 0 --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_add_key_album","params":{"playlist":"Hello","key":0,"append":"back"}}'
```

#### Example Request 2
Append at playlist index 4.
```bash
festival-cli playlist_add_key_album --playlist Hello --key 0 --append index --index 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_add_key_album","params":{"playlist":"Hello","key":0,"append":"index","index":4}}'
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