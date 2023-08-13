# playlist_remove

{{#include ../../marker/s}} v1.0.0`

---

Remove a [`Playlist`](playlist.md).

Does nothing if the `Playlist` did not exist.

#### Inputs
| Field    | Type   | Description |
|----------|--------|-------------|
| playlist | string | The name of the `Playlist` to remove

#### Outputs
| Field   | Type    | Description |
|---------|---------|-------------|
| existed | boolean | If the `Playlist` existed or not

#### Example Request
```bash
festival-cli playlist_remove --playlist new
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_remove","params":{"playlist":"new"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "existed": true
  },
  "id": 0
}
```
