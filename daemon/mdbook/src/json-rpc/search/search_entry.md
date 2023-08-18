# search_entry

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Input a `string`, retrieve an array of [`Song`](/common-objects/song.md)'s (in [`Entry`](/common-objects/entry.md) form), sorted by how similar their titles are to the input.

#### Inputs

| Field | Type                                                                     | Description |
|-------|--------------------------------------------------------------------------|-------------|
| input | string                                                                   | The string to match against, to use as input
| kind  | string, one of `all`, `sim60`, `sim70`, `sim80`, `top25`, `top5`, `top1` | See [`Search/Kind`](/json-rpc/search/index.md#Kind)

#### Outputs

| Field   | Type           | Description |
|---------|----------------|-------------|
| entries | `Entry` object | See [`Entry`](/common-objects/entry.md)

#### Example Request
```bash
festival-cli search_entry --input time --kind top1
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search_entry","params":{"input":"time","kind":"top1"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "entries": [
      {
        "path": "/home/hinto/Music/Kero Kero Bonito/Time 'n' Place/Time Today.flac",
        "key_artist": 148,
        "key_album": 665,
        "key_song": 6768,
        "artist": "Kero Kero Bonito",
        "album": "Time 'n' Place",
        "song": "Time Today"
      }
    ]
  },
  "id": 0
}
```
