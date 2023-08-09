# playlist_remove_entry
Remove an [`Entry`](playlist.md) from a [`Playlist`](playlist.md).

#### Inputs
If `to` already exists, it will be overwritten.

| Field    | Type             | Description |
|----------|------------------|-------------|
| playlist | string           | The name of the `Playlist`
| index    | unsigned integer | The index of the entry in the playlist"

#### Outputs
This method errors if the `playlist` does not exist.

| Field   | Type    | Description |
|---------|---------|-------------|
| existed | boolean | If the `index` existed or not

#### Example Request
Remove the 1st entry in playlist "Hello"
```bash
festival-cli playlist_remove_entry --playlist Hello --index 0 
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_remove_entry","params":{"playlist":"Hello","index":0}}'
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
