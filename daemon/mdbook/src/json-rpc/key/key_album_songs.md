# key_album_songs

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Input an [`Album`](../../common-objects/album.md) [key](../../common-objects/key.md), retrieve all of its [`Song`](../../common-objects/song.md)'s.

The `Song`'s are sorted by `Track + Disc order`.

#### Inputs

| Field | Type                                           | Description |
|-------|------------------------------------------------|-------------|
| key   | `Artist` key (unsigned integer)                | See [`Key`](../../common-objects/key.md)

#### Outputs

| Field | Type                    | Description |
|-------|-------------------------|-------------|
| len   | unsigned integer        | How many `Song`'s there are
| songs | array of `Song` objects | See [`Song`](../../common-objects/song.md)

#### Example Request
```bash
festival-cli key_album_songs --key 222
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_album_songs","params":{"key":222}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "songs": [
      {
        "title": "SUNFLOWER",
        "key": 2444,
        "album": 222,
        "runtime": 252,
        "sample_rate": 44100,
        "track": 1,
        "disc": null,
        "mime": "audio/mpeg",
        "extension": "mp3"
      },
      {
        "title": "BEST FRIEND",
        "key": 2398,
        "album": 222,
        "runtime": 262,
        "sample_rate": 44100,
        "track": 2,
        "disc": null,
        "mime": "audio/mpeg",
        "extension": "mp3"
      }
    ]
  },
  "id": 0
}
```
