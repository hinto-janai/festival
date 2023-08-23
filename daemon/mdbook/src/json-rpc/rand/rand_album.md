# rand_album

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Access a random [`Album`](../../common-objects/album.md) in your [`Collection`](../../common-objects/collection.md).

#### Inputs

`None`

#### Outputs

| Field | Type           | Description |
|-------|----------------|-------------|
| album | `Album` object | See [`Album`](../../common-objects/album.md)

#### Example Request
```bash
festival-cli rand_album
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"rand_album"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "album": {
      "title": "hug",
      "key": 243,
      "artist": 65,
      "release": "2016",
      "runtime": 1276,
      "song_count": 5,
      "songs": [
        2541,
        2546,
        2550,
        2554,
        2556
      ],
      "discs": 0,
      "art": 220954,
      "genre": null
    }
  },
  "id": 0
}
```
