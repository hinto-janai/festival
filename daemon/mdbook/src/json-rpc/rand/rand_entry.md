# rand_entry

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Access a random [`Entry`](/common-objects/entry.md).

#### Inputs

`None`

#### Outputs

| Field | Type           | Description |
|-------|----------------|-------------|
| entry | `Entry` object | See [`Entry`](/common-objects/entry.md)

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
      "path": "/home/hinto/Music/Rex Orange County/RAINBOW/SUNFLOWER.mp3",
      "key_artist": 65,
      "key_album": 237,
      "key_song": 2539,
      "artist": "Rex Orange County",
      "album": "RAINBOW",
      "song": "SUNFLOWER"
    }
  },
  "id": 0
}
```
