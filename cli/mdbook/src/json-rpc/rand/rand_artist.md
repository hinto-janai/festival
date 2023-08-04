# rand_artist
Access a random [`Artist`](../../common-objects/artist.md).

#### Inputs

`None`

#### Outputs

| Field  | Type            | Description |
|--------|-----------------|-------------|
| artist | `Artist` object | See [`Artist`](../../common-objects/artist.md)

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"rand_artist"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "artist": {
      "name": "Rex Orange County",
      "key": 65,
      "runtime": 7583,
      "albums": [
        237
      ],
      "songs": [
        2800,
        2803,
        2809
      ]
    }
  },
  "id": 0
}
```
