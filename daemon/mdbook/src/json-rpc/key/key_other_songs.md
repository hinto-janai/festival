# key_other_songs

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Input a [`Song`](/common-objects/song.md) [key](/common-objects/key.md), retrieve all [`Song`](/common-objects/song.md)'s in the same [`Album`](/common-objects/album.md).

The `Song`'s are sorted by `Track + Disc order`.

#### Inputs

| Field | Type                                         | Description |
|-------|----------------------------------------------|-------------|
| key   | `Song` key (unsigned integer)                | See [`Key`](/common-objects/key.md)

#### Outputs

| Field | Type                    | Description |
|-------|-------------------------|-------------|
| len   | unsigned integer        | How many `Song`'s there are
| songs | array of `Song` objects | See [`Song`](/common-objects/song.md)

#### Example Request
```bash
festival-cli key_other_songs --key 2444
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_other_songs","params":{"key":2444}}'
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
