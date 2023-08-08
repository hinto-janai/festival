# playlist_count
Retreive the count of all [`Playlist`](playlist.md)'s.

#### Inputs
`None`

#### Outputs
| Field | Type             | Description |
|-------|------------------|-------------|
| count | unsigned integer | How many `Playlist`'s there are

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_count"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "count": 3
  },
  "id": "festival-cli v1.0.0"
}
```
