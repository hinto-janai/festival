# playlist_new
Create a new empty [`Playlist`](playlist.md), overwriting an existing one.

#### Inputs
| Field    | Type   | Description |
|----------|--------|-------------|
| playlist | string | The name of the new `Playlist`

#### Outputs
| Field   | Type    | Description |
|---------|---------|-------------|
| existed | boolean | If the `Playlist` already existed (and thus, was overwritten)

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_new","params":{"playlist":"my_new_playlist"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "existed": false
  },
  "id": 0
}
```
