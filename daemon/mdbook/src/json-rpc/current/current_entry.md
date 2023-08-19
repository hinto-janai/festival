# current_entry

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Access the currently set [`Song`](../../common-objects/song.md), in [`Entry`](../../common-objects/entry.md) form.

#### Inputs

`None`

#### Outputs

| Field | Type           | Description |
|-------|----------------|-------------|
| entry | `Entry` object | See [`Entry`](../../common-objects/entry.md)

#### Example Request
```bash
festival-cli current_entry
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"current_entry"}'
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
