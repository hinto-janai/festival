# playlist_new

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Create a new empty [`Playlist`](../../common-objects/playlist.md), overwriting an existing one.

#### Inputs
| Field    | Type   | Description |
|----------|--------|-------------|
| playlist | string | The name of the new `Playlist`

#### Outputs
| Field   | Type                                                    | Description |
|---------|---------------------------------------------------------|-------------|
| len     | optional (maybe-null) unsigned integer                  | If the `Playlist` existed (and thus, overwritten), the amount of [`Playlist Entry`](../../common-objects/playlist.md)'s it had is returned, else if it didn't exist, `null`
| entries | optional (maybe-null) array of `Playlist Entry` objects | If the `Playlist` existed (and thus, overwritten), its [`Playlist Entry`](../../common-objects/playlist.md)'s are returned, else if it didn't exist, `null`

#### Example Request
```bash
festival-cli playlist_new --playlist "Playlist 1"
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_new","params":{"playlist":"Playlist 1"}}'
```

#### Example Response 1
The playlist did not previously exist:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": null
    "entries": null
  },
  "id": 0
}
```

#### Example Response 2
The playlist previously existed, it was empty, and was overwritten:
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

#### Example Response 3
The playlist previously existed, it contained this 1 `Playlist Entry`, and was overwritten:
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
