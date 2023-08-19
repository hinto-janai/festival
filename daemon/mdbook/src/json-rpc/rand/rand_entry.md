# rand_entry

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Access a random [`Entry`](../../common-objects/entry.md) in your [`Collection`](../../common-objects/collection.md).

#### Inputs

`None`

#### Outputs

| Field | Type           | Description |
|-------|----------------|-------------|
| entry | `Entry` object | See [`Entry`](../../common-objects/entry.md)

#### Example Request
```bash
festival-cli rand_entry
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"rand_entry"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "entry": {
      "path": "/home/hinto/Music/カネコアヤノ/祝祭/Home Alone.flac",
      "key_artist": 65,
      "key_album": 269,
      "key_song": 2825,
      "artist": "カネコアヤノ",
      "album": "祝祭",
      "song": "Home Alone"
    }
  },
  "id": 0
}
```
