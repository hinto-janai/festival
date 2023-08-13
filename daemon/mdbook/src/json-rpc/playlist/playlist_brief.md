# playlist_brief

{{#include ../../marker/s}} v1.0.0`

---

Retrieve the names of all [`Playlist`](playlist.md)'s.

Names are sorted in [lexicographical order](https://en.wikipedia.org/wiki/Lexicographic_order).

#### Inputs
`None`

#### Outputs
| Field     | Type                | Description |
|-----------|---------------------|-------------|
| len       | unsigned integer    | How many `Playlist`'s there are
| playlists | array of `string`'s | The names of all `Playlist`'s

#### Example Request
```bash
festival-cli playlist_brief
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_brief"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 3,
    "playlists": [
      "Playlist A",
      "Playlist B",
      "Playlist C"
    ]
  },
  "id": 0
}
```
