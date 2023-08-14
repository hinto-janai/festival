# map_entry

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Input an `Artist` name, `Album` title, and `Song` title, retrieve an [`Entry`](/common-objects/entry.md).

#### Inputs

| Field  | Type   | Description |
|--------|--------|-------------|
| artist | string | `Artist` name
| album  | string | `Album` title
| song   | string | `Song` title

#### Outputs

| Field | Type           | Description |
|-------|----------------|-------------|
| entry | `Entry` object | See [`Entry`](/common-objects/entry.md)

#### Example Request
```bash
festival-cli map_entry --artist "Rex Orange County" --album RAINBOW --song SUNFLOWER
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"map_entry","params":{"artist":"Rex Orange County","album":"RAINBOW","song":"SUNFLOWER"}}'
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
