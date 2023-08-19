# playlist_clone

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Clone an existing [`Playlist`](../../common-objects/playlist.md) and all it's [`Entry`](../../common-objects/playlist.md)'s into a new one.

This method errors if `from` does not exist.

If `to` already exists, it will be overwritten.

#### Inputs
| Field | Type   | Description |
|-------|--------|-------------|
| from  | string | The name of the `Playlist` to clone FROM
| to    | string | The name of the new `Playlist` to clone TO

#### Outputs
| Field   | Type                                                    | Description |
|---------|---------------------------------------------------------|-------------|
| len     | optional (maybe-null) unsigned integer                  | If `to` already existed (and thus, overwritten), the amount of [`Playlist Entry`](../../common-objects/playlist.md)'s it had is returned, else if it didn't exist, `null`
| entries | optional (maybe-null) array of `Playlist Entry` objects | If `to` already existed, its [`Playlist Entry`](../../common-objects/playlist.md)'s are returned, else if it didn't exist, `null`

#### Example Request
```bash
festival-cli playlist_clone --from original --to clone
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_remove","params":{"from":"original","to":"clone"}}'
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
