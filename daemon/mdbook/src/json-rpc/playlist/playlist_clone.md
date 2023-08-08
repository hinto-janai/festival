# playlist_clone
Clone an existing [`Playlist`](playlist.md) and all it's [`Entry`](playlist.md)'s into a new one.

#### Inputs
If `to` already exists, it will be overwritten.

| Field | Type   | Description |
|-------|--------|-------------|
| from  | string | The name of the `Playlist` to clone FROM
| to    | string | The name of the new `Playlist` to clone TO

#### Outputs
This method errors if `from` does not exist.

| Field   | Type    | Description |
|---------|---------|-------------|
| existed | boolean | If the `to` playlist already existed or not

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_remove","params":{"from":"original","to":"clone"}}'
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
