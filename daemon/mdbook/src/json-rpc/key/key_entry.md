# key_entry

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Input a `Song` [key](../../common-objects/key.md), retrieve an [`Entry`](../../common-objects/entry.md).

#### Inputs

| Field | Type                                           | Description |
|-------|------------------------------------------------|-------------|
| key   | `Song` key (unsigned integer)                  | See [`Key`](../key.md)

#### Outputs

| Field | Type           | Description |
|-------|----------------|-------------|
| entry | `Entry` object | See [`Entry`](../../common-objects/entry.md)

#### Example Request
```bash
festival-cli key_entry --key 5151
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_entry","params":{"key":5151}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "entry": {
      "path": "/home/hinto/Music/song.flac",
      "key_artist": 108,
      "key_album": 488,
      "key_song": 5151,
      "artist": "Artist Name",
      "album": "Album Title",
      "song": "Song Title"
    }
  },
  "id": 0
}
```
