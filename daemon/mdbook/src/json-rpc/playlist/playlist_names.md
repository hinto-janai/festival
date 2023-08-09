# playlist_names
Retrieve the names of all [`Playlist`](playlist.md)'s.

Names are sorted in [lexicographical order](https://en.wikipedia.org/wiki/Lexicographic_order).

#### Inputs
`None`

#### Outputs
This method outputs an un-named array of `string`'s.

#### Example Request
```bash
festival-cli playlist_names
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_names"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": [
    "Hello",
    "Playlist 2",
    "hello"
  ],
  "id": "festival-cli v1.0.0"
}
```
